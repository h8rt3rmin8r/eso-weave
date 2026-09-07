# Checklist: Input Authorization

- [x] Every applicable gate has a shared atomic representation.
- [x] Safe-to-unsafe transitions advance a monotonic epoch.
- [x] Queued weave work captures and validates its epoch.
- [x] Running weave work validates gates and epoch at waits and emissions.
- [x] Cancellation starts no new generated Down operation.
- [x] Matching held-input releases remain possible after cancellation.
- [x] Physical pass-through and self-originated recursion behavior remain unchanged.
- [x] Input callbacks contain no wait, controller lock, or synthesis.
- [x] Fishing checks suspension at controller and final sink boundaries.
- [x] Suspending clears Fishing state and deadline but preserves the request.
- [x] Unsuspending emits nothing and stale work never replays.
- [x] Menu deferral and focus behavior remain explicitly tested.
- [x] Light, heavy, bash, initial cast, reel, recast, and timeout paths are covered.
