# Plan 037: Death Recovery Safety

Status: Complete, Archived

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
6. Publish the completed implementation in v0.15.1 while keeping installed field
   verification independently tracked by issue #110.

This slice changed runtime safety and the managed PixelBeacon protocol semantics
without adding a payload block, setting, persisted field, or dependency. S067
merged in PR #128, closed issue #109, and shipped in v0.15.1. Issue #110 remains
in Release verification and does not reactivate or block this completed plan.
