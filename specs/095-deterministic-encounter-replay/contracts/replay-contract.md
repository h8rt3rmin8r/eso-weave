# Replay Contract

## Current profile boundary

`schema_version == 2 && addon_version >= 3` requires normalization profile v1.
The profile API version equals the envelope API version. Enum fields are exact
finite integers; result sets are nonempty, bounded, unique, and semantically
non-conflicting.

Numeric unit IDs use the documented integer domain. Positive exact integers are
interned; nil, non-numeric, non-integral, non-positive, and out-of-range values
map to anonymous actor 0 in both producer and replay. Accepted IDs use exact
base-10 integer keys in both implementations, avoiding runtime `tostring`
rounding.

## Replay input

Only the validated profile, raw observations, raw loss metadata, capture status,
and structural timing metadata may affect derivation. Supplied `events` are
comparison input only.

## Replay output

The replay event sequence uses the existing normalized event model. Comparison
is exact and ordered across source sequence, projection ordinal, monotonic time,
kind, and payload.

## Failure policy

- Structurally valid complete-current disagreement: divergent, value-free
  validation error, no storage.
- Structurally invalid projection mutation: structural validation error, no
  replay or storage.
- Declared loss or discontinuity: indeterminate, degraded compatibility import.
- Malformed current profile: validation error, no replay or storage.
- V1 or addon-v2 schema-v2: unavailable, compatibility import unchanged.

## Bounds

Processing is linear in observations plus values plus projections. Existing raw,
actor, event, string, and parser ceilings apply. At most one controlled mismatch
location is retained. Replay performs no I/O or mutation.
