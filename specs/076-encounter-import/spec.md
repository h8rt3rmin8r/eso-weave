# Feature Specification: Encounter SavedVariables Import

**Feature Branch**: `codex/s076-encounter-import`  
**Created**: 2026-09-10  
**Status**: Draft  
**Input**: User description: "Import encounter SavedVariables into a user-owned local store"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Import a terminal encounter capture safely (Priority: P1)

As an ESO Weave user, I can explicitly select a terminal encounter SavedVariables
file and import it into a local encounter store without the application executing
the input or silently accepting malformed data.

**Why this priority**: Every later encounter summary and metric depends on a safe,
faithful raw-data boundary.

**Independent Test**: Import a valid complete capture and a valid truthful partial
capture, then confirm that each produces one durable receipt and one immutable raw
record. Attempt equivalent malformed, oversized, truncated, executable, and
schema-incompatible inputs and confirm that none changes the store.

**Acceptance Scenarios**:

1. **Given** a valid S075 terminal capture, **When** the user explicitly imports it
   with the matching source channel, **Then** the importer stores the canonical
   capture and returns a receipt containing its hashes, channel, status, and counts.
2. **Given** a valid capture whose status is `partial` and whose loss accounting is
   internally consistent, **When** it is imported, **Then** the importer preserves
   that truthful partial state rather than treating it as corrupt or complete.
3. **Given** malformed syntax, executable Lua, an unexpected root, a mismatched
   channel, an unsupported schema, an over-limit value, or inconsistent terminal
   invariants, **When** import is attempted, **Then** import fails without changing
   any previously valid store content.
4. **Given** an input file that changes while it is being read, is a symbolic link
   or reparse point, or is outside the importer's resolved read boundary, **When**
   import is attempted, **Then** the importer rejects it without publishing data.

---

### User Story 2 - Preserve raw capture identity and provenance (Priority: P1)

As a user, I can rely on every accepted encounter remaining identifiable,
immutable, local, and attributable to its live or PTS source channel.

**Why this priority**: Silent mutation, cross-channel promotion, or loss of unknown
identifiers would make future derived metrics untrustworthy.

**Independent Test**: Import a capture containing unknown numeric identifiers,
inspect the stored canonical bytes, reimport the same capture, and attempt a
different capture with the same encounter identity. Confirm preservation,
idempotency, and collision rejection.

**Acceptance Scenarios**:

1. **Given** a valid capture containing unknown ability, effect, result, unit, or
   action identifiers, **When** it is imported, **Then** those values are retained
   unchanged and are not required to resolve through the catalog.
2. **Given** a previously imported canonical capture, **When** the exact capture is
   imported again, **Then** the operation is idempotent and does not create a
   duplicate raw record.
3. **Given** a different canonical capture with the same session and encounter
   identity, **When** import is attempted, **Then** the importer reports an identity
   collision and preserves the existing record.
4. **Given** a successful import, **When** the store is inspected, **Then** its raw
   record cannot be updated through the supported API and remains separate from
   both user configuration and `catalog.sqlite`.

---

### User Story 3 - Own the local data lifecycle explicitly (Priority: P2)

As a user, I can list, back up, and explicitly delete my imported encounter data,
and I receive predictable behavior when the store is corrupt or has an unsupported
schema version.

**Why this priority**: User ownership requires practical control over retention and
recovery, not merely local storage.

**Independent Test**: List imported receipts, create and verify a backup, delete one
encounter and then all encounters through explicit calls, and exercise corrupt and
future-version stores without losing a known-valid store or backup.

**Acceptance Scenarios**:

1. **Given** a valid store, **When** a backup is requested, **Then** a self-contained
   SQLite snapshot is atomically published at the user-selected path and a SHA-256
   receipt identifies the resulting backup bytes and schema version.
2. **Given** an existing backup target, **When** a backup is requested, **Then** the
   target is replaced atomically only after the new snapshot and receipt hash are
   complete.
3. **Given** one or more imported encounters, **When** the user explicitly requests
   deletion of one identity or all data, **Then** only the requested raw records are
   removed in one transaction and the result reports the number removed.
4. **Given** a corrupt or unsupported future-version store, **When** any mutating
   operation is attempted, **Then** the operation fails without replacing,
   recreating, or deleting that store.

### Edge Cases

- A valid file is exactly at the byte, event, string, nesting, token, or table-entry
  limit.
