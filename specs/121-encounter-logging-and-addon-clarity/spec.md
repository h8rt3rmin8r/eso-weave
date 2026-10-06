# Feature Specification: Encounter Logging and Addon Clarity

**Feature Branch**: `codex/s121-encounter-logging-and-addon-clarity`
**Created**: 2026-10-05
**Status**: Implemented and reviewed; owner final review and merge pending
**Input**: Owner-authorized S121, issues #249 and #222, full spec-kit/autopilot delivery and official PR with at most two Codex review rounds.

## User Scenarios & Testing

### User Story 1 - Understand setup and addon state (Priority: P1)

A player can identify ESO Weave Data, distinguish it from PixelBeacon, install or update the correct package, enable it inside ESO, and understand which desktop facts describe installation versus current or saved game activity.

**Why this priority**: The owner could not understand the advanced-feature interface, including the addon required for recording.
**Independent Test**: Repository presentation fixtures cover absent, unmanaged, outdated, incompatible, unknown, busy, installed and reload-needed states with actionable explanations, without running ESO.
**Acceptance Scenarios**:

1. **Given** no data addon, **when** its details are opened, **then** installation, in-game enablement and reload instructions identify the correct package and environment.
2. **Given** an installed package, **when** its status is shown, **then** installation does not imply enablement, loading or recording; unavailable facts remain explicit.
3. **Given** foreign files, incompatible API or failed lifecycle work, **when** recovery is shown, **then** it explains the condition and supported next action without changing ownership or compatibility rules.

### User Story 2 - Record, save, import and clear deliberately (Priority: P1)

A player chooses single or continuous recording, understands waiting and partial results, saves addon data to disk, imports accepted encounters into desktop history, and understands each deletion's exact scope.

**Why this priority**: Confusing saved/current state and recording/import language makes the entire workflow unusable.
**Independent Test**: Mocked desktop state and Lua command fixtures cover setup, recording, stopping, interrupted/failed capture, save/import readiness and separate clear/delete operations.
**Acceptance Scenarios**:

1. **Given** recording is off, **when** command help or desktop guidance is read, **then** the mode/channel/toggle/status/save/import sequence is complete and accurate.
2. **Given** saved data while ESO runs, **when** the desktop displays it, **then** it says it is the last disk save and directs current-status questions to the in-game command.
3. **Given** incomplete, duplicate, busy, failed or incompatible import, **when** a result is shown, **then** it names the outcome, preserves truthful quality/unknown state, and explains an available recovery.
4. **Given** a clear or delete confirmation, **when** it is read, **then** it distinguishes addon encounter data, catalog collection, and desktop imported history, including preservation effects.

### User Story 3 - Understand where encounter results come from (Priority: P2)

A reader sees a readable encounter-lineage figure and full text equivalent connecting ordered observations, validation/loss, immutable original data, catalog resolution, versioned metrics and separately qualified recommendations.

**Why this priority**: Issue #222 supports the workflow audit by joining scattered explanations; it cannot substitute for repairing interface copy.
**Independent Test**: Documentation policy and rendering checks cover the owned offline figure, matching text, content-width and 200-percent zoom readability.
**Acceptance Scenarios**:

1. **Given** an unknown ID or declared missing observations, **when** the flow is read, **then** preserved original data, incomplete results and unresolved catalog matches are distinct.
2. **Given** native combat-log ingestion, **when** the diagram is read, **then** it remains visibly provisional and distinct from saved addon data, without a Combat Metrics parity claim.

### Edge Cases

- Missing AddOns location, Live/PTS mismatch, foreign or broken package, unknown API support, lifecycle failure, and restart/reload distinctions.
- No saved file, recording not saved yet, unknown controller version, invalid preserved state, waiting/active/stopped/failed sessions, partial prefixes, interrupted reloads and full storage.
- No encounters, duplicates, growing sessions, failed refresh/detail/import/delete, unknown IDs, unavailable rates, loss and suppressed recommendations.
- Catalog collection beside encounter recording; catalog clear versus encounter clear versus desktop delete; native game logs versus application diagnostics.

## Requirements

### Functional Requirements

