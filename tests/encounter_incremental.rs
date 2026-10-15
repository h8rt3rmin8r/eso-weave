use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    canonical_native_bytes, logical_native_identity, parse_native_log, project_native_encounter,
    IncrementalNativeParser, NativeCheckpoint, NativeDiagnosticCode, NativeEncounter,
    NativeParserError, NativeParserStatus, NativeTerminationReason, MAX_NATIVE_FEED_BYTES,
};

fn source(epoch: u64) -> String {
    format!(
        "0,BEGIN_LOG,{epoch},15,\"FixtureRealm\",\"zz\",\"fixture.game\"\r\n\
         0,UNIT_ADDED,1,PLAYER,T,0,1,F,1,1,\"Canary, Ω\",\"@Fixture\",1,1,1,0,PLAYER_ALLY,F\r\n\
         1,ABILITY_INFO,100,\"Quoted \\\"ability\\\"\",\"fixture.dds\",F,F\r\n\
         2,PLAYER_INFO,1,[100],[1],[[HEAD,1,F,1,NONE,NORMAL,0,NONE,F,0,NORMAL]],[100],[]\r\n\
         10,BEGIN_COMBAT\r\n\
         20,COMBAT_EVENT,DAMAGE,PHYSICAL,STAMINA,300,7,10,100,1,100/100,100/100,100/100,0/100,0/0,0,0.25,0.25,0,*\r\n\
         30,END_COMBAT\r\n\
         40,END_LOG\r\n"
    )
}

fn feed(parser: &mut IncrementalNativeParser, bytes: &[u8]) -> Vec<NativeEncounter> {
    let mut offset = 0;
    let mut result = Vec::new();
    while offset < bytes.len() {
        let end = (offset + MAX_NATIVE_FEED_BYTES).min(bytes.len());
        let output = parser.feed(&bytes[offset..end]).unwrap();
        assert!(output.accepted > 0);
        offset += output.accepted;
        if let Some(completed) = output.completed {
            assert_eq!(
                completed.logical_identity,
                logical_native_identity(&completed.encounter).unwrap()
            );
            result.push(completed.encounter);
        }
    }
    result
}

fn parse(bytes: &[u8]) -> Vec<NativeEncounter> {
    feed(&mut IncrementalNativeParser::new(Channel::Live), bytes)
}

#[test]
fn every_byte_partition_preserves_original_evidence_context_and_projection() {
    let source = source(1_700_000_000_000);
    let baseline = parse(source.as_bytes());
    assert_eq!(baseline.len(), 1);
    assert_eq!(baseline[0].schema_version, 2);
    for split in 0..=source.len() {
        let mut parser = IncrementalNativeParser::new(Channel::Live);
        let mut actual = feed(&mut parser, &source.as_bytes()[..split]);
        actual.extend(feed(&mut parser, &source.as_bytes()[split..]));
        assert_eq!(actual, baseline, "split {split}");
        assert_eq!(parser.position().accepted_bytes, source.len() as u64);
        assert_eq!(parser.position().complete_bytes, source.len() as u64);
    }
    let old = parse_native_log(source.as_bytes(), Channel::Live)
        .unwrap()
        .encounters
        .remove(0);
    assert_eq!(old.records, baseline[0].records);
    assert_eq!(old.context, baseline[0].context);
    assert_eq!(
        logical_native_identity(&old).unwrap(),
        logical_native_identity(&baseline[0]).unwrap()
    );
    let old_projection = project_native_encounter(&old).unwrap();
    let new_projection = project_native_encounter(&baseline[0]).unwrap();
    assert_eq!(old_projection.damage_hit, new_projection.damage_hit);
    assert_eq!(
        old_projection.damage_overflow,
        new_projection.damage_overflow
    );
    assert_eq!(new_projection.local_damage_hit, Some(300));
}

