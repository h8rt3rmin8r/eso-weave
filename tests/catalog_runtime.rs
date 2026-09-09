mod catalog_support;

use rusqlite::Connection;

use catalog_support::{CatalogSandbox, CHANGED_LIVE_FIXTURE, LIVE_FIXTURE};
use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::{CatalogAccess, CatalogDiagnosticKind, Channel, EntityKind};

#[test]
fn typed_reader_opens_release_and_entity_read_only() {
    let sandbox = CatalogSandbox::new();
    let path = sandbox.path("catalog.sqlite");
    build_catalog(&BuildRequest::new(LIVE_FIXTURE, &path, Channel::Live)).unwrap();

    let catalog = CatalogAccess::open_or_empty(&path);
    assert!(catalog.is_available());
    let release = catalog.release().expect("release query").unwrap();
    assert_eq!(release.catalog_version, "s070-live-1");
    assert_eq!(release.channel, Channel::Live);
    let ability = catalog.ability(100).expect("ability query").unwrap();
    assert_eq!(ability.kind, EntityKind::Ability);
    assert_eq!(ability.stable_id, 100);
    assert_eq!(ability.integer_attribute("cost"), Some(42));
    let effect = catalog
        .entity(EntityKind::Effect, 200)
        .expect("effect query")
        .unwrap();
    assert!(effect.observed_only);
}

#[test]
fn missing_corrupt_and_incompatible_catalogs_degrade_to_empty() {
    let sandbox = CatalogSandbox::new();
    let missing = CatalogAccess::open_or_empty(sandbox.path("missing.sqlite"));
    assert_eq!(
        missing.diagnostic().unwrap().kind,
        CatalogDiagnosticKind::Missing
    );
    assert!(missing.ability(100).unwrap().is_none());

    let corrupt_path = sandbox.path("corrupt.sqlite");
    std::fs::write(&corrupt_path, b"not a sqlite database").unwrap();
    let corrupt = CatalogAccess::open_or_empty(&corrupt_path);
    assert_eq!(
        corrupt.diagnostic().unwrap().kind,
        CatalogDiagnosticKind::Corrupt
    );

    let incompatible_path = sandbox.path("incompatible.sqlite");
    build_catalog(&BuildRequest::new(
        LIVE_FIXTURE,
        &incompatible_path,
        Channel::Live,
    ))
    .unwrap();
    Connection::open(&incompatible_path)
        .unwrap()
        .pragma_update(None, "user_version", 99)
        .unwrap();
    let incompatible = CatalogAccess::open_or_empty(&incompatible_path);
    assert_eq!(
        incompatible.diagnostic().unwrap().kind,
        CatalogDiagnosticKind::Incompatible
    );

    let older_path = sandbox.path("older.sqlite");
    build_catalog(&BuildRequest::new(LIVE_FIXTURE, &older_path, Channel::Live)).unwrap();
    Connection::open(&older_path)
        .unwrap()
        .pragma_update(None, "user_version", 0)
        .unwrap();
    let older = CatalogAccess::open_or_empty(&older_path);
    assert_eq!(
        older.diagnostic().unwrap().kind,
        CatalogDiagnosticKind::Incompatible
    );
}

#[test]
fn semantic_mutation_is_detected_and_old_handle_stays_version_bound() {
    let sandbox = CatalogSandbox::new();
    let first_path = sandbox.path("first.sqlite");
    let second_path = sandbox.path("second.sqlite");
    build_catalog(&BuildRequest::new(LIVE_FIXTURE, &first_path, Channel::Live)).unwrap();
    build_catalog(&BuildRequest::new(
        CHANGED_LIVE_FIXTURE,
        &second_path,
        Channel::Live,
    ))
    .unwrap();

    let first = CatalogAccess::open_or_empty(&first_path);
    let second = CatalogAccess::open_or_empty(&second_path);
    assert_eq!(
        first.release().unwrap().unwrap().catalog_version,
        "s070-live-1"
    );
    assert_eq!(
        second.release().unwrap().unwrap().catalog_version,
        "s070-live-2"
    );
    assert_eq!(
        first
            .ability(100)
            .unwrap()
            .unwrap()
            .integer_attribute("cost"),
        Some(42)
    );

    drop(first);
    let connection = Connection::open(&first_path).unwrap();
    connection
        .execute(
            "UPDATE entity_attribute SET integer_value = 999 WHERE name = 'cost'",
            [],
        )
        .unwrap();
    drop(connection);
    let tampered = CatalogAccess::open_or_empty(&first_path);
    assert_eq!(
        tampered.diagnostic().unwrap().kind,
        CatalogDiagnosticKind::Checksum
    );
}

#[test]
fn runtime_locator_uses_only_documented_roots() {
    let executable = std::path::Path::new("C:/Program Files/ESO Weave/eso-weave.exe");
    let candidates = eso_weave::catalog::catalog_candidates(executable, None);
    assert_eq!(
        candidates[0],
        std::path::Path::new("C:/Program Files/ESO Weave/catalog/catalog.sqlite")
    );
    assert!(candidates
        .iter()
        .all(|path| path.ends_with("catalog/catalog.sqlite")));
}

#[test]
fn catalog_status_is_visible_and_truthful() {
    let missing = CatalogAccess::empty(CatalogDiagnosticKind::Missing, "Catalog unavailable");
    let line = eso_weave::app::status_line_catalog(&missing);
    assert_eq!(line.title, "Catalog");
    assert_eq!(line.state_text, "Catalog unavailable");
    assert_eq!(line.role, eso_weave::app::StatusRole::Warning);

    let sandbox = CatalogSandbox::new();
    let path = sandbox.path("catalog.sqlite");
    build_catalog(&BuildRequest::new(LIVE_FIXTURE, &path, Channel::Live)).unwrap();
    let available = CatalogAccess::open_or_empty(path);
    let line = eso_weave::app::status_line_catalog(&available);
    assert_eq!(line.role, eso_weave::app::StatusRole::Healthy);
    assert!(line.state_text.contains("s070-live-1"));
}
