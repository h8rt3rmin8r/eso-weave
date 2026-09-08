# Spec-kit Analysis: Settings Runtime Parity

## Pre-implementation Gate

**Result**: PASS

**Date**: 2026-09-07

## Authority and Scope

- Issue #95 is actionable and independently closeable.
- Its shared boundary is settings-runtime parity across persisted Fishing and Pixel Bus configuration, the live owners, Settings UI, tests, and canonical documentation.
- Block-size hot switching, embedded documentation expansion, release work, and field verification remain outside S062.

## Constitution Alignment

- The full spec-kit artifact chain exists before application implementation.
- Changed Fishing configuration has an explicit zero-input cancellation contract.
- Reader updates stay on the existing worker and outside the input-hook path.
- Tolerance changes close stale safety evidence, and running block geometry remains coherent with PixelBeacon.
- No dependency, settings schema, thread, add-on protocol, process-memory, or packet expansion is proposed.

## Consistency Review

- All 19 functional requirements map to tasks and verification evidence.
- The hybrid policy gives scalar settings a live owner without pretending cross-process geometry can switch atomically.
- Complete-value messages and latest-value draining resolve rapid auto-apply edits without GUI blocking.
- Equality no-ops prevent unrelated settings edits from disturbing Fishing or reader state.
- Save confirmation remains a persistence claim; field help carries the runtime timing contract.

## Coverage Review

| Risk | Requirement | Planned evidence |
| --- | --- | --- |
| Mixed Fishing configuration generations | FR-001, FR-004 through FR-006 | controller phase, deadline, no-input, and re-enable tests |
| Missing Fishing key editor | FR-002, FR-003 | headless UI, string, and persistence tests |
| Stale reader configuration | FR-007 through FR-009 | wake, coalescing, reader, and cadence tests |
| Stale safety authorization | FR-010 | tolerance invalidation and routed gate tests |
| Geometry or heartbeat drift | FR-011 through FR-013 | merge and AppModel regression tests |
| Invalid or failed updates | FR-014, FR-015 | sanitizer, bounds, disconnect, and diagnostics tests |
| Misleading interface/docs | FR-016 through FR-018 | semantic copy and documentation policy tests |
| Lifecycle traceability | FR-019 | plan index and migration-ledger policy tests |

## Resolved Clarifications

1. All Fishing fields and scalar reader controls apply live; Block Size remains staged.
2. Changed requested Fishing work stops and must be explicitly re-enabled; unchanged settings are a no-op.
3. Reader updates are complete values delivered through a wakeable non-blocking channel and coalesced newest-wins.
4. Tolerance changes close cached safety evidence before fresh decoding.
5. The existing save toast remains a narrow persistence confirmation.

## Findings

No CRITICAL, HIGH, or unresolved ambiguity remains. Implementation may begin under TDD.

## Post-implementation Gate

**Result**: PASS

**Date**: 2026-09-07

## Implementation Evidence

- Fishing applies complete sanitized configuration through its existing controller lock. Changed requested or active work clears request, recovery, state, and deadlines with `SettingsChanged` and no sink access; equality remains a strict no-op.
- The Settings modal exposes an accessible Interact Key sourced from `Key::ALL`, constrains timing and interval controls to domain bounds, and distinguishes live scalar controls from staged Block Size.
- AppModel publishes complete `LiveReaderConfig` values without blocking and retains startup geometry when scalar edits are mixed with a Block Size change.
- The worker's update-aware deadline wait wakes for configuration, drains queued bursts newest-wins, and applies one shared boundary to decoding and poll cadence.
- Tolerance changes close menu, life, world, roll-dodge, travel, and Fishing evidence before same-iteration resampling; block size and heartbeat timeout cannot enter the live update type.
- Disconnected delivery leaves the runtime snapshot unchanged, logs the failure, and preserves sanitized configuration on disk for next-start recovery.
- Plan 031 is archived with PR #99 evidence, plan 032 is the sole active plan, and DEF-005 is covered by runtime, UI, test, and canonical documentation evidence.

## Validation Evidence

- `cargo fmt --all -- --check` passed.
- `cargo clippy --all-targets --all-features --locked -- -D warnings` passed.
- All 655 Rust tests passed under all targets and features with the lockfile enforced.
- Documentation policy passed all 53 fixture tests and the generated-site check; mdBook test, build, link checking, spelling, release-note, text-hygiene, whitespace, and encoding checks passed.
- Independent runtime, concurrency, safety, UI, test, documentation, and spec-kit reviews completed. Wake synchronization, first-launch copy, obsolete-claim policy, shared worker application, and disconnected persistence findings were resolved.
- First-round hosted review identified stale Fishing detector state at a tolerance boundary. The cache is now invalidated after the five safety-closing events, and a regression proves the next safe sample republishes `FishingStarted`.
- Second-round hosted review found that menu recovery followed Fishing recovery and that Fishing-only configuration changes did not invalidate reader history. Menu recovery now precedes Fishing edges, and a monotonic controller configuration generation makes the worker discard stale B1 history before sampling. Reader, controller, and complete routing regressions cover both corrections.

## Documentation Outcome

No standalone blog article was added. S062 corrects an existing Settings defect rather than introducing a significant feature announcement, and the repository intentionally keeps the archived website lifecycle separate. Canonical user and developer pages carry the complete behavior contract.

## Findings

No CRITICAL, HIGH, or unresolved finding remains. S062 is ready for delivery.