#[test]
fn completed_fight_is_returned_before_end_log_and_suffix_is_unaccepted() {
    let source = source(1_700_000_000_000);
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let output = parser.feed(source.as_bytes()).unwrap();
    assert!(output.completed.is_some());
    assert_eq!(&source.as_bytes()[output.accepted..], b"40,END_LOG\r\n");
    assert_eq!(output.status, NativeParserStatus::ReadingRun);
    let before = parser.checkpoint().unwrap();
    for _ in 0..4 {
        let empty = parser.feed(&[]).unwrap();
        assert_eq!(empty.accepted, 0);
        assert!(empty.completed.is_none());
        assert!(empty.diagnostics.is_empty());
        assert_eq!(parser.checkpoint().unwrap().as_bytes(), before.as_bytes());
    }
}

#[test]
fn checkpoint_after_each_byte_resumes_pending_utf8_context_and_fight_exactly() {
    let source = source(1_700_000_000_000);
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let mut actual = Vec::new();
    for byte in source.as_bytes() {
        actual.extend(feed(&mut parser, &[*byte]));
        let encoded = parser.checkpoint().unwrap();
        let decoded = NativeCheckpoint::from_bytes(encoded.as_bytes()).unwrap();
        assert_eq!(decoded.channel(), Channel::Live);
        assert_eq!(decoded.position(), parser.position());
        parser = IncrementalNativeParser::restore(&decoded, Channel::Live).unwrap();
    }
    assert_eq!(actual, parse(source.as_bytes()));
}

#[test]
fn checkpoint_corruption_version_channel_and_state_inconsistency_are_rejected() {
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(
        &mut parser,
        b"0,BEGIN_LOG,1700000000000,15,\"R\",\"zz\",\"G\"\n10,BEGIN_COMBAT\n",
    );
    let checkpoint = parser.checkpoint().unwrap();
    assert_eq!(
        IncrementalNativeParser::restore(&checkpoint, Channel::Pts).unwrap_err(),
        NativeParserError::ChannelMismatch
    );
    let value: serde_json::Value = serde_json::from_slice(checkpoint.as_bytes()).unwrap();
    for (key, replacement) in [
        ("version", serde_json::json!(999)),
        (
            "position",
            serde_json::json!({"accepted_bytes": 0, "complete_bytes": 9, "record_count": 2}),
        ),
        ("combat_ordinal", serde_json::json!(999)),
        ("unrecognized", serde_json::json!(true)),
    ] {
        let mut invalid = value.clone();
        invalid[key] = replacement;
        let bytes = serde_json::to_vec(&invalid).unwrap();
        assert!(NativeCheckpoint::from_bytes(&bytes).is_err(), "{key}");
    }
    assert!(NativeCheckpoint::from_bytes(b"{\"private\":\"Canary\"").is_err());
    let debug = format!("{checkpoint:?}");
    assert!(!debug.contains("BEGIN_LOG"));
    assert!(!debug.contains("1700000000000"));
}

#[test]
fn chunked_unknown_evidence_is_preserved_and_recovery_requires_a_fresh_valid_header() {
    let first = source(1_700_000_000_000).replace(
        "30,END_COMBAT",
        "25,FUTURE_RECORD,\"UnknownCanary\"\r\n30,END_COMBAT",
    );
    let bad = "0,BEGIN_LOG,1700000001000,15,\"R\",\"zz\",\"G\"\n10,BEGIN_COMBAT\n20,COMBAT_EVENT,PRIVATE_CANARY\n30,END_COMBAT\n31,BEGIN_COMBAT\n32,END_COMBAT\n";
    let final_run = source(1_700_000_002_000);
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let mut actual = feed(&mut parser, first.as_bytes());
    let output = parser.feed(bad.as_bytes()).unwrap();
    assert!(output.completed.is_none());
    assert_eq!(output.status, NativeParserStatus::Recovering);
    assert!(output
        .diagnostics
        .iter()
        .any(|d| d.code == NativeDiagnosticCode::MalformedKnownRecord));
    assert!(!format!("{:?}", output.diagnostics).contains("PRIVATE_CANARY"));
    actual.extend(feed(&mut parser, final_run.as_bytes()));
    assert_eq!(actual.len(), 2);
    assert_eq!(actual[0].unknown_record_count, 1);
    assert!(actual[0]
        .records
        .iter()
        .any(|r| r.line.contains("UnknownCanary")));
    assert_eq!(actual[1].run.header.epoch_ms, 1_700_000_002_000);
}

