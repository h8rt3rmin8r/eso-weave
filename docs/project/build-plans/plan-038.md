# Plan 038: ESO Game-Data Foundation

Status: Active

Sequence:

1. S068 defined the source, coverage, provenance, live/PTS promotion,
   redistribution, and collector-input contract. It merged in PR #130 and
   closed issue #112.
2. S069 defined the external encounter model, deterministic synthetic
   projection, and Combat Metrics parity roadmap. It merged in PR #137 and
   closed issue #113.
3. S070 is in progress and implements the deterministic, versioned SQLite
   catalog compiler tracked by issue #114, including atomic publication,
   rollback evidence, typed read-only access, and supported package layouts.
4. Later slices continue the catalog foundation through issues #115 through
   #118 and the encounter pipeline through issues #132 through #136 according
   to their native dependency order. Issue #131 verifies live Combat Metrics
   parity only after its encounter prerequisites are delivered.

Issue #132 is also newly eligible, but issue #114 remains the next outcome in
the order established by epic #111 and supplies the stable catalog, version,
and join primitives used by downstream encounter work. Installed v0.15.1
verification in issue #110, catalog field verification in issue #129, and live
Combat Metrics verification in issue #131 remain independent verification work
and do not block the active implementation sequence.
