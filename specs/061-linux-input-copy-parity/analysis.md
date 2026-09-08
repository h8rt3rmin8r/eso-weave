# Spec-kit Analysis: Linux Input and Copy Parity

## Pre-implementation Gate

**Result**: PASS

**Date**: 2026-09-07

## Authority and Scope

- Issues #93 and #96 are actionable and independently closeable.
- Their shared boundary is behavioral parity: Linux device capabilities, initial safety state, diagnostics, UI strings, tests, and documentation must describe and enforce the same runtime contract.
- Issue #95, embedded documentation, release work, and field verification remain outside S061.

## Constitution Alignment

- The full spec-kit artifact chain exists before application implementation.
- Physical key pass-through, recursion prevention, and generated-input fail-closed behavior have explicit contracts and deterministic tests.
- Capability logic is extracted into pure functions so Linux CI does not require input devices.
- No dependency, settings schema, process-memory, packet, or addon-protocol expansion is proposed.

## Consistency Review

- All 18 functional requirements map to tasks and verification evidence.
- The key capability union resolves pass-through loss without broadening the application binding domain.
- Rebuilding an early app-only virtual device before grab resolves the startup ordering race.
- Key-only forwarding matches advertised capabilities; actual key emission errors cannot be swallowed.
- Unavailable menu evidence is closed at construction, transition, diagnostics, and documentation layers.
- Latency and logging strings describe the implementation's additive 300 ms cap and persisted global capture filter.

## Coverage Review

| Risk | Requirement | Planned evidence |
| --- | --- | --- |
| Missing E/F3 capabilities | FR-001, FR-002, FR-005 | exhaustive key mapping and capability tests |
| Ordinary key loss after grab | FR-003, FR-004, FR-006 | physical capability union tests |
| Silent pass-through failure | FR-007, FR-008 | forwarding seam error and metadata tests |
| Startup fail-open | FR-010 through FR-013 | controller defaults and first-sample routing tests |
| Misleading latency copy | FR-014, FR-017 | semantic string tests |
| Misleading logging copy | FR-015, FR-017 | semantic string and persistence linkage tests |
| Documentation drift | FR-016 | coverage manifest and documentation policy tests |
| Lifecycle traceability | FR-018 | plan index and migration-ledger policy tests |

## Resolved Clarifications

1. Physical key capability union happens before grab, including replacement of an early app-only virtual device.
2. Only KEY events are forwarded; non-key metadata is deliberately ignored.
3. All genuine forwarded-key emission failures terminate the backend with an explicit error.
4. Missing menu evidence starts and remains gated until explicit gameplay evidence arrives.
5. Issue #96's no-behavior-change premise is overridden only for the discovered startup fail-open defect.

## Findings

No CRITICAL, HIGH, or unresolved ambiguity remains. Implementation may begin under TDD.

## Post-implementation Gate

**Result**: PASS

**Date**: 2026-09-07

## Implementation Evidence

- Linux capability construction derives from `Key::ALL`, includes both mouse buttons, preserves physical-only key codes, and upgrades an early virtual device before grab.
- Exhaustive Linux unit tests cover all 13 application keys, the shipped E and F3 bindings, physical capability union, rebuild policy, metadata exclusion, and explicit forwarding failure.
- Input and Fishing now initialize fail-closed, game exit restores that state, and regression tests prove generated work remains blocked until fresh gameplay evidence arrives; a pending initial Fishing request then starts without another toggle.
- PixelBus tests prove the first valid gameplay observation is published after an unavailable startup state.
- Semantic string tests reject the obsolete latency and Live Log claims, while canonical documentation and coverage records now describe implemented behavior.
- Plan 030 is archived with PR #98 evidence, plan 031 is the sole active plan, and issue #95 remains the only deferred issue in policy fixtures.

## Validation Evidence

- `cargo fmt --all -- --check` passed.
- `cargo clippy --all-targets --all-features --locked -- -D warnings` passed.
- `cargo test --all-targets --all-features --locked --quiet` passed.
- Documentation policy passed all 52 tests, and mdBook test, build, spelling, text hygiene, and whitespace gates passed.
- Independent code, Linux, safety, test, documentation, and spec-kit reviews completed; all findings at 80% confidence or higher were resolved.
- Local Linux cross-target compilation reached the external `ring` build before stopping on the unavailable `x86_64-linux-gnu-gcc`; Ubuntu CI remains the authoritative Linux compiler and test gate.

## Findings

No CRITICAL, HIGH, or unresolved finding remains. S061 is ready for delivery.