#[test]
fn unsupported_version_and_deferred_serialization_are_controlled_not_unknown() {
    for (source, expected) in [
        (source(1_700_000_000_000).replace(",15,", ",99,"), NativeDiagnosticCode::UnsupportedLogVersion),
        ("0,BEGIN_LOG,timeSinceEpochMS=1700000000000,logVersion=15,realmName=\"R\",language=\"zz\",gameVersion=\"G\"\n".into(), NativeDiagnosticCode::UnsupportedSerialization),
    ] {
        let mut parser = IncrementalNativeParser::new(Channel::Live);
        let output = parser.feed(source.as_bytes()).unwrap();
        assert!(output.completed.is_none());
        assert!(output.diagnostics.iter().any(|d| d.code == expected));
        assert_eq!(output.status, NativeParserStatus::Recovering);
    }
}

#[test]
fn feed_limit_is_atomic_and_overlong_tail_discards_boundedly_until_newline() {
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let checkpoint = parser.checkpoint().unwrap();
    assert_eq!(
        parser
            .feed(&vec![b'x'; MAX_NATIVE_FEED_BYTES + 1])
            .unwrap_err(),
        NativeParserError::FeedTooLarge
    );
    assert_eq!(
        parser.checkpoint().unwrap().as_bytes(),
        checkpoint.as_bytes()
    );
    let output = parser.feed(&vec![b'x'; MAX_NATIVE_FEED_BYTES]).unwrap();
    assert_eq!(output.accepted, MAX_NATIVE_FEED_BYTES);
    assert_eq!(output.status, NativeParserStatus::Recovering);
    assert!(parser.checkpoint().unwrap().as_bytes().len() < 4096);
    let output = parser.feed(b"\n").unwrap();
    assert!(output
        .diagnostics
        .iter()
        .any(|d| d.code == NativeDiagnosticCode::LineLimit));
    assert_eq!(
        feed(&mut parser, source(1_700_000_000_000).as_bytes()).len(),
        1
    );
}

#[test]
fn explicit_generation_termination_retains_incomplete_evidence_without_fake_fight() {
    let prefix = "0,BEGIN_LOG,1700000000000,15,\"R\",\"zz\",\"G\"\n10,BEGIN_COMBAT\n20,PARTIAL";
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(&mut parser, prefix.as_bytes());
    assert_eq!(
        parser.feed(&[]).unwrap().status,
        NativeParserStatus::ReadingCombat
    );
    let terminal = parser
        .terminate_generation(NativeTerminationReason::Replaced)
        .unwrap();
    assert_eq!(terminal.incomplete_combat_count, 1);
    assert_eq!(terminal.incomplete_run_count, 1);
    assert_eq!(terminal.unterminated_line_count, 1);
    assert_eq!(
        parser
            .terminate_generation(NativeTerminationReason::Replaced)
            .unwrap(),
        terminal
    );
    assert_eq!(
        parser.feed(b"\n").unwrap_err(),
        NativeParserError::GenerationTerminated
    );
    let checkpoint = parser.checkpoint().unwrap();
    assert_eq!(
        IncrementalNativeParser::restore(&checkpoint, Channel::Live)
            .unwrap()
            .position(),
        parser.position()
    );
}

#[test]
fn copied_and_extracted_runs_share_logical_identity_without_losing_physical_provenance() {
    let first = source(1_700_000_000_000);
    let second = source(1_700_000_100_000);
    let concatenated = format!("{first}{second}");
    let together = parse(concatenated.as_bytes());
    let alone = parse(second.as_bytes()).remove(0);
    assert_ne!(together[1].run.run_ordinal, alone.run.run_ordinal);
    assert_ne!(together[1].first_ordinal, alone.first_ordinal);
    assert_ne!(
        canonical_native_bytes(&together[1]).unwrap(),
        canonical_native_bytes(&alone).unwrap()
    );
    assert_eq!(together[1].run.session_id, alone.run.session_id);
    assert_eq!(together[1].encounter_id, alone.encounter_id);
    assert_eq!(
        logical_native_identity(&together[1]).unwrap(),
        logical_native_identity(&alone).unwrap()
    );
    assert_ne!(
        logical_native_identity(&together[0]).unwrap().run_key,
        logical_native_identity(&alone).unwrap().run_key
    );
    let legacy = parse_native_log(concatenated.as_bytes(), Channel::Live).unwrap();
    assert_eq!(
        logical_native_identity(&legacy.encounters[1]).unwrap(),
        logical_native_identity(&alone).unwrap()
    );
}

