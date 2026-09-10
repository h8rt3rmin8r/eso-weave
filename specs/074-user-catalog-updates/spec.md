# Feature Specification: User-Initiated Catalog Updates

**Feature Branch**: `codex/s074-user-catalog-updates`

**Created**: 2026-09-09

**Status**: Implemented, pull request pending

**Input**: Issue #118, "Add a user-initiated game-data update workflow with staged progress"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Understand catalog update status without delaying startup (Priority: P1)

A user can start ESO Weave normally and receive a truthful, dismissible catalog
notice that distinguishes the active Live catalog, a newer observed Live game
version, a PTS preview, an available reviewed candidate, a required collector
capture, an offline or stale check, and an unsupported catalog schema.

**Why this priority**: Every later update action depends on one shared version
model, and startup must remain useful when checking fails.

**Independent Test**: Feed invented active-catalog, observed-version, candidate,
collector, and offline states through the pure resolver and UI projection, then
verify every state and channel without opening a database or network connection
on the GUI thread.

**Acceptance Scenarios**:

1. **Given** a valid Live catalog and no newer evidence, **When** the background
   check completes, **Then** the app reports Catalog current without blocking
   startup.
2. **Given** a newer observed Live game version but no accepted candidate,
   **When** the check completes, **Then** the app reports New live data available
   and never starts an update automatically.
3. **Given** only a newer PTS candidate, **When** status is resolved, **Then** it
   is presented as a PTS preview and never as a required Live update.
4. **Given** an unavailable source or timeout, **When** the check finishes,
   **Then** the last accepted catalog remains active and the notice reports an
   offline or stale check.

---

### User Story 2 - Install a reviewed Live candidate safely (Priority: P1)

A user can explicitly choose a review-format candidate from the application-owned
import area, inspect its channel, version, schema, provenance summary, size,
coverage summary, and origin warning, then acknowledge that local integrity does
not prove download origin before installing it. The previous accepted catalog
remains a verified rollback point.

**Why this priority**: S073 produces review candidates, but they deliver no user
value until the application can accept one without risking its working catalog.

**Independent Test**: Install an invented reviewed candidate into a temporary
user-data root, inject failures before publication and first open, and verify
that the active pointer and previous catalog either advance together or remain
unchanged.

**Acceptance Scenarios**:

1. **Given** a compatible review-format Live candidate from a source the user
   trusts, **When** the user acknowledges the origin boundary and confirms
   installation, **Then** the app verifies it in staging, installs it under an
   immutable content identity, opens it read-only, atomically selects it, and
   retains the previous selection for rollback.
2. **Given** a checksum, schema, channel, integrity, disk, cancellation, or first
   open failure, **When** installation stops, **Then** the previous catalog stays
   active and the failed candidate is never selected.
3. **Given** a valid PTS candidate, **When** it is inspected, **Then** it remains
   a separately labeled preview and cannot replace the active Live catalog.
4. **Given** a successful install, **When** the app restarts, **Then** the
   verified user-data catalog overrides the bundled fallback without modifying
   the installation directory.

---

### User Story 3 - Follow truthful progress and cancel safely (Priority: P1)

A user can open a keyboard-operable update modal and follow named stages with
honest determinate or indeterminate progress, concise diagnostics, elapsed time,
and a cancel action that cannot leave a partial catalog active.

**Why this priority**: Catalog construction and verification can be long-running;
the user needs control and accurate feedback rather than a frozen interface or
fabricated percentage.

**Independent Test**: Drive a deterministic worker seam through every stage,
cancel before and during installation, close the app during a run, and assert
the projected modal state, announcements, and durable recovery receipt.

**Acceptance Scenarios**:

1. **Given** a stage with a known byte or record total, **When** work advances,
   **Then** the modal shows completed and total units plus a determinate bar.
2. **Given** a stage without an honest total, **When** work advances, **Then**
   the modal uses an indeterminate state and never fabricates a percentage.
3. **Given** a cancellable stage, **When** the user cancels, **Then** the worker
   stops at a safe boundary, records Cancelled, and preserves the active catalog.
4. **Given** installation has reached its atomic selection boundary, **When**
   cancellation or shutdown is requested, **Then** completion or rollback is
   serialized before shutdown continues.

---

### User Story 4 - Complete a collector-assisted local update (Priority: P2)

A user who needs API-visible data can explicitly choose a collector-assisted
build, review exactly what will be collected, install or update the separately
managed collector, follow in-game save-boundary instructions, and import only a
flushed, restricted SavedVariables capture.

