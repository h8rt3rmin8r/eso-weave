use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender, TryRecvError};
use std::thread::JoinHandle;
use std::time::Duration;

use crate::catalog::Channel;
use crate::encounter::{
    EncounterHistoryService, EncounterIdentity, EncounterProjection, EncounterSummary,
    HistoryDiagnostic, ImportOutcome, LossRange, MetricQuality, MetricResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryOperation {
    Refresh,
    Import,
    DeleteOne,
    DeleteAll,
}

#[derive(Debug)]
enum HistoryCommand {
    Refresh,
    Import {
        source_path: PathBuf,
        expected_channel: Channel,
    },
    LoadDetail(EncounterIdentity),
    DeleteOne(EncounterIdentity),
    DeleteAll,
    Stop,
}

#[derive(Debug)]
pub enum HistoryEvent {
    Snapshot {
        encounters: Vec<EncounterSummary>,
        operation: HistoryOperation,
        message: Option<String>,
    },
    Detail {
        identity: EncounterIdentity,
        result: Result<Box<EncounterProjection>, HistoryDiagnostic>,
    },
    Failed {
        operation: HistoryOperation,
        diagnostic: HistoryDiagnostic,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HistoryWorkerBusy;

pub struct EncounterHistoryWorker {
    command_tx: Option<SyncSender<HistoryCommand>>,
    event_rx: Receiver<HistoryEvent>,
    join: Option<JoinHandle<()>>,
    service: EncounterHistoryService,
}

impl EncounterHistoryWorker {
    pub fn spawn(service: EncounterHistoryService) -> Self {
        let (command_tx, command_rx) = mpsc::sync_channel(1);
        let (event_tx, event_rx) = mpsc::channel();
        let thread_service = service.clone();
        let join = std::thread::spawn(move || {
            while let Ok(command) = command_rx.recv() {
                let event = match command {
                    HistoryCommand::Refresh => {
                        snapshot_event(&thread_service, HistoryOperation::Refresh, None)
                    }
                    HistoryCommand::Import {
                        source_path,
                        expected_channel,
                    } => match thread_service.import_current(source_path, expected_channel) {
                        Ok(receipt) => snapshot_event(
                            &thread_service,
                            HistoryOperation::Import,
                            Some(match receipt.outcome {
                                ImportOutcome::Imported => "Encounter imported.".into(),
                                ImportOutcome::AlreadyPresent => {
                                    "Encounter is already present in local history.".into()
                                }
                            }),
                        ),
                        Err(diagnostic) => HistoryEvent::Failed {
                            operation: HistoryOperation::Import,
                            diagnostic,
                        },
                    },
                    HistoryCommand::LoadDetail(identity) => {
                        let result = thread_service.detail(&identity).map(Box::new);
                        HistoryEvent::Detail { identity, result }
                    }
                    HistoryCommand::DeleteOne(identity) => {
                        match thread_service.delete_one(&identity) {
                            Ok(receipt) => snapshot_event(
                                &thread_service,
                                HistoryOperation::DeleteOne,
                                Some(if receipt.deleted_records == 0 {
                                    "The encounter was already absent.".into()
                                } else {
                                    "Encounter deleted from local history.".into()
                                }),
                            ),
                            Err(diagnostic) => HistoryEvent::Failed {
                                operation: HistoryOperation::DeleteOne,
                                diagnostic,
                            },
                        }
                    }
                    HistoryCommand::DeleteAll => match thread_service.delete_all() {
                        Ok(receipt) => snapshot_event(
                            &thread_service,
                            HistoryOperation::DeleteAll,
                            Some(format!(
                                "Deleted {} local encounter record{}.",
                                receipt.deleted_records,
                                if receipt.deleted_records == 1 {
                                    ""
                                } else {
                                    "s"
                                }
                            )),
                        ),
                        Err(diagnostic) => HistoryEvent::Failed {
                            operation: HistoryOperation::DeleteAll,
                            diagnostic,
                        },
                    },
                    HistoryCommand::Stop => break,
                };
                if event_tx.send(event).is_err() {
                    break;
                }
            }
        });
        Self {
            command_tx: Some(command_tx),
            event_rx,
            join: Some(join),
            service,
        }
    }

    pub fn set_catalog_path(&self, catalog_path: PathBuf) {
        self.service.set_catalog_path(catalog_path);
    }

    pub fn refresh(&self) -> Result<(), HistoryWorkerBusy> {
        self.send(HistoryCommand::Refresh)
    }

    pub fn import_current(
        &self,
        source_path: PathBuf,
        expected_channel: Channel,
    ) -> Result<(), HistoryWorkerBusy> {
        self.send(HistoryCommand::Import {
            source_path,
            expected_channel,
        })
    }

    pub fn load_detail(&self, identity: EncounterIdentity) -> Result<(), HistoryWorkerBusy> {
        self.send(HistoryCommand::LoadDetail(identity))
    }

    pub fn delete_one(&self, identity: EncounterIdentity) -> Result<(), HistoryWorkerBusy> {
        self.send(HistoryCommand::DeleteOne(identity))
    }

    pub fn delete_all(&self) -> Result<(), HistoryWorkerBusy> {
        self.send(HistoryCommand::DeleteAll)
    }

    pub fn try_receive(&self) -> Result<HistoryEvent, TryRecvError> {
        self.event_rx.try_recv()
    }

    pub fn receive_timeout(&self, timeout: Duration) -> Result<HistoryEvent, RecvTimeoutError> {
        self.event_rx.recv_timeout(timeout)
    }

    fn send(&self, command: HistoryCommand) -> Result<(), HistoryWorkerBusy> {
        self.command_tx
            .as_ref()
            .ok_or(HistoryWorkerBusy)?
            .try_send(command)
            .map_err(|_| HistoryWorkerBusy)
    }
}

impl Drop for EncounterHistoryWorker {
    fn drop(&mut self) {
        if let Some(sender) = self.command_tx.take() {
            let _ = sender.send(HistoryCommand::Stop);
        }
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

fn snapshot_event(
    service: &EncounterHistoryService,
    operation: HistoryOperation,
    message: Option<String>,
) -> HistoryEvent {
    match service.snapshot() {
        Ok(encounters) => HistoryEvent::Snapshot {
            encounters,
            operation,
            message,
        },
        Err(diagnostic) => HistoryEvent::Failed {
            operation,
            diagnostic,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricPresentation<'a> {
    pub label: &'a str,
    pub value: String,
    pub quality: &'static str,
    pub loss: Vec<String>,
}

pub fn metric_presentation<'a>(label: &'a str, result: &MetricResult) -> MetricPresentation<'a> {
    MetricPresentation {
        label,
        value: metric_value(result),
        quality: quality_label(result.quality),
        loss: loss_labels(&result.loss_ranges),
    }
}

pub const fn quality_label(quality: MetricQuality) -> &'static str {
    match quality {
        MetricQuality::Complete => "Complete",
        MetricQuality::Degraded => "Degraded",
    }
}

pub fn loss_labels(ranges: &[LossRange]) -> Vec<String> {
    ranges
        .iter()
        .map(|range| {
            format!(
                "Sequences {}-{} ({})",
                range.missing_sequence_from, range.missing_sequence_to, range.reason
            )
        })
        .collect()
}

fn metric_value(result: &MetricResult) -> String {
    let Some(value) = result.value else {
        return "Unavailable".into();
    };
    match result.unit.as_str() {
        "ratio" => format!("{:.1}%", value * 100.0),
        "damage-per-second" => format!("{value:.2} damage/s"),
        "effective-healing-per-second" => format!("{value:.2} effective healing/s"),
        unit => format!("{value:.2} {unit}"),
    }
}
