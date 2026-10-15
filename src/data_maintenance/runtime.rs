//! A bounded coordinator owns discovery/eligibility while one serialized worker
//! performs compilation. Cancellation remains responsive during a build.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime};

use sha2::{Digest, Sha256};

use super::{discover_contexts, Context, MaintenanceControl};
use crate::beacon::{self, Environment};
use crate::catalog::Channel;
use crate::catalog_update::{
    AutomaticCatalogOutcome, CancellationToken, CatalogResolution, CatalogUpdateService,
};
use crate::database_query::DatabaseQueryService;

const COORDINATE_INTERVAL: Duration = Duration::from_millis(250);
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(2);
const RECONCILE_INTERVAL: Duration = Duration::from_secs(60);
const RETRY_MAX: Duration = Duration::from_secs(60);

#[derive(Clone, Copy)]
struct RuntimeTiming {
    coordinate: Duration,
    discovery: Duration,
    reconcile: Duration,
    retry_max: Duration,
}
impl Default for RuntimeTiming {
    fn default() -> Self {
        Self {
            coordinate: COORDINATE_INTERVAL,
            discovery: DISCOVERY_INTERVAL,
            reconcile: RECONCILE_INTERVAL,
            retry_max: RETRY_MAX,
        }
    }
}

#[derive(Default)]
struct Schedule {
    eligible: bool,
    environment: Option<Environment>,
    publish: bool,
    epoch: u64,
    busy: bool,
    next: Option<Instant>,
    signature: Option<(u64, Option<SystemTime>, Option<SystemTime>)>,
    last_reconcile: Option<Instant>,
    failures: u32,
    selected_path: Option<PathBuf>,
}

struct Job {
    context: Context,
    service: CatalogUpdateService,
    token: CancellationToken,
    state: Arc<Mutex<Schedule>>,
    publish: bool,
    epoch: u64,
    timing: RuntimeTiming,
}

/// Latest activation is coalesced, so a sleeping UI never loses the final selected
/// generation and a burst cannot allocate an unbounded event queue.
pub struct MaintenanceRuntime {
    stop: Arc<AtomicBool>,
    control: MaintenanceControl,
    latest: Arc<Mutex<Option<CatalogResolution>>>,
    coordinator: Option<JoinHandle<()>>,
    worker: Option<JoinHandle<()>>,
}

impl MaintenanceRuntime {
    pub fn spawn(
        config: PathBuf,
        bundled: PathBuf,
        control: MaintenanceControl,
        queries: DatabaseQueryService,
    ) -> Self {
        Self::spawn_inner(config, bundled, control, queries, None, None)
    }

    pub fn spawn_with_history(
        config: PathBuf,
        bundled: PathBuf,
        control: MaintenanceControl,
        queries: DatabaseQueryService,
        history: Option<crate::encounter::EncounterHistoryService>,
    ) -> Self {
        Self::spawn_inner(config, bundled, control, queries, None, history)
    }

    /// Injected context inventory for deterministic filesystem/service fixtures.
    pub fn spawn_with_contexts(
        config: PathBuf,
        bundled: PathBuf,
        control: MaintenanceControl,
        queries: DatabaseQueryService,
        contexts: Vec<Context>,
    ) -> Self {
        Self::spawn_inner(config, bundled, control, queries, Some(contexts), None)
    }

    fn spawn_inner(
        config: PathBuf,
        bundled: PathBuf,
        control: MaintenanceControl,
        queries: DatabaseQueryService,
        contexts: Option<Vec<Context>>,
        history: Option<crate::encounter::EncounterHistoryService>,
    ) -> Self {
        Self::spawn_inner_timed(
            config,
            bundled,
            control,
            queries,
            contexts,
            history,
            RuntimeTiming::default(),
        )
    }

