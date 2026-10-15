//! Independent bounded native-log discovery and incremental ingest, without UI events.

use std::collections::BTreeMap;
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

use super::{discover_contexts, Context, MaintenanceControl, NativeServicePhase};
use crate::beacon;
use crate::bounded_file::{read_append_range, MAX_APPEND_READ};
use crate::catalog::Channel;
use crate::catalog_update::CancellationToken;
use crate::encounter::{
    EncounterHistoryService, EncounterProcessor, IncrementalNativeParser, NativeCheckpoint,
    NativeIngestBatch, NativeIngestWriter,
};

const MAX_SOURCES: usize = 64;
const MAX_CACHE_BYTES: usize = 128 * 1024 * 1024;
const DIRECTORY_TURN: usize = 256;
const DISCOVERY_INTERVAL: Duration = Duration::from_secs(2);
const TICK: Duration = Duration::from_millis(50);

struct Cached {
    parser: IncrementalNativeParser,
    revision: Option<u64>,
    witness: Option<String>,
    encoded_bytes: usize,
}

struct Source {
    root: PathBuf,
    logs: PathBuf,
    path: PathBuf,
    channel: Channel,
    key: String,
    token: CancellationToken,
    cache: Option<Cached>,
    next: Instant,
    failures: u32,
}

struct Scan {
    context: Context,
    entries: Option<ReadDir>,
    token: CancellationToken,
}

/// Native lifetime is independent of Catalog compilation and interface visibility.
pub struct NativeMaintenanceRuntime {
    stop: Arc<AtomicBool>,
    control: MaintenanceControl,
    worker: Option<JoinHandle<()>>,
}

impl NativeMaintenanceRuntime {
    pub fn spawn(history: EncounterHistoryService, control: MaintenanceControl) -> Self {
        Self::spawn_inner(history, control, None)
    }

    /// Deterministic supported-root seam; production uses actual platform discovery.
    pub fn spawn_with_contexts(
        history: EncounterHistoryService,
        control: MaintenanceControl,
        contexts: Vec<Context>,
    ) -> Self {
        Self::spawn_inner(history, control, Some(contexts))
    }

    fn spawn_inner(
        history: EncounterHistoryService,
        control: MaintenanceControl,
        contexts: Option<Vec<Context>>,
    ) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = stop.clone();
        let worker_control = control.clone();
        let worker = thread::spawn(move || run(history, worker_control, worker_stop, contexts));
        Self {
            stop,
            control,
            worker: Some(worker),
        }
    }
}

impl Drop for NativeMaintenanceRuntime {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        self.control.cancel_native_jobs();
        let deadline = Instant::now() + Duration::from_secs(2);
        if let Some(worker) = self.worker.take() {
            while !worker.is_finished() && Instant::now() < deadline {
                thread::sleep(Duration::from_millis(10));
            }
            if worker.is_finished() {
                let _ = worker.join();
            }
        }
    }
}

fn hash(value: &[u8]) -> String {
    Sha256::digest(value)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn source_key(root: &Path, path: &Path, channel: Channel) -> String {
    let root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    let path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let mut digest = Sha256::new();
    digest.update(b"native-source/v1");
    for component in [
        root.as_os_str().as_encoded_bytes(),
        path.as_os_str().as_encoded_bytes(),
        channel.as_str().as_bytes(),
    ] {
        digest.update((component.len() as u64).to_be_bytes());
        digest.update(component);
    }
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn is_native_name(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            let name = name.to_ascii_lowercase();
            name.ends_with(".log")
                && (name == "encounter.log"
                    || name.starts_with("encounter_")
                    || name.starts_with("encounter-"))
        })
}

