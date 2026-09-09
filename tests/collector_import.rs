mod catalog_support;

use std::fs;

use catalog_support::{bytes, CatalogSandbox};
use eso_weave::catalog::compiler::{build_catalog, verify_catalog, BuildRequest};
use eso_weave::catalog::model::CatalogBundle;
use eso_weave::catalog::Channel;
use eso_weave::collector::{
    embedded_checksum, import_capture, parse_capture, ImportRequest, MAX_CAPTURE_BYTES,
};

const LIVE: &str = include_str!("../specs/071-bounded-discovery-exporter/fixtures/live.lua");
const PTS: &str = include_str!("../specs/071-bounded-discovery-exporter/fixtures/pts.lua");

fn fixture(template: &str) -> String {
    template.replace("@collector_checksum@", &embedded_checksum())
}

#[test]
fn valid_live_and_pts_captures_stage_and_compile() {
    for (template, channel, version) in [
        (LIVE, Channel::Live, "fixture-live-1"),
        (PTS, Channel::Pts, "fixture-pts-1"),
    ] {
        let sandbox = CatalogSandbox::new();
        let input = sandbox.path("capture.lua");
        let staged = sandbox.path("staged.json");
        let catalog = sandbox.path("catalog.sqlite");
        fs::write(&input, fixture(template)).unwrap();

        let receipt = import_capture(&ImportRequest::new(&input, &staged, channel, version))
            .expect("valid capture stages");
        assert_eq!(receipt.status, "staged");
        assert_eq!(receipt.record_count, 3);
        assert_eq!(receipt.chunk_count, 1);
        assert_eq!(receipt.channel, channel);
        assert_eq!(receipt.capture_sha256.len(), 64);
        assert_eq!(receipt.staged_sha256.len(), 64);
        let serialized_receipt = serde_json::to_string(&receipt).unwrap();
        assert!(!serialized_receipt.contains("capture.lua"));
        assert!(!serialized_receipt.contains("anonymous-fixture"));
        assert_eq!(receipt.scope_key_sha256.len(), 64);

        let bundle: CatalogBundle = serde_json::from_slice(&bytes(&staged)).unwrap();
        let bundle = bundle.normalize_and_validate().expect("S070-valid bundle");
        assert!(bundle
            .coverage
            .iter()
            .all(|coverage| coverage.completeness.as_str() != "exhaustive"));
        assert!(bundle
            .localized_text
            .iter()
            .all(|text| text.redistribution.as_str() == "user-generated-only"));

        build_catalog(&BuildRequest::new(&staged, &catalog, channel)).unwrap();
        let report = verify_catalog(&catalog).unwrap();
        assert_eq!(report.channel, channel);
        assert_eq!(report.catalog_version, version);
    }
}

#[test]
fn repeated_imports_are_byte_stable() {
    let sandbox = CatalogSandbox::new();
    let input = sandbox.path("capture.lua");
    let first = sandbox.path("first.json");
    let second = sandbox.path("second.json");
    fs::write(&input, fixture(LIVE)).unwrap();

    import_capture(&ImportRequest::new(
        &input,
        &first,
        Channel::Live,
        "stable-1",
    ))
    .unwrap();
    import_capture(&ImportRequest::new(
        &input,
        &second,
        Channel::Live,
        "stable-1",
    ))
    .unwrap();
    assert_eq!(bytes(first), bytes(second));
}

