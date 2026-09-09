use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::Channel;
use eso_weave::catalog_pipeline::{
    build_candidate, build_candidate_with_cancel, build_candidate_with_fetcher, verify_candidate,
    PipelineError, PipelineRun, SourceFetcher,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const LIVE_BUNDLE: &str = "specs/070-catalog-compiler/fixtures/minimal-live.json";
const LIVE_CAPTURE: &str = "specs/071-bounded-discovery-exporter/fixtures/live.lua";
const CANDIDATE_FILES: &[&str] = &[
    "build-report.json",
    "catalog.sqlite",
    "checksums.json",
    "diff.json",
    "icons.json",
    "manifest.json",
    "sources.json",
    "validation.json",
    "verify-report.json",
];

struct Sandbox {
    root: tempfile::TempDir,
    workspace: PathBuf,
    sources: PathBuf,
    icons: PathBuf,
    candidates: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let workspace = root.path().join("workspace");
        fs::create_dir_all(&workspace).unwrap();
        Self {
            sources: root.path().join("source-cache"),
            icons: root.path().join("icon-cache"),
            candidates: root.path().join("candidates"),
            root,
            workspace,
        }
    }

    fn run(&self, request: &Path, allow_network: bool) -> PipelineRun {
        PipelineRun::new(
            request,
            &self.workspace,
            &self.sources,
            &self.icons,
            &self.candidates,
            allow_network,
        )
    }

    fn request_path(&self) -> PathBuf {
        self.workspace.join("request.json")
    }
}

struct MockFetcher {
    bytes: Vec<u8>,
    calls: Cell<usize>,
    error: Option<String>,
}

impl SourceFetcher for MockFetcher {
    fn fetch(&self, _uri: &str, _max_bytes: u64) -> Result<Vec<u8>, PipelineError> {
        self.calls.set(self.calls.get() + 1);
        match &self.error {
            Some(message) => Err(PipelineError::Acquisition(message.clone())),
            None => Ok(self.bytes.clone()),
        }
    }
}

#[test]
fn offline_candidate_is_deterministic_review_safe_and_verifiable() {
    let first = offline_live_sandbox();
    let first_receipt = build_candidate(&first.run(&first.request_path(), false)).unwrap();
    let first_candidate = first.candidates.join(&first_receipt.relative_path);
    let verified = verify_candidate(&first_candidate).unwrap();
    assert_eq!(verified.candidate_sha256, first_receipt.candidate_sha256);
    assert_eq!(verified.channel, Channel::Live);

    let mut names = fs::read_dir(&first_candidate)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, CANDIDATE_FILES);

    let all_reports = CANDIDATE_FILES
        .iter()
        .filter(|name| **name != "catalog.sqlite")
        .flat_map(|name| fs::read(first_candidate.join(name)).unwrap())
        .collect::<Vec<_>>();
    let report_text = String::from_utf8(all_reports).unwrap();
    assert!(!report_text.contains(first.root.path().to_string_lossy().as_ref()));
    assert!(!report_text.contains("Training Pulse"));

    let second = offline_live_sandbox();
    let second_receipt = build_candidate(&second.run(&second.request_path(), false)).unwrap();
    assert_eq!(
        first_receipt.candidate_sha256,
        second_receipt.candidate_sha256
    );
    assert_eq!(
        first_receipt.catalog_semantic_sha256,
        second_receipt.catalog_semantic_sha256
    );
}

#[test]
fn pipeline_observes_cancellation_between_major_build_stages() {
    let sandbox = offline_live_sandbox();
    let mut checkpoints = 0;
    let error = build_candidate_with_cancel(&sandbox.run(&sandbox.request_path(), false), || {
        checkpoints += 1;
        checkpoints == 5
    })
    .unwrap_err();
    assert!(matches!(error, PipelineError::Cancelled));
    assert!(!sandbox.candidates.join("live").exists());
}

