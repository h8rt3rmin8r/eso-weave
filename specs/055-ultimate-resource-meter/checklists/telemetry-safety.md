# Telemetry Safety Checklist

- [x] Protocol v1 through v4 payload counts remain frozen.
- [x] Every new field has distinct high-bit markers and a complement checksum.
- [x] No value is reconstructed from rounded percentages.
- [x] Zero current is distinct from unavailable.
- [x] Zero or missing maximum fails closed in presentation.
- [x] Front and back costs fail independently.
- [x] Unknown active bar selects neither cost.
- [x] Signal loss clears stale Ultimate state.
- [x] Activation baseline precedes world Active.
- [x] Ultimate is excluded from all synthesis and auto-potion types.
- [x] Accessibility communicates meaning independently of purple and green.
- [x] Live-game verification is deferred to a separate governed issue.
