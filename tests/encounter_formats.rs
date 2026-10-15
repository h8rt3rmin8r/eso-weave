//! Synthetic fixtures derived from S131's pinned static v15 producer writers.
//! These exercise serialization contracts, not installed-game behavior.
use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    canonical_native_bytes, logical_native_identity, project_native_encounter,
    IncrementalNativeParser, NativeCheckpoint, NativeDiagnosticCode, NativeEncounter,
};

const PROFILES: [&str; 3] = [
    include_str!("fixtures/native/normal-inline-v15.log"),
    include_str!("fixtures/native/verbose-separate-v15.log"),
    include_str!("fixtures/native/verbose-inline-v15.log"),
];

fn feed(parser: &mut IncrementalNativeParser, bytes: &[u8]) -> Vec<NativeEncounter> {
    let mut offset = 0;
    let mut fights = Vec::new();
    while offset < bytes.len() {
        let output = parser.feed(&bytes[offset..]).unwrap();
        assert!(output.diagnostics.is_empty(), "{:?}", output.diagnostics);
        assert!(output.accepted > 0);
        offset += output.accepted;
        if let Some(completed) = output.completed {
            fights.push(completed.encounter);
        }
    }
    fights
}

fn parse(source: &str) -> Vec<NativeEncounter> {
    feed(
        &mut IncrementalNativeParser::new(Channel::Live),
        source.as_bytes(),
    )
}

#[test]
fn producer_derived_profiles_retain_exact_evidence_and_equivalent_quantities() {
    for source in PROFILES {
        let fights = parse(source);
        assert_eq!(fights.len(), 1);
        let fight = &fights[0];
        let projection = project_native_encounter(fight).unwrap();
        assert_eq!(projection.damage_hit, 300);
        assert_eq!(projection.damage_overflow, 7);
        assert_eq!(projection.local_damage_hit, Some(300));
        assert_eq!(projection.cast_start_count, 1);
        assert_eq!(projection.effect_change_count, 1);
        assert_eq!(fight.unknown_context_count, 1); // Frozen trial classification.
        for record in fight.context.iter().chain(&fight.records) {
            assert!(source[record.byte_offset as usize..].starts_with(&record.line));
            assert_eq!(
                record.line,
                source.lines().nth(record.ordinal as usize - 1).unwrap()
            );
        }
        assert_eq!(fight.run.header.record.line, source.lines().next().unwrap());
        let encoded = canonical_native_bytes(fight).unwrap();
        let decoded: NativeEncounter = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(canonical_native_bytes(&decoded).unwrap(), encoded);
        assert_eq!(
            logical_native_identity(&decoded).unwrap(),
            logical_native_identity(fight).unwrap()
        );
    }
}

#[test]
fn every_byte_partition_and_checkpoint_restore_preserve_each_profile() {
    for source in PROFILES {
        let expected = parse(source);
        for split in 0..=source.len() {
            let mut parser = IncrementalNativeParser::new(Channel::Live);
            let mut actual = feed(&mut parser, &source.as_bytes()[..split]);
            // Restore inside labels, quotes, arrays, UTF-8, and pending fights.
            let checkpoint = parser.checkpoint().unwrap();
            let checkpoint = NativeCheckpoint::from_bytes(checkpoint.as_bytes()).unwrap();
            parser = IncrementalNativeParser::restore(&checkpoint, Channel::Live).unwrap();
            actual.extend(feed(&mut parser, &source.as_bytes()[split..]));
            assert_eq!(actual, expected, "split {split}");
        }
        let crlf = source.replace('\n', "\r\n");
        assert_eq!(parse(&crlf).len(), 1);
    }
}

#[test]
fn profiles_can_change_within_a_run_and_fight_without_a_marker() {
    let lines: Vec<_> = PROFILES
        .iter()
        .map(|s| s.lines().collect::<Vec<_>>())
        .collect();
    let mixed = (0..lines[0].len())
        .map(|i| lines[i % 3][i])
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let fights = parse(&mixed);
    assert_eq!(fights.len(), 1);
    assert_eq!(
        project_native_encounter(&fights[0]).unwrap().damage_hit,
        300
    );
}

#[test]
fn malformed_known_labels_and_inline_tails_require_fresh_header_recovery() {
    for source in [
        PROFILES[2].replace("hitValue: 300", "hitvalue: 300"),
        PROFILES[2].replace("\tblockable: T", "\tblockable: MAYBE"),
        PROFILES[2].replace("\tenchantCP: F", "\tenchantCp: F"),
        PROFILES[0].replace("F,T,1,100/100", "F,MAYBE,1,100/100"),
    ] {
        let combined = source + PROFILES[2];
        let mut parser = IncrementalNativeParser::new(Channel::Live);
        let mut offset = 0;
        let mut completed = 0;
        let mut diagnostics = Vec::new();
        while offset < combined.len() {
            let output = parser.feed(&combined.as_bytes()[offset..]).unwrap();
            offset += output.accepted;
            completed += usize::from(output.completed.is_some());
            diagnostics.extend(output.diagnostics);
        }
        assert_eq!(completed, 1);
        assert!(diagnostics
            .iter()
            .any(|d| d.code == NativeDiagnosticCode::MalformedKnownRecord));
    }
}

