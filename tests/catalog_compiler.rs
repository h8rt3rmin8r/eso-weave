mod catalog_support;

use std::fs::{self, File};

use catalog_support::{bytes, CatalogSandbox, CHANGED_LIVE_FIXTURE, LIVE_FIXTURE, PTS_FIXTURE};
use eso_weave::catalog::compiler::{build_catalog, diff_catalogs, verify_catalog, BuildRequest};
use eso_weave::catalog::Channel;

#[test]
fn repeated_builds_are_byte_and_semantically_deterministic() {
    let sandbox = CatalogSandbox::new();
    let first = sandbox.path("first.sqlite");
    let second = sandbox.path("second.sqlite");
    let third = sandbox.path("third.sqlite");

    let reports = [&first, &second, &third].map(|output| {
        build_catalog(&BuildRequest::new(LIVE_FIXTURE, output, Channel::Live))
            .expect("build deterministic catalog")
    });

    assert_eq!(reports[0].semantic_sha256, reports[1].semantic_sha256);
    assert_eq!(reports[1].semantic_sha256, reports[2].semantic_sha256);
    assert_eq!(reports[0].artifact_sha256, reports[1].artifact_sha256);
    assert_eq!(bytes(&first), bytes(&second));
    assert_eq!(bytes(&second), bytes(&third));
    assert_eq!(reports[0].validation.integrity_check, "ok");
    assert_eq!(reports[0].validation.foreign_key_violations, 0);
    assert_eq!(reports[0].schema_version, 1);
    assert_eq!(reports[0].entity_counts.get("ability"), Some(&1));
}

#[test]
fn normalized_input_hash_ignores_json_formatting() {
    let sandbox = CatalogSandbox::new();
    let compact_path = sandbox.path("compact.json");
    let value: serde_json::Value =
        serde_json::from_slice(&fs::read(LIVE_FIXTURE).unwrap()).unwrap();
    fs::write(&compact_path, serde_json::to_vec(&value).unwrap()).unwrap();
    let pretty_output = sandbox.path("pretty.sqlite");
    let compact_output = sandbox.path("compact.sqlite");
    let pretty = build_catalog(&BuildRequest::new(
        LIVE_FIXTURE,
        &pretty_output,
        Channel::Live,
    ))
    .unwrap();
    let compact = build_catalog(&BuildRequest::new(
        compact_path,
        &compact_output,
        Channel::Live,
    ))
    .unwrap();
    assert_eq!(pretty.input_sha256, compact.input_sha256);
    assert_eq!(pretty.semantic_sha256, compact.semantic_sha256);
    assert_eq!(bytes(pretty_output), bytes(compact_output));
}

#[test]
fn missing_categories_become_explicit_unknown_coverage() {
    let sandbox = CatalogSandbox::new();
    let output = sandbox.path("pts.sqlite");
    let report = build_catalog(&BuildRequest::new(PTS_FIXTURE, &output, Channel::Pts))
        .expect("build empty PTS catalog");
    assert_eq!(report.coverage_counts.get("unknown"), Some(&15));
    assert_eq!(report.channel, Channel::Pts);
}

#[test]
fn invalid_provenance_and_relationships_fail_before_publication() {
    let sandbox = CatalogSandbox::new();
    let destination = sandbox.path("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        LIVE_FIXTURE,
        &destination,
        Channel::Live,
    ))
    .expect("seed last known good catalog");
    let before = bytes(&destination);

    let invalid = sandbox.mutated_fixture("invalid.json", |value| {
        value["relations"][0]["to"]["stable_id"] = 999.into();
    });
    let error = build_catalog(&BuildRequest::new(invalid, &destination, Channel::Live))
        .expect_err("orphan relation must fail");

    assert!(error.to_string().contains("relationship"));
    assert_eq!(bytes(&destination), before);
}

