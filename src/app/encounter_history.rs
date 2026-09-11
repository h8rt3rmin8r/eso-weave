use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, SyncSender, TryRecvError};
use std::thread::JoinHandle;
use std::time::Duration;

use crate::catalog::Channel;
use crate::encounter::{
    EncounterHistoryService, EncounterIdentity, EncounterProjection, EncounterSummary,
    HistoryDiagnostic, ImportOutcome, LossRange, MetricQuality, MetricResult,
};
use crate::recommendation::{
    generate_recommendations, AdviceQualification, AdviceRule, RecommendationAvailability,
    RecommendationReport,
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
        result: Result<Box<EncounterDetail>, HistoryDiagnostic>,
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
                        let result = thread_service
                            .detail(&identity)
                            .map(EncounterDetail::from_projection)
                            .map(Box::new);
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

#[derive(Debug, Clone, PartialEq)]
pub struct EncounterDetail {
    pub projection: EncounterProjection,
    pub recommendations: RecommendationReport,
}

impl EncounterDetail {
    fn from_projection(projection: EncounterProjection) -> Self {
        let recommendations = generate_recommendations(&projection);
        Self {
            projection,
            recommendations,
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationPresentation {
    pub heading: &'static str,
    pub status: &'static str,
    pub summary: &'static str,
    pub reasons: Vec<String>,
    pub items: Vec<AdvicePresentation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvicePresentation {
    pub title: &'static str,
    pub body: String,
    pub qualification: &'static str,
    pub qualification_reasons: Vec<String>,
    pub citation: String,
}

pub fn recommendation_presentation(report: &RecommendationReport) -> RecommendationPresentation {
    let status = match report.availability {
        RecommendationAvailability::Ready => "Evidence status: Ready",
        RecommendationAvailability::Qualified => "Evidence status: Qualified",
        RecommendationAvailability::Suppressed => "Evidence status: Suppressed",
    };
    let summary = match report.availability {
        RecommendationAvailability::Suppressed => {
            "Advice is suppressed because the evidence did not pass every s090-v1 gate."
        }
        _ if report.advice.is_empty() => {
            "No provisional review prompt crossed the s090-v1 thresholds."
        }
        _ => {
            "These deterministic review prompts are provisional and do not claim cause or an optimal rotation."
        }
    };
    let items = report
        .advice
        .iter()
        .map(|item| {
            let (title, body) = match item.rule {
                AdviceRule::DominantDamageShare => (
                    "Review dominant observed damage share",
                    format!(
                        "Ability {} contributed {:.1}% of observed outgoing ability damage. Compare that concentration with your intended encounter context.",
                        item.target_id,
                        item.observed_ratio * 100.0
                    ),
                ),
                AdviceRule::LowEffectUptime => (
                    "Review low observed effect uptime",
                    format!(
                        "Effect {} had {:.1}% observed uptime. Compare the observed gaps with the uptime you intended for this encounter.",
                        item.target_id,
                        item.observed_ratio * 100.0
                    ),
                ),
            };
            let qualification = match item.qualification {
                AdviceQualification::Provisional => "Provisional review prompt",
                AdviceQualification::ProvisionalQualified => {
                    "Provisional, qualified review prompt"
                }
            };
            let citation = format!(
                "Evidence: encounter {} in session {} | raw {} | projection schema {} | metric algorithm {} | catalog {} (schema {}, semantic {}) | {} API {} | recommendation {} (schema {})",
                item.citation.encounter_id,
                item.citation.session_id,
                item.citation.raw_content_sha256,
                item.citation.projection_schema_version,
                item.citation.metric_algorithm_version,
                item.citation.catalog_version,
                item.citation.catalog_schema_version,
                item.citation.catalog_semantic_sha256,
                item.citation.channel,
                item.citation.api_version,
                item.citation.recommendation_policy_version,
                item.citation.recommendation_schema_version
            );
            AdvicePresentation {
                title,
                body,
                qualification,
                qualification_reasons: if matches!(
                    item.qualification,
                    AdviceQualification::ProvisionalQualified
                ) {
                    let mut reasons = report
                        .reasons
                        .iter()
                        .filter(|reason| {
                            matches!(
                                reason.kind,
                                crate::recommendation::RecommendationReasonKind::DeclaredCaptureLoss
                                    | crate::recommendation::RecommendationReasonKind::UnknownCatalogIds
                            )
                        })
                        .map(|reason| reason.message.clone())
                        .collect::<Vec<_>>();
                    if report.reasons.iter().any(|reason| {
                        reason.kind
                            == crate::recommendation::RecommendationReasonKind::DeclaredCaptureLoss
                    }) {
                        reasons.extend(loss_labels(&report.evidence.loss_ranges));
                    }
                    reasons
                } else {
                    Vec::new()
                },
                citation,
            }
        })
        .collect();
    RecommendationPresentation {
        heading: "Provisional Recommendations",
        status,
        summary,
        reasons: report
            .reasons
            .iter()
            .map(|reason| reason.message.clone())
            .collect(),
        items,
    }
}
