# Plan 038: ESO Game-Data Foundation

Status: Active

Sequence:

1. S068 defined the source, coverage, provenance, live/PTS promotion,
   redistribution, and collector-input contract. It merged in PR #130 and
   closed issue #112.
2. S069 is next and defines the external encounter model and Combat Metrics
   parity boundary under issue #113.
3. Later slices implement the deterministic SQLite compiler, bounded collector,
   local-only icon path, reviewed discovery pipeline, and explicit user updater
   in the order established by epic #111.

S068 did not implement the catalog, collector, icon resolver, or updater. Game
experiments that require live or PTS characters remain separately tracked in
issue #129 and cannot support an exhaustive claim until their evidence is
recorded.
