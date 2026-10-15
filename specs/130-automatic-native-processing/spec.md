# Feature Specification: Automatic Native Processing and Workflow Retirement

**Feature Branch**: `codex/s130-automatic-native-processing`
**Created**: 2026-10-15
**Status**: Specification validated, normal/separate v15 scope approved
**Input**: Owner kickoff of S130 for #289, #290 and #292-#294. Automatic push/official PR and at most two Codex review rounds are authorized. Stop for owner final review and merge.

## User Scenarios & Testing

### User Story 1 - Process native fights automatically (Priority: P1)

The player enables ESO native logging. An eligible desktop automatically discovers actual Live/PTS logs, catches up completed fights and processes growing files without a picker, import or viewer.

**Why this priority**: Completes automatic native processing after S129's combined package/preferences foundation.
**Independent Test**: Synthetic application startup, catchup and active append with normal/separate version 15 and no visible encounter window.

**Acceptance Scenarios**:

1. Given completed historical fights, when the eligible desktop starts, then bounded catchup stores them without interaction or source mutation.
2. Given split records/Unicode and an active fight, when bytes arrive, then pending bytes wait and a valid combat end publishes independently of logging-run end.
3. Given unsupported or malformed input, when parsing proceeds, then good committed evidence survives and only verified recovery boundaries resume output.
4. Given normal/separate version 15 output, when read, then supported source facts remain available without changing game settings.

### User Story 2 - Resume without loss or duplication (Priority: P1)

Restart, copy, rotation, truncation and failure preserve completed fights, source facts and durable progress while keeping distinct genuine runs distinct.

**Why this priority**: Automatic retries cannot silently lose or duplicate history.
**Independent Test**: Compare uninterrupted and interrupted ingestion, failure injection, old stores and file-generation changes.

**Acceptance Scenarios**:

1. Given a crash at either side of persistence, when restarted, then completed fights and progress agree, without duplicate or missing committed results.
2. Given rotation/replacement/truncation/copy, when a source generation changes, then partial tails/context never mix and existing canonical evidence stays intact.
3. Given disabled processing or addon absence, when queued work completes, then no commit crosses the declared stop boundary and retained reads remain usable.
4. Given supported retained deletion, when catchup retries, then deliberately deleted records are not resurrected.

### User Story 3 - Read coherent data without a viewer (Priority: P1)

Local clients keep authenticated read-only access to preserved addon/native history and supported calculations while background services own processing and Catalog refresh.

**Why this priority**: Removing windows must preserve their required data capabilities.
**Independent Test**: Native commits, Catalog swaps, historical stores, concurrent HTTP/MCP reads, restart and disable transitions with no selected encounter.

**Acceptance Scenarios**:

1. Given a Catalog swap, when consumers read/calculate, then results bind actual source/Catalog/algorithm generations and stale work cannot overwrite new results.
2. Given absent/disabled collection, when clients query existing databases, then data access remains independent of collection eligibility.
3. Given missing Catalog or addon-only observations, when native facts are calculated, then supported facts remain available and unsupported quantities stay explicitly unavailable.

### User Story 4 - Use the streamlined interface and accurate guide (Priority: P2)

Keep one PixelBeacon lifecycle and the two independent disable controls. Remove Encounter History, Catalog/manual-processing interactions and obsolete player commands. Current help/assets agree with implemented behavior.

**Why this priority**: Finishes approved interaction requirements after background ownership exists.
**Independent Test**: UI/accessibility journeys across themes/scales/layouts plus current-source/generated/embedded documentation audits.

**Acceptance Scenarios**:

1. Given startup/success/absence/failure/recovery, when navigating, then removed viewers, imports, dialogs and notices never appear or acquire replacement destinations.
2. Given absent combined addon, when shown, then both disable controls are inactive and both services dormant while saved preferences remain intact.
3. Given active documentation/assets, when following instructions, then no removed current workflow remains and historical compatibility references are explicitly justified.

### Edge Cases

- Arbitrary byte splits, UTF-8, CRLF, quoted commas/escapes, nested groups, repeated empty feeds and quiet active fights.
- Multiple run headers, nested/orphan boundaries, reversed clocks, unknown evidence, malformed known records and unsupported versions.
- Corrupt/mismatched checkpoints, huge lifetime logs, adversarial context/token amplification, missing/late sources, locks and permission errors.
- Disk-full/DB contention, restart, backup/deletion, disable/removal/shutdown races, independent environments and incompatible Catalogs.

## Requirements

### Functional Requirements

