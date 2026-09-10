# Feature Specification: Quality-Aware Encounter History UI

**Feature Branch**: `codex/s078-encounter-history-ui`
**Created**: 2026-09-10
**Status**: In progress
**Input**: Issue #135 and the S077 encounter-metrics handoff

## User Scenarios and Testing

### User Story 1 - Import and browse local encounter history (Priority: P1)

A user opens Encounter History, imports the retained terminal capture from the
explicitly selected ESO environment, and browses deterministic raw summaries
without using a terminal or exposing encounter data outside the computer.

**Independent Test**: Point the history service at a valid SavedVariables fixture
and empty application data root, import once, refresh, and verify one summary.
Repeat the import and verify that the same record is reported as already present.

**Acceptance Scenarios**:

1. **Given** a resolved Live or PTS AddOns environment and terminal capture,
   **when** the user chooses Import Current Capture, **then** the existing bounded
   S076 importer stores it under the application encounter-data directory and the
   history refreshes.
2. **Given** no encounter store or an empty valid store, **when** history opens,
   **then** a clear empty state appears without creating or replacing a store.
3. **Given** a corrupt or unsupported store, **when** history opens, **then** the
   store remains untouched and an actionable unavailable state appears.
4. **Given** a repeated import of the same canonical capture, **when** import
   completes, **then** the UI reports that it was already present and does not add
   a duplicate.

---

### User Story 2 - Inspect observed metrics with visible quality (Priority: P1)

A user selects one encounter and sees its observed metrics, algorithm version,
catalog provenance, capture completeness, declared loss, and unresolved IDs.

**Independent Test**: Load complete and degraded fixtures through the history
service. Verify that complete facts, degraded labels, exact loss ranges, unknown
IDs, algorithm version, and catalog version are all present in the view data.

**Acceptance Scenarios**:

1. **Given** a compatible catalog and complete capture, **when** the user selects
   the encounter, **then** DPS, effective HPS, ability damage share, effect uptime,
   and cast order appear as observed values with complete quality.
2. **Given** a valid partial capture with declared loss, **when** details load,
   **then** every affected result remains visible as observed and degraded, with
   the exact missing sequence ranges and reasons.
3. **Given** unknown numeric references, **when** details load, **then** sorted
   unknown IDs remain visible beside the catalog version and semantic identity.
4. **Given** a missing, corrupt, channel-incompatible, or API-incompatible catalog,
   **when** details are requested, **then** the raw summary remains visible and a
   specific catalog state replaces only the derived detail.
5. **Given** a zero-duration metric, **when** details load, **then** the UI says the
   observed value is unavailable rather than displaying zero or dividing by zero.

---

### User Story 3 - Control local retention explicitly (Priority: P2)

A user can remove one selected encounter or all encounters through deliberate,
confirmed actions while unrelated application features continue working.

**Independent Test**: Seed two encounters, request deletion without confirmation,
confirm deletion of one, then confirm deletion of all. Verify counts after every
step and confirm that no automatic deletion occurs.

**Acceptance Scenarios**:

1. **Given** a selected encounter, **when** Delete Encounter is chosen, **then** a
   confirmation names the local record and no deletion occurs before confirmation.
2. **Given** multiple encounters, **when** Delete All is confirmed, **then** only
   encounter raw records are removed and the empty state appears.
3. **Given** a failed deletion or refresh, **when** the operation ends, **then** the
   prior store remains in place and the UI reports the failure.

### Edge Cases

- The configured application data directory or AddOns environment is unavailable.
- The source capture disappears or changes during import.
- The encounter store is absent, empty, corrupt, locked, or future-versioned.
- A summary disappears between refresh and detail loading or deletion.
- The active catalog changes while the history window is open.
- A compatible catalog resolves no IDs or leaves many IDs unknown.
- Long histories, cast sequences, IDs, and loss lists require scrolling.
- The main window is narrow or the history window is resized below its ideal size.

## Requirements

### Functional Requirements

- **FR-001**: Encounter History MUST be an explicit user-opened local UI surface.
- **FR-002**: The UI MUST use one application-owned encounter store below the
  resolved application data root and MUST keep it separate from settings and the
  catalog database.
- **FR-003**: Import MUST occur only after an explicit user action, use the
  selected ESO environment, and call the bounded non-executing S076 importer.
- **FR-004**: The UI MUST NOT scan arbitrary directories, execute SavedVariables,
  upload, transmit, or add telemetry for encounter data.
- **FR-005**: Missing input and missing store states MUST be non-destructive and
  clearly distinct from corrupt or unsupported data.
- **FR-006**: History listing MUST be deterministic and MUST expose encounter
  channel, complete or partial status, time range, stored count, omitted count,
  and stable encounter identity.
- **FR-007**: Selecting one history entry MUST load the immutable raw record and
  calculate S077 projection data on demand against the active catalog path.
