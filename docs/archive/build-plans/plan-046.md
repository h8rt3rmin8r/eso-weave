# Plan 046: Ultimate Auto Potion Watch

Status: Complete, Archived

Sequence:

1. S110 implemented issue #173 by adding Ultimate as an optional, independently
   persisted Auto Potion resource watch. It consumes the existing atomic
   Ultimate telemetry, applies exact inclusive current-over-maximum percentage
   comparison, preserves deterministic OR behavior and every existing action
   gate, and extends settings, diagnostics, tests, and documentation.

No addon, Pixel Bus protocol, potion-selection, quickslot-selection, cooldown,
retry, session-enablement, or synthesis behavior changed. The sequence completed
when [PR #219](https://github.com/h8rt3rmin8r/eso-weave/pull/219) merged and
closed issue #173.
