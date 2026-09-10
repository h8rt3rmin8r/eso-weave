# Metric Integrity Checklist: S077

- [x] Encounter duration uses validated elapsed end time, not mixed clock domains.
- [x] Cast ordering uses sequence only.
- [x] Player attribution uses source type, not actor ID.
- [x] Effective healing subtracts overflow with saturation.
- [x] Damage shares avoid a zero denominator.
- [x] Effect intervals are clipped and unioned rather than summed.
- [x] Zero duration has an explicit unavailable representation.
- [x] Results carry algorithm, sequence range, unit, quality, and loss evidence.
- [x] Catalog ID lists are sorted, deduplicated, and disjoint.
- [x] Exact channel/API compatibility is required.
- [x] Later catalog resolution cannot mutate raw data or metric values.
- [x] Checked arithmetic rejects aggregate overflow.
- [x] Canonical output contains no timestamp, path, or sensitive payload.
