# Feature Specification: Unified PixelBeacon and Automatic Catalog Maintenance

**Feature Branch**: `codex/s129-automatic-catalog-maintenance`
**Created**: 2026-10-15
**Status**: Specified
**Input**: Owner kickoff for S129, issues #285, #286, #287, #288 and #291, automatic push and official PR, at most two Codex review rounds, owner final review and merge.

## User Scenarios & Testing

### User Story 1 - One companion package (Priority: P1)

Players install and maintain one PixelBeacon addon. Recognized old installations migrate automatically through that lifecycle, preserving observations, settings and history.

**Why this priority**: Automatic capabilities need one unambiguous installed package and data owner.
**Independent Test**: Temporary AddOns trees cover neither/one/both legacy packages, current combined, interrupted migration, unmanaged/conflicting inventory, independent Live/PTS roots, race/failure injection and exact-byte preservation.
**Acceptance Scenarios**:
1. Either or both managed legacy packages migrate through normal PixelBeacon deployment to one complete package.
2. Links, raced targets, foreign entries and unknown ownership never broaden replacement/removal.
3. Catalog bootstrap failure does not prevent existing beacon observations and geometry.

### User Story 2 - Definitions maintain themselves (Priority: P1)

Eligible Catalog collection starts when game data is ready. Stable completed data becomes the active Catalog without commands, selection, reload/save instructions, acknowledgement or maintenance interaction.

**Why this priority**: Collection through activation must provide one useful path.
**Independent Test**: Lua event doubles and changing saved-data fixtures cover initialization, combat/loading, interruption, natural saves, cold/warm startup and later generations without native logging.
**Acceptance Scenarios**:
1. Startup consumes existing complete data without requiring a subsequent save.
2. Bounded automatic collection resumes/restarts after interruption while retaining the last complete generation.
3. Partial writes, lost notifications, locks, bad input and failed activation use bounded retry and preserve last-good data.
4. Semantic replay and encounter-only shared-file rewrites do not churn effective selection.

### User Story 3 - Useful accumulated definitions (Priority: P1)

Independent captures populate the real packaged starting Catalog. Later characters, locales and scopes improve it without erasing supported facts or treating new provenance as removed coverage.

**Why this priority**: Empty valid builds and provenance-dependent rejection do not provide maintenance.
**Independent Test**: Query actual entities, attributes, relationships, localization, provenance and coverage after independent first/second captures, reversed input order and replay.
**Acceptance Scenarios**:
1. The actual packaged artifact accepts an independent first capture and a changed second capture.
2. Disjoint scopes/locales retain their union, including shared entities with multiple parents.
3. Genuine integrity/coverage failures preserve working data; unsupported definitions remain unknown without fabrication.
4. Channel, game/API, schema/collector and historical compatibility remain explicit.

### User Story 4 - Independent persistent optouts (Priority: P1)

Two direct controls persist independently: Disable Catalog maintenance and Disable native encounter log processing. Both default to automatic operation requested. Without a recognized combined addon, both services are dormant and both controls inactive, retaining saved choices.

**Why this priority**: Automatic work requires direct, durable owner control and truthful eligibility.
**Independent Test**: Settings migration/round trips, persistence failures, pure eligibility/cancellation generations, package/root transitions and mouse/keyboard/accessibility fixtures.
**Acceptance Scenarios**:
1. Controls persist immediately without Apply/confirmation and never change game logging or gameplay features.
2. Disable stops desktop scheduling/stale commits; enable reconciles retained data automatically.
3. Absence, inspection uncertainty, reinstall or Settings environment selection cannot reset choices or authorize another root.
4. Loaded Lua receives the persisted Catalog choice at the next natural addon load, without live SavedVariables writes or forced reload.

### Edge Cases

Migration interruption/postcommit cleanup failure; forged/missing markers; foreign files and links; Live/PTS coexistence; game running/unknown; API failure; duplicate initialization; combat/loading; partial/locked files; corrupt selection; unsupported versions; storage exhaustion; postcommit receipt failure; disable/shutdown races; multiple characters/locales/parents; unsupported source categories; preference persistence failure.

## Requirements

### Functional Requirements

