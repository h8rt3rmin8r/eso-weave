use std::fs;
use std::path::PathBuf;

use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::version::GameVersion;
use eso_weave::catalog::{CatalogRelease, Channel};
use eso_weave::catalog_pipeline::{build_candidate, inspect_candidate, PipelineRun};
use eso_weave::catalog_update::{
    resolve_availability, AvailabilityInput, CancellationToken, CatalogTarget,
    CatalogUpdateService, CatalogUpdateWorker, CheckFreshness, LiveUpdateState, UpdateError,
    UpdateStage, WorkerEvent,
};
use eso_weave::collector::{import_capture, ImportRequest};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const LIVE_BUNDLE: &str = "specs/070-catalog-compiler/fixtures/minimal-live.json";
const CHANGED_LIVE_BUNDLE: &str = "specs/070-catalog-compiler/fixtures/minimal-live-changed.json";

struct Sandbox {
    root: tempfile::TempDir,
    config: PathBuf,
    bundled: PathBuf,
    workspace: PathBuf,
    service: CatalogUpdateService,
}

impl Sandbox {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let config = root.path().join("config");
        let bundled = root.path().join("bundled/catalog.sqlite");
        fs::create_dir_all(bundled.parent().unwrap()).unwrap();
        build_catalog(&BuildRequest::new(LIVE_BUNDLE, &bundled, Channel::Live)).unwrap();
        let workspace = root.path().join("candidate-workspace");
        fs::create_dir_all(&workspace).unwrap();
        let service = CatalogUpdateService::new(&config, &bundled);
        service.prepare().unwrap();
        Self {
            root,
            config,
            bundled,
            workspace,
            service,
        }
    }

    fn candidate(&self, bundle_path: &str, catalog_version: &str) -> PathBuf {
        self.candidate_with_release(bundle_path, catalog_version, None, None)
    }

    fn candidate_with_release(
        &self,
        bundle_path: &str,
        catalog_version: &str,
        game_version: Option<&str>,
        api_version: Option<u32>,
    ) -> PathBuf {
        let mut value: Value = serde_json::from_slice(&fs::read(bundle_path).unwrap()).unwrap();
        value["release"]["catalog_version"] = Value::String(catalog_version.into());
        value["release"]["tool_version"] = Value::String("s074-test".into());
        if let Some(game_version) = game_version {
            value["release"]["game_version"] = Value::String(game_version.into());
            value["source_snapshots"][0]["game_version"] = Value::String(game_version.into());
        }
        if let Some(api_version) = api_version {
            value["release"]["api_version"] = json!(api_version);
            value["source_snapshots"][0]["api_version"] = json!(api_version);
        }
        let bundle = serde_json::to_vec_pretty(&value).unwrap();
        fs::write(self.workspace.join("input.json"), &bundle).unwrap();
        fs::write(self.workspace.join("empty-source.bin"), []).unwrap();
        fs::write(self.workspace.join("source-policy.json"), policy()).unwrap();
        let mut request = request(&bundle, catalog_version);
        let release = &value["release"];
        request["version"]["channel"] = release["channel"].clone();
        request["version"]["game_version"] = release["game_version"].clone();
        request["version"]["api_version"] = release["api_version"].clone();
        request["version"]["locales"] = release["locales"].clone();
        request["sources"][0]["channel"] = release["channel"].clone();
        request["sources"][0]["game_version"] = release["game_version"].clone();
        request["sources"][0]["api_version"] = release["api_version"].clone();
        request["sources"][0]["locale"] = release["locales"][0].clone();
        let snapshot = &value["source_snapshots"][0];
        request["sources"].as_array_mut().unwrap().push(json!({
            "id": snapshot["snapshot_id"],
            "role": "provenance",
            "channel": snapshot["channel"],
            "game_version": snapshot["game_version"],
            "api_version": snapshot["api_version"],
            "locale": snapshot["locale"],
            "revision": snapshot["revision"],
            "sha256": snapshot["raw_sha256"],
            "max_bytes": 1,
            "uri": snapshot["uri"],
            "local_path": "empty-source.bin",
            "license_scope": snapshot["license_scope"],
            "redistribution": snapshot["redistribution"]
        }));
        let request_path = self.workspace.join("request.json");
        fs::write(&request_path, serde_json::to_vec_pretty(&request).unwrap()).unwrap();
        let roots = self.service.roots();
        let receipt = build_candidate(&PipelineRun::new(
            &request_path,
            &self.workspace,
            roots.source_cache(),
            roots.icon_cache(),
            roots.import_root(),
            false,
        ))
        .unwrap();
        roots.import_root().join(receipt.relative_path)
    }

    fn bundled_release(&self) -> CatalogRelease {
        eso_weave::catalog::CatalogAccess::open_or_empty(&self.bundled)
            .release()
            .unwrap()
            .unwrap()
    }
}