#[test]
fn signed_observed_quantities_are_retained_without_unsigned_wrap_or_zero_fill() {
    let source = PROFILES[2]
        .replace("hitValue: 300", "hitValue: -3")
        .replace("overflow: 7", "overflow: -1")
        .replace("durationMS: 100", "durationMS: -1");
    let fights = parse(&source);
    assert_eq!(fights.len(), 1);
    let value = serde_json::to_value(project_native_encounter(&fights[0]).unwrap()).unwrap();
    assert_eq!(value["damage_hit"], -3);
    assert_eq!(value["damage_overflow"], -1);
    assert_eq!(value["local_damage_hit"], -3);
    assert_eq!(value["local_damage_per_second"], serde_json::Value::Null);
}

#[test]
fn inline_effect_optionals_null_metadata_and_expanded_targets_are_independent() {
    for source in [PROFILES[0], PROFILES[2]] {
        for synergy in [true, false] {
            for remove in [true, false] {
                let mut value = source.to_owned();
                if !synergy {
                    value = value
                        .replace("\tgrantsSyngeryAbilityId: 0", "")
                        .replace("BUFF,NONE,DEFAULT,0,", "BUFF,NONE,DEFAULT,");
                }
                if !remove {
                    value = value
                        .replace("\tplayerInitiatedRemoveCastTrackId: 99", "")
                        .replace(",*,99\n", ",*\n");
                }
                assert_eq!(parse(&value).len(), 1, "synergy={synergy}, remove={remove}");
            }
        }
        let null = source
            .replace("\"Synthetic, Ω\\skill\"", "\"\"")
            .replace("\"/fixture/ability.dds\"", "\"\"")
            .replace("BUFF,NONE,DEFAULT,0,", "\"\",\"\",\"\",0,")
            .replace("\"\",\"\",\"\",0,", "\"\",\"\",\"\",")
            .replace("\tgrantsSyngeryAbilityId: 0", "")
            .replace(
                "effectType: BUFF\tstatusEffectType: NONE\teffectBarDisplayBehaviour: DEFAULT",
                "effectType: \"\"\tstatusEffectType: \"\"\teffectBarDisplayBehaviour: \"\"",
            );
        assert_eq!(parse(&null).len(), 1);
    }
}

#[test]
fn verbose_unknown_payloads_and_literal_punctuation_are_retained_without_guessing() {
    let source = PROFILES[2].to_owned();
    let unknown = "offsetMS: 18\tlineType: FUTURE_RECORD\tFutureCase: [\"a,b: c\\path\"\t42]\n";
    let source = source.replace(
        "offsetMS: 20\tlineType: END_COMBAT",
        &(unknown.to_owned() + "offsetMS: 20\tlineType: END_COMBAT"),
    );
    let fight = parse(&source).remove(0);
    assert_eq!(fight.unknown_record_count, 1);
    assert!(fight.records.iter().any(|r| r.line == unknown.trim_end()));
    let header = source.replace(
        "realmName: \"SyntheticRealm\"",
        "realmName: \"R\\literal, Ω\"",
    );
    assert_eq!(parse(&header)[0].run.header.realm, "R\\literal, Ω");
}

#[test]
fn normal_metadata_literal_backslashes_are_supported_without_reinterpreting_legacy_escapes() {
    let source = PROFILES[0].replace("Synthetic ability", "Path\\literal");
    assert_eq!(parse(&source).len(), 1);
    let legacy = "0,BEGIN_LOG,1700000000000,15,\"R\\\\compat\",\"zz\",\"G\"\n10,BEGIN_COMBAT\n20,END_COMBAT\n";
    assert_eq!(parse(legacy)[0].run.header.realm, "R\\compat");
}

#[test]
fn verbose_bounds_and_version_diagnostics_cannot_be_hidden_by_unknown_payloads() {
    for payload in [
        format!("[{}1{}]", "[".repeat(8), "]".repeat(8)),
        format!("[{}1]", "1\t".repeat(4096)),
        "X".repeat(65 * 1024),
    ] {
        let source = format!(
            "{}offsetMS: 5\tlineType: FUTURE_RECORD\tdata: {payload}\n{}",
            PROFILES[2].lines().next().unwrap().to_owned() + "\n",
            PROFILES[2]
        );
        let mut parser = IncrementalNativeParser::new(Channel::Live);
        let output = parser.feed(source.as_bytes()).unwrap();
        assert!(!output.diagnostics.is_empty());
        assert!(output.completed.is_some());
    }
    let source = PROFILES[2].replace("logVersion: 15", "logVersion: 16");
    let output = IncrementalNativeParser::new(Channel::Live)
        .feed(source.as_bytes())
        .unwrap();
    assert_eq!(
        output.diagnostics[0].code,
        NativeDiagnosticCode::UnsupportedLogVersion
    );
}