#[test]
fn schema2_physical_provenance_can_exceed_old_lifetime_coordinate_limits() {
    let mut capture = parse(source(1_700_000_000_000).as_bytes()).remove(0);
    let before = logical_native_identity(&capture).unwrap();
    let ordinal_shift = 3_000_000;
    let offset_shift = 300 * 1024 * 1024;
    capture.run.run_ordinal = 20_000;
    capture.run.header.record.ordinal += ordinal_shift;
    capture.run.header.record.byte_offset += offset_shift;
    capture.first_ordinal += ordinal_shift;
    capture.last_ordinal += ordinal_shift;
    for record in capture.context.iter_mut().chain(capture.records.iter_mut()) {
        record.ordinal += ordinal_shift;
        record.byte_offset += offset_shift;
    }
    assert!(canonical_native_bytes(&capture).is_ok());
    assert_eq!(logical_native_identity(&capture).unwrap(), before);
    assert_eq!(
        project_native_encounter(&capture).unwrap().local_damage_hit,
        Some(300)
    );
}

#[test]
fn pending_tail_does_not_decode_split_utf8_and_unfinished_run_never_finalizes_itself() {
    let source = source(1_700_000_000_000);
    let omega = source.find('Ω').unwrap();
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(&mut parser, &source.as_bytes()[..omega + 1]);
    assert!(parser.position().accepted_bytes > parser.position().complete_bytes);
    let checkpoint = parser.checkpoint().unwrap();
    let mut resumed = IncrementalNativeParser::restore(&checkpoint, Channel::Live).unwrap();
    assert_eq!(feed(&mut resumed, &source.as_bytes()[omega + 1..]).len(), 1);
}

#[test]
fn maximum_length_record_accepts_a_crlf_split_at_the_cr_byte() {
    let header = "0,BEGIN_LOG,1700000000000,15,\"R\",\"zz\",\"G\"\n";
    let prefix = "1,FUTURE_RECORD,\"";
    let line = format!("{prefix}{}\"", "x".repeat(65536 - prefix.len() - 1));
    assert_eq!(line.len(), 65536);
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(&mut parser, header.as_bytes());
    assert!(parser
        .feed(format!("{line}\r").as_bytes())
        .unwrap()
        .diagnostics
        .is_empty());
    let checkpoint = parser.checkpoint().unwrap();
    let mut parser = IncrementalNativeParser::restore(&checkpoint, Channel::Live).unwrap();
    let result = parser.feed(b"\n").unwrap();
    assert!(result.diagnostics.is_empty());
    assert_eq!(result.status, NativeParserStatus::ReadingRun);
}

#[test]
fn reversed_endpoints_count_the_interrupted_fight_and_never_emit_it() {
    let source = source(1_700_000_000_000).replace("30,END_COMBAT", "9,END_COMBAT");
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let result = parser.feed(source.as_bytes()).unwrap();
    assert!(result.completed.is_none());
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.incomplete_evidence_count == 1));
    let ended = parser
        .terminate_generation(NativeTerminationReason::Replaced)
        .unwrap();
    assert_eq!(ended.incomplete_combat_count, 1);
}

#[test]
fn checkpoint_detects_syntactically_valid_payload_corruption() {
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(
        &mut parser,
        b"0,BEGIN_LOG,1700000000000,15,\"R\",\"zz\",\"G\"\n",
    );
    let mut encoded: serde_json::Value =
        serde_json::from_slice(parser.checkpoint().unwrap().as_bytes()).unwrap();
    encoded["record_count"] = serde_json::json!(1000);
    assert!(NativeCheckpoint::from_bytes(&serde_json::to_vec(&encoded).unwrap()).is_err());
    encoded.as_object_mut().unwrap().remove("record_count");
    encoded["position"]["record_count"] = serde_json::json!(1000);
    assert!(NativeCheckpoint::from_bytes(&serde_json::to_vec(&encoded).unwrap()).is_err());
}

