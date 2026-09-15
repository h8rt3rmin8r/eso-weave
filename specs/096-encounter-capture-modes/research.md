# Research: Encounter Capture Modes

## Control authority

**Decision**: Keep the desktop command vocabulary at zero. Addon-owned mode and
channel selection plus one `/ewencounter toggle` transition are authoritative.
The desktop presents only last-saved spool facts.

**Rationale**: S092 found no documented inbound ESO API with acceptable delivery,
acknowledgement, persistence, and account-visibility properties. Generated
bindings, typed slash commands, native-action piggybacking, and live
SavedVariables writes remain unsafe or unreliable.

**Alternatives considered**:

- Desktop button with generated input: rejected by the S092 no-go.
- Desktop requested-mode configuration: rejected because it cannot reach the
  addon and would create a second drifting authority.
- New custom addon binding: rejected because its server-associated visibility is unknown.

## Session representation

**Decision**: Add a distinct outer `state_schema_version = 1`, addon version 4
controller around terminal schema-v2/addon-format-v3 encounter records. Use one
session identity, fixed positive ordinals, append-only interruption markers, and
at most one nonterminal current record.

**Rationale**: S095 replay, actor allocation, loss ranges, and event sequences
are encounter-local. Keeping each terminal member self-contained preserves
replay and canonical bytes while the wrapper supplies continuous-session facts.
A distinct state-version key makes legacy singleton dispatch unambiguous.

**Alternatives considered**:

- Multiple start/end pairs in one capture: rejected because it breaks replay,
  loss, actor, and metric boundaries.
- Bump every nested capture version: rejected because normal callback-started
  records need no new canonical format.
- Overwrite only the latest encounter: rejected as silent data loss.

## Mid-combat activation

**Decision**: Query `IsUnitInCombat("player")` during explicit enablement. If
true, begin immediately from an exact API authority record, mark the capture
partial with controlled reason `started-mid-combat`, and stop at the next real
combat-exit callback. Do not invent a missed range or callback.

**Rationale**: The user outcome requires immediate activation, but events before
enablement are unknowable. A truthful partial capture can retain subsequent exact
facts without claiming a complete fight. S095 already classifies partial replay
as indeterminate.

**Alternatives considered**:

- Wait for a clean boundary: rejected by #183.
- Synthesize a combat-entry callback: rejected because raw authority must be exact.
- Estimate missed observations: rejected because prefix cardinality is unknowable.

## Bounds and pressure

**Decision**: Treat the existing 32 MiB, 100,000 normalized-event, and 100,000
raw-observation limits as aggregate session-spool limits. Add 1,024 encounter and
1,024 interruption caps plus separate current-terminal and outer-failure reserves.
Exhaustion hard-fails without eviction.

**Rationale**: Multiplying the old per-capture budget by continuous encounters
could violate the 128 MiB shared-file boundary and exhaust the game client. A
fixed aggregate preserves known memory expectations and makes storage pressure explicit.

**Alternatives considered**:

- Per-encounter 32 MiB: rejected as unbounded aggregate growth.
- Ring buffer: rejected because old evidence would disappear silently.
- Desktop acknowledgement and pruning: rejected because no safe live command path exists.

## Recovery policy

**Decision**: Persist explicit continuous authority across addon reload and
relog. Finalize a durably saved active record as partial, append one bounded gap
marker, and resume waiting under the same session. Interrupted active single
capture stops. Failed sessions never auto-retry. Desktop exit does nothing.

**Rationale**: Continuous means until user disablement, but only flushed evidence
can be recovered. Markers preserve the distinction between an observed gap and
an unknowable pre-flush crash.

**Alternatives considered**:

- Stop continuous on every reload: rejected because routine reloads would defeat
  the approved durable mode.
- Resume without a marker: rejected as a false uninterrupted claim.
- Auto-retry hard failures: rejected because it can repeat loss or pressure.

## Import and store

**Decision**: Parse legacy singleton captures and the new controller through one
version dispatch. Validate every terminal member and the entire session graph,
then append encounters and an immutable small session snapshot in one SQLite
transaction. Advance store schema to v4 while preserving canonical format v2.

**Rationale**: A growing spool is retried from an unchanged SavedVariables file.
Atomic preflight, identity/hash equality, ordinal uniqueness, and monotonic
snapshot prefixes make retries idempotent without rewriting prior facts.

**Alternatives considered**:

- Loop existing single imports: rejected because later failure can leave a partial batch.
- Infer order from timestamp or ID: rejected because timestamps can repeat or regress.
- Mutable session row: rejected because later input could rewrite historical interruption state.

## Identifier and diagnostic safety

**Decision**: Cap displayed opaque identifiers at 128 ASCII bytes, validate all
closed enums and checked counts before projection, and keep every rejection
message value-free.

**Rationale**: The current parser is byte-bounded but accepts much longer opaque
IDs than the UI needs. Continuous session metadata increases the number of
hostile fields that could otherwise reach diagnostics or display.

## Documentation surface

**Decision**: Update the canonical mdBook, changelog, machine contract, and build
plan. Do not create a blog subsystem.

**Rationale**: The repository has no blog publication system. Adding one solely
to satisfy a generic workflow template would be disproportionate and outside
issue #183; canonical manuals are the established publication surface.