**Why this priority**: Reviewed candidates may not cover account- or
character-visible surfaces, but the collector boundary must remain explicit and
cannot be confused with live desktop processing.

**Independent Test**: Use a temporary AddOns root and invented restricted
capture to exercise managed installation, Waiting for user/game, hostile input
rejection, local candidate construction, capture deletion, and collector
uninstall without touching PixelBeacon.

**Acceptance Scenarios**:

1. **Given** collector-assisted mode, **When** the user reviews the plan, **Then**
   the app lists collected categories, excluded or pseudonymized data, and the
   `/reloadui`, logout, or exit save boundary before installation.
2. **Given** an unflushed or unchanged capture, **When** the app waits, **Then**
   it reports Waiting for user/game and does not claim active processing.
3. **Given** a flushed valid capture, **When** import begins, **Then** it is parsed
   by the S071 restricted grammar, never executed or uploaded, and passed through
   the S073 candidate gates before acceptance.
4. **Given** completion, **When** the user requests cleanup, **Then** the capture
   can be deleted and the separately managed collector can be uninstalled
   without modifying PixelBeacon.

---

### User Story 5 - Roll back and diagnose an accepted update (Priority: P2)

A user can inspect a redacted update receipt, restore the previous verified Live
catalog, and obtain support diagnostics that identify the failure stage without
revealing personal paths, capture contents, or localized source records.

**Why this priority**: Recovery must be a first-class user action rather than an
implicit filesystem repair procedure.

**Independent Test**: Install two invented Live candidates, roll back through
the same verified selection boundary, and inspect receipts and logs for stable
codes and absence of sandbox paths or private fixture content.

**Acceptance Scenarios**:

1. **Given** an accepted update with a previous selection, **When** Roll back is
   confirmed, **Then** the previous candidate is reopened and verified before
   the active selection changes.
2. **Given** any success, cancellation, rollback, or failure, **When** its receipt
   is inspected, **Then** it records version identities, hashes, source summary,
   outcome, and failure stage without local paths or personal content.

### Edge Cases

- The active pointer is missing, truncated, non-canonical, linked, or names a
  missing, linked, corrupt, incompatible, PTS, or tampered catalog.
- The bundled fallback is missing or corrupt while the user selection is valid,
  and the inverse case where only the bundled fallback remains valid.
- Two update requests start concurrently, or shutdown races the final install.
- The candidate import root, staging root, versions root, receipt root, active
  pointer, or candidate path aliases another root through nesting, links,
  reparse points, case, or path normalization.
- A candidate changes after inspection, grows while copied, contains extra
  files, disagrees with its directory hash, or is already installed with
  matching or conflicting bytes.
- Available disk space is exhausted before or after an immutable candidate copy.
- Cancellation arrives during a non-interruptible filesystem operation.
- A prior process left an identifiable staging directory or an incomplete
  operation receipt.
- A Live candidate is older than the active Live catalog, or a PTS API number is
  numerically greater than Live.
- The collector capture is absent, unchanged since instructions were shown,
  oversized, malformed, linked, not yet flushed, or from the wrong channel.
- UI focus starts inside the modal, cycles within it, restores to the invoking
  control, and applies explicit Escape rules during idle, cancellable, and
  commit-boundary states.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: One catalog-version model MUST represent the active catalog,
  bundled fallback, application-supported schema, collector compatibility,
  observed Live game version, available Live candidates, PTS previews, and last
  successful or stale check.
- **FR-002**: The startup check MUST run off the GUI thread with bounded network
  and filesystem operations and MUST never block first-window presentation.
- **FR-003**: The status resolver MUST distinguish Catalog current, New live
  data available, PTS preview available, Collector capture required, Update
  ready to import, Offline/stale check, and Unsupported schema.
- **FR-004**: Live and PTS identities MUST remain structurally distinct. No
  numeric version comparison may promote PTS or present it as a required Live
  update.
- **FR-005**: Checking, acquisition, collector lifecycle, candidate construction,
  installation, selection, rollback, and cleanup MUST begin only from explicit
  user actions except for the bounded read-only startup status check.
- **FR-006**: The application MUST discover reviewed candidates only within a
  documented user-owned import root and MUST verify the complete S073 candidate
  contract before presenting one as installable.
- **FR-006A**: Candidate hashes and manifests MUST be described as integrity
  evidence, not authentication. The modal MUST require an explicit acknowledgement
  that the user obtained the candidate from a trusted review or release source.
