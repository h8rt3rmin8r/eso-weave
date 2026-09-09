use crate::catalog::version::{parse_commit_message_version, GameVersion};
use crate::catalog::{CatalogRelease, Channel};
use crate::catalog_pipeline::CandidateSummary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckFreshness {
    Fresh,
    Offline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveUpdateState {
    CatalogCurrent,
    NewLiveDataAvailable,
    UpdateReadyToImport,
    CollectorCaptureRequired,
    UnsupportedSchema,
    OfflineStaleCheck,
    CatalogUnavailable,
}

#[derive(Debug, Clone)]
pub struct AvailabilityInput {
    pub active: Option<CatalogRelease>,
    pub observed_live: Option<GameVersion>,
    pub freshness: CheckFreshness,
    pub live_candidates: Vec<CandidateSummary>,
    pub pts_candidates: Vec<CandidateSummary>,
    pub collector_capture_required: bool,
    pub supported_schema: u32,
}

#[derive(Debug, Clone)]
pub struct CatalogAvailability {
    pub live_state: LiveUpdateState,
    pub live_candidate: Option<CandidateSummary>,
    pub pts_preview: Option<CandidateSummary>,
}

pub fn resolve_availability(input: AvailabilityInput) -> CatalogAvailability {
    let observed = observed_state(&input);
    let live_candidate = newest(input.live_candidates, Channel::Live);
    let pts_preview = newest(input.pts_candidates, Channel::Pts);
    let live_state = if let Some(candidate) = &live_candidate {
        if candidate.catalog_schema != input.supported_schema {
            LiveUpdateState::UnsupportedSchema
        } else if candidate_is_newer(candidate, input.active.as_ref()) {
            LiveUpdateState::UpdateReadyToImport
        } else if input.collector_capture_required {
            LiveUpdateState::CollectorCaptureRequired
        } else {
            observed
        }
    } else if input.collector_capture_required {
        LiveUpdateState::CollectorCaptureRequired
    } else {
        observed
    };
    CatalogAvailability {
        live_state,
        live_candidate,
        pts_preview,
    }
}

fn observed_state(input: &AvailabilityInput) -> LiveUpdateState {
    let Some(active) = &input.active else {
        return LiveUpdateState::CatalogUnavailable;
    };
    if input.freshness == CheckFreshness::Offline {
        return LiveUpdateState::OfflineStaleCheck;
    }
    let active_version = parse_commit_message_version(&active.game_version);
    if input.observed_live > active_version {
        LiveUpdateState::NewLiveDataAvailable
    } else {
        LiveUpdateState::CatalogCurrent
    }
}

fn newest(mut candidates: Vec<CandidateSummary>, channel: Channel) -> Option<CandidateSummary> {
    candidates.retain(|candidate| candidate.channel == channel);
    candidates.into_iter().max_by(|left, right| {
        left.api_version
            .cmp(&right.api_version)
            .then_with(|| {
                parse_commit_message_version(&left.game_version)
                    .cmp(&parse_commit_message_version(&right.game_version))
            })
            .then_with(|| left.catalog_version.cmp(&right.catalog_version))
            .then_with(|| left.candidate_sha256.cmp(&right.candidate_sha256))
    })
}

fn candidate_is_newer(candidate: &CandidateSummary, active: Option<&CatalogRelease>) -> bool {
    let Some(active) = active else {
        return true;
    };
    candidate.api_version > active.api_version
        || (candidate.api_version == active.api_version
            && (parse_commit_message_version(&candidate.game_version)
                > parse_commit_message_version(&active.game_version)
                || candidate.catalog_semantic_sha256 != active.semantic_sha256))
}