- **FR-001**: Incremental native parsing retains verbatim incomplete tails, explicit context and independently completed fights; quiet/empty input never fabricates interruption/completion.
- **FR-002**: S130 supports normal/separate native log version 15 using reviewed grammar/tests. Normal/inline (#296) and verbose/separate plus verbose/inline (#297) follow separately. Separate observed evidence, synthetic coverage, unknown evidence, malformed known shapes and unsupported input.
- **FR-003**: Stable source/run/fight identity covers chunking, restart, append, copies and rotation without conflating genuine runs. Preserve old identities/canonical evidence by compatible reading or transactional migration.
- **FR-004**: Validate restart state and source association before output; preserve offsets, ordered context/provenance and uncertainty. Resume only at verified recovery boundaries.
- **FR-005**: Bound feed/read/record/context/fight/queue/retry/transaction work rather than lifetime file size/age. Keep parser independent of UI, watchers and eligibility.
- **FR-006**: Discover actual redirected Windows Documents and supported Linux Steam/Proton Live/PTS roots, overrides and later appearing sources; catch up without UI and never relabel work using later selection.
- **FR-007**: Reuse independent persistent preferences and combined-package eligibility; coordinate stop/cancellation/shutdown with commits. Stored reads remain available regardless of collection eligibility.
- **FR-008**: Commit fights and progress consistently, detect source generations, preserve retryability after failure and never change ESO log bytes.
- **FR-009**: Leave game logging/serialization/anonymity untouched; stale-file existence is not active logging evidence. Never enable/mirror legacy recording, synthesize commands or add gameplay ingress.
- **FR-010**: Preserve historical addon/native bytes, hashes, identity/order/context/loss and supported results. Do not resurrect supported deletions or reset corrupt stores.
- **FR-011**: Application services own processing/refresh. Catalog generations reach typed access, queries, calculations and caches coherently; reject stale results and skip unchanged rebuilds.
- **FR-012**: Retain opt-in authenticated read-only HTTP/MCP access to fixed catalog/encounters databases. Evolve public observations with tested compatibility/versioning and truthful background state.
- **FR-013**: Native independent facts work without Catalog. Catalog joins require compatible evidence; unavailable capabilities never fabricate API/callback values or zero measurements.
- **FR-014**: Remove the entire Encounter History viewer/menu, manual source/import/delete/recording workflows and remaining Catalog/Data surfaces, obsolete player commands/help/messages and unreachable presentation state. Retain one PixelBeacon lifecycle and exactly two accessible disable controls.
- **FR-015**: Inventory/reconcile every affected current prose/help/navigation/contract/policy/test/screenshot/diagram/caption/generated/embedded surface. Give retained compatibility/history references concrete reasons and prevent stale current workflows returning.
- **FR-016**: Prove deterministic parser/storage/service/UI/API acceptance and required repository gates. Separate tested evidence from unperformed installed behavior; complete at most two Codex rounds and stop before owner merge. No release/tag or owner field-testing task.

### Key Entities

- **Source generation**: Actual environment/file identity, verified content association, committed offset and validated restart state.
- **Native run/fight**: Stable identity and immutable ordered evidence, completion/uncertainty, provenance and qualified observations.
- **Ingest transaction**: Completed output and durable progress acknowledged together.
- **Processing generation**: Eligibility/cancellation, source, Catalog and algorithm identities binding work/results.
- **Current-surface inventory**: Each affected current surface and its update/removal or justified compatibility disposition.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Single-feed and arbitrary meaningful byte splits produce equivalent completed fights/source facts for normal/separate version 15.
- **SC-002**: Restart/commit-failure journeys retain completed fights exactly once, preserve source bytes and separate actual environments/runs.
- **SC-003**: Catchup and live append work with all removed windows permanently absent and zero manual-processing actions.
- **SC-004**: Disable/absence/shutdown and independent source failures preserve stored reads and prevent post-stop ingest within declared boundaries.
- **SC-005**: Historical store fixtures preserve evidence/identity/capabilities; missing facts never become invented measurements.
- **SC-006**: Zero removed controls/navigation or unexplained stale current references remain in audited application/help/contracts/assets/generated output.
- **SC-007**: Required local/final-head hosted gates pass, all findings resolve within two rounds, and delivery stops before owner merge.

## Assumptions

- S129 is merged at `9a2580d07238822a2952f488856a419cc63a8f14`; use its combined package, automatic Catalog and preference/cancellation foundation.
- All five issues belong to this slice. A partial service or proposed future behavior cannot close any acceptance gap.
- ESO owns logging and files; this desktop only observes sources and retains local data.
- Deterministic repository/synthetic evidence is required; owner field testing and a release are outside this scope.

## Clarifications

### Session 2026-10-15

- Q: Retain a passive viewer after retiring manual processing? A: No. #292 requires the entire Encounter History viewer and menu removed, without a replacement destination.
- Q: Run native processing without the combined addon? A: No. Reuse S129 per-root eligibility and independent persisted optouts; stored reads stay available.
- Q: Does an existing log establish active game logging? A: No. Report activity unknown without a current verified observation; completed catchup is independent.
- Q: Can missing native facts become addon-equivalent calculations? A: No. Preserve source-independent facts and explicitly unavailable capabilities.
- Q: May synthetic fixtures invent producer layouts for unsupported profiles? A: No. #289 requires declared grammar grounded in reviewed primary sources. Synthetic partitions exercise a substantiated grammar; they do not establish its producer fidelity.

### Owner scope revision 2026-10-15

The owner explicitly approved normal/separate native log version 15 for S130. Normal/inline support is tracked in #296 and verbose/separate plus verbose/inline in #297. Unsupported layouts remain controlled outcomes; no game setting changes or player format-fixing workflow is introduced. All other #289/#290/#292-#294 acceptance, automatic push/PR and the two-round review cap remain in force.
