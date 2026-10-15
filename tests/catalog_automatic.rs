use std::fs;

use eso_weave::catalog::accumulate::{accumulate, content_sha256, read_bundle};
use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::Channel;
use eso_weave::catalog_update::{
    AutomaticCatalogOutcome, CancellationToken, CatalogTarget, CatalogUpdateService, UpdateStage,
};
use eso_weave::collector::{adler32_hex, parse_capture, CollectorChunk, RecordParent};
use serde_json::Value;

fn capture() -> String {
    let source =
        fs::read_to_string("specs/092-data-addon-foundation/fixtures/catalog-live.lua").unwrap();
    let marker = "[\"collector_checksum\"] = \"";
    let start = source.find(marker).unwrap() + marker.len();
    let end = start + source[start..].find('"').unwrap();
    format!(
        "{}{}{}",
        &source[..start],
        eso_weave::collector::embedded_checksum(),
        &source[end..]
    )
}

#[test]
fn independent_first_capture_populates_actual_packaged_baseline() {
    let root = tempfile::tempdir().unwrap();
    let bundled = root.path().join("bundled.sqlite");
    build_catalog(&BuildRequest::new(
        "assets/catalog/baseline.json",
        &bundled,
        Channel::Live,
    ))
    .unwrap();
    let service = CatalogUpdateService::new(root.path().join("config"), &bundled);
    let saved = root.path().join("EsoWeaveData.lua");
    let boundary = service.capture_wait_boundary(&saved).unwrap();
    fs::write(&saved, capture()).unwrap();
    let candidate = service
        .build_collector_candidate(&saved, &boundary, &CancellationToken::new(), |_| {})
        .unwrap();
    assert!(candidate.diff.entities_added >= 3);
    assert_eq!(candidate.diff.entities_removed, 0);
    assert_eq!(candidate.diff.coverage_removed, 0);
}

