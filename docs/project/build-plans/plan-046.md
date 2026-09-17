# Plan 046: Ultimate Auto Potion Watch

Status: Active

Sequence:

1. S110 implements issue #173 by adding Ultimate as an optional, independently
   persisted Auto Potion resource watch. It consumes the existing atomic
   Ultimate telemetry, applies exact inclusive current-over-maximum percentage
   comparison, preserves deterministic OR behavior and every existing action
   gate, and extends settings, diagnostics, tests, and documentation.

No addon, Pixel Bus protocol, potion-selection, quickslot-selection, cooldown,
retry, session-enablement, or synthesis behavior changes in this plan. Installed
release verification issues #110, #129, #131, and #190 retain their independent
Release verification lifecycle.