#[test]
fn candidate_inspection_is_verified_and_redacted() {
    let sandbox = Sandbox::new();
    let path = sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-2");
    let summary = inspect_candidate(&path).unwrap();
    assert_eq!(summary.channel, Channel::Live);
    assert_eq!(summary.catalog_version, "s074-live-2");
    assert!(summary.total_bytes > 0);
    assert!(summary.source_count > 0);
    let json = serde_json::to_string(&summary).unwrap();
    assert!(!json.contains(sandbox.root.path().to_string_lossy().as_ref()));
    assert!(!json.contains("Training Pulse"));
}

#[test]
fn availability_keeps_live_and_pts_policy_separate() {
    let active = CatalogRelease {
        schema_version: 1,
        catalog_version: "live-1".into(),
        channel: Channel::Live,
        game_version: "12.0.8".into(),
        api_version: 101050,
        semantic_sha256: "a".repeat(64),
    };
    let current = resolve_availability(AvailabilityInput {
        active: Some(active.clone()),
        observed_live: Some(GameVersion::new([12, 0, 8, 0])),
        freshness: CheckFreshness::Fresh,
        live_candidates: Vec::new(),
        pts_candidates: Vec::new(),
        collector_capture_required: false,
        supported_schema: 1,
    });
    assert_eq!(current.live_state, LiveUpdateState::CatalogCurrent);
    assert!(current.pts_preview.is_none());

    let newer = resolve_availability(AvailabilityInput {
        active: Some(active),
        observed_live: Some(GameVersion::new([12, 1, 0, 0])),
        freshness: CheckFreshness::Fresh,
        live_candidates: Vec::new(),
        pts_candidates: Vec::new(),
        collector_capture_required: false,
        supported_schema: 1,
    });
    assert_eq!(newer.live_state, LiveUpdateState::NewLiveDataAvailable);
}

#[test]
fn availability_resolver_covers_every_live_state_and_pts_preview() {
    let sandbox = Sandbox::new();
    let live = inspect_candidate(sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-availability"))
        .unwrap();
    let pts = inspect_candidate(sandbox.candidate(
        "specs/070-catalog-compiler/fixtures/minimal-pts.json",
        "s074-pts-availability",
    ))
    .unwrap();
    let active = sandbox.bundled_release();
    let input = |live_candidates, pts_candidates, collector_capture_required, freshness| {
        AvailabilityInput {
            active: Some(active.clone()),
            observed_live: Some(GameVersion::new([12, 0, 8, 0])),
            freshness,
            live_candidates,
            pts_candidates,
            collector_capture_required,
            supported_schema: 1,
        }
    };
    let ready = resolve_availability(input(
        vec![live.clone()],
        Vec::new(),
        false,
        CheckFreshness::Fresh,
    ));
    assert_eq!(ready.live_state, LiveUpdateState::UpdateReadyToImport);

    let mut unsupported = live;
    unsupported.catalog_schema = 2;
    assert_eq!(
        resolve_availability(input(
            vec![unsupported],
            Vec::new(),
            false,
            CheckFreshness::Fresh,
        ))
        .live_state,
        LiveUpdateState::UnsupportedSchema
    );
    assert_eq!(
        resolve_availability(input(Vec::new(), Vec::new(), true, CheckFreshness::Fresh,))
            .live_state,
        LiveUpdateState::CollectorCaptureRequired
    );
    assert_eq!(
        resolve_availability(input(
            Vec::new(),
            Vec::new(),
            false,
            CheckFreshness::Offline,
        ))
        .live_state,
        LiveUpdateState::OfflineStaleCheck
    );
    let preview = resolve_availability(input(Vec::new(), vec![pts], false, CheckFreshness::Fresh));
    assert_eq!(preview.live_state, LiveUpdateState::CatalogCurrent);
    assert_eq!(preview.pts_preview.unwrap().channel, Channel::Pts);
    assert_eq!(
        resolve_availability(AvailabilityInput {
            active: None,
            observed_live: None,
            freshness: CheckFreshness::Fresh,
            live_candidates: Vec::new(),
            pts_candidates: Vec::new(),
            collector_capture_required: false,
            supported_schema: 1,
        })
        .live_state,
        LiveUpdateState::CatalogUnavailable
    );
}

#[test]
fn reviewed_install_survives_restart_and_rolls_back_to_bundled() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-2");
    let cancel = CancellationToken::new();
    let mut progress = Vec::new();
    let installed = sandbox
        .service
        .install(&candidate, true, &cancel, |event| progress.push(event))
        .unwrap();
    assert!(progress
        .iter()
        .any(|event| event.stage == UpdateStage::Installing));
    let stages = progress.iter().map(|event| event.stage).collect::<Vec<_>>();
    let position = |stage| stages.iter().position(|value| *value == stage).unwrap();
    assert!(position(UpdateStage::WaitingForLock) < position(UpdateStage::Validating));
    assert!(position(UpdateStage::Validating) < position(UpdateStage::Installing));
    assert!(position(UpdateStage::Installing) < position(UpdateStage::Opening));
    assert!(position(UpdateStage::Opening) < position(UpdateStage::Committing));
    assert!(position(UpdateStage::Committing) < position(UpdateStage::Complete));
    assert!(progress
        .iter()
        .filter(|event| event.total_bytes > 0)
        .all(|event| { event.completed_bytes <= event.total_bytes }));
    assert_eq!(
        installed.selection.active,
        CatalogTarget::User {
            candidate_sha256: installed.candidate.candidate_sha256.clone(),
        }
    );
    assert_eq!(installed.selection.previous, Some(CatalogTarget::Bundled));

    let restarted = CatalogUpdateService::new(&sandbox.config, &sandbox.bundled);
    let resolution = restarted.resolve_catalog();
    assert_eq!(resolution.target, installed.selection.active);
    assert_eq!(
        resolution
            .access
            .release()
            .unwrap()
            .unwrap()
            .catalog_version,
        "s074-live-2"
    );

    let rolled_back = restarted
        .rollback(&CancellationToken::new(), |_| {})
        .unwrap();
    assert_eq!(rolled_back.selection.active, CatalogTarget::Bundled);
    assert_eq!(
        restarted
            .resolve_catalog()
            .access
            .release()
            .unwrap()
            .unwrap()
            .catalog_version,
        "s070-live-1"
    );
}

