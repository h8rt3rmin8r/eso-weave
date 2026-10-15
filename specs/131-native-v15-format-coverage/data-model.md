# S131 Data Model

The new semantic record view is ephemeral and bounded by the existing physical
line/token/depth limits. Its ordered fields retain duplicate names at their
typed positions. Inline ability tails have a numeric reference, quoted name and
icon, and two booleans; effect tails have three enum values (or three quoted
empty defaults) and an optional scalar synergy ID. These facts remain evidenced
by the original line, never by fabricated standalone metadata.

NativeRecord, NativeHeader, NativeRun and NativeEncounter keep their schema 1/2
serialized shapes, physical offsets, record order and exact line strings.
Logical identities keep the existing header tuple and fight ordinal; evidence
fingerprints keep raw bytes and existing classification counts. Distinct raw
serializations of the same fight are divergent evidence, not silently deduped.

Checkpoint version 1 retains the current run, context, pending fight and byte
tail. Format is record-local, so no profile flag is stored. Restoration uses
the same decoder and bounds as ingestion. Schema 7 storage and atomic fight plus
progress transactions remain unchanged. Historical schema 1 and existing normal
schema 2 evidence are read without rewriting, migration or reclassification.

NativeProjection retains source-qualified observed quantities. Signed producer
quantities use signed arithmetic; negative observations qualify derived rates.
No names, script IDs or native version are promoted to addon callback or Catalog
API authority. Existing unavailable quantities stay unavailable.