- **FR-007**: S074 MUST NOT define an unreviewed remote candidate feed, treat
  workflow artifacts as a stable update service, or download source or game
  asset bytes silently.
- **FR-008**: A candidate summary MUST show channel, game version, API version,
  catalog and schema versions, size, source/provenance summary, coverage or diff
  summary, collector requirement, and compatibility before confirmation.
- **FR-009**: The application MUST reject PTS, older Live, unsupported-schema,
  corrupt, checksum-mismatched, rights-policy-invalid, incomplete, or unverified
  candidates before Live installation.
- **FR-010**: Accepted catalog bytes MUST live in a versioned user-data root and
  MUST never modify the bundled installation directory.
- **FR-011**: Installation MUST copy into a unique same-filesystem staging
  directory, validate all S073 manifest and catalog invariants, publish the
  immutable version without replacement, reopen it read-only, then atomically
  replace a small canonical active-selection record.
- **FR-012**: The previous active selection MUST remain available until the new
  catalog has passed first open and MUST be retained as the rollback point.
- **FR-013**: Runtime catalog resolution MUST prefer a verified active user-data
  Live catalog and fall back to the bundled catalog on missing, invalid,
  incompatible, PTS, or corrupt user selection with a visible diagnostic.
- **FR-014**: Rollback MUST verify and open the previous Live catalog before
  atomically changing selection and MUST never select PTS.
- **FR-015**: Only one update or rollback operation may own the catalog update
  root at a time. Concurrent requests MUST fail visibly without changing state.
- **FR-016**: Shutdown and the final selection boundary MUST be serialized so a
  process cannot exit between an ambiguous partial selection write and recovery.
- **FR-017**: Incomplete staging directories and interrupted operation receipts
  MUST be identifiable and recoverable on the next startup without deleting any
  accepted immutable catalog.
- **FR-018**: Every operation MUST report stable ordered stages drawn from
  Checking, Locating sources, Waiting for capture, Validating, Normalizing,
  Building, Resolving icons, Integrity checking, Installing, Opening, Complete,
  Cancelled, and Failed.
- **FR-019**: Progress MUST be determinate only when an honest total exists;
  otherwise it MUST be explicitly indeterminate. Stage detail may include
  bounded record or byte counts and elapsed time.
- **FR-020**: Cancellation MUST be cooperative at documented safe boundaries.
  The final pointer replacement is non-interruptible and MUST complete or restore
  the preceding canonical pointer before reporting an outcome.
- **FR-021**: The UI MUST expose a dismissible update notice and a responsive
  modal with current and candidate summaries, source choice, progress, concise
  diagnostics, post-update summary, rollback, and cleanup actions.
- **FR-022**: The modal MUST be keyboard operable, trap focus while open, restore
  focus to its invoking control, define Escape and cancel behavior, announce
  meaningful stage changes without per-record chatter, communicate status in
  text as well as color, and honor reduced motion.
- **FR-023**: Collector-assisted mode MUST reuse the S071 separately managed
  collector lifecycle and restricted parser and MUST never execute Lua, upload a
  capture, modify PixelBeacon, synthesize input, or claim in-memory game state.
- **FR-024**: Before collector installation, the UI MUST explain collected
  categories, excluded or pseudonymized data, and the supported `/reloadui`,
  logout, or exit SavedVariables flush boundaries.
- **FR-025**: A collector capture MUST be demonstrably flushed after the user
  entered the waiting stage before import can begin. Unchanged or unstable input
  remains Waiting for user/game.
- **FR-026**: Collector-assisted construction MUST pass through the S073 pipeline
  and the same S070 catalog verification and S072 placeholder behavior as a
  reviewed prebuilt candidate.
- **FR-027**: The user MUST be able to delete a completed local capture and
  uninstall the managed collector through existing containment and marker gates.
- **FR-028**: Success, cancellation, rollback, recovery, and failure MUST attempt
  bounded canonical redacted receipts with old and new version identities,
  hashes, source summary, result, and stable failure stage. Receipt persistence
  failure MUST preserve selection and surface the stable `receipt-write-failed`
  finding in memory.
- **FR-029**: Receipts, logs, notices, and UI diagnostics MUST NOT contain local
  absolute paths, capture contents, account or character identifiers,
  credentials, localized source text, or icon bytes.
- **FR-030**: Injected offline, unavailable source, unsupported schema, malformed
  capture, checksum mismatch, disk exhaustion, cancellation, first-open failure,
  concurrent request, and shutdown-race cases MUST preserve the last known-good
  active catalog.