#[test]
fn request_and_bundle_channels_must_match_without_pts_promotion() {
    let sandbox = offline_live_sandbox();
    let path = sandbox.request_path();
    let mut request: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    request["mode"] = json!("pts");
    request["version"]["channel"] = json!("pts");
    request["version"]["api_version"] = json!(101051);
    fs::write(&path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();

    let error = build_candidate(&sandbox.run(&path, false)).unwrap_err();
    assert!(error.to_string().contains("channel"));
    assert!(!sandbox.candidates.exists());
}

#[test]
fn source_hash_mismatch_and_corrupt_cache_fail_before_publication() {
    let sandbox = offline_live_sandbox();
    let path = sandbox.request_path();
    let mut request: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    request["sources"][0]["sha256"] = json!("a".repeat(64));
    fs::write(&path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();
    assert!(build_candidate(&sandbox.run(&path, false)).is_err());
    assert!(!sandbox.candidates.exists());

    let sandbox = offline_live_sandbox();
    let receipt = build_candidate(&sandbox.run(&sandbox.request_path(), false)).unwrap();
    let source_hash = source_hash_from_request(&sandbox.request_path());
    fs::write(
        sandbox.sources.join(format!("{source_hash}.bin")),
        b"corrupt",
    )
    .unwrap();
    assert!(build_candidate(&sandbox.run(&sandbox.request_path(), false)).is_err());
    assert!(sandbox.candidates.join(receipt.relative_path).is_dir());
}

#[test]
fn remote_sources_require_both_network_gates_and_support_explicit_stale_cache() {
    let sandbox = remote_live_sandbox();
    let fetcher = MockFetcher {
        bytes: fs::read(LIVE_BUNDLE).unwrap(),
        calls: Cell::new(0),
        error: None,
    };
    assert!(
        build_candidate_with_fetcher(&sandbox.run(&sandbox.request_path(), false), &fetcher)
            .is_err()
    );
    assert_eq!(fetcher.calls.get(), 0);

    let receipt =
        build_candidate_with_fetcher(&sandbox.run(&sandbox.request_path(), true), &fetcher)
            .unwrap();
    assert_eq!(fetcher.calls.get(), 1);
    let source_report = fs::read_to_string(
        sandbox
            .candidates
            .join(&receipt.relative_path)
            .join("sources.json"),
    )
    .unwrap();
    assert!(source_report.contains("pinned-remote"));

    let warm = build_candidate_with_fetcher(&sandbox.run(&sandbox.request_path(), true), &fetcher)
        .unwrap();
    assert_eq!(fetcher.calls.get(), 1);
    assert_eq!(warm.candidate_sha256, receipt.candidate_sha256);

    let mut request: Value =
        serde_json::from_slice(&fs::read(sandbox.request_path()).unwrap()).unwrap();
    request["network"]["refresh"] = json!(true);
    request["network"]["allow_stale_cache"] = json!(true);
    fs::write(
        sandbox.request_path(),
        serde_json::to_vec_pretty(&request).unwrap(),
    )
    .unwrap();
    let failing = MockFetcher {
        bytes: Vec::new(),
        calls: Cell::new(0),
        error: Some("offline".into()),
    };
    let stale = build_candidate_with_fetcher(&sandbox.run(&sandbox.request_path(), true), &failing)
        .unwrap();
    let report = fs::read_to_string(
        sandbox
            .candidates
            .join(&stale.relative_path)
            .join("sources.json"),
    )
    .unwrap();
    assert!(report.contains("stale-cache"));
    let validation: Value = serde_json::from_slice(
        &fs::read(
            sandbox
                .candidates
                .join(&stale.relative_path)
                .join("validation.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(validation["findings"], json!(["stale-cache-reuse:bundle"]));
}

#[test]
fn collector_capture_mode_builds_a_catalog_but_never_copies_the_capture() {
    let sandbox = Sandbox::new();
    let capture = fs::read(LIVE_CAPTURE).unwrap();
    fs::write(sandbox.workspace.join("capture.lua"), &capture).unwrap();
    write_policy(&sandbox.workspace);
    let mut request = request_json(
        "user-capture",
        "live",
        "capture",
        "collector-capture",
        "capture.lua",
        &sha256(&capture),
        "12.0.8",
        101050,
        "s073-capture",
        "0.15.1",
        json!(["en"]),
    );
    request["sources"][0]["revision"] =
        json!("7571d13a1040ccea25a4c8ea714061e5dbce650373684dabba8f7bc7ba7969ae");
    request["sources"][0]["uri"] = json!("user-local-savedvariables");
    request["sources"][0]["locale"] = json!("en");
    request["sources"][0]["license_scope"] = json!("user-generated-local-only");
    request["sources"][0]["redistribution"] = json!("user-generated-only");
    fs::write(
        sandbox.request_path(),
        serde_json::to_vec_pretty(&request).unwrap(),
    )
    .unwrap();

    let receipt = build_candidate(&sandbox.run(&sandbox.request_path(), false)).unwrap();
    let candidate = sandbox.candidates.join(receipt.relative_path);
    assert!(candidate.join("catalog.sqlite").is_file());
    assert!(!fs::read_dir(candidate)
        .unwrap()
        .any(|entry| entry.unwrap().file_name() == "capture.lua"));
}

#[test]
fn bundle_source_rights_must_match_the_acquired_inventory() {
    let sandbox = offline_live_sandbox();
    let path = sandbox.request_path();
    let mut request: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    request["sources"][1]["redistribution"] = json!("user-generated-only");
    fs::write(&path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();

    let error = build_candidate(&sandbox.run(&path, false)).unwrap_err();
    assert!(error.to_string().contains("identity or rights"));
    assert!(!sandbox.candidates.exists());
}

#[test]
fn rejected_bundles_do_not_publish_staged_source_cache_entries() {
    let sandbox = offline_live_sandbox();
    let path = sandbox.request_path();
    let malformed = b"{\"not\":\"a catalog bundle\"}\n";
    fs::write(sandbox.workspace.join("input.json"), malformed).unwrap();
    let mut request: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    request["sources"][0]["sha256"] = json!(sha256(malformed));
    fs::write(&path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();

    assert!(build_candidate(&sandbox.run(&path, false)).is_err());
    assert!(!sandbox.candidates.exists());
    assert!(sandbox.sources.is_dir());
    assert_eq!(fs::read_dir(&sandbox.sources).unwrap().count(), 0);
}

#[test]
fn cross_channel_baselines_and_removal_thresholds_block_new_candidates() {
    let sandbox = offline_live_sandbox();
    let pts = sandbox.workspace.join("baseline-pts.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-pts.json",
        &pts,
        Channel::Pts,
    ))
    .unwrap();
    let mut request: Value =
        serde_json::from_slice(&fs::read(sandbox.request_path()).unwrap()).unwrap();
    request["baseline"] = json!("baseline-pts.sqlite");
    fs::write(
        sandbox.request_path(),
        serde_json::to_vec_pretty(&request).unwrap(),
    )
    .unwrap();
    let error = build_candidate(&sandbox.run(&sandbox.request_path(), false)).unwrap_err();
    assert!(error.to_string().contains("baseline"));
    assert!(!sandbox.candidates.exists());

    let sandbox = offline_live_sandbox();
    let live = sandbox.workspace.join("baseline-live.sqlite");
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-live-changed.json",
        &live,
        Channel::Live,
    ))
    .unwrap();
    let mut request: Value =
        serde_json::from_slice(&fs::read(sandbox.request_path()).unwrap()).unwrap();
    request["baseline"] = json!("baseline-live.sqlite");
    fs::write(
        sandbox.request_path(),
        serde_json::to_vec_pretty(&request).unwrap(),
    )
    .unwrap();
    let error = build_candidate(&sandbox.run(&sandbox.request_path(), false)).unwrap_err();
    assert!(error.to_string().contains("removal count"));
    assert!(!sandbox.candidates.exists());
}

#[test]
fn coverage_completeness_regressions_consume_the_removal_threshold() {
    let sandbox = offline_live_sandbox();
    let baseline = sandbox.workspace.join("baseline-live.sqlite");
    build_catalog(&BuildRequest::new(LIVE_BUNDLE, &baseline, Channel::Live)).unwrap();

    let input = sandbox.workspace.join("input.json");
    let mut bundle: Value = serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
    bundle["coverage"][0]["completeness"] = json!("unknown");
    bundle["coverage"][0]["limits"] = json!("coverage deliberately reduced for test");
    let bundle = serde_json::to_vec_pretty(&bundle).unwrap();
    fs::write(&input, &bundle).unwrap();

    let path = sandbox.request_path();
    let mut request: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    request["sources"][0]["sha256"] = json!(sha256(&bundle));
    request["baseline"] = json!("baseline-live.sqlite");
    fs::write(&path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();

    let error = build_candidate(&sandbox.run(&path, false)).unwrap_err();
    assert!(error.to_string().contains("coverage removal count 1"));
    assert!(!sandbox.candidates.exists());
}

#[test]
fn localized_text_redistribution_changes_block_publication() {
    let sandbox = offline_live_sandbox();
    let baseline = sandbox.workspace.join("baseline-live.sqlite");
    build_catalog(&BuildRequest::new(LIVE_BUNDLE, &baseline, Channel::Live)).unwrap();

    let input = sandbox.workspace.join("input.json");
    let mut bundle: Value = serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
    bundle["localized_text"][0]["redistribution"] = json!("user-generated-only");
    let bundle = serde_json::to_vec_pretty(&bundle).unwrap();
    fs::write(&input, &bundle).unwrap();

    let path = sandbox.request_path();
    let mut request: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    request["sources"][0]["sha256"] = json!(sha256(&bundle));
    request["baseline"] = json!("baseline-live.sqlite");
    fs::write(&path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();

    let error = build_candidate(&sandbox.run(&path, false)).unwrap_err();
    assert!(error.to_string().contains("redistribution change"));
    assert!(!sandbox.candidates.exists());
}

#[test]
fn failed_final_install_publishes_neither_source_nor_icon_cache() {
    let oracle = offline_live_sandbox();
    let candidate_sha256 = build_candidate(&oracle.run(&oracle.request_path(), false))
        .unwrap()
        .candidate_sha256;

    let sandbox = offline_live_sandbox();
    let channel = sandbox.candidates.join("live");
    fs::create_dir_all(&channel).unwrap();
    fs::write(channel.join(&candidate_sha256), b"occupied").unwrap();

    assert!(build_candidate(&sandbox.run(&sandbox.request_path(), false)).is_err());
    assert!(sandbox.sources.is_dir());
    assert_eq!(fs::read_dir(&sandbox.sources).unwrap().count(), 0);
    assert!(!sandbox.icons.exists());
}

#[test]
fn committed_mode_fixtures_build_and_output_root_aliases_fail_closed() {
    for request in [
        "offline-live-request.json",
        "live-request.json",
        "pts-request.json",
        "capture-request.json",
    ] {
        let outputs = tempfile::tempdir().unwrap();
        let receipt = build_candidate(&PipelineRun::new(
            format!("specs/073-reviewed-catalog-pipeline/fixtures/{request}"),
            ".",
            outputs.path().join("sources"),
            outputs.path().join("icons"),
            outputs.path().join("candidates"),
            false,
        ))
        .unwrap_or_else(|error| panic!("{request}: {error}"));
        verify_candidate(
            outputs
                .path()
                .join("candidates")
                .join(receipt.relative_path),
        )
        .unwrap();
    }

    let sandbox = offline_live_sandbox();
    let error = build_candidate(&PipelineRun::new(
        sandbox.request_path(),
        &sandbox.workspace,
        &sandbox.sources,
        &sandbox.sources,
        &sandbox.candidates,
        false,
    ))
    .unwrap_err();
    assert!(error.to_string().contains("distinct and unnested"));
}

#[test]
fn candidate_tampering_is_detected_without_changing_the_candidate() {
    let sandbox = offline_live_sandbox();
    let receipt = build_candidate(&sandbox.run(&sandbox.request_path(), false)).unwrap();
    let candidate = sandbox.candidates.join(receipt.relative_path);
    let report = candidate.join("validation.json");
    let original = fs::read(&report).unwrap();
    fs::write(&report, b"{}\n").unwrap();
    assert!(verify_candidate(&candidate).is_err());
    fs::write(&report, original).unwrap();
    verify_candidate(&candidate).unwrap();
}

fn offline_live_sandbox() -> Sandbox {
    let sandbox = Sandbox::new();
    let bundle = fs::read(LIVE_BUNDLE).unwrap();
    fs::write(sandbox.workspace.join("input.json"), &bundle).unwrap();
    fs::write(sandbox.workspace.join("empty-source.bin"), []).unwrap();
    write_policy(&sandbox.workspace);
    let mut request = request_json(
        "offline",
        "live",
        "bundle",
        "normalized-bundle",
        "input.json",
        &sha256(&bundle),
        "12.0.8",
        101050,
        "s070-live-1",
        "s070-v1",
        json!(["en-US"]),
    );
    request["sources"].as_array_mut().unwrap().push(json!({
        "id": "synthetic-live",
        "role": "provenance",
        "channel": "live",
        "game_version": "12.0.8",
        "api_version": 101050,
        "locale": "en-US",
        "revision": "s070-fixture-1",
        "sha256": sha256(&[]),
        "max_bytes": 1,
        "uri": "project://specs/070/minimal-live",
        "local_path": "empty-source.bin",
        "license_scope": "project-authored-synthetic",
        "redistribution": "allowed"
    }));
    fs::write(
        sandbox.request_path(),
        serde_json::to_vec_pretty(&request).unwrap(),
    )
    .unwrap();
    sandbox
}

fn remote_live_sandbox() -> Sandbox {
    let sandbox = Sandbox::new();
    let bundle = fs::read(LIVE_BUNDLE).unwrap();
    fs::write(sandbox.workspace.join("empty-source.bin"), []).unwrap();
    write_remote_policy(&sandbox.workspace, &sha256(&bundle));
    let mut request = request_json(
        "live",
        "live",
        "bundle",
        "normalized-bundle",
        "",
        &sha256(&bundle),
        "12.0.8",
        101050,
        "s070-live-1",
        "s070-v1",
        json!(["en-US"]),
    );
    request["network"] = json!({
        "enabled": true,
        "refresh": false,
        "allow_stale_cache": false
    });
    request["sources"][0]["local_path"] = Value::Null;
    request["sources"][0]["revision"] = json!("f76cf16c4e5be7b234d15dc7f676febffa64c5bb");
    request["sources"][0]["uri"] = json!(
        "https://raw.githubusercontent.com/esoui/esoui/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/fixture.json"
    );
    request["sources"].as_array_mut().unwrap().push(json!({
        "id": "synthetic-live",
        "role": "provenance",
        "channel": "live",
        "game_version": "12.0.8",
        "api_version": 101050,
        "locale": "en-US",
        "revision": "s070-fixture-1",
        "sha256": sha256(&[]),
        "max_bytes": 1,
        "uri": "project://specs/070/minimal-live",
        "local_path": "empty-source.bin",
        "license_scope": "project-authored-synthetic",
        "redistribution": "allowed"
    }));
    fs::write(
        sandbox.request_path(),
        serde_json::to_vec_pretty(&request).unwrap(),
    )
    .unwrap();
    sandbox
}

#[allow(clippy::too_many_arguments)]
fn request_json(
    mode: &str,
    channel: &str,
    input_id: &str,
    role: &str,
    local_path: &str,
    hash: &str,
    game_version: &str,
    api_version: u32,
    catalog_version: &str,
    tool_version: &str,
    locales: Value,
) -> Value {
    json!({
        "schema_version": 1,
        "mode": mode,
        "version": {
            "channel": channel,
            "game_version": game_version,
            "api_version": api_version,
            "catalog_version": catalog_version,
            "catalog_schema": 1,
            "locales": locales,
            "tool_version": tool_version
        },
        "input_source": input_id,
        "source_policy": "source-policy.json",
        "sources": [{
            "id": input_id,
            "role": role,
            "channel": channel,
            "game_version": game_version,
            "api_version": api_version,
            "locale": "en-US",
            "revision": "project-fixture",
            "sha256": hash,
            "max_bytes": 67108864,
            "uri": "project://s073/input",
            "local_path": if local_path.is_empty() { Value::Null } else { json!(local_path) },
            "license_scope": "project-authored-synthetic",
            "redistribution": "allowed"
        }],
        "network": {
            "enabled": false,
            "refresh": false,
            "allow_stale_cache": false
        },
        "baseline": null,
        "icon_source": null,
        "thresholds": {
            "entities_removed": 0,
            "localized_text_removed": 0,
            "relations_removed": 0,
            "coverage_removed": 0,
            "icon_references_removed": 0
        }
    })
}

fn write_policy(workspace: &Path) {
    fs::write(
        workspace.join("source-policy.json"),
        b"{\"schema_version\":1,\"source_snapshots\":[]}\n",
    )
    .unwrap();
}

fn write_remote_policy(workspace: &Path, hash: &str) {
    let policy = json!({
        "schema_version": 1,
        "source_snapshots": [{
            "id": "bundle",
            "channel": "live",
            "game_version": "12.0.8",
            "api_version": 101050,
            "locale": "en-US",
            "revision": "f76cf16c4e5be7b234d15dc7f676febffa64c5bb",
            "sha256": hash,
            "uri": "https://raw.githubusercontent.com/esoui/esoui/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/fixture.json",
            "license_scope": "project-authored-synthetic"
        }]
    });
    fs::write(
        workspace.join("source-policy.json"),
        serde_json::to_vec_pretty(&policy).unwrap(),
    )
    .unwrap();
}

fn source_hash_from_request(path: &Path) -> String {
    let request: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    request["sources"][0]["sha256"]
        .as_str()
        .unwrap()
        .to_string()
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
