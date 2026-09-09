# Encounter Governance Checklist: S069

- [x] Raw encounter data is user-owned and separate from bundled catalog data.
- [x] Pixel Bus is excluded from bulk encounter transport.
- [x] Future imports are bounded, atomic, validated, and non-executing.
- [x] Event identity, order, monotonic timing, duplicates, gaps, and resets are explicit.
- [x] Declared loss degrades affected projections instead of disappearing.
- [x] Actor identity is encounter-local and privacy-minimized by default.
- [x] Raw observations are immutable and derived records are rebuildable.
- [x] Unknown catalog IDs survive and can resolve without raw mutation.
- [x] Live and PTS evidence cannot be silently mixed.
- [x] Minimum metrics declare formulas, versions, ranges, units, and quality.
- [x] Synthetic evidence is not presented as live Combat Metrics parity.
- [x] Storage estimates distinguish exact fixture measurements from provisional extrapolation.
- [x] Capture, import, calculation, UI, and recommendation handoffs are ordered.
- [x] Observation remains independent of action automation.
