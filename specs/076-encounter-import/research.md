# Research: Encounter SavedVariables Import

## Authorities Reviewed

- Issue #133 and parent issue #113
- `docs/project/encounter-model.json`
- `specs/069-encounter-model/`
- `specs/075-encounter-capture/`
- `addon/EsoWeaveEncounter/EsoWeaveEncounter.lua`
- Existing collector parser, bounded-read, atomic-file, catalog, and CLI modules
- SQLite transaction and backup support already available through rusqlite

## Decision 1: Parse a fixed data grammar without Lua

**Decision**: Extract the existing collector SavedVariables parser into a
crate-private shared parser with caller-supplied root and limits. It accepts one
assignment, bracketed string or integer keys, tables, strings, integers,
booleans, and nil. It rejects all remaining Lua syntax.

**Rationale**: The existing parser already implements the correct non-execution
boundary, duplicate-key detection, array continuity, string escapes, nesting,
token, and entry limits. Copying it would create two security implementations.

**Alternatives considered**:

- Execute the file in a sandboxed Lua runtime: rejected because no runtime is a
  sufficiently safe authority for hostile files and the contract forbids it.
- Add a general Lua parser dependency: rejected because the accepted language is
  intentionally smaller and dependency expansion is unnecessary.
- Copy collector parsing code: rejected because duplicated bounds would drift.

## Decision 2: Validate typed facts and exact payload shapes

**Decision**: Deserialize into schema-v1 typed structures, deny unknown envelope
fields, and validate exact allowed keys, types, numeric domains, and stable string
tokens for each event kind.

**Rationale**: The S075 JSON schema establishes the envelope but intentionally
models payloads as a scalar map. At a hostile boundary, a generic string field
could carry a personal name. The production emitter is the authoritative payload
shape and supplies a finite allowlist.

**Alternatives considered**:

- Validate only the JSON schema: rejected because cross-field invariants and
  privacy-sensitive payload tokens are not expressible there.
- Drop unknown payload fields: rejected because silent rewriting hides invalid
  producer behavior.

## Decision 3: Canonical JSON owns content identity

**Decision**: Serialize validated typed structures as compact UTF-8 JSON in
declared struct field order, with payload and warning maps ordered
lexicographically. Hash those bytes with SHA-256. Separately hash the exact stable
source bytes.

**Rationale**: Canonical facts must be independent of whitespace and Lua table
ordering. A separate source hash preserves audit value without treating syntax as
a different encounter.

**Alternatives considered**:

- Hash the SavedVariables bytes only: rejected because harmless serializer
  differences would defeat idempotency.
- Implement RFC 8785: rejected because the accepted values are integers,
  booleans, bounded strings, arrays, and ordered structures; a narrower versioned
  format is easier to verify.

## Decision 4: Use a dedicated immutable SQLite store

**Decision**: Create encounter store schema version 1 in a caller-selected SQLite
file. Store one canonical BLOB per encounter with indexed metadata. A trigger
rejects updates, while explicit deletes remain supported.

**Rationale**: Transactions give atomic publication and deletion. Unique indexes
make idempotency and identity collision behavior enforceable. The dedicated file
preserves the raw, derived, catalog, and configuration storage boundaries.

**Alternatives considered**:

- Add tables to `catalog.sqlite`: rejected because the catalog is shipped,
  replaceable reference data rather than user-owned observations.
- Store captures in configuration: rejected by the constitution.
- Directory plus manifest: rejected because multi-file publication creates
  orphan and reconciliation states without improving this slice.

## Decision 5: Preserve corrupt and unsupported stores

**Decision**: Run SQLite integrity and schema checks before mutation. Initialize
only a truly empty schema-version-zero database. Reject unrelated, corrupt, or
future-version databases in place.

**Rationale**: Automatically recreating a bad database can destroy the only copy
of user-owned observations. Version 1 has no legacy migration, but the opening
rules establish safe future ownership.

## Decision 6: Snapshot backup and explicit deletion

**Decision**: Use SQLite's online backup API to create a consistent temporary
snapshot, close it, hash the resulting bytes, then atomically publish it. Provide
transactional deletion by semantic identity and explicit deletion of all raw
records. Never prune automatically.

**Rationale**: This is the smallest lifecycle surface that makes user ownership
real before the desktop history UI. A backup hash is useful only after the final
snapshot bytes are closed and stable.

**Alternatives considered**:

- Copy an open database file: rejected because journaling can make a raw copy
  inconsistent.
- Export custom JSON: rejected because it would require a second restore contract
  and loses a byte-complete store snapshot.

## Decision 7: Reuse the maintainer CLI

**Decision**: Add `encounter-import`, `encounter-list`, `encounter-backup`, and
`encounter-delete` commands to `catalog-compiler`, all backed by public library
operations.

**Rationale**: The binary already owns explicit non-interactive repository data
maintenance. A testable CLI is valuable before #135, while a second shipped
binary would expand packaging and release work.

## Bounds

| Boundary | Limit | Authority |
| --- | ---: | --- |
| Source file | 67,108,864 bytes | S069 provisional import ceiling |
| Stored events | 100,000 | S075 schema v1 |
| Estimated capture bytes | 33,554,432 | S075 schema v1 |
| Parser string | 65,536 bytes | S069 provisional import ceiling |
| Parse nesting | 16 levels | S076 defensive design |
| Parse tokens | 3,000,000 | S076 production-envelope allowance |
| Table entries | 2,000,000 | S076 production-envelope allowance |
| Encounter actors | 0 through 4,096 | S075 emitter bound |

The parser token and entry ceilings permit a full S075 capture while the source
byte ceiling remains the independent memory and work bound.

## Resolved Clarifications

1. A truthful partial capture is accepted; malformed or truncated input is not.
2. The caller must supply an expected channel and the importer never promotes it.
3. Canonical normalized facts, not Lua syntax, are the immutable raw record.
4. Exact canonical duplicate import is idempotent. Same identity with changed
   facts is an error.
5. Unknown numeric identifiers are retained and do not require catalog lookup.
6. Backup and deletion are explicit; there is no automatic retention policy.
7. Store paths are caller-selected and are never saved as application settings.
8. Compression, discovery, derived metrics, UI, and cloud behavior remain out of
   scope.
