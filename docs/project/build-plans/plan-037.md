# Plan 037: Death Recovery Safety

Status: Active

Sequence:

1. Replace immediate post-death Alive publication with a deterministic addon-side
   death-episode arbiter for ghost, world-activation, and no-load recovery.
2. Extend B21 recovery diagnostics while preserving its position, marker,
   checksum, Alive, Dead, and legacy fail-closed behavior.
3. Route recovery captures so current action-driving observations reach
   controllers before Alive reopens the shared life gate.
4. Invalidate queued and running weave work, cancel Fishing deadlines, and start
   a complete new auto-potion retry episode across the death boundary.
5. Add private-safe recovery diagnostics, canonical documentation, changelog,
   focused safety tests, and full CI parity before closing issue #109.
6. Leave issue #110 in Release verification until an explicitly authorized
   v0.15.1 artifact passes the live scenario matrix.

This slice changes runtime safety and the managed PixelBeacon protocol semantics
without adding a payload block, setting, persisted field, dependency, or release.
