# Auto Potion Safety Checklist: Ultimate Resource Watch

**Purpose**: Protect action authorization and fail-closed behavior while adding Ultimate
**Created**: 2026-09-17

## Observation Authority

- [x] Existing `UltimateTelemetry` is the only Ultimate source
- [x] Current and maximum remain atomic
- [x] Presentation retention is excluded from action inputs
- [x] Signal loss and lifecycle invalidation remain authoritative

## Trigger Mathematics

- [x] Inclusive equality is explicit
- [x] Cross multiplication avoids floating-point and truncation errors
- [x] Unknown and zero maximum fail closed
- [x] Above-maximum current cannot be clamped into eligibility
- [x] Threshold 0 and 100 behavior is specified

## Action Safety

- [x] Existing gate order is unchanged
- [x] Existing Quickslot synthesis path is unchanged
- [x] Cooldown and retry behavior are unchanged
- [x] Deterministic OR ordering preserves legacy causes
- [x] No new thread, timer, input action, or addon signal is introduced

## Configuration and Presentation

- [x] Legacy configuration defaults Ultimate disabled
- [x] Current configuration emits the fourth watch
- [x] Settings use the existing accessible control model
- [x] Diagnostics and status name Ultimate truthfully
- [x] User and maintainer documentation are in scope