- **FR-031**: Windows and Linux MUST use the same state machine, file contracts,
  and path-containment rules, with platform-specific roots supplied through an
  explicit seam.
- **FR-032**: User and maintainer documentation MUST explain detection, candidate
  placement, update stages, collector save boundaries, cancellation, rollback,
  capture deletion, recovery, privacy, and support diagnostics.

### Key Entities

- **Catalog Environment**: Application-supported schema, bundled fallback,
  user-data roots, collector version, and current observed Live game/API evidence.
- **Catalog Selection**: Canonical active and previous immutable Live candidate
  identities, selection generation, and last completed operation receipt.
- **Update Availability**: One resolved Live state plus independent PTS preview,
  check freshness, candidate summaries, and collector requirement.
- **Candidate Summary**: Redacted inspected metadata for an integrity-verified S073
  candidate, including version, hashes, size, sources, diff, icons, and
  compatibility. It never claims authenticated origin.
- **Update Request**: Explicit user choice of reviewed candidate or local
  collector-assisted build plus cancellation authority.
- **Update Operation**: Exclusive state machine with stage, progress, elapsed
  time, cancellation boundary, logs, and terminal result.
- **Update Receipt**: Canonical redacted durable evidence for success, rollback,
  cancellation, recovery, or failure.
- **Catalog Roots**: User-owned import, staging, immutable versions, receipts,
  source cache, icon cache, and active-selection paths with non-aliasing rules.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The first application window remains independent of the catalog
  status worker, and every injected timeout or source failure yields a usable UI
  with the prior catalog.
- **SC-002**: One resolver produces the seven required Live status states and an
  independent PTS preview for 100 percent of the decision matrix fixtures.
- **SC-003**: Every injected validation, copy, disk, cancellation, first-open,
  concurrency, and shutdown failure changes zero bytes in the preceding active
  selection and accepted catalog.
- **SC-004**: A successful install and rollback each select only a verified Live
  catalog, survive restart, and leave the bundled fallback unchanged.
- **SC-005**: Every update stage has an exact visible text state; 100 percent of
  percentages shown in tests have a real total, and unknown totals show no
  percentage.
- **SC-006**: Keyboard tests cover modal entry, full focus cycling, Escape rules,
  cancellation, terminal dismissal, and focus restoration at narrow and normal
  window sizes.
- **SC-007**: Collector-assisted fixtures prove no Lua execution, upload,
  PixelBeacon mutation, unstable capture import, or activation outside the S073
  verification boundary.
- **SC-008**: Automated privacy scans find zero fixture paths, private values,
  source payloads, capture payloads, localized text, or icon bytes in receipts,
  logs, notices, and candidate summaries.
- **SC-009**: Equivalent Windows and Linux runs produce the same selection,
  receipt, availability, and progress-state contracts.

## Assumptions

- S070 remains authoritative for catalog schema, deterministic compilation,
  integrity, semantic checksum, and read-only opening.
- S071 remains authoritative for collector lifecycle, capture bounds, and the
  restricted SavedVariables grammar.
- S072 remains authoritative for user-local icon transformation and placeholder
  fallback.
- S073 remains authoritative for review candidate verification, source policy,
  channel identity, and collector-assisted construction.
- No stable approved remote candidate distribution endpoint exists yet.
  Reviewed candidates are imported through a documented user-owned directory;
  adding a feed, signature authority, or automatic download requires separate
  source-policy and threat-model work.
- S073 candidate identity proves internal integrity, not authenticated origin;
  the user remains responsible for obtaining an imported candidate from a
  trusted reviewed source until a separate signing authority exists.
- The existing bounded GitHub Live version check may observe game-version drift,
  but it is not evidence that an installable candidate exists.
- Verification issues #110, #129, and #131 remain separate field-evidence work
  and do not block deterministic S074 implementation tests.

## Out of Scope

- Silent background downloads, automatic updates, unattended collector runs,
  automatic `/reloadui`, launching ESO, or input synthesis.
- Treating a PTS candidate as Live or adding automatic PTS promotion.
- A new remote catalog service, GitHub Actions artifact client, signing service,
  credential flow, or release protocol.
- New collector categories, combat telemetry, encounter storage, metrics, or
  recommendations from issues #132 through #136.
- Bundling, uploading, or redistributing game icon bytes or capture contents.
- Replacing the application binary, addon manager, or package manager.