fn scan_sources(scans: &mut BTreeMap<PathBuf, Scan>, sources: &mut Vec<Source>) {
    let per_root = (MAX_SOURCES / scans.len().max(1)).max(1);
    let mut retained = BTreeMap::<PathBuf, usize>::new();
    sources.retain(|source| {
        let count = retained.entry(source.root.clone()).or_default();
        *count += 1;
        *count <= per_root
    });
    for scan in scans.values_mut() {
        if sources.len() >= MAX_SOURCES {
            break;
        }
        let Some(parent) = scan.context.addons_root.parent() else {
            continue;
        };
        let logs = parent.join("Logs");
        let Ok(metadata) = fs::symlink_metadata(&logs) else {
            continue;
        };
        if !metadata.is_dir() || crate::bounded_file::is_link_like(&metadata) {
            continue;
        }
        let Ok(canonical) = fs::canonicalize(&logs) else {
            continue;
        };
        if scan.entries.is_none() {
            scan.entries = fs::read_dir(&logs).ok();
        }
        let Some(entries) = scan.entries.as_mut() else {
            continue;
        };
        let mut exhausted = false;
        for _ in 0..DIRECTORY_TURN {
            if sources.len() >= MAX_SOURCES {
                break;
            }
            if sources
                .iter()
                .filter(|source| source.root == scan.context.addons_root)
                .count()
                >= per_root
            {
                break;
            }
            let entry = match entries.next() {
                Some(Ok(entry)) => entry,
                Some(Err(_)) => continue,
                None => {
                    exhausted = true;
                    break;
                }
            };
            let path = entry.path();
            if !is_native_name(&path)
                || sources
                    .iter()
                    .any(|source| source.path == path && source.root == scan.context.addons_root)
            {
                continue;
            }
            let Ok(metadata) = fs::symlink_metadata(&path) else {
                continue;
            };
            if !metadata.is_file() || crate::bounded_file::is_link_like(&metadata) {
                continue;
            }
            let channel = match scan.context.environment {
                beacon::Environment::Live => Channel::Live,
                beacon::Environment::Pts => Channel::Pts,
            };
            let token = scan.token.clone();
            if token.is_cancelled() {
                continue;
            }
            sources.push(Source {
                key: source_key(&scan.context.addons_root, &path, channel),
                root: scan.context.addons_root.clone(),
                logs: canonical.clone(),
                path,
                channel,
                token,
                cache: None,
                next: Instant::now(),
                failures: 0,
            });
        }
        if exhausted {
            scan.entries = None;
        }
    }
}

fn restore(source: &Source, writer: &NativeIngestWriter) -> Result<Cached, ()> {
    let checkpoint = writer.checkpoint(&source.key).map_err(|_| ())?;
    match checkpoint {
        Some(checkpoint) => {
            let parsed =
                NativeCheckpoint::from_bytes(&checkpoint.checkpoint_bytes).map_err(|_| ())?;
            let parser =
                IncrementalNativeParser::restore(&parsed, source.channel).map_err(|_| ())?;
            if parser.position().accepted_bytes != checkpoint.consumed_bytes {
                return Err(());
            }
            Ok(Cached {
                parser,
                revision: Some(checkpoint.revision),
                witness: Some(checkpoint.generation),
                encoded_bytes: checkpoint.checkpoint_bytes.len(),
            })
        }
        None => Ok(Cached {
            parser: IncrementalNativeParser::new(source.channel),
            revision: None,
            witness: None,
            encoded_bytes: 0,
        }),
    }
}

enum Turn {
    Progress(u64, bool),
    Idle,
}

