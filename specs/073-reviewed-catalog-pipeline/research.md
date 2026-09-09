# S073 Research and Decisions

## Decision 1: One orchestration command, no second data parser

**Decision**: Extend `catalog-compiler` with pipeline commands and compose the
S071 importer, S070 compiler/verifier/diff, and S072 icon cache.

**Rationale**: These modules already own the hard parsing and data contracts.
Adding a stock UI documentation parser during orchestration would create a new
semantic authority not specified or tested by the preceding slices.

**Rejected alternatives**:

- A second executable would duplicate CLI parsing and packaging work.
- A general plugin system would expand executable input and trust surface.
- Executing third-party conversion tools would violate source isolation.

## Decision 2: Content pins before interpretation

**Decision**: Every source has one role, one channel, one immutable revision,
one expected SHA-256, one byte cap, and exactly one local or HTTPS location.
Bytes enter a content-addressed cache only after stable bounded reading and hash
verification. Pipeline input is referenced by source ID.

**Rationale**: A single acquisition boundary makes offline, Live, PTS, and
capture modes auditable and lets the later stages operate only on verified
cache objects.

**Rejected alternatives**:

- Mutable latest URLs cannot reproduce shipped candidates.
- Paths embedded in reports disclose developer machines.
- Trusting an existing cache filename without rereading permits corruption.

## Decision 3: Narrow optional network adapter

**Decision**: Network access requires a CLI opt-in and a request that allows it.
Only `https://raw.githubusercontent.com/` immutable revision paths are accepted.
The revision must be a 40-character lowercase commit hash present in the URI.
Redirects are errors, the global timeout is 15 seconds, and the body is streamed
through the declared cap plus one byte before exact hash verification.

**Rationale**: Current approved machine-readable source snapshots are pinned in
GitHub. A hardcoded host and immutable path rule avoid turning maintainer tooling
into a general network fetcher or SSRF surface.

**Rejected alternatives**:

- Following redirects can leave the reviewed host and policy boundary.
- A request-provided host allowlist can approve itself and has no authority.
- Shelling out to curl weakens cross-platform limits and error typing.

## Decision 4: Candidate bytes are review-safe by allowlist

**Decision**: Source objects, collector captures, normalized input bundles, and
icon cache objects remain outside candidates. The candidate includes only the
catalog, canonical reports, checksums, manifest, and a redacted icon receipt.

**Rationale**: The compiled catalog contains only S068-approved normalized
facts. Source and local image bytes have different redistribution and privacy
lifecycles and do not need to be copied for a semantic review.

**Rejected alternatives**:

- Bundling raw sources simplifies reproduction but can redistribute prohibited
  material or personal captures.
- Bundling transformed user icons conflicts with S072 user-local-only status.
- Omitting hashes would make external source retention unverifiable.

## Decision 5: Immutable directory publication

**Decision**: Construct the complete candidate in a temporary directory beneath
the candidate root, hash an ordered canonical manifest, verify each allowlisted
artifact, and rename without replacement to `<channel>/<manifest-sha256>`.

**Rationale**: Readers cannot observe partial output, repeated runs reuse exact
candidates, and failure cannot damage prior review evidence.

**Rejected alternatives**:

- A mutable `latest` directory makes interruption and concurrent readers unsafe.
- Automatic cleanup adds deletion authority and can remove retained evidence.
- Overwriting an identity hides nondeterminism or corruption.

## Decision 6: Thresholds block publication, not construction evidence

**Decision**: Compare only verified same-channel baselines. Stable configured
limits cover removals from all S070 diff surfaces and coverage removals. A
blocking finding prevents candidate publication and produces a redacted failure
receipt outside the immutable candidate root.

**Rationale**: Review policy should fail closed on likely coverage regressions
while keeping ordinary additions visible. The request records the threshold
rather than hiding it in code.

**Rejected alternatives**:

- Blocking every change makes the pipeline unusable for discovery.
- Publishing candidates with blocking findings blurs validation and acceptance.
- Diffing across channels creates meaningless removals and additions.

## Decision 7: Workflow cannot accept its own output

**Decision**: Add a SHA-pinned read-only workflow with manual and scheduled
triggers. It builds a fixture candidate for CI and may build a checked request,
then uploads only the candidate directory. It contains no repository write,
release, acceptance, or promotion step.

**Rationale**: GitHub automation supplies reproducible review evidence without
granting the pipeline authority to change the product.

**Rejected alternatives**:

- Automated commits or pull requests make source discovery an unreviewed writer.
- Release publication before acceptance bypasses the issue's main safety gate.
- Secrets are unnecessary for public immutable sources and increase exposure.
