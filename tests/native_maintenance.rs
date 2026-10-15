use eso_weave::data_maintenance::{DataMaintenancePrefs, MaintenanceControl};
use std::sync::{mpsc, Arc};
use std::time::Duration;

fn native_run(epoch: u64) -> String {
    format!("0,BEGIN_LOG,{epoch},15,\"SyntheticRealm\",\"zz\",\"fixture.game\"\n10,BEGIN_COMBAT\n20,END_COMBAT\n")
}

fn wait_count(store: &std::path::Path, expected: usize) {
    let deadline = std::time::Instant::now() + Duration::from_secs(8);
    loop {
        let count = eso_weave::encounter::list_encounters(store)
            .map(|rows| rows.len())
            .unwrap_or(0);
        if count == expected {
            return;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "stored {count}, expected {expected}"
        );
        std::thread::sleep(Duration::from_millis(30));
    }
}

fn install_fixture(
    root: &std::path::Path,
    environment: eso_weave::beacon::Environment,
) -> std::path::PathBuf {
    let addons = root.join(match environment {
        eso_weave::beacon::Environment::Live => "live/AddOns",
        eso_weave::beacon::Environment::Pts => "pts/AddOns",
    });
    std::fs::create_dir_all(&addons).unwrap();
    eso_weave::beacon::install_configured(
        &addons,
        eso_weave::beacon::RunningState::NotRunning,
        101050,
        3,
        true,
        environment,
    )
    .unwrap();
    addons
}

#[test]
fn empty_native_source_keeps_retained_reads_coherent_through_disable_absence_and_append() {
    use eso_weave::beacon::Environment;
    use eso_weave::data_maintenance::{Context, NativeMaintenanceRuntime, NativeServicePhase};
    use eso_weave::database_query::{DatabaseQueryService, QueryRequest};
    use eso_weave::encounter::EncounterHistoryService;
    let root = tempfile::tempdir().unwrap();
    let addons = install_fixture(root.path(), Environment::Live);
    let logs = addons.parent().unwrap().join("Logs");
    std::fs::create_dir_all(&logs).unwrap();
    let source = logs.join("Encounter.log");
    std::fs::write(&source, []).unwrap();
    let history = EncounterHistoryService::new(
        root.path().join("application"),
        root.path().join("absent-catalog.sqlite"),
    );
    let control = MaintenanceControl::new(DataMaintenancePrefs {
        catalog_disabled: true,
        native_processing_disabled: true,
    });
    control.set_presence(true);
    let runtime = NativeMaintenanceRuntime::spawn_with_contexts(
        history.clone(),
        control.clone(),
        vec![Context {
            addons_root: addons.clone(),
            environment: Environment::Live,
        }],
    );
    std::thread::sleep(Duration::from_millis(150));
    assert!(history.snapshot().unwrap().is_empty());
    assert!(!history.store_path().exists());
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: false,
        },
        true,
    );
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while !history.store_path().exists()
        || control.native_observation().phase != NativeServicePhase::Waiting
    {
        assert!(
            std::time::Instant::now() < deadline,
            "empty source was not observed"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(history.snapshot().unwrap().is_empty());
    assert!(eso_weave::encounter::list_encounters(history.store_path())
        .unwrap()
        .is_empty());
    let queries = DatabaseQueryService::new(
        root.path().join("absent-catalog.sqlite"),
        Some(history.store_path().to_path_buf()),
    );
    let tokio = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    tokio.block_on(async {
        let inventory = queries
            .inventory(tokio_util::sync::CancellationToken::new())
            .await
            .unwrap();
        let encounters = inventory
            .databases
            .iter()
            .find(|d| d.database_id == "encounters")
            .unwrap();
        assert!(encounters.available);
        assert!(encounters
            .objects
            .iter()
            .any(|o| o.name == "native_encounters"));
        queries
            .execute(
                "encounters",
                QueryRequest::new("SELECT count(*) FROM native_encounters"),
                tokio_util::sync::CancellationToken::new(),
            )
            .await
            .unwrap();
    });
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: true,
        },
        true,
    );
    std::thread::sleep(Duration::from_millis(150));
    assert!(history.snapshot().unwrap().is_empty());
    assert_eq!(std::fs::read(&source).unwrap(), Vec::<u8>::new());
    eso_weave::beacon::uninstall(&addons, eso_weave::beacon::RunningState::NotRunning).unwrap();
    control.set_presence(false);
    assert!(history.snapshot().unwrap().is_empty());
    tokio.block_on(async {
        queries
            .execute(
                "encounters",
                QueryRequest::new("SELECT count(*) FROM native_encounters"),
                tokio_util::sync::CancellationToken::new(),
            )
            .await
            .unwrap();
    });
    let bytes = native_run(1_700_000_000_000);
    std::fs::write(&source, &bytes).unwrap();
    assert!(history.snapshot().unwrap().is_empty());
    install_fixture(root.path(), Environment::Live);
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: false,
        },
        true,
    );
    control.set_presence(true);
    wait_count(history.store_path(), 1);
    assert_eq!(std::fs::read(&source).unwrap(), bytes.as_bytes());
    drop(runtime);
}

