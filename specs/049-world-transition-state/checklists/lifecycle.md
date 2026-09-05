# Lifecycle Safety Checklist: World Transition State

- [x] Unknown is the startup and unavailable value.
- [x] Deactivation is the sole authority for Transitioning.
- [x] Activation is the sole authority for Active.
- [x] Active follows, never precedes, the complete dependent baseline.
- [x] Periodic convergence cannot reopen Active.
- [x] Invalid and lost signal clear the companion state.
- [x] Game process exit clears the shared observation immediately.
- [x] Duplicate events remain idempotent.
- [x] Older addons degrade to Unknown.
- [x] No synthesis behavior changes in this slice.
