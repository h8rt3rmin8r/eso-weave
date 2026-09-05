# Implementation Plan: Travel Safety

## Summary

Extend the pixel protocol with a bounded travel state, route it through the application's existing safety architecture, and close every synthesized-input path unless both world and travel state are safe.

## Technical Context

- Rust desktop application with a Lua ESO addon
- Pixel protocol with frozen legacy layouts
- Existing fail-safe life, world, sprint, and roll-dodge gates
- Deterministic Rust and Lua contract tests plus manual live-game validation

## Constitution Check

- Specification precedes implementation: PASS
- Test-first implementation is explicit: PASS
- Legacy protocol lengths remain frozen: PASS
- Fail-safe behavior and no replay are explicit: PASS
- Full format, lint, and test gates precede commits: PASS
- Live validation boundary is documented: PASS

## Implementation Phases

1. Add failing protocol and addon contract tests for v4 and B24.
2. Implement the addon detector and lifecycle behavior.
3. Add failing safety tests for hook, weave, fishing, potion, ordering, and no replay.
4. Route travel state through the Rust application and UI.
5. Update documentation, changelog, compatibility evidence, and validation instructions.
6. Run complete quality gates, audit encoding, push, open the PR, and complete two review rounds at most.

## Risk Controls

- Preserve v1, v2, and v3 framing exactly.
- Default missing or malformed travel data to `Unknown`.
- Keep physical input outside the generated-input gate.
- Rebaseline cooldown on activation so an existing cooldown cannot create a false edge.
- Bound pending state with cancellation and watchdog recovery.