fn lua(value: &Value) -> String {
    match value {
        Value::Null => "nil".into(),
        Value::Bool(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => serde_json::to_string(v).unwrap(),
        Value::Array(values) => format!(
            "{{{}}}",
            values
                .iter()
                .enumerate()
                .map(|(i, v)| format!("[{}]={}", i + 1, lua(v)))
                .collect::<Vec<_>>()
                .join(",")
        ),
        Value::Object(values) => format!(
            "{{{}}}",
            values
                .iter()
                .map(|(k, v)| format!("[{}]={}", serde_json::to_string(k).unwrap(), lua(v)))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

fn saved(edit: impl FnOnce(&mut eso_weave::collector::CollectorEnvelope)) -> String {
    let mut envelope = parse_capture(capture().as_bytes()).unwrap();
    edit(&mut envelope);
    for coverage in &mut envelope.coverage {
        coverage.record_count = envelope
            .records
            .iter()
            .filter(|r| r.category == coverage.category)
            .count();
    }
    let payload = envelope
        .records
        .iter()
        .map(|r| serde_json::to_string(r).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    envelope.chunks = vec![CollectorChunk {
        sequence: 1,
        record_count: envelope.records.len(),
        byte_count: payload.len(),
        checksum: adler32_hex(payload.as_bytes()),
        payload,
    }];
    format!(
        "EsoWeaveDataSaved={{[\"schema_version\"]=1,[\"addon_version\"]=1,[\"catalog\"]={}}}\n",
        lua(&serde_json::to_value(envelope).unwrap())
    )
}

struct AutomaticSandbox {
    root: tempfile::TempDir,
    service: CatalogUpdateService,
    input: std::path::PathBuf,
}
impl AutomaticSandbox {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        let bundled = root.path().join("bundled.sqlite");
        build_catalog(&BuildRequest::new(
            "assets/catalog/baseline.json",
            &bundled,
            Channel::Live,
        ))
        .unwrap();
        let service = CatalogUpdateService::new(root.path().join("config"), &bundled);
        let input = root.path().join("EsoWeaveData.lua");
        Self {
            root,
            service,
            input,
        }
    }
    fn apply(&self, data: &str) -> AutomaticCatalogOutcome {
        fs::write(&self.input, data).unwrap();
        self.service
            .maintain_collector_capture(&self.input, &CancellationToken::new(), |_| {})
            .unwrap()
    }
    fn bundle(&self) -> eso_weave::catalog::model::CatalogBundle {
        read_bundle(&self.service.resolve_catalog().path).unwrap()
    }
}

#[test]
fn startup_activates_existing_data_and_semantic_replay_keeps_generation() {
    let sandbox = AutomaticSandbox::new();
    assert!(matches!(
        sandbox.apply(&capture()),
        AutomaticCatalogOutcome::Activated(_)
    ));
    assert!(sandbox
        .service
        .resolve_catalog()
        .access
        .ability(100)
        .unwrap()
        .is_some());
    let before = sandbox.service.load_selection().unwrap();
    let data = saved(|e| {
        e.started_at = "1792065600".into();
        e.finished_at = "1792065601".into();
        e.scope_key = "another-character".into();
    });
    assert!(matches!(
        sandbox.apply(&data),
        AutomaticCatalogOutcome::Unchanged(_)
    ));
    assert_eq!(sandbox.service.load_selection().unwrap(), before);
}

#[test]
fn changed_second_capture_preserves_disjoint_entities_locales_and_parents() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    let second = saved(|e| {
        e.locale = "fr".into();
        e.finished_at = "2026-10-15T12:00:01Z".into();
        let mut ability = e
            .records
            .iter()
            .find(|r| r.stable_id == 100)
            .unwrap()
            .clone();
        ability.stable_id = 200;
        ability.source_key = "player-skills/ability/200".into();
        ability.name = Some("Deuxieme".into());
        e.records.push(ability);
        let mut line = e
            .records
            .iter()
            .find(|r| r.stable_id == 10)
            .unwrap()
            .clone();
        line.stable_id = 20;
        line.source_key = "player-skills/skill-line/20".into();
        e.records.push(line);
        let ability = e.records.iter_mut().find(|r| r.stable_id == 100).unwrap();
        ability.name = Some("Nouvelle".into());
        ability.parents.push(RecordParent {
            relation: "skill-line-has-ability".into(),
            kind: eso_weave::catalog::EntityKind::SkillLine,
            stable_id: 20,
        });
    });
    assert!(matches!(
        sandbox.apply(&second),
        AutomaticCatalogOutcome::Activated(_)
    ));
    let bundle = sandbox.bundle();
    assert!(bundle.entities.iter().any(|e| e.entity.stable_id == 200));
    assert!(bundle
        .localized_text
        .iter()
        .any(|t| t.entity.stable_id == 100 && t.locale == "en" && t.value == "Fixture Ability"));
    assert!(bundle
        .localized_text
        .iter()
        .any(|t| t.entity.stable_id == 100 && t.locale == "fr" && t.value == "Nouvelle"));
    assert_eq!(
        bundle
            .relations
            .iter()
            .filter(|r| r.to.stable_id == 100)
            .count(),
        2
    );
    assert_eq!(sandbox.service.load_selection().unwrap().generation, 2);
}

#[test]
fn conflicting_observations_are_order_independent_and_provenance_stays_bounded() {
    let a = AutomaticSandbox::new();
    a.apply(&capture());
    let first = a.bundle();
    let newer = saved(|e| {
        e.finished_at = "1792065601".into();
        e.records
            .iter_mut()
            .find(|r| r.stable_id == 100)
            .unwrap()
            .name = Some("Updated".into());
    });
    a.apply(&newer);
    let forward = a.bundle();
    let b = AutomaticSandbox::new();
    b.apply(&newer);
    b.apply(&capture());
    let reverse = b.bundle();
    assert_eq!(
        content_sha256(&forward).unwrap(),
        content_sha256(&reverse).unwrap()
    );
    assert!(forward
        .localized_text
        .iter()
        .any(|t| t.entity.stable_id == 100 && t.value == "Updated"));
    let merged = accumulate(forward.clone(), first).unwrap();
    assert_eq!(
        content_sha256(&merged).unwrap(),
        content_sha256(&forward).unwrap()
    );
    assert!(
        merged.source_records.len()
            <= forward.entities.len()
                + forward.attributes.len()
                + forward.relations.len()
                + forward.localized_text.len()
    );
}

#[test]
fn cancellation_and_incomplete_writes_preserve_selected_catalog() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    let before = sandbox.service.load_selection().unwrap();
    fs::write(&sandbox.input, "EsoWeaveDataSaved={").unwrap();
    assert!(sandbox
        .service
        .maintain_collector_capture(&sandbox.input, &CancellationToken::new(), |_| {})
        .is_err());
    assert_eq!(sandbox.service.load_selection().unwrap(), before);
    fs::write(
        &sandbox.input,
        saved(|e| {
            e.records
                .iter_mut()
                .find(|r| r.stable_id == 100)
                .unwrap()
                .name = Some("Changed".into());
            e.finished_at = "2026-10-15T12:00:01Z".into();
        }),
    )
    .unwrap();
    let cancel = CancellationToken::new();
    assert!(sandbox
        .service
        .maintain_collector_capture(&sandbox.input, &cancel, |p| {
            if p.stage == UpdateStage::Opening {
                cancel.cancel();
            }
        })
        .is_err());
    assert_eq!(sandbox.service.load_selection().unwrap(), before);
    assert!(matches!(
        sandbox
            .service
            .maintain_collector_capture(&sandbox.input, &CancellationToken::new(), |_| {})
            .unwrap(),
        AutomaticCatalogOutcome::Activated(_)
    ));
}

#[test]
fn corrupt_active_recovers_retained_last_good_before_packaged_baseline() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    let first = sandbox.service.load_selection().unwrap().active;
    sandbox.apply(&saved(|e| {
        e.finished_at = "2026-10-15T12:00:01Z".into();
        e.records
            .iter_mut()
            .find(|r| r.stable_id == 100)
            .unwrap()
            .name = Some("Changed".into());
    }));
    let active = sandbox.service.resolve_catalog().path;
    fs::write(active, b"corrupt").unwrap();
    let recovered = sandbox.service.resolve_catalog();
    assert_eq!(recovered.target, first);
    assert!(recovered.access.ability(100).unwrap().is_some());
    assert!(recovered.warning.unwrap().contains("Recovered"));
}

#[test]
fn successive_updates_retain_only_active_previous_and_bounded_receipts() {
    let sandbox = AutomaticSandbox::new();
    for i in 0..5 {
        sandbox.apply(&saved(|e| {
            e.finished_at = format!("2026-10-15T12:00:0{i}Z");
            e.records
                .iter_mut()
                .find(|r| r.stable_id == 100)
                .unwrap()
                .name = Some(format!("Version {i}"));
        }));
    }
    let versions = sandbox.service.roots().root().join("versions/live");
    assert_eq!(fs::read_dir(versions).unwrap().count(), 2);
    assert_eq!(
        fs::read_dir(sandbox.service.roots().import_root().join("live"))
            .unwrap()
            .count(),
        2
    );
    assert!(
        fs::read_dir(sandbox.service.roots().source_cache())
            .unwrap()
            .count()
            <= 2
    );
    assert!(matches!(
        sandbox.service.load_selection().unwrap().active,
        CatalogTarget::User { .. }
    ));
}

#[test]
fn pts_maintenance_has_independent_selection_and_never_replaces_live() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    let live = sandbox.service.load_selection().unwrap();
    let bundled = sandbox.root.path().join("bundled.sqlite");
    let pts = CatalogUpdateService::new_for_channel(
        sandbox.root.path().join("config"),
        bundled,
        Channel::Pts,
    );
    let data = saved(|e| e.channel = Channel::Pts);
    fs::write(&sandbox.input, &data).unwrap();
    assert!(matches!(
        pts.maintain_collector_capture(&sandbox.input, &CancellationToken::new(), |_| {})
            .unwrap(),
        AutomaticCatalogOutcome::Activated(_)
    ));
    assert_eq!(
        pts.resolve_catalog()
            .access
            .release()
            .unwrap()
            .unwrap()
            .channel,
        Channel::Pts
    );
    assert_eq!(sandbox.service.load_selection().unwrap(), live);
    assert!(sandbox
        .service
        .maintain_collector_capture(&sandbox.input, &CancellationToken::new(), |_| {})
        .is_err());
}