    fn spawn_inner_timed(
        config: PathBuf,
        bundled: PathBuf,
        control: MaintenanceControl,
        queries: DatabaseQueryService,
        contexts: Option<Vec<Context>>,
        history: Option<crate::encounter::EncounterHistoryService>,
        timing: RuntimeTiming,
    ) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let latest = Arc::new(Mutex::new(None));
        let (send, receive) = mpsc::sync_channel::<Job>(1);
        let worker_stop = stop.clone();
        let worker_latest = latest.clone();
        let worker_config = config.clone();
        let worker_bundled = bundled.clone();
        let worker = thread::spawn(move || {
            if let Some(history) = &history {
                register_retained_registries(
                    &worker_config,
                    &worker_bundled,
                    history,
                    &worker_stop,
                );
            }
            while !worker_stop.load(Ordering::Acquire) {
                let job = match receive.recv_timeout(timing.coordinate) {
                    Ok(job) => job,
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                };
                if worker_stop.load(Ordering::Acquire) {
                    job.token.cancel();
                }
                run_job(job, &queries, &worker_latest, history.as_ref());
            }
        });
        let coordinator_stop = stop.clone();
        let coordinator_control = control.clone();
        let coordinator = thread::spawn(move || {
            coordinate(
                &config,
                &bundled,
                &coordinator_control,
                &coordinator_stop,
                send,
                contexts,
                timing,
            );
        });
        Self {
            stop,
            control,
            latest,
            coordinator: Some(coordinator),
            worker: Some(worker),
        }
    }

    pub fn take_activation(&self) -> Option<CatalogResolution> {
        self.latest.lock().unwrap_or_else(|e| e.into_inner()).take()
    }
}

impl Drop for MaintenanceRuntime {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        self.control.cancel_all_jobs();
        let deadline = Instant::now() + Duration::from_secs(2);
        for slot in [&mut self.coordinator, &mut self.worker] {
            if let Some(join) = slot.take() {
                while !join.is_finished() && Instant::now() < deadline {
                    thread::sleep(Duration::from_millis(10));
                }
                if join.is_finished() {
                    let _ = join.join();
                }
                // A bounded in-process compiler stage may finish after shutdown's
                // wait budget. Its token is cancelled and it cannot publish work.
            }
        }
    }
}