- **FR-008**: The implementation MUST NOT add a derived database or write
  projections into the raw store. Derived details are disposable in-memory state.
- **FR-009**: Detail presentation MUST show projection schema version, algorithm
  version, catalog schema version, catalog version, catalog semantic SHA-256,
  channel, API version, and raw content SHA-256.
- **FR-010**: Detail presentation MUST show observed DPS, effective HPS, ability
  damage share, effect uptime, and ordered cast sequence without implying live
  Combat Metrics parity.
- **FR-011**: Every metric MUST show complete or degraded quality. Degraded results
  MUST show exact sorted missing sequence ranges and reasons.
- **FR-012**: Unknown positive numeric IDs MUST remain visible and sorted. Known ID
  counts MUST also be visible as catalog-coverage context.
- **FR-013**: Unavailable numeric results MUST be labeled unavailable rather than
  coerced to zero.
- **FR-014**: Catalog missing, catalog corrupt, channel mismatch, API mismatch,
  store corruption, missing encounter, and operation failure MUST produce clear,
  stable presentation states while preserving any raw summaries already loaded.
- **FR-015**: Import, list, projection, and deletion I/O MUST run outside the GUI
  render thread through a bounded worker command channel.
- **FR-016**: The UI MUST prevent conflicting history operations and expose a
  visible busy state until the current operation completes.
- **FR-017**: One-record and all-record deletion MUST require separate explicit
  confirmations and MUST use the S076 transactional deletion operations.
- **FR-018**: The feature MUST perform no automatic pruning, deletion, migration,
  catalog mutation, retention decision, recommendation, or gameplay action.
- **FR-019**: Encounter history state MUST remain independent from Weaving,
  Fishing, Auto Potion, Pixel Bus, input authorization, and game focus.
- **FR-020**: UI strings and pure presentation helpers MUST be testable without a
  window manager; critical rendered interactions MUST have headless egui coverage.
- **FR-021**: Canonical documentation MUST describe the app-owned store, explicit
  import, observed-quality wording, catalog mismatch behavior, and deletion flow.

### Key Entities

- **EncounterHistoryService**: Owns explicit paths and composes S076 storage with
  S077 projection without owning gameplay or catalog mutation.
- **HistorySnapshot**: Empty or deterministically ordered raw encounter summaries.
- **EncounterDetail**: One rebuildable in-memory S077 projection and its visible
  provenance, quality, loss, and catalog-coverage facts.
- **HistoryDiagnostic**: Stable category and safe message for source, store,
  catalog, compatibility, or missing-record failure.
- **HistoryWorker**: Single background command processor for refresh, import,
  detail load, and confirmed deletion.

## Success Criteria

- **SC-001**: A valid explicit capture import appears in history on the next worker
  result, and 100 repeated imports still produce one raw record.
- **SC-002**: Complete and degraded fixtures expose all five metric families with
  correct quality, exact loss, versions, hashes, and unknown IDs.
- **SC-003**: Missing/corrupt stores and missing/corrupt/mismatched catalogs map to
  distinct tested states without replacing the store or hiding loaded summaries.
- **SC-004**: Deleting one of two records leaves exactly one; deleting all leaves a
  clear empty state; no deletion occurs before confirmation.
- **SC-005**: Headless rendered tests can open history, select an encounter, observe
  quality language, enter and cancel confirmations, and remain usable at the
  repository's narrow-window test size.
- **SC-006**: Full Rust CI parity and repository documentation, encoding,
  whitespace, forbidden-dash, JSON, spelling, and link gates pass.

## Clarifications

### Session 2026-09-10

- Q: Where should UI-managed raw history live? A: Under the resolved per-user
  application data root at `encounters/encounters.sqlite`, separate from the JSON
  settings file and immutable catalog.
- Q: Should S078 add a derived history database? A: No. List raw summaries and
  calculate only the selected detail in memory. Persisted projections remain
  unjustified until a measured query or caching need exists.
- Q: How does import select a file and channel? A: Derive the fixed
  `SavedVariables/EsoWeaveEncounter.lua` path and Live or PTS channel from the
  existing explicit AddOns environment setting. Do not add arbitrary scanning.
- Q: May long-running operations block rendering? A: No. A single background
  worker serializes disk operations and returns typed UI events.
- Q: Does observed mean equivalent to Combat Metrics? A: No. S078 presents the
  versioned S077 algorithm. Issue #131 separately owns live parity evidence.

## Dependencies

- Issue #133 and S076 bounded import and raw lifecycle (complete).
- Issue #134 and S077 versioned metric projection (complete).
- Issue #135 owns this slice.
- Issue #131 retains live Combat Metrics parity verification.

## Out of Scope

Encounter addon installation or arming controls, automatic capture discovery,
arbitrary file pickers, backups, exports, projection persistence, caching,
retention recommendations, live parity claims, build recommendations, uploads,
telemetry, and any generated-input behavior.
