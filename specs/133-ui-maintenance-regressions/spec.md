# Feature Specification: Interface and maintenance regressions

**Feature Branch**: `codex/133-ui-maintenance-regressions`

**Created**: 2026-10-15

**Status**: Complete, owner-merged in PR #311 on 2026-10-16

**Input**: Implement #306-#310 with regression coverage and a reviewed PR, stopping before merge.

## User Scenarios & Testing

### User Story 1 - Stable log geometry (Priority: P1)

Opening View Log uses existing spare space and never pushes visible content off screen.

**Independent Test**: Render multiple frames and inspect actual viewport commands.

**Acceptance Scenarios**:

1. Given sufficient client space, when opening/closing the log repeatedly, then size and position remain unchanged.
2. Given insufficient space near display edges, when opening the log, then only the shortfall is requested within usable display bounds.
3. Given a maximized or small display, then controls remain accessible without forced outer-window growth.

### User Story 2 - Persistent log visibility (Priority: P1)

The last explicit open/closed selection survives normal restarts and unrelated settings saves.

**Independent Test**: Save and reconstruct the model and GUI from temporary preferences.

**Acceptance Scenarios**:

1. Given an open log, when restarting, then it remains open with its saved height and verbosity.
2. Given a later closed selection, when restarting, then it remains closed.
3. Older preferences without visibility load with the existing closed default.

### User Story 3 - Modal uninstall confirmation (Priority: P1)

PixelBeacon removal uses a titled overlay with explicit confirmation/cancellation and no dashboard displacement.

**Independent Test**: Headless interactions plus temporary AddOns retention fixtures.

**Acceptance Scenarios**:

1. Given the dashboard, when opening confirmation, then the controls and minimum geometry do not shift.
2. Cancellation, Escape, and other dismissal do not remove anything; explicit confirmation invokes existing guarded removal once.
3. SavedVariables, history and neighbors survive removal.

### User Story 4 - Actionable migration diagnostics (Priority: P1)

Background package reconciliation handles ordinary package states correctly and reports actual failures without repeating unchanged warnings.

**Independent Test**: Temporary package fixtures, controlled time, and captured diagnostics.

**Acceptance Scenarios**:

1. Missing/current/unmanaged packages do not generate unexplained migration warnings.
2. An unchanged error is reported once per root until it changes or recovers; retries remain bounded and active.
3. Recovery pending and committed cleanup pending have distinct truthful outcomes.

### User Story 5 - Concise PixelBeacon status (Priority: P2)

Normal installation status has no API-support suffix. Only known unsupported installed packages receive compatibility warnings.

**Independent Test**: Supported, unsupported, unknown, absent and unmanaged fixtures.

**Acceptance Scenarios**:

1. Current supported/unknown installations show concise lifecycle text.
2. Proven incompatibility shows warning text and an appropriate next action without mistaking bundled capability for installed evidence.

### Edge Cases

Restored open panes; rapid open/close; DPI and display changes; nonzero/negative desktop origins; small usable areas; maximized windows; stale settings drafts; absent package files; legacy and manual inventories; repeated/changed failures; multiple Live/PTS roots; recovery and cleanup after commit.

## Requirements

### Functional Requirements

- **FR-001**: Preserve chosen outer geometry when log content already fits; bound necessary growth to usable desktop space.
- **FR-002**: Preserve control accessibility and splitter non-overlap through small-display and multi-frame transitions.
- **FR-003**: Persist and initialize log visibility independently of height and capture verbosity, including legacy preference loading.
- **FR-004**: Render a real uninstall modal without dashboard reflow; only explicit confirmation removes managed files.
- **FR-005**: Preserve all existing package ownership, transactional recovery, SavedVariables, history and neighboring files.
- **FR-006**: Show concise normal lifecycle state; warn about API support only from known installed incompatibility.
- **FR-007**: Repair reproducible migration failure conditions and retain diagnostic evidence without logging private payloads.
- **FR-008**: Deduplicate unchanged failures per root while continuing bounded retries; expose changed failure and recovery outcomes truthfully.
- **FR-009**: Update conflicting historical contract annotations and canonical documentation; pass formatting, Clippy, tests and applicable policy gates before commit.

### Key Entities

- UI preference: persisted log visibility, separately from pane height and logging verbosity.
- Viewport geometry: current inner/outer bounds, usable display bounds, maximized state.
- Migration attempt: bounded retry time, last diagnostic identity, root/environment and actual lifecycle outcome.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Repeated log toggles with sufficient room emit zero size/position changes.
- **SC-002**: Both visibility choices survive save/reconstruction with unchanged verbosity and pane height.
- **SC-003**: The uninstall modal produces zero underlying dashboard displacement and no removal on dismissal.
- **SC-004**: Normal PixelBeacon status contains zero routine supported/unknown API suffixes.
- **SC-005**: Repeated identical failures emit one warning per root/failure episode while recovery remains possible.

## Assumptions and Clarifications

The owner's current issues supersede unconditional grow/shrink and inline-confirmation rules in S029/S030. A missing visibility field retains the existing closed fresh-install default. Package maintenance remains independent of game process presence. No dependency changes, release, tag, merge, or owner field-testing task is included. Exact installed failure details are sought read-only; fixtures and newly retained error categories must distinguish evidence gaps from proven causes.

## Delivery

The owner merged PR #311 as `be1a054a45d18a74dfaa3e9b7947e902e3d6b18b`;
issues #306-#310 are closed and Done. See the [verification receipt](verification.md)
for original validation and later merge evidence. Plan059 is
[archived](../../docs/archive/build-plans/plan-059.md). S133 remains unreleased.