#[test]
fn automatic_native_catchup_append_restart_copy_and_truncation_need_no_viewer_or_catalog() {
    use eso_weave::beacon::Environment;
    use eso_weave::data_maintenance::{Context, NativeMaintenanceRuntime};
    use eso_weave::encounter::EncounterHistoryService;
    use std::io::Write;
    let root = tempfile::tempdir().unwrap();
    let addons = install_fixture(root.path(), Environment::Live);
    let logs = addons.parent().unwrap().join("Logs");
    std::fs::create_dir_all(&logs).unwrap();
    let path = logs.join("Encounter.log");
    let initial = native_run(1_700_000_000_000);
    std::fs::write(&path, &initial).unwrap();
    let history = EncounterHistoryService::new(
        root.path().join("application"),
        root.path().join("absent-catalog.sqlite"),
    );
    let control = MaintenanceControl::new(DataMaintenancePrefs {
        catalog_disabled: true,
        native_processing_disabled: false,
    });
    control.set_presence(true);
    let contexts = vec![Context {
        addons_root: addons,
        environment: Environment::Live,
    }];
    let runtime = NativeMaintenanceRuntime::spawn_with_contexts(
        history.clone(),
        control.clone(),
        contexts.clone(),
    );
    wait_count(history.store_path(), 1);
    assert_eq!(std::fs::read(&path).unwrap(), initial.as_bytes());
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: true,
        },
        true,
    );
    let appended = native_run(1_700_000_010_000);
    std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(appended.as_bytes())
        .unwrap();
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(
        eso_weave::encounter::list_encounters(history.store_path())
            .unwrap()
            .len(),
        1
    );
    control.apply(
        DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: false,
        },
        true,
    );
    wait_count(history.store_path(), 2);
    drop(runtime);
    let original = std::fs::read(&path).unwrap();
    std::fs::write(logs.join("Encounter-copy.log"), &original).unwrap();
    let runtime =
        NativeMaintenanceRuntime::spawn_with_contexts(history.clone(), control.clone(), contexts);
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(
        eso_weave::encounter::list_encounters(history.store_path())
            .unwrap()
            .len(),
        2
    );
    let replaced = native_run(1_700_000_020_000);
    std::fs::write(&path, &replaced).unwrap();
    wait_count(history.store_path(), 3);
    assert_eq!(std::fs::read(&path).unwrap(), replaced.as_bytes());
    assert_eq!(
        std::fs::read(logs.join("Encounter-copy.log")).unwrap(),
        original
    );
    let db = rusqlite::Connection::open(history.store_path()).unwrap();
    let derived: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM encounter_derivations WHERE source_kind='native-log'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(derived, 3);
    drop(db);
    let before = std::time::Instant::now();
    drop(runtime);
    assert!(before.elapsed() < Duration::from_secs(3));
    assert!(!control.native_observation().worker_running);
}

