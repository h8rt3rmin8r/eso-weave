# Plan 038: ESO Game-Data Foundation

Status: Active

Sequence:

1. S068 defined the source, coverage, provenance, live/PTS promotion,
   redistribution, and collector-input contract. It merged in PR #130 and
   closed issue #112.
2. S069 defined the external encounter model, deterministic synthetic
   projection, and Combat Metrics parity roadmap. It merged in PR #137 and
   closed issue #113.
3. S070 implemented the deterministic, versioned SQLite catalog compiler. It
   merged in PR #138 and closed issue #114.
4. S071 is in progress and implements the bounded in-game discovery exporter
   tracked by issue #115, including a separate addon, restricted non-executing
   import, atomic staging, and managed lifecycle isolation.
5. Later slices continue the catalog foundation through issues #116 through
   #118 and the encounter pipeline through issues #132 through #136 according
   to their native dependency order. Issue #131 verifies live Combat Metrics
   parity only after its encounter prerequisites are delivered.

Issue #132 is also eligible, but issue #115 remains next in the order established
by epic #111 and supplies reviewed local discovery records to the catalog built
in S070. Installed v0.15.1
verification in issue #110, catalog field verification in issue #129, and live
Combat Metrics verification in issue #131 remain independent verification work
and do not block the active implementation sequence.
