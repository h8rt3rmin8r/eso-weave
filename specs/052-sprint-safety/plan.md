# Implementation Plan: Sprint Safety

## Summary

Complete the reserved keyboard-mode on-foot sprint axis in B9 with a bounded action-slot detector, then consume only explicit sprint state to defer auto-potion attempts until current conditions are eligible again.

## Technical Context

- Rust desktop application with a Lua ESO addon
- Pixel protocol v4 with 25 blocks and frozen legacy layouts
- Existing B9 movement marker and reserved sprint codes
- Existing 100 ms addon active-world tick and reader change detection
- Pure auto-potion rule plus controller-owned lifecycle gates
- Deterministic Rust and Lua contract tests, followed by live-game validation

## Constitution Check

- Specification precedes implementation: PASS
- Clarification records the bounded platform scope: PASS
- Test-first implementation is explicit: PASS
- Protected functions, process memory, and network traffic remain out of scope: PASS
- No input-hook blocking work or new polling thread: PASS
- Protocol geometry and legacy layouts remain frozen: PASS
- Fail-safe stale evidence and no replay are explicit: PASS
- Full format, lint, locked test, encoding, and corruption gates precede commits: PASS
- Live validation boundary is documented: PASS

## Implementation Phases

1. Add failing B9 sprint decoder, presentation, and compatibility tests.
2. Add failing addon contract tests for evidence, exclusions, debounce, hysteresis, watchdog, events, and lifecycle reset.
3. Implement the bounded addon detector and B9 publisher without a protocol bump.
4. Add failing auto-potion rule, controller, routing, diagnostics, recovery, and precedence tests.
5. Implement sprint consumption and truthful presentation.
6. Update protocol, validation, issue, plan, and changelog documentation.
7. Run complete quality gates, push, open a closing PR, and complete no more than two Codex review rounds.

## Risk Controls

- Do not equate speed or stamina with sprint.
- Do not publish explicit sprint in gamepad mode or while mounted.
- Bound positive inference with debounce and a watchdog.
- Keep `0xE0` reserved and rejected.
- Block only explicit `Sprinting`, so unsupported movement cannot deadlock auto-potion.
- Re-evaluate current eligibility after sprint instead of replaying a stored attempt.
