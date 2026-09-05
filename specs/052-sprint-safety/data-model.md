# Data Model

## MovementSignal

| State | Meaning | Auto-potion |
| --- | --- | --- |
| `Unknown` | B9 is absent, malformed, stale, or unsupported | Does not block indefinitely |
| `OnFoot` | Player is on foot without explicit sprint evidence | Eligible subject to other gates |
| `Mounted` | Player is mounted; gallop is not inferred | Eligible subject to other gates |
| `Sprinting` | Debounced keyboard-mode on-foot sprint evidence | Blocked |

## Addon Sprint Detector

- Candidate state: sprint or ordinary
- Candidate start timestamp
- Last qualifying sprint timestamp
- Published sprint flag
- Hard exclusions: lifecycle invalid, mounted, gamepad mode, not moving, swimming, falling, dead, reincarnating, or roll dodging
- Soft evidence: every ordinary active-bar slot has a non-cost state failure

## Auto-potion State

- Current movement observation joins the existing controller-owned gates.
- `BlockReason::Sprinting` is lower precedence than lifecycle, focus, beacon, suspension, context, life, world, and travel gates.
- No pending attempt is stored. Recovery evaluates current resources, quickslot, cooldown, and every gate.