#[test]
fn bounded_and_data_only_inputs_fail_before_construction() {
    let sandbox = CatalogSandbox::new();
    let oversized = sandbox.path("oversized.json");
    File::create(&oversized)
        .unwrap()
        .set_len(eso_weave::catalog::model::MAX_INPUT_BYTES + 1)
        .unwrap();
    let oversized_output = sandbox.path("oversized.sqlite");
    let error = build_catalog(&BuildRequest::new(
        oversized,
        &oversized_output,
        Channel::Live,
    ))
    .expect_err("oversized input must fail before reading");
    assert!(error.to_string().contains("limit"));
    assert!(!oversized_output.exists());

    let lua = sandbox.path("SavedVariables.lua");
    fs::write(&lua, b"EsoWeaveSaved = { ability = 100 }").unwrap();
    let lua_output = sandbox.path("lua.sqlite");
    let error = build_catalog(&BuildRequest::new(lua, &lua_output, Channel::Live))
        .expect_err("SavedVariables Lua must remain non-executable input");
    assert!(error.to_string().contains("JSON"));
    assert!(!lua_output.exists());

    let long_string = sandbox.mutated_fixture("long-string.json", |value| {
        value["release"]["catalog_version"] = "x"
            .repeat(eso_weave::catalog::model::MAX_STRING_BYTES + 1)
            .into();
    });
    let error = build_catalog(&BuildRequest::new(
        long_string,
        sandbox.path("long-string.sqlite"),
        Channel::Live,
    ))
    .expect_err("oversized string must fail validation");
    assert!(error.to_string().contains("string"));
}

#[test]
fn invalid_source_hash_and_coverage_overclaim_are_rejected() {
    let sandbox = CatalogSandbox::new();
    let bad_hash = sandbox.mutated_fixture("bad-hash.json", |value| {
        value["source_snapshots"][0]["raw_sha256"] = "not-a-sha256".into();
    });
    let error = build_catalog(&BuildRequest::new(
        bad_hash,
        sandbox.path("bad-hash.sqlite"),
        Channel::Live,
    ))
    .expect_err("invalid source hash must fail");
    assert!(error.to_string().contains("SHA-256"));

    let overclaim = sandbox.mutated_fixture("overclaim.json", |value| {
        value["coverage"][0]["completeness"] = "exhaustive".into();
        value["coverage"][0]["limits"] = "".into();
    });
    let error = build_catalog(&BuildRequest::new(
        overclaim,
        sandbox.path("overclaim.sqlite"),
        Channel::Live,
    ))
    .expect_err("coverage cannot exceed approved evidence");
    assert!(error.to_string().contains("approved source contract"));
}

#[test]
fn duplicate_and_transient_identities_are_rejected() {
    let sandbox = CatalogSandbox::new();
    let duplicate_source = sandbox.mutated_fixture("duplicate-source.json", |value| {
        let first = value["source_records"][0].clone();
        value["source_records"].as_array_mut().unwrap().push(first);
    });
    let error = build_catalog(&BuildRequest::new(
        duplicate_source,
        sandbox.path("duplicate-source.sqlite"),
        Channel::Live,
    ))
    .expect_err("duplicate source record must fail");
    assert!(error.to_string().contains("duplicate source record"));

    let conflicting_source = sandbox.mutated_fixture("conflicting-source.json", |value| {
        let mut conflict = value["source_records"][0].clone();
        conflict["record_id"] = "conflicting-source-record".into();
        conflict["content_sha256"] =
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
        value["source_records"]
            .as_array_mut()
            .unwrap()
            .push(conflict);
    });
    let error = build_catalog(&BuildRequest::new(
        conflicting_source,
        sandbox.path("conflicting-source.sqlite"),
        Channel::Live,
    ))
    .expect_err("conflicting source keys must fail");
    assert!(error.to_string().contains("duplicate source record key"));

    let duplicate = sandbox.mutated_fixture("duplicate.json", |value| {
        let first = value["entities"][0].clone();
        value["entities"].as_array_mut().unwrap().push(first);
    });
    let error = build_catalog(&BuildRequest::new(
        duplicate,
        sandbox.path("duplicate.sqlite"),
        Channel::Live,
    ))
    .expect_err("duplicate identity must fail");
    assert!(error.to_string().contains("duplicate"));

    let transient = sandbox.mutated_fixture("transient.json", |value| {
        value["attributes"][1]["name"] = "array-index".into();
    });
    let error = build_catalog(&BuildRequest::new(
        transient,
        sandbox.path("transient.sqlite"),
        Channel::Live,
    ))
    .expect_err("transient index must fail");
    assert!(error.to_string().contains("version-scoped-order"));
}

#[test]
fn channel_mismatch_cannot_publish() {
    let sandbox = CatalogSandbox::new();
    let error = build_catalog(&BuildRequest::new(
        PTS_FIXTURE,
        sandbox.path("live.sqlite"),
        Channel::Live,
    ))
    .expect_err("PTS must not publish as live");
    assert!(error.to_string().contains("channel"));
}