fn ingest_turn(
    source: &mut Source,
    writer: &mut NativeIngestWriter,
    control: &MaintenanceControl,
) -> Result<Turn, ()> {
    if source.token.is_cancelled() {
        source.cache = None;
        return Ok(Turn::Idle);
    }
    let mut cache = match source.cache.take() {
        Some(cache) => cache,
        None => restore(source, writer)?,
    };
    let position = cache.parser.position().accepted_bytes;
    let snapshot = read_append_range(&source.logs, &source.path, position, MAX_APPEND_READ);
    let truncated = matches!(
        snapshot,
        Err(crate::bounded_file::StableReadError::Invalid(
            "native source was truncated"
        ))
    );
    let generation_changed = match &snapshot {
        Ok(snapshot) => cache
            .witness
            .as_ref()
            .is_some_and(|expected| snapshot.witness(0).ok().as_ref() != Some(expected)),
        Err(_) if truncated => true,
        Err(crate::bounded_file::StableReadError::Io(error))
            if error.kind() == std::io::ErrorKind::NotFound =>
        {
            // Rotation may remove an old pathname permanently. Its durable checkpoint
            // remains available if that pathname returns, but it must release a slot.
            return Ok(Turn::Idle);
        }
        Err(_) => return Err(()),
    };
    let snapshot = if generation_changed {
        // Retain terminal pending evidence separately before replacing its source checkpoint.
        if let Some(previous) = &cache.witness {
            let archive_key =
                hash(format!("native-terminated/v1:{}:{previous}", source.key).as_bytes());
            if writer.checkpoint(&archive_key).map_err(|_| ())?.is_none() {
                let reason = if truncated {
                    crate::encounter::NativeTerminationReason::Truncated
                } else {
                    crate::encounter::NativeTerminationReason::Replaced
                };
                cache.parser.terminate_generation(reason).map_err(|_| ())?;
                let checkpoint = cache.parser.checkpoint().map_err(|_| ())?;
                writer
                    .commit_batch(
                        NativeIngestBatch {
                            source_key: archive_key,
                            generation: previous.clone(),
                            expected_revision: None,
                            consumed_bytes: position,
                            checkpoint_bytes: checkpoint.as_bytes().to_vec(),
                            completed: None,
                        },
                        |commit| {
                            control
                                .with_native_commit(&source.root, &source.token, || commit.commit())
                        },
                    )
                    .map_err(|_| ())?;
            }
        }
        cache.parser = IncrementalNativeParser::new(source.channel);
        cache.witness = None;
        read_append_range(&source.logs, &source.path, 0, MAX_APPEND_READ).map_err(|_| ())?
    } else {
        snapshot.map_err(|_| ())?
    };
    if snapshot.bytes.is_empty() {
        source.cache = Some(cache);
        return Ok(Turn::Idle);
    }
    let feed = cache.parser.feed(&snapshot.bytes).map_err(|_| ())?;
    if feed.accepted == 0 {
        return Err(());
    }
    let skipped = !feed.diagnostics.is_empty();
    let witness = snapshot.witness(feed.accepted).map_err(|_| ())?;
    let checkpoint = cache.parser.checkpoint().map_err(|_| ())?;
    let bytes = checkpoint.as_bytes().to_vec();
    let encoded_bytes = bytes.len();
    let receipt = writer
        .commit_batch(
            NativeIngestBatch {
                source_key: source.key.clone(),
                generation: witness.clone(),
                expected_revision: cache.revision,
                consumed_bytes: cache.parser.position().accepted_bytes,
                checkpoint_bytes: bytes,
                completed: feed.completed,
            },
            |commit| control.with_native_commit(&source.root, &source.token, || commit.commit()),
        )
        .map_err(|_| ())?;
    cache.revision = Some(receipt.revision);
    cache.witness = Some(witness);
    cache.encoded_bytes = encoded_bytes;
    source.cache = Some(cache);
    Ok(Turn::Progress(receipt.imported_count as u64, skipped))
}

