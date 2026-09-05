# Geometry Safety Checklist

**Purpose**: Prevent plausible-but-wrong payload decoding
**Created**: 2026-09-04

## Authority

- [x] Exactly one side chooses negotiated columns
- [x] Companion measurement is validation-only
- [x] Header positions do not depend on announced geometry

## Validation

- [x] Magic and version are checked before payload bytes
- [x] Both count bytes have distinct markers and complement checksums
- [x] Recognized corruption cannot fall back to legacy
- [x] Legacy requires a positive legacy heartbeat
- [x] Numeric and measured-surface bounds are explicit

## Geometry

- [x] Header and payload share one cell-position formula
- [x] Capture extent and sample points share one `BusLayout`
- [x] Current cells fit one row at the minimum supported width
- [x] Exact boundary and first-wrap behavior are testable

## Lifecycle and Cost

- [x] Resize and scale-only convergence paths are defined
- [x] Steady state has one capture
- [x] Growth has at most one recapture
- [x] Layout state is change-detected and not persisted
