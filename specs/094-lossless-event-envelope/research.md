# Research: Lossless Subscribed-Event Envelope

## Decision 1: Raw authority is separate from normalized compatibility events

**Decision**: Add a raw source-observation stream and link normalized events to
their primary raw sequence and projection ordinal.

**Rationale**: One callback can yield zero, one, or multiple normalized facts.
Using normalized event sequence as raw authority would continue dropping unknown
callbacks and would misrepresent one-to-many projections.

**Rejected alternative**: Add raw arguments to each normalized event. This cannot
represent unknown callbacks or shared getter observations without duplication.

## Decision 2: Finite numbers use integer-only IEEE-754 descriptors

**Decision**: Encode sign, integer significand, and binary exponent as tagged
data produced from `math.frexp`, including explicit negative zero.

**Rationale**: The restricted parser intentionally accepts integer syntax only.
An integer-only descriptor preserves every finite Lua double without locale,
decimal-parser, or JSON floating-point ambiguity.

**Rejected alternatives**: Rounding to integers is lossy. `tostring` is not a
round-trip contract. Decimal JSON would expand the hostile parser and can lose
lexical identity.

## Decision 3: Loss is whole-observation and terminally bounded

**Decision**: Preflight all tagged values before insertion. Once raw retention
cannot continue, advance source sequence for each omitted observation, record one
exact contiguous loss range and reason, mark partial, and retain terminal reserve.

**Rationale**: Truncating a string or partial argument list is silent corruption.

## Decision 4: Compatibility normalization remains in Lua for S094

**Decision**: Retain the current normalized stream, link it to raw source evidence,
and retain every API read it consumes. Unknown raw sources need no projection.

**Rationale**: Rebuilding every metric input in Rust at the same time as the raw
and storage migration would make issue #186's first tranche too broad. The raw
stream makes later deterministic renormalization possible.

**Rejected alternative**: Rewrite all normalization and metric consumers now.
That would couple a foundation change to unrelated behavior changes.

## Decision 5: Mixed-version storage uses transactional schema v2 migration

**Decision**: Store schema v2 adds per-row canonical-format version and accepts
capture schemas 1 and 2. Migration copies v1 canonical blobs byte-for-byte before
recreating immutability protection.

**Rationale**: Existing hashes identify user-owned immutable bytes. Fabricating
raw values or reserializing v1 would change evidence.

## Decision 6: Product raw values stay local and diagnostics stay value-free

**Decision**: Issue #186 removes capture-side identity omission only. Upload,
telemetry, logs, receipts, errors, public evidence, and history UI remain unable
to disclose payload values.

**Rationale**: Complete local evidence does not require outbound disclosure.

## Decision 7: Canonical docs replace a new blog subsystem

**Decision**: Update the versioned manual, changelog, build plan, and
machine-readable contract instead of adding a blog.

**Rationale**: The development skill requests a blog post, but this repository
has no blog surface and prior project decisions designate the manual and
changelog as the canonical publication path. Adding a blog subsystem would be a
disproportionate architectural expansion, so S094 explicitly deviates here.