#[test]
fn version_transition_retains_historical_domain_and_history_projection() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    for i in 0..3 {
        sandbox.apply(&saved(|e| {
            e.api_version = 101051;
            e.game_version = "12.1.0".into();
            e.finished_at = format!("2026-10-15T12:00:0{i}Z");
            e.records
                .iter_mut()
                .find(|r| r.stable_id == 100)
                .unwrap()
                .name = Some(format!("Next API {i}"));
        }));
    }
    let historical = sandbox
        .service
        .resolve_catalog_for_version(Channel::Live, 101050, "12.0.8")
        .unwrap();
    assert!(historical.access.ability(100).unwrap().is_some());
    assert!(sandbox
        .service
        .resolve_catalog_for_version(Channel::Pts, 101050, "12.0.8")
        .is_none());
    assert!(sandbox
        .service
        .resolve_catalog_for_version(Channel::Live, 101050, "12.1.0")
        .is_none());
    let current = sandbox.service.resolve_catalog().path;
    let history = eso_weave::encounter::EncounterHistoryService::new(
        sandbox.root.path().join("history-app"),
        current,
    );
    let encounter = sandbox.root.path().join("encounter.lua");
    fs::copy("tests/fixtures/encounter/valid-complete.lua", &encounter).unwrap();
    history.import_current(&encounter, Channel::Live).unwrap();
    let snapshot = history.snapshot().unwrap();
    let detail = history
        .detail(&eso_weave::encounter::EncounterIdentity::from(&snapshot[0]))
        .unwrap();
    assert_eq!(detail.catalog_join.api_version, 101050);
    assert_eq!(
        sandbox
            .service
            .resolve_catalog()
            .access
            .release()
            .unwrap()
            .unwrap()
            .api_version,
        101051
    );
}