#[test]
fn representative_fixture_keeps_context_and_nonself_targets_across_small_chunks() {
    let source =
        include_bytes!("../specs/127-native-completed-import/fixtures/representative-v15.log");
    let baseline = parse(source);
    assert_eq!(baseline.len(), 2);
    for size in [1, 2, 7, 31, 256, 4096] {
        let mut parser = IncrementalNativeParser::new(Channel::Live);
        let mut actual = Vec::new();
        for bytes in source.chunks(size) {
            actual.extend(feed(&mut parser, bytes));
        }
        assert_eq!(actual, baseline, "chunk size {size}");
    }
}

#[test]
fn logical_stream_crosses_old_file_and_record_limits_without_lifetime_retention() {
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(
        &mut parser,
        b"0,BEGIN_LOG,1700000000000,15,\"R\",\"zz\",\"G\"\n",
    );
    let line = format!(
        "1,UNIT_CHANGED,1,1,1,\"{}\",\"\",1,1,1,0,PLAYER_ALLY,F\n",
        "x".repeat(96)
    );
    let chunk = line.repeat(1024);
    assert!(chunk.len() < MAX_NATIVE_FEED_BYTES);
    for _ in 0..2000 {
        let output = parser.feed(chunk.as_bytes()).unwrap();
        assert_eq!(output.accepted, chunk.len());
        assert!(output.completed.is_none());
        assert!(output.diagnostics.is_empty());
    }
    assert!(parser.position().record_count > 2_000_000);
    assert!(parser.position().accepted_bytes > 256 * 1024 * 1024);
    assert!(parser.checkpoint().unwrap().as_bytes().len() < 4096);
    let completed = parser
        .feed(b"10,BEGIN_COMBAT\n30,END_COMBAT\n")
        .unwrap()
        .completed
        .unwrap();
    assert!(completed.encounter.first_ordinal > 2_000_000);
    assert!(completed.encounter.records[0].byte_offset > 256 * 1024 * 1024);
    assert!(canonical_native_bytes(&completed.encounter).is_ok());
}

#[test]
fn logical_stream_crosses_old_run_count_ceiling() {
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    for ordinal in 0..10_001 {
        let source = format!(
            "0,BEGIN_LOG,{},15,\"R\",\"zz\",\"G\"\n1,END_LOG\n",
            1_700_000_000_000_u64 + ordinal
        );
        let output = parser.feed(source.as_bytes()).unwrap();
        assert!(output.diagnostics.is_empty());
    }
    let source =
        "0,BEGIN_LOG,1700000999999,15,\"R\",\"zz\",\"G\"\n10,BEGIN_COMBAT\n30,END_COMBAT\n";
    let output = parser.feed(source.as_bytes()).unwrap();
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.completed.unwrap().encounter.run.run_ordinal, 10_002);
}

#[test]
fn context_and_pending_fight_limits_recover_without_partial_completion() {
    let header = "0,BEGIN_LOG,1700000000000,15,\"R\",\"zz\",\"G\"\n";
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(&mut parser, header.as_bytes());
    let context = format!("1,FUTURE_RECORD,\"{}\"\n", "x".repeat(60_000));
    let mut exceeded = false;
    for _ in 0..150 {
        let output = parser.feed(context.as_bytes()).unwrap();
        exceeded |= output
            .diagnostics
            .iter()
            .any(|d| d.code == NativeDiagnosticCode::ContextLimit);
        assert!(output.completed.is_none());
    }
    assert!(exceeded);
    assert!(parser.checkpoint().unwrap().as_bytes().len() < 4096);
    assert_eq!(
        feed(&mut parser, source(1_700_000_100_000).as_bytes()).len(),
        1
    );

    let mut parser = IncrementalNativeParser::new(Channel::Live);
    feed(&mut parser, format!("{header}10,BEGIN_COMBAT\n").as_bytes());
    let chunk = "20,HEALTH_REGEN,1,1,1/1,1/1,1/1,0/1,0/0,0,0,0,0\n".repeat(1000);
    let mut exceeded = false;
    for _ in 0..201 {
        let output = parser.feed(chunk.as_bytes()).unwrap();
        exceeded |= output
            .diagnostics
            .iter()
            .any(|d| d.code == NativeDiagnosticCode::FightLimit);
        assert!(output.completed.is_none());
    }
    assert!(exceeded);
    assert_eq!(
        parser
            .terminate_generation(NativeTerminationReason::Replaced)
            .unwrap()
            .incomplete_combat_count,
        1
    );
}

