# Analysis Gate: Deterministic Encounter Replay

## Coverage

Every functional requirement maps to at least one user story, contract, and task.
Replay authority and exact comparison map to US1/T005-T013. Partial and legacy
truthfulness map to US2/T007/T014-T017. The decision surface maps to
US3/T018-T021. Security and publication map to T022-T027.

## Consistency

- Capture schema remains 2 while addon version advances to 3.
- Current complete captures require a profile and exact verification.
- Current partial captures are indeterminate and preserve degraded behavior.
- V1 and pre-profile-v2 captures are unavailable for replay but remain importable.
- No source family, capture mode, metric, recommendation, or authority expands.

## Ambiguity Audit

No NEEDS CLARIFICATION marker remains. Runtime enum provenance, legacy dispatch,
raw-loss semantics, API correlation, divergence rejection, diagnostic privacy,
performance bounds, and publication surfaces are explicit.

## Risk Audit

- Profile spoofing is constrained by strict shape and semantic validation, then
  checked against the compatibility projections produced under that profile.
- Compatibility events cannot influence replay derivation.
- Missing raw observations never receive a false exactness claim.
- Strict API batching avoids nearest-neighbor guesses.
- A strict lifecycle requires one opening combat transition, one adjacent
  stopping transition, one matching finish marker, and no post-finish input.
- Actor type guards mirror the producer's nil and non-numeric fallback behavior.
- Numeric unit IDs are constrained to the documented positive exact-integer
  domain and exact decimal keys before actor interning, avoiding Lua/Rust
  formatting drift.
- Replay is pure, linear, checked, and bounded.
- Public diagnostics expose only constant value-free outcome categories.

## Gate Result

PASS. The slice is coherent, proportional, testable, and ready for red tests.

## Test-First Evidence

The first focused command was:

`cargo test --locked --test encounter_addon representative_capture_retains_every_selected_source_and_links_projections`

It failed to compile because `assess_replay` and `ReplayAssessment` did not yet
exist. After implementation, focused production-Lua differential, tamper, nil,
legacy, loss, ceiling, store-migration, and docs-policy tests passed. The full
merge gate remains the publication checkpoint.

Independent code, security, and domain reviews then identified terminal-state
enforcement, producer-compatible actor guards, exact projection mutation
coverage, stale profile documentation, and a mispinned PTS source snapshot.
The implementation now rejects deactivation/continuation/duplicate-finish
sequences, differentially verifies nil and non-numeric actors, exercises every
compared projection component plus omission/insertion/order, documents the
actual profile, and pins API 101051 with its machine-readable duel signature
delta.

## Hosted Gate

Pull request #201 completed Windows and Ubuntu CI, documentation validation,
dependency review, and CodeQL successfully. The automatic Codex review and the
single authorized manual second round both completed without findings. No
review thread remains open, and no third review round was requested.
