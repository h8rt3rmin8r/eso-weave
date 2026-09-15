# Research: Deterministic Encounter Replay

## Decision 1: Keep capture schema v2 and add an addon-v3 profile

**Decision**: Current producers attach `normalization_profile` with algorithm
version, API version, and bounded runtime enum semantics. Addon version advances
to 3; schema remains 2.

**Rationale**: ESO callback documentation names the constants but does not provide
all stable numeric engine values. Tests also intentionally use synthetic values.
Capturing the semantic mapping makes production and tests independently replayable
without hard-coding undocumented numbers or changing the accepted schema.

**Rejected**: Hard-coded constants (not authoritative), schema v3 (unnecessary
compatibility break), and trusting compatibility events (defeats raw authority).

## Decision 2: Four replay outcomes

**Decision**: `verified`, `divergent`, `indeterminate`, and `unavailable` remain
distinct. Complete current captures must be verified. Divergence rejects import.
Loss or malformed correlation is indeterminate. V1 and pre-profile v2 are
unavailable but retain compatibility import.

**Rationale**: Missing raw data can change actor IDs and shared byte-budget
behavior, so it cannot safely prove agreement or disagreement.

## Decision 3: Pure bounded replay and exact comparison

**Decision**: Replay consumes only structurally validated raw observations and
profile metadata, then compares ordered `(source_sequence, projection_ordinal,
monotonic_ms, kind, payload)` tuples. Source-specific API batches use strict
bounded state machines. Numeric conversion matches Lua exactly and fails closed.

**Rationale**: This prevents circular trust, ambiguous API association, value
leakage, and resource amplification.

## Decision 4: Retain the S094 source surface

**Decision**: Add no callback or API families. Complete the decision record for
the existing 11 callbacks, six API sources, two lifecycle sources, and reviewed
excluded families.

**Rationale**: Combining source expansion with a replay equivalence proof would
obscure failures and exceed the final bounded tranche of issue #186.

## Decision 5: Pin primary API evidence

**Decision**: Pin Live API 101050 at commit
`f76cf16c4e5be7b234d15dc7f676febffa64c5bb` (2026-08-10) and PTS API
101051 at commit `1baf1131560c2bcd38ffd2bd070728273b25f934`
(2026-08-31).

**Rationale**: The official API documentation provides selected callback and API
signatures. The machine contract records that the excluded
`EVENT_DUEL_FINISHED` callback changes from eight arguments on Live to nine on
PTS, renames the crossplay display-name argument, and appends a platform display
name. The delta remains visible rather than being collapsed into one assumed
contract.

**Sources**:

- https://github.com/esoui/esoui/tree/f76cf16c4e5be7b234d15dc7f676febffa64c5bb
- https://github.com/esoui/esoui/blob/f76cf16c4e5be7b234d15dc7f676febffa64c5bb/ESOUIDocumentation.txt
- https://github.com/esoui/esoui/tree/1baf1131560c2bcd38ffd2bd070728273b25f934
- https://github.com/esoui/esoui/blob/1baf1131560c2bcd38ffd2bd070728273b25f934/ESOUIDocumentation.txt

## Decision 6: Use established publication surfaces

**Decision**: Publish user-facing changes in the canonical manual, architecture,
testing strategy, changelog, and chronological build plan.

**Rationale and explicit deviation**: The development skill requests a blog, but
this repository has no blog subsystem. Creating one solely for S095 would be a
disproportionate architectural expansion. The existing reviewed documentation
surfaces satisfy the project publication contract.