#[test]
fn live_and_pts_catalogs_cannot_replace_each_other() {
    let sandbox = CatalogSandbox::new();
    let destination = sandbox.path("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        LIVE_FIXTURE,
        &destination,
        Channel::Live,
    ))
    .unwrap();
    let before = bytes(&destination);

    let error = build_catalog(&BuildRequest::new(PTS_FIXTURE, &destination, Channel::Pts))
        .expect_err("PTS must not replace an existing live catalog");
    assert!(error
        .to_string()
        .contains("refusing to replace live catalog"));
    assert_eq!(bytes(destination), before);
}

#[test]
fn failure_report_and_publish_gate_preserve_last_known_good() {
    let sandbox = CatalogSandbox::new();
    let destination = sandbox.path("catalog.sqlite");
    build_catalog(&BuildRequest::new(
        LIVE_FIXTURE,
        &destination,
        Channel::Live,
    ))
    .unwrap();
    let before = bytes(&destination);

    let invalid = sandbox.mutated_fixture("invalid-report.json", |value| {
        value["entities"][0]["source_records"] = serde_json::json!(["missing-record"]);
    });
    let report_path = sandbox.path("failure.json");
    let mut request = BuildRequest::new(invalid, &destination, Channel::Live);
    request.report_path = Some(report_path.clone());
    build_catalog(&request).expect_err("invalid provenance must fail");
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(report_path).unwrap()).unwrap();
    assert_eq!(report["status"], "failed");
    assert!(!report["errors"].as_array().unwrap().is_empty());
    assert_eq!(bytes(&destination), before);

    let mut request = BuildRequest::new(CHANGED_LIVE_FIXTURE, &destination, Channel::Live);
    request.report_path = Some(sandbox.root.path().to_path_buf());
    build_catalog(&request).expect_err("report publication failure must abort replacement");
    assert_eq!(bytes(destination), before);
}

#[test]
fn changed_catalog_produces_stable_categorized_diff_and_rollback() {
    let sandbox = CatalogSandbox::new();
    let destination = sandbox.path("catalog.sqlite");
    let report_path = sandbox.path("report.json");
    build_catalog(&BuildRequest::new(
        LIVE_FIXTURE,
        &destination,
        Channel::Live,
    ))
    .expect("seed catalog");
    let previous = bytes(&destination);

    let mut request = BuildRequest::new(CHANGED_LIVE_FIXTURE, &destination, Channel::Live);
    request.report_path = Some(report_path.clone());
    let report = build_catalog(&request).expect("publish changed catalog");

    assert_eq!(
        bytes(destination.with_extension("sqlite.rollback")),
        previous
    );
    assert!(destination.with_extension("sqlite.rollback.json").is_file());
    assert!(report
        .diff
        .entities
        .added
        .iter()
        .any(|key| key.contains("101")));
    assert!(report
        .diff
        .entities
        .removed
        .iter()
        .any(|key| key.contains("effect")));
    assert!(!report.diff.localized_text.changed.is_empty());
    assert!(!report.diff.localized_text.added.is_empty());
    assert!(!report.diff.relations.added.is_empty());
    assert!(!report.diff.relations.removed.is_empty());
    assert!(!report.diff.coverage.added.is_empty());
    assert!(!report.diff.coverage.removed.is_empty());
    assert!(!report.diff.icon_references.removed.is_empty());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&fs::read(report_path).unwrap()).unwrap()
            ["status"],
        "published"
    );

    let direct = diff_catalogs(destination.with_extension("sqlite.rollback"), &destination)
        .expect("diff catalogs");
    assert_eq!(direct, report.diff);
}

#[test]
fn verification_reports_schema_integrity_hash_and_ranges() {
    let sandbox = CatalogSandbox::new();
    let output = sandbox.path("catalog.sqlite");
    build_catalog(&BuildRequest::new(LIVE_FIXTURE, &output, Channel::Live)).unwrap();
    let report = verify_catalog(&output).expect("verify catalog");
    assert_eq!(report.schema_version, 1);
    assert_eq!(report.integrity_check, "ok");
    assert_eq!(report.foreign_key_violations, 0);
    assert_eq!(report.id_ranges["ability"].min, 100);
    assert_eq!(report.id_ranges["ability"].max, 100);
}