- **FR-001**: One deterministic PixelBeacon inventory/manifest/version and reviewed API declaration preserve every beacon/decoder/geometry contract (#285).
- **FR-002**: Recognized managed legacy packages migrate automatically with staged replacement/recovery, exact ownership and race/link checks, preserving saved/unexpected/neighbor/environment bytes (#285).
- **FR-003**: Beacon/Catalog have independent state/events/failures; Catalog failure cannot stop beacon or auto-enable/mirror the old encounter recorder (#285).
- **FR-004**: Automatic collection detects actual environment/version/locale/scope, covers retained definition groups, coalesces relevant refreshes and resumes after combat/loading (#286).
- **FR-005**: Scanning/finalization/checksum/memory/retry/generations are bounded; interruption preserves last complete data; game save requests never imply durability or require player action (#286).
- **FR-006**: Independent captures populate the packaged baseline and deterministically accumulate supported facts/relationships/locales/provenance, separating semantic coverage from acquisition identity (#287).
- **FR-007**: Genuine integrity/regression checks, explicit authoritative removal, unknown observations, historical access and channel/version/schema isolation remain; audit and implement source/category paths for retained capabilities (#287).
- **FR-008**: Bounded background reconciliation starts without delaying the first window, discovers eligible contexts and existing/later completed data without manual boundaries/acknowledgement (#288).
- **FR-009**: Verify/open immutable candidates before atomic selection, coherently notify retained consumers, preserve compatible history and recover last-good before packaged fallback (#288).
- **FR-010**: Bound retries/queues/retention/shutdown; semantic replay/unrelated writes do not churn; cancellation blocks stale selection and receipt failure cannot undo valid commit (#288).
- **FR-011**: Two typed booleans persist independently through migration/import/export; missing values request automatic operation; persistence failure has truthful current-run/durable semantics (#291).
- **FR-012**: Shared per-root eligibility distinguishes compatible presence from overwrite ownership; absence means both dormant/controls inactive; compatible manually maintained packages permit optouts without granting mutation (#291).
- **FR-013**: Accessible direct controls stop desktop work independently, retain data/progress and resume automatically; Catalog startup preference applies at natural Lua load (#291).
- **FR-014**: Preserve authenticated reads, history and unrelated gameplay. No new maintenance banner/modal/chat/manual prerequisite. Update affected docs without claiming #289/#290/#292/#293/#294 complete.
- **FR-015**: Complete analysis, test-first regressions and required gates, official PR, at most two reviews and owner merge handoff; no release/tag or owner field-testing requirement.

### Key Entities

Combined installation (root, identity/capabilities, ownership); independent persisted optouts; effective eligibility (root, inspection, cancellation generation); complete collection (identity, environment/version/locale/scope, payload); accumulated Catalog (facts, semantic coverage, lineage); selected immutable generation (verified identity/path/history); processing receipt (input/outcome separate from semantic selection).

## Success Criteria

- **SC-001**: All migration rows produce one coherent installation or preserve previous/unrelated bytes after failure.
- **SC-002**: Automatic startup/later-save journeys require zero maintenance commands, acknowledgements or save instructions.
- **SC-003**: Independent first/second captures contribute queryable facts, preserve supported union, and replay leaves effective data unchanged.
- **SC-004**: Every preference/availability combination is deterministic; both controls are inactive without eligible presence; stale work cannot commit after reconciled stop.
- **SC-005**: Final-head automated gates pass and all findings resolve, with synthetic/repository evidence distinguished from unperformed game behavior.

## Clarifications

### Session 2026-10-15

- Native optout owns preference/eligibility/cancellation seams before #290 consumes them; implementing the native watcher is subsequent work.
- #292 owns full encounter/viewer removal after #293 transfers processing. This slice retires separate lifecycle/routine Catalog interactions as necessary for its five complete outcomes.
- Priority save is a request; completed disk data requires stable naturally saved evidence.
- Empty valid builds do not count as useful population; independent captures must add queryable supported facts.
- Owner-selected topology supersedes old two-package/manual-Catalog product rules; amend those while preserving ownership/data integrity.
- A recognized manual combined package permits only an exact, separately validated startup-configuration update. That configuration contract does not grant ownership to replace/delete its manifest, modules or extra files. Transactional configuration recovery precedes eligibility inspection.
- ESO names saved-data files after the addon manifest, independently of the saved table name. Preserve legacy EsoWeaveData.lua in place and automatically ingest both it and the unified package's PixelBeacon.lua, with independent partial-write handling and compatible accumulation. Existing encounter/history readers retain their legacy input; no game-owned saved-data file is rewritten or moved.
- Package inspection, configuration/mutation and recovery share a per-root serialization boundary. Background reconciliation must not clean another in-process lifecycle transaction's journal or hold a Windows file handle across package retirement; external ownership and race checks remain required.

## Assumptions

Reuse existing Rust/Lua/SQLite and Windows/Linux root seams. No new game ingress transport, distributed seed prerequisite, native logging dependency, history retention policy or release authorization. Audit the full five issue acceptance lists before closure.