- **FR-001**: Inventory every relevant public label, tooltip, modal, generated diagnostic, command message, setting, menu and linked/bundled guide across state branches in a reviewable audit matrix.
- **FR-002**: Each matrix entry MUST identify surface/state, previous wording, confusion, final wording or justified retention and exact source/help location. No unresolved entry may be hidden by a glossary-only fix.
- **FR-003**: Essential player copy MUST explain what a feature or status means, why it matters, its current-versus-saved scope and an available next action where needed; technical details may remain explicitly explained and secondary.
- **FR-004**: Consistently distinguish application diagnostic logs, native ESO combat logs, ESO Weave Data recording, the game's saved addon data (SavedVariables), imported desktop history and catalog game-data definitions.
- **FR-005**: Explain package roles, ownership, installation/update/repair/removal, enablement/loading/reload and environment/API mismatch at point of use, preserving truthful unknown states.
- **FR-006**: Explain both existing recording modes and all command/setup/status/stop/save/import/view/clear steps, including waiting, ongoing, completed, interrupted, partial and failed states.
- **FR-007**: Import and history messages MUST describe readiness, progress, empty/busy/failure/duplicate results, exact loss and unresolved definitions without leaking raw values or implying live control.
- **FR-008**: Every destructive confirmation MUST state exactly which data is removed and which addon, catalog, saved file and desktop history are preserved, consistent with actual operations.
- **FR-009**: Public and bundled help MUST match the final vocabulary and complete workflow, including the separate native-log path and application logging settings.
- **FR-010**: Add one repository-owned offline-safe accessible lineage figure with full nearby text equivalent, visible version/quality gates, kind-specific catalog lookup, original data identity, qualified native path and separately gated provisional recommendations.
- **FR-011**: Govern the figure through existing manifest/policy coverage, source-contract/update-trigger records and documented desktop/narrow/200-percent zoom checks.
- **FR-012**: Preserve recording/import/lifecycle/retention/automation behavior, data integrity and security tests; add no prerequisites, restrictions, transport, raw-data disclosure or new game commands.
- **FR-013**: Complete test-first presentation/workflow coverage and required automated repository checks, then authorized push/official PR, respond to every review, use at most two Codex rounds, and await owner merge after green CI.

### Key Entities

- **Audit entry**: One located message or coherent message family, its reachable states, former/final wording, rationale and coverage.
- **Recording state**: Current in-game activity or last saved disk snapshot, always distinguished.
- **Encounter lineage**: Original ordered observations, declared loss, stored identity, catalog definitions, versioned derived results and qualified review prompts.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All discovered relevant surfaces and state branches are accounted for; every audit entry is repaired or explicitly justified.
- **SC-002**: A complete installation-to-clear workflow uses the same terms in desktop, addon commands and help, with no unexplained essential jargon or false live status.
- **SC-003**: All listed edge-state fixture checks and existing safety tests pass without field verification.
- **SC-004**: The lineage figure and text name the same stages/gates and remain readable at content width, narrow width and 200-percent zoom without network assets.
- **SC-005**: Required local and hosted checks are green, every review finding is answered/resolved, and the official PR is ready for owner final review without merging.

## Assumptions

- Existing game-command names and storage/control contracts remain unchanged; this is a comprehension repair, not a behavior redesign.
- No release tag, live-game session, installed-app check or owner verification request belongs to S121. Publishing S120 separately remains a recommendation, not authorization to release.
- S121 incorporates #222's historically reserved S115 figure without marking that unused reservation complete; #221 and #223 remain separate.

## Clarifications

### Session 2026-10-05

- Q: Does a complete audit require a new capture UI/control channel? A: No. Repair and explain every existing surface, keeping commands in ESO and saved-data import on desktop.
- Q: Can implementation terminology remain? A: Only in explained secondary detail or maintainer documentation; essential instructions use plain wording and exact actions.
- Q: Are absent/foreign/version/error states part of the audit? A: Yes, with accurate recovery and no unsupported promise to clear unknown-version data.
- Q: Does native logging become supported by the figure? A: No. It stays explicitly provisional; addon recording is the supported documented path.
- Q: Is a patch release included? A: No. Owner authorized S121 implementation and PR, not a release or merge.
