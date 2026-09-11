# Recommendation Report Contract

## Input

1. The sole input is one immutable S077 `EncounterProjection`.
2. Generation performs no I/O and does not reopen the catalog or raw store.
3. The projection remains unchanged and independently renderable as observed facts.

## Version and provenance

1. The report schema begins at `1` and policy version is `s090-v1`.
2. Every advice item cites recommendation schema and policy, encounter and session
   identities, raw SHA-256, projection schema, metric algorithm, catalog schema and
   version, catalog semantic SHA-256, channel, and API version.
3. Citations contain no timestamp, local path, personal identity, or mutable label.

## Global gates

1. Any projection other than schema 1 with the `s069-v1` algorithm suppresses all
   advice.
2. Recommendation-bearing metric receipts must match the supported algorithm and
   projection range, and their quality must agree with their exact loss ranges.
   Any mismatch suppresses all advice.
3. Duration below 10,000 ms suppresses all advice.
4. Fewer than three observed casts suppresses all advice.
5. Exact declared loss at or above 10 percent of the inclusive sequence span
   suppresses all advice.
6. Exact declared loss below 10 percent qualifies all otherwise eligible advice.
7. Invalid sequence-span, reversed loss-range, or loss arithmetic fails closed to
   suppression.

## Catalog uncertainty

1. Any unknown positive ID qualifies otherwise eligible known-target advice.
   Resolution is entity-kind scoped, so an ability and effect sharing one
   numeric ID cannot make each other eligible.
2. An unknown target never produces a recommendation.
3. Valid damage share belonging to unknown ability IDs is summed.
4. Unknown damage share at or above 25 percent suppresses only the dominant-damage
   rule.
5. Unknown IDs unrelated to a known-effect candidate do not suppress that effect
   prompt.

## Rules

1. Dominant damage selects at most one known ability with a valid observed share at
   or above 40 percent. Higher share wins, then lower ID.
2. Low uptime selects at most one known effect with valid observed uptime at or
   below 50 percent. Lower uptime wins, then lower ID.
3. Dominant damage precedes low uptime.
4. Invalid or unavailable values do not produce items.
5. Reports contain at most two items.

## Presentation

1. Observed Metrics render before a distinct Provisional Recommendations section.
2. Ready means evidence gates passed; it does not remove the provisional label.
3. Qualified items display every applicable qualification next to the item.
4. Suppressed reports show all suppression reasons and no advice.
5. Empty eligible reports state that no provisional review prompt crossed the
   versioned thresholds.
6. Advice uses neutral review language and makes no causal, optimality, parity,
   build-correctness, or guaranteed-improvement claim.

## Isolation

The recommendation domain has no dependency on input, weave, fishing, potion,
game focus, Pixel Bus, addon lifecycle, network, telemetry, storage, settings,
logs, or catalog mutation. The UI exposes no recommendation action control.
