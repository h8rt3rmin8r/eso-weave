# Feature Specification: Completed Native Encounter Log Import

**Feature Branch**: `codex/s127-native-completed-import`
**Created**: 2026-10-13
**Status**: Implemented; owner review and merge pending
**Slice**: S127
**Issue**: [#272](https://github.com/h8rt3rmin8r/eso-weave/issues/272)
**Input**: Complete native completed-file import under spec-kit/autopilot; automatic push and official PR, at most two review rounds, owner final review and merge.

## Clarifications

### Session 2026-10-13

- Q: Does native import include live tailing or logger activation? A: Completed-file selection/import only; these remain future work.
- Q: What establishes the environment when the log does not declare it? A: Explicit selected Live/PTS environment, labeled as user-selected provenance.
- Q: How are structurally malformed known records handled? A: Reject the complete import before history writes; keep source and history unchanged. Unknown records remain counted and retained, with capability limitations.
- Q: How is an unterminated final line handled? A: Do not consume it as a complete native record; report its presence, and do not label an unclosed fight complete.

## User Scenarios & Testing

### User Story 1 - Import a completed native recording (Priority: P1)

A player selects Native Encounter Log, chooses a completed local recording, inspects its supported format and completed fights, and imports those fights into Encounter History alongside existing addon captures.

**Why this priority**: The existing native recording should become useful without requiring the player to record it again through an addon.
**Independent Test**: Import a sanitized representative file with at least two completed fights and inspect both in history.

**Acceptance Scenarios**:
1. Given a supported completed native file, when the player selects and imports it, then completed fights appear with native provenance and original order.
2. Given existing addon history, when native fights are imported, then addon data and source files remain unchanged.
3. Given a file larger than the addon spool limit, when it is within the declared native limit, then inspection and import work without blocking the desktop interaction thread.

### User Story 2 - Understand native evidence and metrics (Priority: P1)

A player sees that a native fight comes from the native logger and understands which measurements its observations support.

**Why this priority**: Native and addon observations cannot be presented as interchangeable evidence.
**Independent Test**: Inspect a native fight containing combat observations, plus one lacking the observations needed for an addon-specific measurement.

**Acceptance Scenarios**:
1. Given native records and version metadata, when viewing the imported fight, then source identity, format and observation coverage are explicit.
2. Given an unsupported measurement, when viewing metrics, then its unavailable reason is visible and no fabricated callback evidence or replay result appears.
3. Given relevant native fields, when importing and reloading, then their ordered values remain exact in the local record while default summaries and diagnostics omit raw values.

### User Story 3 - Recover from incomplete or unsupported input (Priority: P2)

A player receives precise counts and recovery guidance when a recording contains unknown records, missing boundaries, malformed data or an incomplete final line.

**Why this priority**: Unsupported or partial observations must not become misleading complete results.
**Independent Test**: Exercise synthetic malformed, unknown, duplicate-boundary, truncated-line, unsupported-version and unfinished-combat files.

**Acceptance Scenarios**:
1. Given unknown records in an otherwise supported file, when inspected, then their count and resulting capability limits remain visible.
2. Given unfinished combat, when imported, then it does not become a completed fight; the outcome explains its disposition.
3. Given a structurally invalid or unsupported source, when inspection or import fails, then prior history and the source remain unchanged and no raw payload appears in the error.
4. Given a replaced or changed source, when a stale inspection finishes, then it cannot authorize or overwrite the current selection.

### User Story 4 - Reimport without corrupting history (Priority: P2)

A player imports the same native recording again, or a copy at another path, without duplicating or relabeling prior encounters.

**Why this priority**: File selection should not change the identity of evidence already imported.
**Independent Test**: Reimport, copy/reimport, delete/reimport, and collision cases with existing native and addon history.

**Acceptance Scenarios**:
1. Given identical native evidence, when reimported from the same or a different path, then counts distinguish existing from new fights and history identities stay stable.
2. Given a collision or late invalid fight, when import is rejected, then no earlier fight or store migration remains committed.
3. Given multiple recording runs in a file, when imported, then their chronology and native provenance remain distinct.

### Edge Cases

- Empty files; unsupported headers, record layouts or versions; wrong source selection.
- Quoted separators, nested field groups, escaped text, invalid encoding, excessive lines or field depth.
- Missing, repeated or reversed combat boundaries; clock reversal; repeated recording runs.
- Unknown record types; malformed known records; records outside combat; unterminated final lines.
- Duplicate files at different paths; content collisions; legacy stores; no completed fights.
- Concurrent replacement, truncation or append while reading; stale worker generations.
- Live and PTS selections whose provenance comes from an explicit user choice rather than invented file evidence.
- Source/history access failures and cancellation leave source and existing history intact.

## Requirements

### Functional Requirements

- **FR-001**: Users MUST explicitly select native completed-file import and a local file, separately from saved-addon input and desktop application logs.
- **FR-002**: Inspection MUST report supported format/version, selected environment provenance, completed-fight count, record/error/unknown/incomplete counts and actionable readiness without raw values.
- **FR-003**: Import MUST group supported completed combats in authoritative file order and keep incomplete or ambiguous boundaries out of completed results.
- **FR-004**: Native records MUST retain relevant original ordered fields exactly and carry native source identity; no addon callback, unavailable timestamp, player identity or replay authority may be manufactured.
- **FR-005**: Metrics MUST expose only measurements supported by actual native observations, with explicit unavailable or reduced-quality reasons for missing evidence.
- **FR-006**: Reimport MUST be deterministic across file paths and preserve existing native and addon history, with explicit new/already-present outcomes.
- **FR-007**: All accepted fights and any necessary history migration MUST commit atomically; rejected imports MUST preserve existing store bytes and schema.
- **FR-008**: Reading/parsing MUST enforce declared native-specific byte, record, field and nesting limits, reject inconsistent source snapshots, and run outside desktop interaction and input threads.
- **FR-009**: Structurally malformed known records MUST reject the import atomically; unknown records MUST remain counted and retained with explicit capability limits. Unterminated final lines MUST NOT become complete records, and unclosed fights MUST NOT become complete encounters. All outcomes MUST use bounded diagnostics without raw values.
- **FR-010**: Source files MUST remain read-only; raw values MUST stay local and absent from logs, receipts, public fixtures and default summaries. Published representative fixtures MUST be sanitized.
- **FR-011**: Existing saved-addon recording, retention, source inspection, imports and history MUST remain supported and isolated from native input.
- **FR-012**: Documentation and policy fixtures MUST distinguish supported completed-file formats from unimplemented live tailing and historical platform qualification; repository evidence MUST NOT be presented as installed/platform verification.
- **FR-013**: S127 MUST include the complete specification sequence, chronological Plan052 archive/Plan053, regression evidence, changelog and tracker mapping, automatic official PR, resolved reviews within two rounds and green final-head CI before owner handoff.

### Key Entities

- **Native source**: An explicitly selected completed recording, observed format/version, chosen environment, immutable source identity and read outcome.
- **Native recording run**: An ordered recording interval whose metadata and combat boundaries belong together.
- **Native encounter**: A completed bounded combat interval with stable identity and original native records.
- **Import outcome**: Counts of new/existing fights and explicit unsupported, incomplete or malformed dispositions.
- **Metric capability**: A measurement's available observations, coverage and reason for unavailable results.

## Success Criteria

### Measurable Outcomes

- **SC-001**: A sanitized representative file with at least two completed combats imports both, in order, through the desktop native workflow.
- **SC-002**: Reimporting that file or an identical copy adds zero duplicates and preserves encounter identities.
- **SC-003**: Every malformed, unknown, truncated, missing-boundary and unsupported-format fixture has an explicit outcome; zero ambiguous fights are labeled complete.
- **SC-004**: Every native metric displayed is traceable to retained native observations; zero fabricated addon callbacks or replay results are produced.
- **SC-005**: Collision, invalid late data and stale-source cases preserve existing source/history evidence and import no partial transaction.
- **SC-006**: A representative-size recording of at least 200000 records can be processed within declared native limits while desktop work stays responsive.
- **SC-007**: All required repository gates and both permitted review stages conclude before owner handoff; installed/game/platform behavior remains unclaimed.

## Assumptions

- The owner-provided local recording and documented native record contracts establish the first supported format variants; unsupported variants get explicit diagnostics.
- Completed-file import is the useful first outcome. Incremental live tailing, logger activation and live flush/platform qualification remain future work.
- Environment is explicitly selected and its origin is documented if the file does not supply authoritative environment evidence.
- Existing history and worker infrastructure can support a source-aware native path without weakening addon validation.
- Repository fixtures and CI are acceptance evidence; no owner field-verification task is requested.