#[test]
fn install_requires_origin_acknowledgement_and_cancellation_preserves_selection() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-2");
    let error = sandbox
        .service
        .install(&candidate, false, &CancellationToken::new(), |_| {})
        .unwrap_err();
    assert!(matches!(error, UpdateError::OriginNotAcknowledged));

    let cancel = CancellationToken::new();
    cancel.cancel();
    let error = sandbox
        .service
        .install(&candidate, true, &cancel, |_| {})
        .unwrap_err();
    assert!(matches!(error, UpdateError::Cancelled));
    assert_eq!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );
}

#[test]
fn cancellation_boundaries_never_leave_an_ambiguous_selection() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-cancel");

    let during_copy = CancellationToken::new();
    let request_cancel = during_copy.clone();
    let error = sandbox
        .service
        .install(&candidate, true, &during_copy, |progress| {
            if progress.stage == UpdateStage::Installing {
                request_cancel.cancel();
            }
        })
        .unwrap_err();
    assert!(matches!(error, UpdateError::Cancelled));
    assert_eq!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );

    let before_open = CancellationToken::new();
    let request_cancel = before_open.clone();
    let error = sandbox
        .service
        .install(&candidate, true, &before_open, |progress| {
            if progress.stage == UpdateStage::Opening {
                request_cancel.cancel();
            }
        })
        .unwrap_err();
    assert!(matches!(error, UpdateError::Cancelled));
    assert_eq!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );

    let at_commit = CancellationToken::new();
    let request_cancel = at_commit.clone();
    let installed = sandbox
        .service
        .install(&candidate, true, &at_commit, |progress| {
            if progress.stage == UpdateStage::Committing {
                request_cancel.cancel();
            }
        })
        .unwrap();
    assert_eq!(installed.selection.generation, 1);
    assert!(matches!(
        installed.selection.active,
        CatalogTarget::User { .. }
    ));
}

#[test]
fn failed_first_open_preserves_the_previous_selection() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-first-open");
    let summary = inspect_candidate(&candidate).unwrap();
    let installed_catalog = sandbox
        .service
        .roots()
        .root()
        .join("versions/live")
        .join(summary.candidate_sha256)
        .join("catalog.sqlite");
    let error = sandbox
        .service
        .install(&candidate, true, &CancellationToken::new(), |progress| {
            if progress.stage == UpdateStage::Opening {
                fs::write(&installed_catalog, b"corrupt").unwrap();
            }
        })
        .unwrap_err();
    assert!(matches!(error, UpdateError::Validation(_)));
    assert_eq!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );
}