#[test]
fn parser_rejects_executable_referential_and_ambiguous_lua() {
    let valid = fixture(LIVE);
    let bad = [
        "EsoWeaveCollectorSaved = function() return {} end",
        "EsoWeaveCollectorSaved = other_table",
        "EsoWeaveCollectorSaved = setmetatable({}, {})",
        "EsoWeaveCollectorSaved = { [\"schema_version\"] = 1 + 0 }",
        "-- comment\nEsoWeaveCollectorSaved = {}",
        "EsoWeaveCollectorSaved = [[long string]]",
        "EsoWeaveCollectorSaved = {}; os.execute(\"x\")",
        "DifferentRoot = {}",
    ];
    for source in bad {
        assert!(
            parse_capture(source.as_bytes()).is_err(),
            "accepted {source}"
        );
    }

    let duplicate = valid.replace(
        "[\"schema_version\"] = 1,",
        "[\"schema_version\"] = 1,\n  [\"schema_version\"] = 1,",
    );
    assert!(parse_capture(duplicate.as_bytes()).is_err());

    let invalid_escape = valid.replace("fixture-na", "fixture\\qna");
    assert!(parse_capture(invalid_escape.as_bytes()).is_err());

    let mixed_shape = "EsoWeaveCollectorSaved = { [1] = true, [\"x\"] = false }";
    assert!(parse_capture(mixed_shape.as_bytes()).is_err());

    let long_string = format!(
        "EsoWeaveCollectorSaved = {{ [\"x\"] = \"{}\" }}",
        "x".repeat(65_537)
    );
    assert!(parse_capture(long_string.as_bytes()).is_err());

    let nested = (0..20).fold("1".to_string(), |value, _| {
        format!("{{ [\"x\"] = {value} }}")
    });
    let deep = format!("EsoWeaveCollectorSaved = {nested}");
    assert!(parse_capture(deep.as_bytes()).is_err());
}

#[test]
fn envelope_rejects_partial_corrupt_and_channel_mismatched_captures() {
    let valid = fixture(LIVE);
    let mutations = [
        valid.replace("[\"status\"] = \"complete\"", "[\"status\"] = \"paused\""),
        valid.replace("[\"schema_version\"] = 1", "[\"schema_version\"] = 2"),
        valid.replace("[\"collector_version\"] = 1", "[\"collector_version\"] = 2"),
        valid.replace("c8fef243", "00000000"),
        valid.replace("[\"byte_count\"] = 687", "[\"byte_count\"] = 686"),
        valid.replace("[\"record_count\"] = 3", "[\"record_count\"] = 4"),
        valid.replace("[\"sequence\"] = 1", "[\"sequence\"] = 2"),
        valid.replace(
            "[\"completeness\"] = \"bounded\"",
            "[\"completeness\"] = \"exhaustive\"",
        ),
    ];
    for source in mutations {
        assert!(parse_capture(source.as_bytes()).is_err());
    }

    let fractional_attribute = valid
        .replace(
            "\\\"version_scoped_order\\\":1",
            "\\\"version_scoped_order\\\":1.5",
        )
        .replace("[\"byte_count\"] = 687", "[\"byte_count\"] = 693")
        .replace("c8fef243", "7202f36c");
    assert!(fractional_attribute.contains("1.5"));
    assert!(parse_capture(fractional_attribute.as_bytes()).is_err());

    let sandbox = CatalogSandbox::new();
    let input = sandbox.path("live.lua");
    fs::write(&input, valid).unwrap();
    let error = import_capture(&ImportRequest::new(
        &input,
        sandbox.path("staged.json"),
        Channel::Pts,
        "wrong-channel",
    ))
    .expect_err("live capture cannot stage as PTS");
    assert!(error.to_string().contains("channel"));
}

#[test]
fn failed_imports_preserve_existing_output_and_reject_aliases() {
    let sandbox = CatalogSandbox::new();
    let input = sandbox.path("capture.lua");
    let output = sandbox.path("staged.json");
    fs::write(&input, fixture(LIVE).replace("c8fef243", "bad00000")).unwrap();
    fs::write(&output, b"existing-output\n").unwrap();
    let before = bytes(&output);

    assert!(import_capture(&ImportRequest::new(
        &input,
        &output,
        Channel::Live,
        "failure-1"
    ))
    .is_err());
    assert_eq!(bytes(&output), before);

    fs::write(&input, fixture(LIVE)).unwrap();
    let error = import_capture(&ImportRequest::new(
        &input,
        &input,
        Channel::Live,
        "alias-1",
    ))
    .expect_err("input/output alias must fail");
    assert!(error.to_string().contains("distinct"));
}

#[test]
fn capture_size_is_checked_before_reading() {
    let sandbox = CatalogSandbox::new();
    let input = sandbox.path("oversized.lua");
    let file = fs::File::create(&input).unwrap();
    file.set_len(MAX_CAPTURE_BYTES + 1).unwrap();
    let error = import_capture(&ImportRequest::new(
        &input,
        sandbox.path("staged.json"),
        Channel::Live,
        "large-1",
    ))
    .expect_err("oversized capture must fail");
    assert!(error.to_string().contains("byte limit"));
}