#[test]
fn malformed_encoding_nested_boundaries_and_excessive_groups_never_emit_partial_fights() {
    let header = b"0,BEGIN_LOG,1700000000000,15,\"R\",\"zz\",\"G\"\n10,BEGIN_COMBAT\n";
    for malformed in [
        b"20,FUTURE_RECORD,\xff\n30,END_COMBAT\n".as_slice(),
        b"20,BEGIN_COMBAT\n30,END_COMBAT\n".as_slice(),
        b"20,PLAYER_INFO,1,[[[[[[[[[1]]]]]]]]],[],[],[],[]\n30,END_COMBAT\n".as_slice(),
    ] {
        let mut parser = IncrementalNativeParser::new(Channel::Live);
        feed(&mut parser, header);
        let output = parser.feed(malformed).unwrap();
        assert!(output.completed.is_none());
        assert!(!output.diagnostics.is_empty());
        assert_eq!(output.status, NativeParserStatus::Recovering);
        let checkpoint = parser.checkpoint().unwrap();
        parser = IncrementalNativeParser::restore(&checkpoint, Channel::Live).unwrap();
        assert_eq!(
            feed(&mut parser, source(1_700_000_100_000).as_bytes()).len(),
            1
        );
    }
}

#[test]
fn schema2_accepts_reviewed_scribed_ability_extension_without_changing_schema1_profile() {
    let source = source(1_700_000_000_000).replace(
        ",\"fixture.dds\",F,F",
        ",\"fixture.dds\",F,F,\"FixtureFocus\",\"FixtureSignature\",\"FixtureAffix\"",
    );
    assert!(parse_native_log(source.as_bytes(), Channel::Live).is_err());
    let result = parse(source.as_bytes());
    assert_eq!(result.len(), 1);
    assert!(result[0]
        .context
        .iter()
        .any(|r| r.line.contains("FixtureFocus")));
}

#[test]
fn split_initial_bom_preserves_logical_identity_and_original_physical_offsets() {
    let source = source(1_700_000_000_000);
    let mut bom = vec![0xef, 0xbb, 0xbf];
    bom.extend_from_slice(source.as_bytes());
    let mut parser = IncrementalNativeParser::new(Channel::Live);
    let mut actual = Vec::new();
    for bytes in bom.chunks(1) {
        actual.extend(feed(&mut parser, bytes));
    }
    assert_eq!(actual.len(), 1);
    assert_eq!(actual[0].run.header.record.byte_offset, 3);
    assert_eq!(
        logical_native_identity(&actual[0]).unwrap(),
        logical_native_identity(&parse(source.as_bytes())[0]).unwrap()
    );
    let legacy = parse_native_log(&bom, Channel::Live).unwrap();
    assert_eq!(legacy.encounters[0].records, actual[0].records);
}

#[test]
fn similar_run_header_with_different_evidence_is_a_collision_not_an_identical_copy() {
    let source = source(1_700_000_000_000);
    let original = logical_native_identity(&parse(source.as_bytes())[0]).unwrap();
    let changed =
        logical_native_identity(&parse(source.replace("300,7,10", "301,7,10").as_bytes())[0])
            .unwrap();
    assert_eq!(original.run_key, changed.run_key);
    assert_eq!(original.fight_key, changed.fight_key);
    assert_ne!(original.evidence_sha256, changed.evidence_sha256);
}