#[test]
fn source_and_selection_storage_failures_preserve_the_previous_target() {
    let source_failure = Sandbox::new();
    let candidate = source_failure.candidate(CHANGED_LIVE_BUNDLE, "s074-live-source-failure");
    let error = source_failure
        .service
        .install(&candidate, true, &CancellationToken::new(), |progress| {
            if progress.stage == UpdateStage::Validating {
                fs::remove_file(candidate.join("catalog.sqlite")).unwrap();
            }
        })
        .unwrap_err();
    assert!(matches!(
        error,
        UpdateError::Pipeline(_) | UpdateError::Io(_)
    ));
    assert_eq!(
        source_failure.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );

    let selection_failure = Sandbox::new();
    let candidate = selection_failure.candidate(CHANGED_LIVE_BUNDLE, "s074-live-selection-failure");
    fs::create_dir(
        selection_failure
            .service
            .roots()
            .root()
            .join("selection.json"),
    )
    .unwrap();
    let error = selection_failure
        .service
        .install(&candidate, true, &CancellationToken::new(), |_| {})
        .unwrap_err();
    assert!(matches!(
        error,
        UpdateError::Io(_) | UpdateError::InvalidSelection(_)
    ));
    assert_eq!(
        selection_failure.service.resolve_catalog().target,
        CatalogTarget::Bundled
    );
}

#[test]
fn pts_candidate_is_discoverable_but_cannot_replace_live() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate(
        "specs/070-catalog-compiler/fixtures/minimal-pts.json",
        "s074-pts-preview",
    );
    let summary = inspect_candidate(&candidate).unwrap();
    assert_eq!(summary.channel, Channel::Pts);
    let error = sandbox
        .service
        .install(&candidate, true, &CancellationToken::new(), |_| {})
        .unwrap_err();
    assert!(matches!(error, UpdateError::Validation(_)));
    assert_eq!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );
}

#[test]
fn older_live_candidate_is_rejected_before_staging() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate_with_release(
        CHANGED_LIVE_BUNDLE,
        "s074-live-older",
        Some("11.9.9"),
        Some(101040),
    );
    let error = sandbox
        .service
        .install(&candidate, true, &CancellationToken::new(), |_| {})
        .unwrap_err();
    assert!(matches!(error, UpdateError::Validation(_)));
    assert_eq!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );
}

#[test]
fn collector_build_requires_a_later_flush_and_stays_in_review_flow() {
    let root = tempfile::tempdir().unwrap();
    let baseline_capture = root.path().join("baseline-capture.lua");
    fs::copy(
        "specs/071-bounded-discovery-exporter/fixtures/live.lua",
        &baseline_capture,
    )
    .unwrap();
    let baseline_bundle = root.path().join("baseline.json");
    import_capture(&ImportRequest::new(
        &baseline_capture,
        &baseline_bundle,
        Channel::Live,
        "baseline-live",
    ))
    .unwrap();
    let bundled = root.path().join("bundled/catalog.sqlite");
    fs::create_dir_all(bundled.parent().unwrap()).unwrap();
    build_catalog(&BuildRequest::new(
        &baseline_bundle,
        &bundled,
        Channel::Live,
    ))
    .unwrap();
    let service = CatalogUpdateService::new(root.path().join("config"), &bundled);
    service.prepare().unwrap();
    let capture = root.path().join("SavedVariables/EsoWeaveCollector.lua");
    fs::create_dir_all(capture.parent().unwrap()).unwrap();
    let boundary = service.capture_wait_boundary(&capture).unwrap();
    fs::copy(
        "specs/071-bounded-discovery-exporter/fixtures/live.lua",
        &capture,
    )
    .unwrap();
    let candidate = service
        .build_collector_candidate(&capture, &boundary, &CancellationToken::new(), |_| {})
        .unwrap();
    assert_eq!(candidate.channel, Channel::Live);
    assert_eq!(
        service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );
    assert!(service
        .discover_candidates()
        .unwrap()
        .iter()
        .any(|found| found.candidate_sha256 == candidate.candidate_sha256));
}

#[test]
fn invalid_selection_degrades_visibly_and_is_not_rewritten() {
    let sandbox = Sandbox::new();
    let selection = sandbox.service.roots().root().join("selection.json");
    let invalid = b"{\"schema_version\":1}\n";
    fs::write(&selection, invalid).unwrap();
    let resolution = sandbox.service.resolve_catalog();
    assert_eq!(resolution.target, CatalogTarget::Bundled);
    assert!(resolution.warning.is_some());
    assert_eq!(fs::read(selection).unwrap(), invalid);
}

