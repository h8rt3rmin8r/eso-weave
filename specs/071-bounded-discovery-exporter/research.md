# Research: Bounded ESO Discovery Exporter

## Decision 1: Dedicated addon, not PixelBeacon

**Decision**: Ship `addon/EsoWeaveCollector` with its own manifest, Lua source,
SavedVariables root, managed marker, version, lifecycle, and slash commands.

**Rationale**: PixelBeacon is latency-sensitive and safety-authoritative. Bulk
enumeration, persistent state, and user data have different failure boundaries.

**Alternatives rejected**: Extending PixelBeacon couples catalog collection to
automation safety. A generic third-party collector does not provide the required
contract, limits, provenance, or ownership proof.

## Decision 2: Fixed SavedVariables grammar with JSON-line payloads

**Decision**: ESO serializes one fixed table root. Envelope fields and chunks
use scalar/table syntax, while each chunk payload is a string containing sorted
canonical JSON records separated by LF.

**Rationale**: The desktop parser can validate a very small Lua data grammar and
delegate record structure to strict `serde_json` without evaluating code.

**Alternatives rejected**: General Lua parsing creates unnecessary executable
surface. An ad hoc delimiter protocol makes escaping and evolution fragile.

## Decision 3: Adler-32 chunks plus desktop SHA-256

**Decision**: The addon calculates Adler-32 for each chunk. Its embedded
`collector_checksum` is SHA-256 over the Lua source after replacing that one
checksum literal with 64 zeroes, avoiding a circular self-hash. The importer
repeats that normalization and also computes SHA-256 for the complete raw
capture, every normalized record, and the staged bundle.

**Rationale**: ESO exposes no supported cryptographic API. Adler-32 is compact,
deterministic, and adequate for accidental truncation detection. SHA-256 remains
the durable desktop provenance boundary.

**Alternatives rejected**: Shipping a handwritten cryptographic implementation
inside the addon adds audit and frame-time risk without authentication value.

## Decision 4: Five bounded iterator adapters

**Decision**: Implement player skills, crafted abilities, item sets, champion
skills, and class/race/companion identity. Each record uses stable IDs, records
indexes only as attributes, and declares character/account/channel/locale scope.

**Rationale**: These families are approved as bounded API iterators by S068 and
are demonstrated in pinned stock UI source. The champion manager iterates
disciplines with `GetNumChampionDisciplines`, resolves stable discipline IDs,
and iterates skills by discipline. Similar pinned stock UI paths support skills
and item-set collections.

**Alternatives rejected**: Guessed ID ranges and universal claims violate the
coverage contract. Owned items and event observations are opportunistic and
belong in later observation work.

## Decision 5: Deterministic S070 staging, never active publication

**Decision**: `catalog-compiler import-collector` validates a capture and writes
an S070 `CatalogBundle` JSON file atomically. A separate existing `build` command
is required to publish SQLite.

**Rationale**: This makes the security boundary visible, preserves reviewable
intermediate data, and reuses S070 validation and compiler guarantees.

**Alternatives rejected**: Direct SQLite mutation bypasses provenance and
rollback gates. A second importer-specific database format duplicates S070.

## Decision 6: Explicit limits and strict parser

**Decision**: Check 64 MiB before parsing; enforce 64 KiB strings, depth 16,
one million syntax tokens, 600,000 table entries, 500,000 records, 64 KiB chunks,
and 1,024 chunks. Reject duplicate keys, non-integer numeric values, unknown
root assignments, expressions, functions, references, long strings, comments,
and trailing syntax.

**Rationale**: Limits must apply before or during allocation. Values leave room
for the provisional S068 record ceiling while keeping the envelope bounded.

## Decision 7: Independent marker-gated lifecycle

**Decision**: Collector install/update/remove accepts an existing AddOns root,
uses `symlink_metadata`, refuses links and non-regular files, writes Lua before
the manifest commit marker, and removes only a marker-proven collector folder.

**Rationale**: This mirrors the proven safety properties of the beacon without
sharing identity or mutating its files. Pure path helpers derive the capture at
`../SavedVariables/EsoWeaveCollector.lua`, so live, liveeu, and pts roots work
without hard-coded global paths.

## Decision 8: Field verification remains separate

**Decision**: Invented multi-class live and PTS fixtures prove contracts; issue
#129 remains responsible for real-client visibility, size, timing, and flush
evidence.

**Rationale**: Installed-game evidence cannot be produced in repository CI and
was explicitly separated by S068.
