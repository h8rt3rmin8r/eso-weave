# Contract: Linux Key Capabilities

## Capability construction

The virtual keyboard advertises the union of:

1. every Linux key mapped from `Key::ALL`;
2. BTN_LEFT and BTN_RIGHT for supported synthesis;
3. every key reported by the selected physical keyboard.

The union is applied before the physical device is grabbed. If synthesis created a narrower virtual device earlier, it is replaced before grab.

## Classification and forwarding

- Known native keys are classified through the application engine.
- Unknown native keys bypass application classification but remain key-only pass-through.
- Suppressed known keys are not forwarded.
- Every other KEY event is emitted to the virtual device and any failure is returned.
- Non-KEY metadata is intentionally outside this virtual-device contract.
