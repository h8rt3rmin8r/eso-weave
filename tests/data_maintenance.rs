use eso_weave::beacon::Environment;
use eso_weave::config::{self, Settings};
use eso_weave::data_maintenance::{
    discover_contexts, Context, DataMaintenancePrefs, Eligibility, MaintenanceControl, Presence,
};

#[test]
fn preferences_default_and_round_trip_independently_without_unknown_keys() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("config.json"),
        r#"{"schema_version":1,"logging":{"level":"debug"}}"#,
    )
    .unwrap();
    let mut settings = config::load(root.path()).settings;
    assert_eq!(settings.data_maintenance, DataMaintenancePrefs::default());
    for catalog_disabled in [false, true] {
        for native_processing_disabled in [false, true] {
            settings.data_maintenance = DataMaintenancePrefs {
                catalog_disabled,
                native_processing_disabled,
            };
            config::save(root.path(), &settings).unwrap();
            let loaded = config::load(root.path());
            assert!(loaded.notices.is_empty());
            assert_eq!(loaded.settings, settings);
        }
    }
}

#[test]
fn effective_state_is_independent_and_absence_never_resets_preferences() {
    for catalog_disabled in [false, true] {
        for native_processing_disabled in [false, true] {
            let prefs = DataMaintenancePrefs {
                catalog_disabled,
                native_processing_disabled,
            };
            for presence in [Presence::Absent, Presence::Unknown, Presence::Eligible] {
                let state = Eligibility::new(prefs, presence);
                assert_eq!(state.controls_enabled, presence == Presence::Eligible);
                assert_eq!(
                    state.catalog,
                    presence == Presence::Eligible && !catalog_disabled
                );
                assert_eq!(
                    state.native_processing,
                    presence == Presence::Eligible && !native_processing_disabled
                );
                assert_eq!(state.preferences, prefs);
            }
        }
    }
}

#[test]
fn stop_cancels_only_its_generation_and_failed_enable_remains_off() {
    let control = MaintenanceControl::new(DataMaintenancePrefs::default());
    control.set_presence(true);
    let catalog = control.catalog_token();
    let native = control.native_token();
    let prefs = DataMaintenancePrefs {
        catalog_disabled: true,
        native_processing_disabled: false,
    };
    control.apply(prefs, true);
    assert!(catalog.is_cancelled());
    assert!(!native.is_cancelled());
    assert!(
        control
            .apply(DataMaintenancePrefs::default(), false)
            .catalog_disabled
    );
    assert!(control.preferences().catalog_disabled);
    control.apply(DataMaintenancePrefs::default(), true);
    assert!(!control.catalog_token().is_cancelled());
    assert!(catalog.is_cancelled());
}

#[test]
fn failed_disable_still_stops_current_run_without_claiming_durability() {
    let control = MaintenanceControl::new(DataMaintenancePrefs::default());
    control.set_presence(true);
    let token = control.native_token();
    let requested = DataMaintenancePrefs {
        catalog_disabled: false,
        native_processing_disabled: true,
    };
    control.apply(requested, false);
    assert!(token.is_cancelled());
    assert!(control.preferences().native_processing_disabled);
    assert!(!control.persisted());
}

#[test]
fn actual_contexts_are_independent_of_lifecycle_selection() {
    let sandbox = tempfile::tempdir().unwrap();
    let live = sandbox.path().join("live/AddOns");
    let pts = sandbox.path().join("pts/AddOns");
    std::fs::create_dir_all(&live).unwrap();
    std::fs::create_dir_all(&pts).unwrap();
    let prefs = eso_weave::beacon::BeaconPrefs {
        path_override: Some(live.clone()),
        environment: Environment::Pts,
    };
    let contexts = discover_contexts(&prefs);
    assert!(contexts.contains(&Context {
        addons_root: live,
        environment: Environment::Live
    }));
    assert!(contexts.contains(&Context {
        addons_root: pts,
        environment: Environment::Pts
    }));
}

#[test]
fn invalid_preference_type_preserves_unrelated_settings_and_reports_notice() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("config.json"), r#"{"schema_version":1,"logging":{"level":"debug"},"data_maintenance":{"catalog_disabled":"yes"}}"#).unwrap();
    let loaded = config::load(root.path());
    assert_eq!(loaded.settings.logging.level, config::LevelName::Debug);
    assert_eq!(
        loaded.settings.data_maintenance,
        DataMaintenancePrefs::default()
    );
    assert!(!loaded.notices.is_empty());
    let _ = Settings::default();
}