- A capture has the minimum two events, repeated monotonic timestamps, or the
  maximum allowed event count.
- A discontinuity is the first event after a dropped interval, multiple declared
  discontinuities occur, or a claimed interval overlaps another interval.
- Counts, first or last sequence values, terminal events, partial reasons, warning
  counts, event identities, or finished timestamps disagree with the event stream.
- A payload uses an unknown field, a disallowed type, a negative identifier or
  count, or a string that could carry a player or account name.
- The source file is deleted, truncated, replaced, or modified during a stable read.
- The store path aliases the input path or backup path.
- An exact duplicate has a different source-file serialization but the same
  canonical capture facts.
- A database exists with schema version zero but already contains unrelated tables.
- Disk exhaustion or process interruption occurs while importing or publishing a
  backup.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST import encounter data only after an explicit caller
  action and MUST NOT scan, watch, upload, or transmit encounter data.
- **FR-002**: The importer MUST accept only the fixed S075 SavedVariables assignment
  rooted at `EsoWeaveEncounterSaved` and MUST parse it as data without executing Lua
  or supporting functions, expressions, metatables, references, or arbitrary code.
- **FR-003**: The importer MUST perform a stable, no-follow bounded read and reject
  source replacement, truncation, symbolic links, Windows reparse points, and files
  outside the resolved source-parent boundary.
- **FR-004**: Before publishing an encounter, the importer MUST validate the capture
  schema, byte limit, event limit, string limits, nesting depth, token count, table
  entry count, integer ranges, enumerations, and kind-specific payload fields.
- **FR-005**: The current import limits MUST be 64 MiB per source file, 100,000
  stored events per encounter, 64 KiB per parser string, 16 levels of nesting,
  3,000,000 tokens, and 2,000,000 table entries. Narrower contract fields MUST retain
  their S075 schema limits.
- **FR-006**: The importer MUST require a caller-supplied expected source channel and
  reject a capture whose `live` or `pts` channel does not match it.
- **FR-007**: The importer MUST validate the S075 terminal-capture invariants,
  including matching event identity, strict sequence progression, nondecreasing
  monotonic time, start and end boundary events, count reconciliation, declared loss
  intervals, truthful complete or partial status, and matching terminal metadata.
- **FR-008**: A discontinuity MUST be the first stored event after its declared
  missing interval; intervals MUST be ordered and non-overlapping; and their total
  missing sequences MUST equal `omitted_event_count`.
- **FR-009**: The importer MUST accept a truthful partial capture and preserve its
  status, reason, warnings, and loss accounting. It MUST reject a capture that merely
  appears partial because it is truncated, corrupt, or internally inconsistent.
- **FR-010**: The importer MUST retain unknown numeric identifiers and all accepted
  source-channel provenance without requiring a matching catalog record.
- **FR-011**: The importer MUST canonicalize accepted facts as deterministic compact
  UTF-8 JSON using the typed field order and lexicographically ordered payload and
  warning keys defined by encounter canonical format version 1.
- **FR-012**: The importer MUST compute a SHA-256 hash of the bounded source bytes and
  a separate SHA-256 hash of the canonical capture bytes.
- **FR-013**: The system MUST store canonical raw captures in a dedicated,
  user-selected SQLite store that is separate from configuration, derived metrics,
  and `catalog.sqlite`.
- **FR-014**: The store MUST preserve each canonical capture as immutable raw bytes,
  indexed by its content hash and by its session and encounter identity. Supported
  operations MUST NOT update raw records.
- **FR-015**: Reimporting the same canonical content MUST be idempotent. Different
  canonical content with the same session and encounter identity MUST be rejected as
  an identity collision.
- **FR-016**: Validation and publication MUST occur as a single logical import: a
  failure before commit MUST leave all previously valid store content unchanged.
- **FR-017**: A new store MUST be initialized transactionally with schema version 1.
  A non-empty unrecognized database or unsupported schema version MUST be rejected
  rather than adopted, downgraded, or recreated.
- **FR-018**: Store integrity and supported schema MUST be checked before mutation.
  A corrupt store MUST remain in place for user-directed recovery.
- **FR-019**: The supported API MUST list import receipts without requiring payload
  execution or catalog resolution, and ordering MUST be deterministic.
- **FR-020**: The supported API MUST provide explicit deletion of one encounter and
  explicit deletion of all encounters. It MUST NOT automatically expire, prune, or
  delete raw data.