fn run(
    history: EncounterHistoryService,
    control: MaintenanceControl,
    stop: Arc<AtomicBool>,
    fixtures: Option<Vec<Context>>,
) {
    let mut sources = Vec::<Source>::new();
    let mut scans = BTreeMap::<PathBuf, Scan>::new();
    let mut writer = None::<NativeIngestWriter>;
    let mut processor = None::<(EncounterProcessor, CancellationToken)>;
    let mut last_discovery = None::<Instant>;
    let mut previous_roots = None;
    let mut cursor = 0usize;
    let mut store_retry = Instant::now();
    let mut store_failures = 0u32;
    let mut processor_retry = Instant::now();
    let mut processor_failures = 0u32;
    while !stop.load(Ordering::Acquire) {
        let now = Instant::now();
        let roots = control.roots();
        let disabled =
            control.preferences().native_processing_disabled || !control.controls_enabled();
        if disabled {
            sources.clear();
            scans.clear();
            writer = None;
            processor = None;
            last_discovery = None;
            control.report_native(NativeServicePhase::Dormant, 0, 0, None);
            thread::sleep(TICK);
            continue;
        }
        if previous_roots.as_ref() != Some(&roots)
            || last_discovery.is_none_or(|last| now.duration_since(last) >= DISCOVERY_INTERVAL)
        {
            let contexts = fixtures
                .clone()
                .unwrap_or_else(|| discover_contexts(&roots));
            let mut eligible = Vec::new();
            for context in contexts.into_iter().take(16) {
                if beacon::inspect_presence(&context.addons_root)
                    .ok()
                    .is_some_and(|presence| presence.eligible())
                {
                    eligible.push(context);
                } else {
                    control.cancel_native_root(&context.addons_root);
                }
            }
            for (root, scan) in &scans {
                if !eligible.iter().any(|context| {
                    context.addons_root == *root && context.environment == scan.context.environment
                }) {
                    control.cancel_native_root(root);
                }
            }
            scans.retain(|root, scan| {
                eligible.iter().any(|context| {
                    context.addons_root == *root && context.environment == scan.context.environment
                })
            });
            sources.retain(|source| {
                !source.token.is_cancelled()
                    && eligible
                        .iter()
                        .any(|context| context.addons_root == source.root)
            });
            for context in eligible {
                let token = control.register_native_job(&context.addons_root);
                let scan = scans.entry(context.addons_root.clone()).or_insert(Scan {
                    context,
                    entries: None,
                    token: token.clone(),
                });
                if scan.token.is_cancelled() {
                    scan.token = token;
                }
            }
            scan_sources(&mut scans, &mut sources);
            previous_roots = Some(roots);
            last_discovery = Some(now);
        }
        if scans.is_empty() {
            sources.clear();
            writer = None;
            processor = None;
            control.report_native(NativeServicePhase::Dormant, 0, 0, None);
            thread::sleep(TICK);
            continue;
        }
        if writer.is_none()
            && now >= store_retry
            && (!sources.is_empty() || history.store_path().is_file())
        {
            let opening_scan = scans.values().next().expect("eligible scans checked above");
            let opening_token = opening_scan.token.clone();
            match NativeIngestWriter::open_with_finalizer(
                history.store_path(),
                opening_token.clone(),
                |commit| {
                    control.with_native_commit(
                        &opening_scan.context.addons_root,
                        &opening_token,
                        || commit.commit(),
                    )
                },
            ) {
                Ok(opened) => {
                    writer = Some(opened);
                    store_failures = 0;
                }
                Err(_) => {
                    store_failures = store_failures.saturating_add(1);
                    store_retry = now + backoff(store_failures);
                }
            }
        }
        let mut phase = NativeServicePhase::Waiting;
        let mut committed = 0;
        let mut diagnostic = None;
        if let Some(writer) = writer.as_mut() {
            if !sources.is_empty() {
                cursor %= sources.len();
                let source = &mut sources[cursor];
                let mut idle = false;
                if now >= source.next {
                    match ingest_turn(source, writer, &control) {
                        Ok(Turn::Progress(count, skipped)) => {
                            phase = NativeServicePhase::Processing;
                            committed = count;
                            source.failures = 0;
                            source.next = now;
                            if skipped {
                                diagnostic = Some("native_input_skipped");
                            }
                        }
                        Ok(Turn::Idle) => {
                            idle = true;
                        }
                        Err(()) => {
                            source.cache = None;
                            source.failures = source.failures.saturating_add(1);
                            source.next = now + backoff(source.failures);
                            phase = NativeServicePhase::RecoverableFailure;
                            diagnostic = Some("native_source_retry");
                        }
                    }
                }
                if idle {
                    sources.remove(cursor);
                } else {
                    cursor = cursor.saturating_add(1);
                }
            }
            if processor
                .as_ref()
                .is_some_and(|(_, token)| token.is_cancelled())
            {
                processor = None;
            }
            if processor.is_none() && now >= processor_retry && history.store_path().is_file() {
                if let Some(scan) = scans.values().next() {
                    match history.processor_with_cancellation(scan.token.clone()) {
                        Ok(opened) => processor = Some((opened, scan.token.clone())),
                        Err(_) => {
                            processor_failures = processor_failures.saturating_add(1);
                            processor_retry = now + backoff(processor_failures);
                        }
                    }
                }
            }
            if let (Some((processor, _)), Some(scan)) = (processor.as_mut(), scans.values().next())
            {
                let token = scan.token.clone();
                if !token.is_cancelled() && now >= processor_retry {
                    if processor
                        .process_pending(1, |commit| {
                            control.with_native_commit(&scan.context.addons_root, &token, || {
                                commit.commit()
                            })
                        })
                        .is_err()
                    {
                        processor_failures = processor_failures.saturating_add(1);
                        processor_retry = now + backoff(processor_failures);
                    } else {
                        processor_failures = 0;
                    }
                }
            }
            if processor_failures > 0 {
                phase = NativeServicePhase::RecoverableFailure;
                diagnostic = Some("encounter_derivation_retry");
            }
        } else if store_failures > 0 {
            phase = NativeServicePhase::RecoverableFailure;
            diagnostic = Some("native_store_retry");
        }
        if diagnostic.is_none() && sources.iter().any(|source| source.failures > 0) {
            phase = NativeServicePhase::RecoverableFailure;
            diagnostic = Some("native_source_retry");
        }
        let mut total = sources
            .iter()
            .filter_map(|source| source.cache.as_ref().map(|cache| cache.encoded_bytes))
            .sum::<usize>();
        for source in &mut sources {
            if total <= MAX_CACHE_BYTES {
                break;
            }
            if let Some(cache) = source.cache.take() {
                total = total.saturating_sub(cache.encoded_bytes);
            }
        }
        control.report_native(phase, sources.len(), committed, diagnostic);
        thread::sleep(TICK);
    }
    control.native_worker_stopped();
}