fn completed_fixture() -> String {
    let source =
        std::fs::read_to_string("specs/092-data-addon-foundation/fixtures/catalog-live.lua")
            .unwrap();
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

fn wait_activation(
    runtime: &eso_weave::data_maintenance::MaintenanceRuntime,
) -> eso_weave::catalog_update::CatalogResolution {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(8);
    loop {
        if let Some(resolution) = runtime.take_activation() {
            if resolution
                .access
                .release()
                .ok()
                .flatten()
                .is_some_and(|release| release.catalog_version.starts_with("local-collector-"))
            {
                return resolution;
            }
        }
        assert!(
            std::time::Instant::now() < deadline,
            "automatic activation did not arrive"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

#[test]
fn startup_consumes_existing_complete_data_without_ui_and_retains_it_after_addon_removal() {
    use eso_weave::beacon::{self, RunningState};
    use eso_weave::data_maintenance::MaintenanceRuntime;
    let root = tempfile::tempdir().unwrap();
    let addons = root.path().join("live/AddOns");
    std::fs::create_dir_all(&addons).unwrap();
    beacon::install_configured(
        &addons,
        RunningState::NotRunning,
        101050,
        3,
        false,
        Environment::Live,
    )
    .unwrap();
    let saved = eso_weave::data_addon::saved_variables_path(&addons).unwrap();
    std::fs::create_dir_all(saved.parent().unwrap()).unwrap();
    let bytes = completed_fixture();
    std::fs::write(&saved, &bytes).unwrap();
    let config = root.path().join("config");
    let bundled = std::path::PathBuf::from("assets/catalog/catalog.sqlite");
    let queries = eso_weave::database_query::DatabaseQueryService::new(bundled.clone(), None);
    let control = MaintenanceControl::new(DataMaintenancePrefs::default());
    let runtime = MaintenanceRuntime::spawn_with_contexts(
        config,
        bundled,
        control.clone(),
        queries,
        vec![Context {
            addons_root: addons.clone(),
            environment: Environment::Live,
        }],
    );
    let resolution = wait_activation(&runtime);
    let database = rusqlite::Connection::open_with_flags(
        &resolution.path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();
    let count: i64 = database
        .query_row("SELECT COUNT(*) FROM entity", [], |row| row.get(0))
        .unwrap();
    assert!(count > 0, "automatic success must populate useful facts");
    assert!(control.controls_enabled());
    assert_eq!(std::fs::read_to_string(&saved).unwrap(), bytes);
    beacon::uninstall(&addons, RunningState::NotRunning).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
    while control.controls_enabled() && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert!(!control.controls_enabled());
    assert!(resolution.path.is_file());
    assert_eq!(std::fs::read_to_string(&saved).unwrap(), bytes);
    let shutdown = std::time::Instant::now();
    drop(runtime);
    assert!(shutdown.elapsed() < std::time::Duration::from_secs(3));
}

#[test]
fn disabled_startup_and_reenable_catchup_need_no_new_save_and_keep_native_independent() {
    use eso_weave::beacon::{self, RunningState};
    use eso_weave::data_maintenance::MaintenanceRuntime;
    let root = tempfile::tempdir().unwrap();
    let addons = root.path().join("live/AddOns");
    std::fs::create_dir_all(&addons).unwrap();
    beacon::install_configured(
        &addons,
        RunningState::NotRunning,
        101050,
        3,
        true,
        Environment::Live,
    )
    .unwrap();
    let saved = eso_weave::data_addon::saved_variables_path(&addons).unwrap();
    std::fs::create_dir_all(saved.parent().unwrap()).unwrap();
    std::fs::write(&saved, completed_fixture()).unwrap();
    let bundled = std::path::PathBuf::from("assets/catalog/catalog.sqlite");
    let queries = eso_weave::database_query::DatabaseQueryService::new(bundled.clone(), None);
    let control = MaintenanceControl::new(DataMaintenancePrefs {
        catalog_disabled: true,
        native_processing_disabled: true,
    });
    let runtime = MaintenanceRuntime::spawn_with_contexts(
        root.path().join("config"),
        bundled,
        control.clone(),
        queries,
        vec![Context {
            addons_root: addons.clone(),
            environment: Environment::Live,
        }],
    );
    std::thread::sleep(std::time::Duration::from_millis(350));
    assert!(runtime.take_activation().is_none());
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: false,
            native_processing_disabled: true,
        },
        true,
    );
    let resolution = wait_activation(&runtime);
    assert!(resolution.access.is_available());
    assert!(control.preferences().native_processing_disabled);
    let context_token = control.register_catalog_job(&addons);
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: true,
        },
        false,
    );
    assert!(context_token.is_cancelled());
    assert!(resolution.path.is_file());
}

#[test]
fn root_cancellation_does_not_stop_another_eligible_context() {
    let control = MaintenanceControl::new(DataMaintenancePrefs::default());
    control.set_presence(true);
    let live = control.register_catalog_job(std::path::Path::new("live"));
    let pts = control.register_catalog_job(std::path::Path::new("pts"));
    control.cancel_catalog_root(std::path::Path::new("pts"));
    assert!(pts.is_cancelled());
    assert!(!live.is_cancelled());
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: false,
        },
        true,
    );
    assert!(live.is_cancelled());
}

