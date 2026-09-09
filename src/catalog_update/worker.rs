use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use crate::catalog_pipeline::CandidateSummary;
use crate::collector::lifecycle::{CollectorStatus, RunningState};

use super::{
    CancellationToken, CaptureFingerprint, CatalogResolution, CatalogUpdateService, InstallOutcome,
    RollbackOutcome, UpdateError, UpdateOperation, UpdateProgress, UpdateResult, UpdateStage,
};

enum WorkerCommand {
    Startup,
    Discover,
    ObserveCapture {
        path: PathBuf,
    },
    InspectCollector {
        addons_root: PathBuf,
    },
    InstallCollector {
        addons_root: PathBuf,
        running: RunningState,
        api_version: u32,
    },
    UninstallCollector {
        addons_root: PathBuf,
        running: RunningState,
    },
    DeleteCapture {
        path: PathBuf,
    },
    BuildCollector {
        path: PathBuf,
        waiting_fingerprint: CaptureFingerprint,
        cancel: CancellationToken,
    },
    Install {
        candidate_sha256: String,
        origin_acknowledged: bool,
        cancel: CancellationToken,
    },
    Rollback {
        cancel: CancellationToken,
    },
    Shutdown,
}

pub enum WorkerEvent {
    StartupComplete {
        candidates: Vec<CandidateSummary>,
        resolution: CatalogResolution,
        recovered_staging: usize,
    },
    DiscoveryComplete(Vec<CandidateSummary>),
    CaptureBoundaryRecorded(CaptureFingerprint),
    CollectorCandidateBuilt(CandidateSummary),
    CollectorState {
        status: Option<CollectorStatus>,
        message: String,
    },
    Progress(UpdateProgress),
    InstallComplete {
        outcome: Box<InstallOutcome>,
        resolution: Box<CatalogResolution>,
    },
    RollbackComplete {
        outcome: Box<RollbackOutcome>,
        resolution: Box<CatalogResolution>,
    },
    Failed(WorkerFailure),
    ReceiptWriteFailed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkerFailure {
    pub code: &'static str,
    pub message: String,
    pub stage: UpdateStage,
}

impl WorkerFailure {
    fn from_error(error: &UpdateError, stage: UpdateStage) -> Self {
        let (code, message) = match error {
            UpdateError::OriginNotAcknowledged => (
                "origin-not-acknowledged",
                "Confirm that the candidate came from a source you trust.".into(),
            ),
            UpdateError::ConcurrentOperation => (
                "operation-in-progress",
                "Another catalog operation is already running.".into(),
            ),
            UpdateError::NoRollbackTarget => (
                "rollback-unavailable",
                "No previous verified catalog is available.".into(),
            ),
            UpdateError::InvalidSelection(_) => (
                "selection-invalid",
                "The saved catalog selection is invalid; the bundled fallback remains active."
                    .into(),
            ),
            UpdateError::Pipeline(_) => (
                "candidate-invalid",
                "The candidate failed complete integrity or policy verification.".into(),
            ),
            UpdateError::Validation(message) => ("update-invalid", message.clone()),
            UpdateError::Io(_) => (
                "storage-failed",
                "Catalog storage could not complete the operation.".into(),
            ),
            UpdateError::Json(_) => (
                "contract-invalid",
                "A catalog update contract was malformed.".into(),
            ),
            UpdateError::Cancelled => ("cancelled", "Catalog operation cancelled.".into()),
        };
        Self {
            code,
            message,
            stage,
        }
    }
}

pub struct CatalogUpdateWorker {
    commands: Sender<WorkerCommand>,
    events: Receiver<WorkerEvent>,
    active_cancel: Arc<Mutex<Option<CancellationToken>>>,
    busy: Arc<AtomicBool>,
    join: Option<JoinHandle<()>>,
}

impl CatalogUpdateWorker {
    pub fn spawn(service: CatalogUpdateService) -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let active_cancel = Arc::new(Mutex::new(None));
        let busy = Arc::new(AtomicBool::new(false));
        let worker_busy = busy.clone();
        let join = thread::spawn(move || {
            while let Ok(command) = command_rx.recv() {
                match command {
                    WorkerCommand::Startup => {
                        let recovered_staging = service.recover_staging().unwrap_or(0);
                        if recovered_staging > 0 {
                            record_terminal(
                                &service,
                                &event_tx,
                                UpdateOperation::Recovery,
                                UpdateResult::Interrupted,
                                UpdateStage::Failed,
                                None,
                                "interrupted-staging-recovered",
                            );
                        }
                        let resolution = service.resolve_catalog();
                        match service.discover_candidates() {
                            Ok(candidates) => {
                                let _ = event_tx.send(WorkerEvent::StartupComplete {
                                    candidates,
                                    resolution,
                                    recovered_staging,
                                });
                            }
                            Err(error) => {
                                let _ = event_tx.send(WorkerEvent::Failed(
                                    WorkerFailure::from_error(&error, UpdateStage::Checking),
                                ));
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::Discover => {
                        match service.discover_candidates() {
                            Ok(candidates) => {
                                let _ = event_tx.send(WorkerEvent::DiscoveryComplete(candidates));
                            }
                            Err(error) => {
                                let _ = event_tx.send(WorkerEvent::Failed(
                                    WorkerFailure::from_error(&error, UpdateStage::Checking),
                                ));
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::ObserveCapture { path } => {
                        match service.capture_wait_boundary(path) {
                            Ok(fingerprint) => {
                                let _ = event_tx
                                    .send(WorkerEvent::CaptureBoundaryRecorded(fingerprint));
                            }
                            Err(error) => {
                                let _ =
                                    event_tx.send(WorkerEvent::Failed(WorkerFailure::from_error(
                                        &error,
                                        UpdateStage::WaitingForCapture,
                                    )));
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::InspectCollector { addons_root } => {
                        let status = crate::collector::lifecycle::status(&addons_root);
                        let _ = event_tx.send(WorkerEvent::CollectorState {
                            status: Some(status),
                            message: "Collector status refreshed.".into(),
                        });
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::InstallCollector {
                        addons_root,
                        running,
                        api_version,
                    } => {
                        match crate::collector::lifecycle::install(
                            &addons_root,
                            running,
                            api_version,
                        ) {
                            Ok(outcome) => {
                                let message = if outcome.reload_required {
                                    "Collector installed. Run /reloadui, log out, or exit ESO before importing its capture."
                                } else {
                                    "Collector installed. Start ESO, then use /reloadui, logout, or exit to flush its capture."
                                };
                                let _ = event_tx.send(WorkerEvent::CollectorState {
                                    status: Some(outcome.status),
                                    message: message.into(),
                                });
                            }
                            Err(_) => {
                                let _ = event_tx.send(WorkerEvent::CollectorState {
                                    status: None,
                                    message: "Collector installation was refused by its ownership or path safety gate."
                                        .into(),
                                });
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::UninstallCollector {
                        addons_root,
                        running,
                    } => {
                        match crate::collector::lifecycle::uninstall(&addons_root, running) {
                            Ok(outcome) => {
                                let _ = event_tx.send(WorkerEvent::CollectorState {
                                    status: Some(outcome.status),
                                    message:
                                        "Managed collector removed. PixelBeacon was not modified."
                                            .into(),
                                });
                            }
                            Err(_) => {
                                let _ = event_tx.send(WorkerEvent::CollectorState {
                                    status: None,
                                    message: "Collector removal was refused because it is absent or not marker-owned."
                                        .into(),
                                });
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::DeleteCapture { path } => {
                        let message = delete_capture(&path).unwrap_or_else(|message| message);
                        let _ = event_tx.send(WorkerEvent::CollectorState {
                            status: None,
                            message: message.into(),
                        });
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::BuildCollector {
                        path,
                        waiting_fingerprint,
                        cancel,
                    } => {
                        let started = Instant::now();
                        let result = service.build_collector_candidate(
                            path,
                            &waiting_fingerprint,
                            &cancel,
                            |mut progress| {
                                progress.elapsed_millis = started.elapsed().as_millis() as u64;
                                let _ = event_tx.send(WorkerEvent::Progress(progress));
                            },
                        );
                        match result {
                            Ok(candidate) => {
                                let _ =
                                    event_tx.send(WorkerEvent::CollectorCandidateBuilt(candidate));
                            }
                            Err(UpdateError::Cancelled) => {
                                record_terminal(
                                    &service,
                                    &event_tx,
                                    UpdateOperation::CollectorBuild,
                                    UpdateResult::Cancelled,
                                    UpdateStage::Cancelled,
                                    None,
                                    "cancelled",
                                );
                                let _ = event_tx.send(WorkerEvent::Cancelled);
                            }
                            Err(error) => {
                                record_terminal(
                                    &service,
                                    &event_tx,
                                    UpdateOperation::CollectorBuild,
                                    UpdateResult::Failed,
                                    UpdateStage::Failed,
                                    None,
                                    "collector-build-failed",
                                );
                                let _ = event_tx.send(WorkerEvent::Failed(
                                    WorkerFailure::from_error(&error, UpdateStage::Failed),
                                ));
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::Install {
                        candidate_sha256,
                        origin_acknowledged,
                        cancel,
                    } => {
                        let started = Instant::now();
                        let receipt_hash = Some(candidate_sha256.clone());
                        let path = service.import_candidate_path(&candidate_sha256);
                        let result = path.and_then(|path| {
                            service.install(&path, origin_acknowledged, &cancel, |mut progress| {
                                progress.elapsed_millis = started.elapsed().as_millis() as u64;
                                let _ = event_tx.send(WorkerEvent::Progress(progress));
                            })
                        });
                        match result {
                            Ok(outcome) => {
                                let resolution = service.resolve_catalog();
                                let _ = event_tx.send(WorkerEvent::InstallComplete {
                                    outcome: Box::new(outcome),
                                    resolution: Box::new(resolution),
                                });
                            }
                            Err(UpdateError::Cancelled) => {
                                record_terminal(
                                    &service,
                                    &event_tx,
                                    UpdateOperation::Install,
                                    UpdateResult::Cancelled,
                                    UpdateStage::Cancelled,
                                    receipt_hash,
                                    "cancelled",
                                );
                                let _ = event_tx.send(WorkerEvent::Cancelled);
                            }
                            Err(error) => {
                                record_terminal(
                                    &service,
                                    &event_tx,
                                    UpdateOperation::Install,
                                    UpdateResult::Failed,
                                    UpdateStage::Failed,
                                    receipt_hash,
                                    "install-failed",
                                );
                                let _ = event_tx.send(WorkerEvent::Failed(
                                    WorkerFailure::from_error(&error, UpdateStage::Failed),
                                ));
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::Rollback { cancel } => {
                        let started = Instant::now();
                        let result = service.rollback(&cancel, |mut progress| {
                            progress.elapsed_millis = started.elapsed().as_millis() as u64;
                            let _ = event_tx.send(WorkerEvent::Progress(progress));
                        });
                        match result {
                            Ok(outcome) => {
                                let resolution = service.resolve_catalog();
                                let _ = event_tx.send(WorkerEvent::RollbackComplete {
                                    outcome: Box::new(outcome),
                                    resolution: Box::new(resolution),
                                });
                            }
                            Err(UpdateError::Cancelled) => {
                                record_terminal(
                                    &service,
                                    &event_tx,
                                    UpdateOperation::Rollback,
                                    UpdateResult::Cancelled,
                                    UpdateStage::Cancelled,
                                    None,
                                    "cancelled",
                                );
                                let _ = event_tx.send(WorkerEvent::Cancelled);
                            }
                            Err(error) => {
                                record_terminal(
                                    &service,
                                    &event_tx,
                                    UpdateOperation::Rollback,
                                    UpdateResult::Failed,
                                    UpdateStage::Failed,
                                    None,
                                    "rollback-failed",
                                );
                                let _ = event_tx.send(WorkerEvent::Failed(
                                    WorkerFailure::from_error(&error, UpdateStage::Failed),
                                ));
                            }
                        }
                        worker_busy.store(false, Ordering::Release);
                    }
                    WorkerCommand::Shutdown => break,
                }
            }
        });
        Self {
            commands: command_tx,
            events: event_rx,
            active_cancel,
            busy,
            join: Some(join),
        }
    }

    pub fn discover(&self) -> bool {
        self.start(WorkerCommand::Discover, None)
    }

    pub fn startup(&self) -> bool {
        self.start(WorkerCommand::Startup, None)
    }

    pub fn observe_capture(&self, path: PathBuf) -> bool {
        self.start(WorkerCommand::ObserveCapture { path }, None)
    }

    pub fn inspect_collector(&self, addons_root: PathBuf) -> bool {
        self.start(WorkerCommand::InspectCollector { addons_root }, None)
    }

    pub fn install_collector(
        &self,
        addons_root: PathBuf,
        running: RunningState,
        api_version: u32,
    ) -> bool {
        self.start(
            WorkerCommand::InstallCollector {
                addons_root,
                running,
                api_version,
            },
            None,
        )
    }

    pub fn uninstall_collector(&self, addons_root: PathBuf, running: RunningState) -> bool {
        self.start(
            WorkerCommand::UninstallCollector {
                addons_root,
                running,
            },
            None,
        )
    }

    pub fn delete_capture(&self, path: PathBuf) -> bool {
        self.start(WorkerCommand::DeleteCapture { path }, None)
    }

    pub fn build_collector(&self, path: PathBuf, waiting_fingerprint: CaptureFingerprint) -> bool {
        let cancel = CancellationToken::new();
        self.start(
            WorkerCommand::BuildCollector {
                path,
                waiting_fingerprint,
                cancel: cancel.clone(),
            },
            Some(cancel),
        )
    }

    pub fn install(&self, candidate_sha256: String, origin_acknowledged: bool) -> bool {
        let cancel = CancellationToken::new();
        self.start(
            WorkerCommand::Install {
                candidate_sha256,
                origin_acknowledged,
                cancel: cancel.clone(),
            },
            Some(cancel),
        )
    }

    pub fn rollback(&self) -> bool {
        let cancel = CancellationToken::new();
        self.start(
            WorkerCommand::Rollback {
                cancel: cancel.clone(),
            },
            Some(cancel),
        )
    }

    pub fn cancel(&self) {
        if let Some(cancel) = self.active_cancel.lock().unwrap().as_ref() {
            cancel.cancel();
        }
    }

    pub fn is_busy(&self) -> bool {
        self.busy.load(Ordering::Acquire)
    }

    pub fn try_recv(&self) -> Result<WorkerEvent, TryRecvError> {
        self.events.try_recv()
    }

    fn start(&self, command: WorkerCommand, cancel: Option<CancellationToken>) -> bool {
        if self
            .busy
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        *self.active_cancel.lock().unwrap() = cancel;
        if self.commands.send(command).is_err() {
            self.busy.store(false, Ordering::Release);
            return false;
        }
        true
    }
}

impl Drop for CatalogUpdateWorker {
    fn drop(&mut self) {
        self.cancel();
        let _ = self.commands.send(WorkerCommand::Shutdown);
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn record_terminal(
    service: &CatalogUpdateService,
    events: &Sender<WorkerEvent>,
    operation: UpdateOperation,
    result: UpdateResult,
    stage: UpdateStage,
    candidate_sha256: Option<String>,
    finding_code: &str,
) {
    if service
        .write_terminal_receipt(operation, result, stage, candidate_sha256, finding_code)
        .is_err()
    {
        let _ = events.send(WorkerEvent::ReceiptWriteFailed);
    }
}

fn delete_capture(path: &std::path::Path) -> Result<&'static str, &'static str> {
    let metadata = std::fs::symlink_metadata(path)
        .map_err(|_| "No collector capture is available to delete.")?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err("Capture deletion refused a linked or non-regular target.");
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if metadata.file_attributes() & 0x0400 != 0 {
            return Err("Capture deletion refused a linked or non-regular target.");
        }
    }
    std::fs::remove_file(path).map_err(|_| "The collector capture could not be deleted.")?;
    Ok("Local collector capture deleted.")
}