#[test]
fn late_logs_cross_environment_and_bad_peer_are_independent() {
    use eso_weave::beacon::Environment;
    use eso_weave::data_maintenance::{Context, NativeMaintenanceRuntime};
    use eso_weave::encounter::EncounterHistoryService;
    let root = tempfile::tempdir().unwrap();
    let live = install_fixture(root.path(), Environment::Live);
    let pts = install_fixture(root.path(), Environment::Pts);
    let history = EncounterHistoryService::new(
        root.path().join("application"),
        root.path().join("no-catalog.sqlite"),
    );
    let control = MaintenanceControl::new(DataMaintenancePrefs::default());
    control.set_presence(true);
    let runtime = NativeMaintenanceRuntime::spawn_with_contexts(
        history.clone(),
        control.clone(),
        vec![
            Context {
                addons_root: live.clone(),
                environment: Environment::Live,
            },
            Context {
                addons_root: pts.clone(),
                environment: Environment::Pts,
            },
        ],
    );
    std::thread::sleep(Duration::from_millis(150));
    assert!(
        !history.store_path().exists(),
        "absent logs do not create an empty history database"
    );
    for addons in [&live, &pts] {
        std::fs::create_dir_all(addons.parent().unwrap().join("Logs")).unwrap();
    }
    let bytes = native_run(1_700_000_000_000);
    std::fs::write(live.parent().unwrap().join("Logs/Encounter.log"), &bytes).unwrap();
    std::fs::write(pts.parent().unwrap().join("Logs/Encounter.log"), &bytes).unwrap();
    std::fs::write(
        live.parent().unwrap().join("Logs/Encounter-bad.log"),
        b"0,BEGIN_LOG,100,99,\"x\",\"zz\",\"test\"\n",
    )
    .unwrap();
    wait_count(history.store_path(), 2);
    let rows = eso_weave::encounter::list_encounters(history.store_path()).unwrap();
    assert!(rows
        .iter()
        .any(|row| row.channel == eso_weave::catalog::Channel::Live));
    assert!(rows
        .iter()
        .any(|row| row.channel == eso_weave::catalog::Channel::Pts));
    let token = control.register_native_job(&live);
    eso_weave::beacon::uninstall(&live, eso_weave::beacon::RunningState::NotRunning).unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while !token.is_cancelled() {
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(control
        .with_native_commit(&live, &token, || Ok(()))
        .is_err());
    assert_eq!(
        eso_weave::encounter::list_encounters(history.store_path())
            .unwrap()
            .len(),
        2
    );
    assert_eq!(control.native_observation().logging_activity, "unknown");
    std::fs::write(
        pts.parent().unwrap().join("Logs/Encounter-new.log"),
        native_run(1_700_000_010_000),
    )
    .unwrap();
    wait_count(history.store_path(), 3);
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        let db = rusqlite::Connection::open(history.store_path()).unwrap();
        let derived: i64 = db
            .query_row(
                "SELECT COUNT(*) FROM encounter_derivations WHERE source_kind='native-log'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        if derived == 3 {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "surviving root must keep deriving after peer removal"
        );
        std::thread::sleep(Duration::from_millis(30));
    }
    drop(runtime);
}

#[test]
fn steam_discovery_enumerates_all_supported_roots_and_libraries() {
    let root = tempfile::tempdir().unwrap();
    let steam_a = root.path().join("steam-a");
    let steam_b = root.path().join("steam-b");
    let library_a = root.path().join("library-a");
    let library_b = root.path().join("library-b");
    for steam in [&steam_a, &steam_b] {
        std::fs::create_dir_all(steam.join("steamapps")).unwrap();
    }
    let quote_path = |path: &std::path::Path| path.display().to_string().replace('\\', "/");
    let vdf = format!("\"libraryfolders\" {{ \"0\" {{ \"path\" \"{}\" \"apps\" {{ \"306130\" \"1\" }} }} \"1\" {{ \"path\" \"{}\" \"apps\" {{ \"306130\" \"1\" }} }} }}", quote_path(&library_a), quote_path(&library_b));
    std::fs::write(steam_a.join("steamapps/libraryfolders.vdf"), &vdf).unwrap();
    std::fs::write(steam_b.join("steamapps/libraryfolders.vdf"), &vdf).unwrap();
    let found = eso_weave::beacon::addons_dirs_in_steam_roots(
        &[steam_a, steam_b],
        eso_weave::beacon::Environment::Pts,
    );
    assert_eq!(found.len(), 2);
    for library in [&library_a, &library_b] {
        assert!(found.contains(&library.join("steamapps/compatdata/306130/pfx/drive_c/users/steamuser/Documents/Elder Scrolls Online/pts/AddOns")));
    }
    assert!(found
        .iter()
        .all(|path| !path.to_string_lossy().contains("/live/")));
}

