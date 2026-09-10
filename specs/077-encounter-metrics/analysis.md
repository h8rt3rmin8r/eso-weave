# S077 Spec-kit Analysis

## Pre-implementation Gate

Result: PASS.

The specification, clarification record, two completed checklists, research,
data model, library/CLI contract, plan, quickstart, and chronological task list
form one bounded implementation slice for issue #134.

## Coverage Matrix

| Concern | Requirement authority | Planned evidence |
| --- | --- | --- |
| Five deterministic metrics | FR-003 through FR-010 | Value, ordering, zero-duration, and repeatability tests |
| Loss-aware quality | FR-011, FR-012 | Partial and invalid-ordering tests |
| Catalog reconciliation | FR-013 through FR-016 | Compatibility and later-known tests |
| Safe derived publication | FR-017 through FR-020 | Bytes, alias, link, failure, and replacement tests |
| Scope and evidence honesty | FR-021, FR-022 | Source inventory and documentation checks |

## Findings and Resolutions

1. Actor ID 1 is not reliably the player because actors are allocated in
   observation order. V1 uses retained combat source type 1.
2. S075 records effect begin/end values without absolute clock origin and omits
   effect slot. V1 uses duration anchored at event monotonic time, clipped and
   unioned. Live semantic parity remains #131.
3. Pet ownership cannot be reconstructed from schema-v1 raw facts. V1 excludes
   pets rather than guessing attribution.
4. A derived SQLite schema would pre-empt #135 query and lifecycle design. S077
   emits explicit atomic canonical JSON that is disposable and rebuildable.
5. A creation timestamp would make equal projections byte-different. Receipt
   provenance uses raw and catalog hashes and versions only.
6. Effect callback IDs may be catalog effects or abilities. The join recognizes
   either while other ability-bearing events query abilities.
7. Zero duration has no rate denominator. Rate and uptime values become null;
   cast order and catalog reconciliation remain available.
8. S076 already owns raw integrity. S077 reuses that boundary and creates no
   second parser or validation dialect.
9. `started_monotonic_ms` is the capture's raw clock origin while event times and
   `ended_monotonic_ms` are elapsed. Duration is the elapsed end value, never a
   subtraction across those clock domains.
10. S069's synthetic model accepted presentation-order changes before S076 froze
    canonical raw storage. S076 now rejects event arrays outside authoritative
    sequence order, so S077 consumes that stricter contract and proves repeatable
    projection of equal validated raw bytes rather than accepting a second raw form.

## Constitution Gate

- Full spec-kit artifacts exist before implementation.
- Raw and catalog authorities remain immutable and separate from derived output.
- The command is explicit, local, non-networked, and non-interactive.
- No observation, metric, or catalog fact authorizes gameplay input.
- Test-first tasks precede implementation.
- Full repository parity and textual hygiene are mandatory.
- No pinned workflow, dependency, packaging, release, license, or toolchain file
  is planned.

No unresolved critical conflict, hidden nondeterminism, raw-plane mutation,
privacy expansion, unsafe publication promise, impossible live-parity claim, or
template token remains. Implementation may begin.

## Post-implementation Gate

Result: PASS.

The reusable encounter projection module loads only through the validated S076
store, opens the catalog through its schema/integrity/checksum reader, enforces
exact channel and API compatibility, and publishes only to an explicit distinct
derived JSON path. The raw store and catalog remain byte-identical in end-to-end
tests.

The actual capture-schema fixture produces observed DPS 300, effective HPS 80,
damage shares 0.5 and 0.5, effect uptime 0.6 after overlap union, and casts 100,
999999, 100. Every result names `s069-v1`, source range, unit, degraded quality,
and the exact declared loss. Tests also cover opaque actor-order safety, saturated
healing, zero-duration unavailability, empty shares, checked aggregate overflow,
repeatable canonical bytes, exact-compatible catalog rejection, output aliases
and links, no-clobber, and later resolution of ID 999999 without changing raw
identity or metric results.

The maintainer CLI prints the same projection it atomically publishes. Canonical
reference, architecture, test-strategy, machine-readable model, active plan, and
changelog records now distinguish repository implementation from issue #131 live
parity. S069's provisional presentation-order acceptance is deliberately not
carried forward because S076 made ordered event arrays part of valid immutable raw
identity.

Formatting, strict Clippy, the complete locked Rust suite, both optimized binaries,
spelling, documentation-policy tests, mdBook examples, book and link build,
generated-site validation, JSON parsing, whitespace, template-token, UTF-8
without BOM, forbidden-dash, and mojibake checks pass. No new dependency, workflow,
packaging, release, license, toolchain, upload, telemetry, UI, recommendation, or
gameplay-action surface was added.