#[test]
fn initial_absence_and_preference_changes_keep_both_service_tokens_cancelled() {
    for catalog_disabled in [false, true] {
        for native_processing_disabled in [false, true] {
            let control = MaintenanceControl::new(DataMaintenancePrefs {
                catalog_disabled,
                native_processing_disabled,
            });
            assert!(!control.controls_enabled());
            assert!(control.catalog_token().is_cancelled());
            assert!(control.native_token().is_cancelled());
            control.apply(DataMaintenancePrefs::default(), true);
            assert!(control.catalog_token().is_cancelled());
            assert!(control.native_token().is_cancelled());
            control.set_presence(true);
            assert!(!control.catalog_token().is_cancelled());
            assert!(!control.native_token().is_cancelled());
            control.set_presence(false);
            assert!(control.catalog_token().is_cancelled());
            assert!(control.native_token().is_cancelled());
        }
    }
}

#[test]
fn root_changes_preserve_stops_and_lifecycle_selection_does_not_relabel_known_roots() {
    let control = MaintenanceControl::new(DataMaintenancePrefs {
        catalog_disabled: true,
        native_processing_disabled: true,
    });
    control.set_presence(true);
    control.set_roots(eso_weave::beacon::BeaconPrefs {
        path_override: Some(std::path::PathBuf::from("opaque/AddOns")),
        environment: Environment::Live,
    });
    assert!(control.catalog_token().is_cancelled());
    assert!(control.native_token().is_cancelled());
    control.apply(DataMaintenancePrefs::default(), true);
    let custom = control.register_catalog_job(std::path::Path::new("opaque/AddOns"));
    control.set_roots(eso_weave::beacon::BeaconPrefs {
        path_override: Some(std::path::PathBuf::from("opaque/AddOns")),
        environment: Environment::Pts,
    });
    assert!(custom.is_cancelled());
    control.set_roots(eso_weave::beacon::BeaconPrefs {
        path_override: Some(std::path::PathBuf::from("actual/live/AddOns")),
        environment: Environment::Live,
    });
    let actual = control.register_catalog_job(std::path::Path::new("actual/live/AddOns"));
    control.set_roots(eso_weave::beacon::BeaconPrefs {
        path_override: Some(std::path::PathBuf::from("actual/live/AddOns")),
        environment: Environment::Pts,
    });
    assert!(!actual.is_cancelled());
}

#[test]
fn uppercase_actual_environment_discovers_sibling_and_deduplicates_actual_roots() {
    let root = tempfile::tempdir().unwrap();
    let live = root.path().join("LIVE/AddOns");
    let pts = root.path().join("pts/AddOns");
    std::fs::create_dir_all(&live).unwrap();
    std::fs::create_dir_all(&pts).unwrap();
    let contexts = discover_contexts(&eso_weave::beacon::BeaconPrefs {
        path_override: Some(live.clone()),
        environment: Environment::Pts,
    });
    assert!(contexts.contains(&Context {
        addons_root: live.clone(),
        environment: Environment::Live
    }));
    assert!(contexts.contains(&Context {
        addons_root: pts.clone(),
        environment: Environment::Pts
    }));
    let live_canonical = std::fs::canonicalize(live).unwrap();
    assert_eq!(
        contexts
            .iter()
            .filter(
                |context| std::fs::canonicalize(&context.addons_root).ok().as_ref()
                    == Some(&live_canonical)
            )
            .count(),
        1
    );
}