fn root_identity(root: &Path) -> String {
    let resolved = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    // AddOns may be shared through a junction while each game root has its own
    // SavedVariables. Include that source parent rather than merging profiles.
    let source_parent = root
        .parent()
        .map(|parent| fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf()))
        .unwrap_or_default();
    #[cfg(windows)]
    let bytes = {
        use std::os::windows::ffi::OsStrExt;
        source_parent
            .as_os_str()
            .encode_wide()
            .chain([0])
            .chain(resolved.as_os_str().encode_wide())
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>()
    };
    #[cfg(not(windows))]
    let bytes = {
        use std::os::unix::ffi::OsStrExt;
        let mut bytes = source_parent.as_os_str().as_bytes().to_vec();
        bytes.push(0);
        bytes.extend_from_slice(resolved.as_os_str().as_bytes());
        bytes
    };
    let digest = Sha256::digest(&bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn context_config(config: &Path, context: &Context, owns_default: bool) -> PathBuf {
    if owns_default {
        return config.to_path_buf();
    }
    let key = root_identity(&context.addons_root);
    config.join("data-contexts").join(key)
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryOwner {
    schema_version: u32,
    root_sha256: String,
}

fn default_registry_owner(config: &Path) -> Result<Option<String>, ()> {
    let parent = config.join("catalog");
    let path = parent.join("context-owner.json");
    if !path.exists() {
        return Ok(None);
    }
    let canonical = fs::canonicalize(&parent).map_err(|_| ())?;
    let bytes =
        crate::bounded_file::read_bounded_stable(&canonical, &path, 1024).map_err(|_| ())?;
    let owner: RegistryOwner = serde_json::from_slice(&bytes).map_err(|_| ())?;
    if owner.schema_version != 1
        || owner.root_sha256.len() != 64
        || !owner
            .root_sha256
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err(());
    }
    Ok(Some(owner.root_sha256))
}

fn claim_default_registry(config: &Path, root: &Path) -> std::io::Result<String> {
    let parent = config.join("catalog");
    fs::create_dir_all(&parent)?;
    let metadata = fs::symlink_metadata(&parent)?;
    if !metadata.is_dir() || crate::bounded_file::is_link_like(&metadata) {
        return Err(std::io::Error::other(
            "registry owner directory is not a real directory",
        ));
    }
    let identity = root_identity(root);
    let mut bytes = serde_json::to_vec_pretty(&RegistryOwner {
        schema_version: 1,
        root_sha256: identity.clone(),
    })?;
    bytes.push(b'\n');
    let mut temporary = tempfile::Builder::new()
        .prefix(".catalog-owner-")
        .tempfile_in(&parent)?;
    temporary.write_all(&bytes)?;
    temporary.as_file().sync_all()?;
    // Another process may already have bound this registry. Never replace it.
    temporary
        .persist_noclobber(parent.join("context-owner.json"))
        .map_err(|error| error.error)?;
    Ok(identity)
}

fn register_retained_registries(
    config: &Path,
    bundled: &Path,
    history: &crate::encounter::EncounterHistoryService,
    stop: &AtomicBool,
) {
    let mut roots = vec![config.to_path_buf()];
    if let Ok(entries) = fs::read_dir(config.join("data-contexts")) {
        roots.extend(entries.take(16).filter_map(Result::ok).filter_map(|entry| {
            let metadata = fs::symlink_metadata(entry.path()).ok()?;
            (metadata.is_dir() && !crate::bounded_file::is_link_like(&metadata))
                .then(|| entry.path())
        }));
    }
    for root in roots {
        for channel in [Channel::Live, Channel::Pts] {
            if stop.load(Ordering::Acquire) {
                return;
            }
            let resolution =
                CatalogUpdateService::new_for_channel(&root, bundled, channel).resolve_catalog();
            history.register_catalog_path(resolution.path);
        }
    }
}

fn reconcile_schedule(
    schedule: &mut Schedule,
    eligible: bool,
    environment: Option<Environment>,
    publish: bool,
) {
    if schedule.eligible != eligible
        || schedule.environment != environment
        || schedule.publish != publish
    {
        schedule.epoch = schedule.epoch.saturating_add(1);
        schedule.eligible = eligible;
        schedule.environment = environment;
        schedule.publish = publish;
        schedule.signature = None;
        schedule.last_reconcile = None;
        schedule.next = None;
        schedule.failures = 0;
    }
}

fn coordinate(
    config: &Path,
    bundled: &Path,
    control: &MaintenanceControl,
    stop: &AtomicBool,
    send: SyncSender<Job>,
    fixture_contexts: Option<Vec<Context>>,
    timing: RuntimeTiming,
) {
    let mut contexts = Vec::new();
    let mut last_discovery = None::<Instant>;
    let mut previous_roots = None;
    let mut schedules = BTreeMap::<PathBuf, Arc<Mutex<Schedule>>>::new();
    let mut migrations = BTreeMap::<PathBuf, Instant>::new();
    let mut config_attempts = BTreeMap::<PathBuf, (bool, Environment, Instant)>::new();
    let mut owner = default_registry_owner(config);
    let mut cursor = 0;
    while !stop.load(Ordering::Acquire) {
        let now = Instant::now();
        let current_roots = control.roots();
        if previous_roots.as_ref() != Some(&current_roots)
            || last_discovery.is_none_or(|last| now.duration_since(last) >= timing.discovery)
        {
            let previous = contexts;
            contexts = fixture_contexts
                .clone()
                .unwrap_or_else(|| discover_contexts(&current_roots));
            contexts.truncate(16);
            previous_roots = Some(current_roots);
            last_discovery = Some(now);
            for context in &contexts {
                if previous.iter().any(|old: &Context| {
                    old.addons_root == context.addons_root && old.environment != context.environment
                }) {
                    control.cancel_catalog_root(&context.addons_root);
                }
            }
            for root in schedules.keys() {
                if !contexts.iter().any(|context| context.addons_root == *root) {
                    control.cancel_catalog_root(root);
                }
            }
            schedules.retain(|root, state| {
                contexts.iter().any(|context| context.addons_root == *root)
                    || state.lock().unwrap_or_else(|e| e.into_inner()).busy
            });
            migrations
                .retain(|root, _| contexts.iter().any(|context| context.addons_root == *root));
            config_attempts
                .retain(|root, _| contexts.iter().any(|context| context.addons_root == *root));
        }
        let prefs = control.preferences();
        let mut eligible = Vec::new();
        for context in &contexts {
            if migrations
                .get(&context.addons_root)
                .is_none_or(|last| now.duration_since(*last) >= timing.retry_max)
            {
                migrations.insert(context.addons_root.clone(), now);
                if beacon::migrate_legacy_if_needed(
                    &context.addons_root,
                    prefs.catalog_disabled,
                    context.environment,
                )
                .is_err()
                {
                    tracing::warn!(target: "eso_weave::data_maintenance", "combined package migration deferred; prior data preserved");
                }
            }
            let presence = beacon::inspect_presence(&context.addons_root).ok();
            if presence.is_some_and(|facts| facts.eligible()) {
                eligible.push(context.clone());
                if presence
                    .and_then(|facts| facts.startup_config)
                    .is_some_and(|startup| {
                        startup.catalog_disabled != prefs.catalog_disabled
                            || startup.environment != context.environment
                    })
                {
                    // The exact config-only transaction also supports compatible
                    // manual packages without adopting their other files.
                    if config_attempts.get(&context.addons_root).is_none_or(
                        |(disabled, environment, last)| {
                            *disabled != prefs.catalog_disabled
                                || *environment != context.environment
                                || now.duration_since(*last) >= timing.retry_max
                        },
                    ) {
                        config_attempts.insert(
                            context.addons_root.clone(),
                            (prefs.catalog_disabled, context.environment, now),
                        );
                        if beacon::write_startup_config(
                            &context.addons_root,
                            prefs.catalog_disabled,
                            context.environment,
                        )
                        .is_err()
                        {
                            tracing::warn!(target:"eso_weave::data_maintenance","Catalog startup preference could not be written; requested desktop preference remains active and retry is bounded");
                        }
                    }
                }
            } else {
                control.cancel_catalog_root(&context.addons_root);
            }
        }
        control.set_presence(!eligible.is_empty());
        let startup_persisted = (!eligible.is_empty()).then(|| {
            eligible.iter().all(|context| {
                beacon::inspect_presence(&context.addons_root)
                    .ok()
                    .and_then(|facts| facts.startup_config)
                    .is_some_and(|startup| {
                        startup.catalog_disabled == prefs.catalog_disabled
                            && startup.environment == context.environment
                    })
            })
        });
        control.report_startup_config_persisted(startup_persisted);
        let selected_live = eligible
            .iter()
            .find(|context| context.environment == Environment::Live)
            .map(|context| context.addons_root.clone());
        if matches!(owner, Ok(None)) {
            if let Some(root) = &selected_live {
                owner = claim_default_registry(config, root)
                    .map(Some)
                    .map_err(|_| ());
                if owner.is_err() {
                    owner = default_registry_owner(config);
                }
            }
        }
        for (root, state) in &schedules {
            let context = contexts.iter().find(|context| context.addons_root == *root);
            let available = !prefs.catalog_disabled
                && eligible.iter().any(|context| context.addons_root == *root);
            let mut state = state.lock().unwrap_or_else(|e| e.into_inner());
            reconcile_schedule(
                &mut state,
                available,
                context.map(|context| context.environment),
                selected_live.as_ref() == Some(root),
            );
        }
        if !prefs.catalog_disabled {
            let start_cursor = cursor;
            for offset in 0..eligible.len() {
                let index = (start_cursor + offset) % eligible.len();
                let context = eligible[index].clone();
                let state = schedules
                    .entry(context.addons_root.clone())
                    .or_default()
                    .clone();
                let mut schedule = state.lock().unwrap_or_else(|e| e.into_inner());
                let publish = selected_live.as_ref() == Some(&context.addons_root);
                reconcile_schedule(&mut schedule, true, Some(context.environment), publish);
                if schedule.busy || schedule.next.is_some_and(|next| next > now) {
                    continue;
                }
                let channel = if context.environment == Environment::Live {
                    Channel::Live
                } else {
                    Channel::Pts
                };
                let owns_default = owner
                    .as_ref()
                    .ok()
                    .and_then(|owner| owner.as_ref())
                    .is_some_and(|owner| {
                        context.environment == Environment::Live
                            && *owner == root_identity(&context.addons_root)
                    });
                let service = CatalogUpdateService::new_for_channel(
                    context_config(config, &context, owns_default),
                    bundled,
                    channel,
                );
                let token = control.register_catalog_job(&context.addons_root);
                let job = Job {
                    context,
                    service,
                    token,
                    state: state.clone(),
                    publish,
                    epoch: schedule.epoch,
                    timing,
                };
                schedule.busy = true;
                if send.try_send(job).is_err() {
                    schedule.busy = false;
                    break;
                }
                cursor = (index + 1) % eligible.len();
            }
        }
        thread::sleep(timing.coordinate);
    }
}

fn run_job(
    job: Job,
    queries: &DatabaseQueryService,
    latest: &Mutex<Option<CatalogResolution>>,
    history: Option<&crate::encounter::EncounterHistoryService>,
) {
    run_job_with_progress(job, queries, latest, history, |_| {});
}

fn publish_resolution(
    job: &Job,
    resolution: CatalogResolution,
    queries: &DatabaseQueryService,
    latest: &Mutex<Option<CatalogResolution>>,
    history: Option<&crate::encounter::EncounterHistoryService>,
    force_event: bool,
) {
    if let Some(history) = history {
        history.register_catalog_path(resolution.path.clone());
    }
    if job.publish {
        queries.set_catalog_path(resolution.path.clone());
        if let Some(history) = history {
            history.set_catalog_path(resolution.path.clone());
        }
        let mut state = job.state.lock().unwrap_or_else(|e| e.into_inner());
        let changed = state.selected_path.as_ref() != Some(&resolution.path);
        state.selected_path = Some(resolution.path.clone());
        if changed || force_event {
            *latest.lock().unwrap_or_else(|e| e.into_inner()) = Some(resolution);
        }
    }
}

fn run_job_with_progress(
    job: Job,
    queries: &DatabaseQueryService,
    latest: &Mutex<Option<CatalogResolution>>,
    history: Option<&crate::encounter::EncounterHistoryService>,
    mut observe: impl FnMut(crate::catalog_update::UpdateProgress),
) {
    let started = Instant::now();
    let path = crate::data_addon::saved_variables_path(&job.context.addons_root);
    let mut reconciled = false;
    let mut signature = None;
    let outcome = (|| {
        if job.token.is_cancelled() {
            return Ok(());
        }
        let validate_selection = {
            let state = job.state.lock().unwrap_or_else(|e| e.into_inner());
            state
                .last_reconcile
                .is_none_or(|last| started.duration_since(last) >= job.timing.reconcile)
        };
        if validate_selection {
            // Retained reads remain available when game-owned source data is
            // missing, partial, or locked. Every context registers its history.
            let resolution = job.service.resolve_catalog();
            if !job.token.is_cancelled() {
                publish_resolution(&job, resolution, queries, latest, history, false);
            }
            reconciled = true;
            if job
                .state
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .last_reconcile
                .is_none()
            {
                job.service.recover_staging()?;
            }
        }
        let Some(path) = path else {
            return Ok(());
        };
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(crate::catalog_update::UpdateError::Io(error)),
        };
        let observed = (
            metadata.len(),
            metadata.modified().ok(),
            metadata.created().ok(),
        );
        {
            let state = job.state.lock().unwrap_or_else(|e| e.into_inner());
            if state.signature == Some(observed)
                && state
                    .last_reconcile
                    .is_some_and(|last| started.duration_since(last) < job.timing.reconcile)
            {
                return Ok(());
            }
        }
        let token = job.token.clone();
        let root = job.context.addons_root.clone();
        let outcome = job
            .service
            .maintain_collector_capture(&path, &token, |progress| {
                if !beacon::inspect_presence(&root)
                    .ok()
                    .is_some_and(|facts| facts.eligible())
                {
                    token.cancel();
                }
                observe(progress);
            })?;
        let committed = matches!(outcome, AutomaticCatalogOutcome::Activated(_));
        // An Activated result proves that the short atomic selection boundary
        // completed. A later stop must not strand readers on the older path.
        if committed || !token.is_cancelled() {
            let resolution = job.service.resolve_catalog();
            publish_resolution(&job, resolution, queries, latest, history, committed);
        }
        signature = Some(observed);
        reconciled = true;
        Ok::<(), crate::catalog_update::UpdateError>(())
    })();
    let finished = Instant::now();
    let mut state = job.state.lock().unwrap_or_else(|e| e.into_inner());
    state.busy = false;
    if state.epoch != job.epoch {
        return;
    }
    if outcome.is_err() && !job.token.is_cancelled() {
        state.failures = state.failures.saturating_add(1).min(6);
        let delay = Duration::from_secs(1_u64 << state.failures).min(job.timing.retry_max);
        state.next = Some(finished + delay);
        if state.failures == 1 {
            tracing::warn!(target: "eso_weave::data_maintenance", "completed Catalog input unavailable or rejected; last verified generation retained, retry bounded");
        }
    } else {
        if !job.token.is_cancelled() {
            if let Some(signature) = signature {
                state.signature = Some(signature);
            }
            if reconciled {
                state.last_reconcile = Some(finished);
            }
        }
        state.failures = 0;
        state.next = Some(finished + job.timing.discovery);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beacon::RunningState;
    use crate::catalog::compiler::{build_catalog, BuildRequest};
    use crate::catalog_update::UpdateStage;

    fn job_fixture() -> (tempfile::TempDir, Job, DatabaseQueryService) {
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
        let capture = crate::data_addon::saved_variables_path(&addons).unwrap();
        std::fs::create_dir_all(capture.parent().unwrap()).unwrap();
        std::fs::copy(
            "specs/092-data-addon-foundation/fixtures/catalog-live.lua",
            &capture,
        )
        .unwrap();
        let bundled = root.path().join("bundled.sqlite");
        build_catalog(&BuildRequest::new(
            "assets/catalog/baseline.json",
            &bundled,
            Channel::Live,
        ))
        .unwrap();
        let service = CatalogUpdateService::new(root.path().join("config"), &bundled);
        let queries = DatabaseQueryService::new(bundled, None);
        let job = Job {
            context: Context {
                addons_root: addons,
                environment: Environment::Live,
            },
            service,
            token: CancellationToken::new(),
            state: Arc::new(Mutex::new(Schedule::default())),
            publish: true,
            epoch: 0,
            timing: RuntimeTiming::default(),
        };
        (root, job, queries)
    }

    #[test]
    fn committed_generation_publishes_even_when_stop_arrives_after_commit() {
        let (_root, job, queries) = job_fixture();
        let service = job.service.clone();
        let token = job.token.clone();
        let latest = Mutex::new(None);
        run_job_with_progress(job, &queries, &latest, None, |progress| {
            if progress.stage == UpdateStage::Complete {
                token.cancel();
            }
        });
        let committed = service.resolve_catalog();
        assert!(committed
            .access
            .release()
            .unwrap()
            .unwrap()
            .catalog_version
            .starts_with("local-collector-"));
        assert_eq!(
            latest.lock().unwrap().as_ref().unwrap().path,
            committed.path
        );
    }

    fn timing() -> RuntimeTiming {
        RuntimeTiming {
            coordinate: Duration::from_millis(10),
            discovery: Duration::from_millis(25),
            reconcile: Duration::from_millis(120),
            retry_max: Duration::from_millis(100),
        }
    }

    fn wait_until(mut condition: impl FnMut() -> bool) {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !condition() {
            assert!(
                Instant::now() < deadline,
                "runtime condition did not arrive"
            );
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn completed_capture(ability: i64, stamp: &str, channel: Channel) -> String {
        let source =
            fs::read_to_string("specs/092-data-addon-foundation/fixtures/catalog-live.lua")
                .unwrap();
        let envelope = crate::collector::parse_capture(source.as_bytes()).unwrap();
        let old = &envelope.chunks[0];
        let mut records = envelope.records;
        for record in &mut records {
            if record.kind == crate::catalog::EntityKind::Ability && record.stable_id == 100 {
                record.stable_id = ability;
                record.source_key = format!("player-skills/ability/{ability}");
            }
        }
        let payload = records
            .iter()
            .map(|record| serde_json::to_string(record).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        source
            .replace(
                &format!(
                    "[\"payload\"] = {}",
                    serde_json::to_string(&old.payload).unwrap()
                ),
                &format!(
                    "[\"payload\"] = {}",
                    serde_json::to_string(&payload).unwrap()
                ),
            )
            .replace(
                &format!("[\"byte_count\"] = {}", old.byte_count),
                &format!("[\"byte_count\"] = {}", payload.len()),
            )
            .replace(
                &format!("[\"checksum\"] = \"{}\"", old.checksum),
                &format!(
                    "[\"checksum\"] = \"{}\"",
                    crate::collector::adler32_hex(payload.as_bytes())
                ),
            )
            .replace(
                "[\"finished_at\"] = \"2026-09-09T12:00:01Z\"",
                &format!("[\"finished_at\"] = \"{stamp}\""),
            )
            .replace(
                "[\"channel\"] = \"live\"",
                &format!("[\"channel\"] = \"{channel}\""),
            )
    }

    fn repeat(job: &Job) -> Job {
        Job {
            context: job.context.clone(),
            service: job.service.clone(),
            token: CancellationToken::new(),
            state: job.state.clone(),
            publish: job.publish,
            epoch: job.epoch,
            timing: job.timing,
        }
    }

    #[test]
    fn partial_source_retries_from_completion_and_repair_clears_backoff() {
        let (_root, mut job, queries) = job_fixture();
        job.timing = timing();
        let source = crate::data_addon::saved_variables_path(&job.context.addons_root).unwrap();
        fs::write(&source, b"EsoWeaveDataSaved={").unwrap();
        let latest = Mutex::new(None);
        for expected in 1..=7 {
            run_job(repeat(&job), &queries, &latest, None);
            let now = Instant::now();
            let state = job.state.lock().unwrap();
            assert_eq!(state.failures, expected.min(6));
            assert!(state.next.unwrap() > now);
            assert!(state.next.unwrap().duration_since(now) <= job.timing.retry_max);
        }
        assert!(matches!(
            job.service.load_selection().unwrap().active,
            crate::catalog_update::CatalogTarget::Bundled
        ));
        fs::write(
            source,
            completed_capture(100, "2026-09-09T12:00:01Z", Channel::Live),
        )
        .unwrap();
        run_job(repeat(&job), &queries, &latest, None);
        assert_eq!(job.state.lock().unwrap().failures, 0);
        assert!(job
            .service
            .resolve_catalog()
            .access
            .ability(100)
            .unwrap()
            .is_some());
    }

    #[test]
    fn lost_notifications_and_identical_metadata_are_reconciled_periodically() {
        let (_root, mut job, queries) = job_fixture();
        job.timing = timing();
        let latest = Mutex::new(None);
        let source = crate::data_addon::saved_variables_path(&job.context.addons_root).unwrap();
        fs::write(
            &source,
            completed_capture(100, "2026-09-09T12:00:01Z", Channel::Live),
        )
        .unwrap();
        run_job(repeat(&job), &queries, &latest, None);
        let first = job.service.load_selection().unwrap();
        let metadata = fs::metadata(&source).unwrap();
        let next = completed_capture(200, "2026-10-15T12:00:01Z", Channel::Live);
        assert_eq!(next.len() as u64, metadata.len());
        fs::write(&source, next).unwrap();
        fs::File::options()
            .write(true)
            .open(&source)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(metadata.modified().unwrap()))
            .unwrap();
        run_job(repeat(&job), &queries, &latest, None);
        assert_eq!(
            job.service.load_selection().unwrap(),
            first,
            "metadata fast path must defer until its bounded full reconcile"
        );
        job.state.lock().unwrap().last_reconcile = Some(Instant::now() - job.timing.reconcile);
        run_job(repeat(&job), &queries, &latest, None);
        assert!(job
            .service
            .resolve_catalog()
            .access
            .ability(200)
            .unwrap()
            .is_some());
        let accepted = job.service.load_selection().unwrap();
        job.state.lock().unwrap().last_reconcile = Some(Instant::now() - job.timing.reconcile);
        run_job(repeat(&job), &queries, &latest, None);
        assert_eq!(
            job.service.load_selection().unwrap(),
            accepted,
            "semantic replay must not select another generation"
        );
    }

    #[test]
    fn disappeared_package_and_precommit_stop_cannot_select_new_data() {
        for remove_package in [false, true] {
            let (_root, job, queries) = job_fixture();
            let latest = Mutex::new(None);
            let token = job.token.clone();
            let service = job.service.clone();
            let root = job.context.addons_root.clone();
            run_job_with_progress(job, &queries, &latest, None, |progress| {
                if remove_package && progress.stage == UpdateStage::Building {
                    beacon::uninstall(&root, RunningState::NotRunning).unwrap();
                }
                if !remove_package && progress.stage == UpdateStage::Committing {
                    token.cancel();
                }
            });
            assert!(matches!(
                service.load_selection().unwrap().active,
                crate::catalog_update::CatalogTarget::Bundled
            ));
        }
    }

    #[test]
    fn eligibility_epoch_invalidates_cache_and_old_job_cannot_reinstate_it() {
        let (_root, job, queries) = job_fixture();
        let latest = Mutex::new(None);
        let state = job.state.clone();
        run_job_with_progress(job, &queries, &latest, None, |progress| {
            if progress.stage == UpdateStage::Building {
                let mut state = state.lock().unwrap();
                reconcile_schedule(&mut state, true, Some(Environment::Live), true);
            }
        });
        let state = state.lock().unwrap();
        assert!(!state.busy);
        assert!(state.signature.is_none());
        assert!(state.last_reconcile.is_none());
        assert!(state.next.is_none());
    }

    #[test]
    fn primary_removal_and_discovery_reordering_never_rebind_another_roots_registry() {
        let (root, job, queries) = job_fixture();
        let addons_a = job.context.addons_root.clone();
        let addons_b = root.path().join("another/live/AddOns");
        fs::create_dir_all(&addons_b).unwrap();
        beacon::install_configured(
            &addons_b,
            RunningState::NotRunning,
            101050,
            3,
            false,
            Environment::Live,
        )
        .unwrap();
        let source = crate::data_addon::saved_variables_path(&addons_b).unwrap();
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        fs::write(
            source,
            completed_capture(200, "2026-09-09T12:00:01Z", Channel::Live),
        )
        .unwrap();
        let config = root.path().join("config");
        let bundled = root.path().join("bundled.sqlite");
        let control = MaintenanceControl::new(super::super::DataMaintenancePrefs::default());
        let contexts = vec![
            Context {
                addons_root: addons_a.clone(),
                environment: Environment::Live,
            },
            Context {
                addons_root: addons_b.clone(),
                environment: Environment::Live,
            },
        ];
        let secondary =
            CatalogUpdateService::new(context_config(&config, &contexts[1], false), &bundled);
        let runtime = MaintenanceRuntime::spawn_inner_timed(
            config.clone(),
            bundled.clone(),
            control,
            queries.clone(),
            Some(contexts.clone()),
            None,
            timing(),
        );
        wait_until(|| {
            job.service
                .resolve_catalog()
                .access
                .ability(100)
                .ok()
                .flatten()
                .is_some()
                && secondary
                    .resolve_catalog()
                    .access
                    .ability(200)
                    .ok()
                    .flatten()
                    .is_some()
        });
        let original = job.service.load_selection().unwrap();
        let secondary_path = secondary.resolve_catalog().path;
        beacon::uninstall(&addons_a, RunningState::NotRunning).unwrap();
        wait_until(|| {
            runtime
                .take_activation()
                .is_some_and(|resolution| resolution.path == secondary_path)
        });
        assert_eq!(job.service.load_selection().unwrap(), original);
        assert!(secondary
            .resolve_catalog()
            .access
            .ability(100)
            .unwrap()
            .is_none());
        drop(runtime);
        let runtime = MaintenanceRuntime::spawn_inner_timed(
            config.clone(),
            bundled,
            MaintenanceControl::new(super::super::DataMaintenancePrefs::default()),
            queries,
            Some(contexts.into_iter().rev().collect()),
            None,
            timing(),
        );
        wait_until(|| {
            runtime
                .take_activation()
                .is_some_and(|resolution| resolution.path == secondary_path)
        });
        assert_eq!(
            default_registry_owner(&config).unwrap(),
            Some(root_identity(&addons_a))
        );
        assert_eq!(job.service.load_selection().unwrap(), original);
        drop(runtime);
    }

    #[test]
    fn readonly_startup_preference_reports_failure_then_retries_without_ownership_adoption() {
        let (root, job, queries) = job_fixture();
        let manifest = job.context.addons_root.join("PixelBeacon/PixelBeacon.txt");
        fs::write(
            &manifest,
            fs::read_to_string(&manifest)
                .unwrap()
                .replace(beacon::MANAGED_MARKER, ""),
        )
        .unwrap();
        let config_file = job
            .context
            .addons_root
            .join("PixelBeacon")
            .join(beacon::STARTUP_CONFIG_FILE);
        let original = fs::metadata(&config_file).unwrap().permissions();
        let mut readonly = original.clone();
        readonly.set_readonly(true);
        fs::set_permissions(&config_file, readonly).unwrap();
        let control = MaintenanceControl::new(super::super::DataMaintenancePrefs {
            catalog_disabled: true,
            native_processing_disabled: false,
        });
        let runtime = MaintenanceRuntime::spawn_inner_timed(
            root.path().join("config"),
            root.path().join("bundled.sqlite"),
            control.clone(),
            queries,
            Some(vec![job.context.clone()]),
            None,
            timing(),
        );
        wait_until(|| control.startup_config_persisted() == Some(false));
        assert!(control.preferences().catalog_disabled);
        assert!(control.persisted());
        assert!(control.controls_enabled());
        fs::set_permissions(&config_file, original).unwrap();
        wait_until(|| control.startup_config_persisted() == Some(true));
        assert_eq!(
            beacon::inspect(&job.context.addons_root).unwrap(),
            beacon::BeaconStatus::Unmanaged
        );
        assert!(!fs::read_to_string(manifest)
            .unwrap()
            .contains(beacon::MANAGED_MARKER));
        drop(runtime);
    }
}