#[test]
fn native_root_stop_rejects_late_commit_without_cancelling_peer() {
    let control = MaintenanceControl::new(DataMaintenancePrefs::default());
    control.set_presence(true);
    let live = std::path::Path::new("synthetic/live/AddOns");
    let pts = std::path::Path::new("synthetic/pts/AddOns");
    let token = control.register_native_job(live);
    let peer = control.register_native_job(pts);
    assert!(control.with_native_commit(live, &token, || Ok(7)).is_ok());
    control.cancel_native_root(live);
    assert!(token.is_cancelled());
    assert!(!peer.is_cancelled());
    assert!(control.with_native_commit(live, &token, || Ok(9)).is_err());
    assert_eq!(
        control.with_native_commit(pts, &peer, || Ok(11)).unwrap(),
        11
    );
    let next = control.register_native_job(live);
    assert!(!next.is_cancelled());
    assert!(control.with_native_commit(live, &token, || Ok(12)).is_err());
}

#[test]
fn native_worker_shutdown_preserves_independent_catalog_and_restart_eligibility() {
    let root = tempfile::tempdir().unwrap();
    let history = eso_weave::encounter::EncounterHistoryService::new(
        root.path(),
        root.path().join("absent.sqlite"),
    );
    let control = MaintenanceControl::new(DataMaintenancePrefs::default());
    control.set_presence(true);
    let catalog = control.register_catalog_job(root.path());
    let native = control.native_token();
    let runtime = eso_weave::data_maintenance::NativeMaintenanceRuntime::spawn_with_contexts(
        history,
        control.clone(),
        Vec::new(),
    );
    drop(runtime);
    assert!(
        !catalog.is_cancelled(),
        "native shutdown cannot cancel Catalog"
    );
    assert!(
        !native.is_cancelled(),
        "runtime lifetime is separate from the persisted native preference generation"
    );
}

#[test]
fn completed_disable_serializes_with_admitted_commit() {
    let control = Arc::new(MaintenanceControl::new(DataMaintenancePrefs::default()));
    control.set_presence(true);
    let root = std::path::PathBuf::from("synthetic/live/AddOns");
    let token = control.register_native_job(&root);
    let (started_tx, started_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let writer = {
        let control = control.clone();
        let root = root.clone();
        let token = token.clone();
        std::thread::spawn(move || {
            control.with_native_commit(&root, &token, || {
                started_tx.send(()).unwrap();
                release_rx.recv_timeout(Duration::from_secs(3)).unwrap();
                Ok(())
            })
        })
    };
    started_rx.recv_timeout(Duration::from_secs(3)).unwrap();
    let (stopping_tx, stopping_rx) = mpsc::channel();
    let (stopped_tx, stopped_rx) = mpsc::channel();
    let stopper = {
        let control = control.clone();
        std::thread::spawn(move || {
            stopping_tx.send(()).unwrap();
            control.apply(
                DataMaintenancePrefs {
                    catalog_disabled: false,
                    native_processing_disabled: true,
                },
                true,
            );
            stopped_tx.send(()).unwrap();
        })
    };
    stopping_rx.recv_timeout(Duration::from_secs(3)).unwrap();
    assert!(matches!(
        stopped_rx.recv_timeout(Duration::from_millis(30)),
        Err(mpsc::RecvTimeoutError::Timeout)
    ));
    release_tx.send(()).unwrap();
    writer.join().unwrap().unwrap();
    stopped_rx.recv_timeout(Duration::from_secs(3)).unwrap();
    stopper.join().unwrap();
    assert!(token.is_cancelled());
    assert!(control
        .with_native_commit(&root, &token, || Ok(()))
        .is_err());
}