- **FR-021**: The supported API MUST create a self-contained SQLite backup through a
  consistent database snapshot, publish it atomically to a user-selected path, and
  return the schema version and SHA-256 hash of the final backup bytes.
- **FR-022**: Import, deletion, backup, and any future migration MUST disable
  interactive prompts and MUST surface actionable typed errors without exposing raw
  event payloads, player names, account names, or local path contents in receipts.
- **FR-023**: No accepted schema field may store player names, account names, chat
  text, free-form notes, or telemetry identifiers. String-valued payload fields MUST
  be restricted to contract-defined bounded enumerations or stable engine tokens.
- **FR-024**: The implementation MUST expose the lifecycle through reusable library
  operations and an explicit non-interactive maintainer CLI surface suitable for
  verification before the later desktop history UI work.
- **FR-025**: The implementation MUST NOT add encounter summaries, metrics,
  recommendations, automatic SavedVariables discovery, history UI, gzip import, or
  catalog enrichment in this slice.

### Key Entities

- **Encounter Import Request**: Explicit source path, user-selected store path, and
  expected `live` or `pts` source channel.
- **Canonical Encounter Capture**: Fully validated S075 terminal envelope serialized
  as encounter canonical JSON version 1.
- **Raw Encounter Record**: Immutable canonical bytes plus source and content hashes,
  schema provenance, encounter identity, channel, terminal status, and event counts.
- **Import Receipt**: Non-sensitive result that identifies whether an import was new
  or idempotent and reports hashes, channel, status, and counts.
- **Encounter Store**: User-owned schema-versioned SQLite database containing only
  raw imported encounter captures and lifecycle metadata.
- **Backup Receipt**: Backup path-independent schema version, byte length, and
  SHA-256 digest returned after atomic snapshot publication.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Every valid complete and truthful partial S075 conformance fixture
  imports successfully and round-trips to byte-identical canonical JSON.
- **SC-002**: All malformed, executable, over-limit, inconsistent, path-race,
  channel-mismatched, corrupt-store, and unsupported-schema fixtures fail while a
  before-and-after store digest or record inventory remains unchanged.
- **SC-003**: Reimporting an accepted capture 100 times leaves exactly one raw record
  and returns an idempotent result after the first import.
- **SC-004**: Unknown numeric identifiers and `live` or `pts` provenance remain
  present and unchanged after import, backup, store reopen, and canonical readback.
- **SC-005**: A backup created from a valid store passes SQLite integrity checking,
  has the reported SHA-256 digest, and contains the same immutable raw-record
  inventory as the source snapshot.
- **SC-006**: Explicit single-record and all-record deletion remove exactly the
  requested number of records, while no test observes automatic deletion or
  mutation of a retained raw record.
- **SC-007**: Importing the 100,000-event production-budget fixture remains within
  the repository's documented desktop memory and elapsed-time budgets on the
  reference verification environment.
- **SC-008**: Repository formatting, lint, unit, integration, documentation,
  dependency, license, advisory, and full test gates pass with no regression to the
  existing collector import behavior.

## Assumptions

- S075 schema version 1 and its terminal capture contract are the sole accepted
  encounter input contract for this slice.
- The desktop caller or maintainer explicitly supplies both source and store paths;
  platform-specific automatic discovery belongs to later UI integration.
- A valid partial capture is usable raw evidence. Derived metrics decide later how
  partial data affects presentation.
- SQLite is an implementation detail of the dedicated raw store, not a promise that
  configuration, catalog, and encounter data will share a database.
- Store schema version 1 is the first encounter-store version, so S076 defines safe
  rejection and future migration ownership but performs no legacy migration.

## Dependencies

- S075 encounter capture contract and fixtures (`specs/075-encounter-capture/`).
- S069 encounter model and validation authority (`docs/project/encounter-model.json`).
- Existing stable bounded-read and atomic-file primitives.
- Existing bundled SQLite and SHA-256 dependencies.

## Out of Scope

- Encounter summary or metric computation (#134).
- Desktop encounter history, detail, deletion, and export UI (#135).
- Recommendations, scoring, or automated coaching (#137).
- Automatic game installation or SavedVariables path discovery.
- Upload, cloud synchronization, telemetry, or sharing.
- Compression support, catalog mutation, or enrichment of unknown identifiers.