fn backoff(failures: u32) -> Duration {
    Duration::from_millis((250u64 << failures.min(8)).min(60_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disappeared_sources_release_the_candidate_budget_without_erasing_checkpoints() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("live/AddOns");
        let logs = directory.path().join("live/Logs");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&logs).unwrap();
        let path = logs.join("Encounter.log");
        fs::write(&path, b"0,BEGIN_LOG,1700000000000,15,\"SyntheticRealm\",\"zz\",\"fixture.game\"\n10,BEGIN_COMBAT\n").unwrap();
        let control = MaintenanceControl::new(super::super::DataMaintenancePrefs::default());
        control.set_presence(true);
        let token = control.register_native_job(&root);
        let key = source_key(&root, &path, Channel::Live);
        let mut source = Source {
            root,
            logs: fs::canonicalize(&logs).unwrap(),
            path: path.clone(),
            channel: Channel::Live,
            key: key.clone(),
            token,
            cache: None,
            next: Instant::now(),
            failures: 0,
        };
        let mut writer = NativeIngestWriter::open_with_cancellation(
            directory.path().join("history.sqlite"),
            source.token.clone(),
        )
        .unwrap();
        assert!(matches!(
            ingest_turn(&mut source, &mut writer, &control),
            Ok(Turn::Progress(0, _))
        ));
        let before = writer.checkpoint(&key).unwrap().unwrap();
        fs::remove_file(path).unwrap();
        assert!(
            matches!(
                ingest_turn(&mut source, &mut writer, &control),
                Ok(Turn::Idle)
            ),
            "a rotated-away filename must not pin a bounded candidate slot forever"
        );
        assert_eq!(
            writer.checkpoint(&key).unwrap().unwrap().checkpoint_bytes,
            before.checkpoint_bytes
        );
    }

    #[test]
    fn late_root_is_not_starved_by_an_existing_full_source_inventory() {
        let directory = tempfile::tempdir().unwrap();
        let mut scans = BTreeMap::new();
        let mut sources = Vec::new();
        let live = directory.path().join("live/AddOns");
        let pts = directory.path().join("pts/AddOns");
        for root in [&live, &pts] {
            fs::create_dir_all(root).unwrap();
            fs::create_dir_all(root.parent().unwrap().join("Logs")).unwrap();
        }
        for index in 0..MAX_SOURCES {
            fs::write(
                live.parent()
                    .unwrap()
                    .join(format!("Logs/Encounter-{index}.log")),
                [],
            )
            .unwrap();
        }
        fs::write(pts.parent().unwrap().join("Logs/Encounter.log"), []).unwrap();
        scans.insert(
            live.clone(),
            Scan {
                context: Context {
                    addons_root: live.clone(),
                    environment: beacon::Environment::Live,
                },
                entries: None,
                token: CancellationToken::default(),
            },
        );
        scan_sources(&mut scans, &mut sources);
        assert_eq!(sources.len(), MAX_SOURCES);
        scans.insert(
            pts.clone(),
            Scan {
                context: Context {
                    addons_root: pts.clone(),
                    environment: beacon::Environment::Pts,
                },
                entries: None,
                token: CancellationToken::default(),
            },
        );
        scan_sources(&mut scans, &mut sources);
        assert!(sources
            .iter()
            .any(|source| source.root == pts && source.channel == Channel::Pts));
        assert!(sources.len() <= MAX_SOURCES);
    }
}
