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
4. S071 implemented the bounded in-game discovery exporter, separate addon,
   restricted non-executing import, atomic staging, and managed lifecycle
   isolation. It merged in PR #139 and closed issue #115.
5. S072 implemented the placeholder-first local icon cache. It merged in PR
   #140 and closed issue #116.
6. S073 implemented the reviewed catalog candidate pipeline. It merged in PR
   #141 and closed issue #117, composing S070, S071, and S072 behind immutable
   version, source, report, and no-clobber publication gates.
7. S074 implemented the user-initiated catalog update, collector handshake,
   immutable user-data selection, rollback, recovery, and redacted receipt
   workflow. It merged in PR #142 and closed issue #118.
8. S075 is in progress under issue #132. It implements the separate, explicitly
   armed, privacy-minimized encounter addon, bounded SavedVariables envelope,
   encounter-local actors, event order, elapsed time, and declared loss.
9. Later slices continue the encounter pipeline through issues #133 through
   #136 according to their native dependency order. Issue #131 verifies live
   Combat Metrics parity only after its encounter prerequisites are delivered.

Issue #132 is the next dependency-ordered implementation after the catalog
foundation and unlocks import issue #133. Installed v0.15.1 verification in
issue #110, catalog field verification in issue #129, and live Combat Metrics
verification in issue #131 remain independent verification work and do not
block the active implementation sequence.