#[test]
fn cloned_history_services_share_registered_pts_context_without_live_publication() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    let live_path = sandbox.service.resolve_catalog().path;
    let history = eso_weave::encounter::EncounterHistoryService::new(
        sandbox.root.path().join("history"),
        &live_path,
    );
    let clone = history.clone();
    let pts = CatalogUpdateService::new_for_channel(
        sandbox.root.path().join("other-context"),
        sandbox.root.path().join("bundled.sqlite"),
        Channel::Pts,
    );
    fs::write(&sandbox.input, saved(|e| e.channel = Channel::Pts)).unwrap();
    pts.maintain_collector_capture(&sandbox.input, &CancellationToken::new(), |_| {})
        .unwrap();
    let encounter = sandbox.root.path().join("pts-encounter.lua");
    let source = fs::read_to_string("tests/fixtures/encounter/valid-complete.lua")
        .unwrap()
        .replace("[\"channel\"] = \"live\"", "[\"channel\"] = \"pts\"");
    fs::write(&encounter, source).unwrap();
    history.import_current(&encounter, Channel::Pts).unwrap();
    let snapshot = history.snapshot().unwrap();
    let identity = eso_weave::encounter::EncounterIdentity::from(&snapshot[0]);
    assert!(history.detail(&identity).is_err());
    clone.register_catalog_path(pts.resolve_catalog().path);
    let projection = history.detail(&identity).unwrap();
    assert_eq!(projection.catalog_join.channel, Channel::Pts);
    assert_eq!(sandbox.service.resolve_catalog().path, live_path);
    assert_eq!(
        sandbox
            .service
            .resolve_catalog()
            .access
            .release()
            .unwrap()
            .unwrap()
            .channel,
        Channel::Live
    );
}

#[test]
fn exact_raw_game_version_is_preserved_for_real_client_captures() {
    let sandbox = AutomaticSandbox::new();
    let version = "eso.live.12.0.8.1234567";
    let data = saved(|e| e.game_version = version.into());
    assert!(matches!(
        sandbox.apply(&data),
        AutomaticCatalogOutcome::Activated(_)
    ));
    let bundle = sandbox.bundle();
    assert_eq!(bundle.release.game_version, version);
    assert!(bundle
        .source_snapshots
        .iter()
        .all(|source| source.game_version == version));
    assert!(bundle
        .entities
        .iter()
        .any(|entity| entity.entity.stable_id == 100));
}

#[test]
fn interrupted_collection_pipeline_staging_recovers_without_removing_user_or_accepted_data() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    let registry = sandbox.service.roots().root();
    let abandoned = [
        registry.join(".catalog-pipeline-abandoned"),
        registry.join("staging/.collector-build-abandoned"),
        registry.join("staging/.collector-input-abandoned"),
    ];
    for path in &abandoned {
        fs::create_dir_all(path).unwrap();
        fs::write(path.join("partial"), b"partial").unwrap();
    }
    let unrelated = registry.join("operator-notes");
    fs::create_dir_all(&unrelated).unwrap();
    let accepted = sandbox.service.resolve_catalog().path;
    assert_eq!(sandbox.service.recover_staging().unwrap(), 3);
    assert!(abandoned.iter().all(|path| !path.exists()));
    assert!(unrelated.is_dir());
    assert!(accepted.is_file());
}

#[test]
fn missing_selection_recovers_accepted_domain_even_without_a_receipt() {
    let sandbox = AutomaticSandbox::new();
    sandbox.apply(&capture());
    let target = sandbox.service.load_selection().unwrap().active;
    fs::remove_file(sandbox.service.roots().root().join("selection.json")).unwrap();
    fs::remove_file(sandbox.service.roots().root().join("receipts/1.json")).unwrap();
    let resolution = sandbox.service.resolve_catalog();
    assert_eq!(resolution.target, target);
    assert!(resolution.access.ability(100).unwrap().is_some());
    assert!(!sandbox
        .service
        .roots()
        .root()
        .join("selection.json")
        .exists());
}