#[test]
fn cross_process_lock_refuses_a_concurrent_install() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-2");
    let lock_path = sandbox.service.roots().root().join("operation.lock");
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)
        .unwrap();
    lock.try_lock().unwrap();
    let error = sandbox
        .service
        .install(&candidate, true, &CancellationToken::new(), |_| {})
        .unwrap_err();
    assert!(matches!(error, UpdateError::ConcurrentOperation));
    lock.unlock().unwrap();
    assert_eq!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::Bundled
    );
}

#[test]
fn receipts_are_canonical_redacted_and_generation_addressed() {
    let sandbox = Sandbox::new();
    let candidate = sandbox.candidate(CHANGED_LIVE_BUNDLE, "s074-live-2");
    let installed = sandbox
        .service
        .install(&candidate, true, &CancellationToken::new(), |_| {})
        .unwrap();
    assert_eq!(installed.selection.last_receipt, Some(1));
    let receipt = fs::read(sandbox.service.roots().root().join("receipts/1.json")).unwrap();
    assert_eq!(receipt.last(), Some(&b'\n'));
    let text = String::from_utf8(receipt).unwrap();
    assert!(!text.contains(sandbox.root.path().to_string_lossy().as_ref()));
    assert!(!text.contains("Training Pulse"));
    assert!(text.contains("\"operation\": \"install\""));
    assert!(text.contains(&installed.candidate.candidate_sha256));
}

#[test]
fn startup_recovery_removes_only_private_staging() {
    let sandbox = Sandbox::new();
    let abandoned = sandbox
        .service
        .roots()
        .root()
        .join("staging/.update-abandoned/live/hash");
    fs::create_dir_all(&abandoned).unwrap();
    fs::write(abandoned.join("partial"), b"partial").unwrap();
    let unrelated = sandbox.service.roots().root().join("staging/operator-note");
    fs::create_dir_all(&unrelated).unwrap();
    assert_eq!(sandbox.service.recover_staging().unwrap(), 1);
    assert!(!abandoned.exists());
    assert!(unrelated.exists());
}

#[test]
fn worker_failures_are_redacted_and_write_terminal_receipts() {
    let sandbox = Sandbox::new();
    let worker = CatalogUpdateWorker::spawn(sandbox.service.clone());
    assert!(worker.install("a".repeat(64), true));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    let failure = loop {
        match worker.try_recv() {
            Ok(WorkerEvent::Failed(failure)) => break failure,
            Ok(_) | Err(std::sync::mpsc::TryRecvError::Empty) => {
                assert!(
                    std::time::Instant::now() < deadline,
                    "worker event timed out"
                );
                std::thread::yield_now();
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                panic!("worker disconnected before its terminal event")
            }
        }
    };
    assert_eq!(failure.code, "update-invalid");
    assert!(!failure
        .message
        .contains(sandbox.root.path().to_string_lossy().as_ref()));
    drop(worker);
    let receipt =
        fs::read_to_string(sandbox.service.roots().root().join("receipts/event-1.json")).unwrap();
    assert!(receipt.contains("\"result\": \"failed\""));
    assert!(!receipt.contains(sandbox.root.path().to_string_lossy().as_ref()));
}

fn request(bundle: &[u8], catalog_version: &str) -> Value {
    json!({
        "schema_version": 1,
        "mode": "offline",
        "version": {
            "channel": "live",
            "game_version": "12.0.8",
            "api_version": 101050,
            "catalog_version": catalog_version,
            "catalog_schema": 1,
            "locales": ["en-US", "fr"],
            "tool_version": "s074-test"
        },
        "input_source": "bundle",
        "source_policy": "source-policy.json",
        "sources": [{
            "id": "bundle",
            "role": "normalized-bundle",
            "channel": "live",
            "game_version": "12.0.8",
            "api_version": 101050,
            "locale": "en-US",
            "revision": "project-fixture",
            "sha256": sha256(bundle),
            "max_bytes": 67108864,
            "uri": "project://s074/input",
            "local_path": "input.json",
            "license_scope": "project-authored-synthetic",
            "redistribution": "allowed"
        }],
        "network": { "enabled": false, "refresh": false, "allow_stale_cache": false },
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

fn policy() -> &'static [u8] {
    b"{\"schema_version\":1,\"source_snapshots\":[]}\n"
}

fn sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
