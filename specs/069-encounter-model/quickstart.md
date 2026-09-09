# Quickstart: External Encounter Model

## Read the authority

Start with `docs/project/encounter-model.json`. The canonical reader guide is
`docs/src/reference/encounter-data-and-metrics.md`. The JSON authority controls
when prose and examples disagree.

## Validate the contract

```powershell
node --test .github/scripts/docs-policy.test.mjs
node .github/scripts/docs-policy.mjs
```

Production validation loads the authority and both synthetic fixtures. A broken
required family, privacy default, ordering rule, projection, or size receipt
fails the policy gate.

## Inspect the deterministic spike

`fixtures/dummy-encounter.json` is a ten-second synthetic encounter. It contains
damage, healing, effect, resource, cast, bar-change, death, resurrection,
boss-health, performance, quickslot, boundary, and discontinuity events.

`fixtures/dummy-projection.json` records the expected metrics, catalog receipts,
raw hash, and measured storage values. The discontinuity intentionally marks all
spanning metrics degraded while retaining their observed values.

## Check unknown-ID behavior

The first catalog receipt leaves the fixture's unknown ability unresolved. The
second resolves it. Both receipts point to the same raw-content hash. A future
implementation must reproduce that behavior without rewriting the encounter.

## Interpret storage numbers

Raw and gzip values for the checked-in fixture are exact. One-hour and
100-encounter projections are simple linear illustrations, not production
retention recommendations. Representative live measurements belong to the
separate verification issue.

## Handoff order

Implement future work in this sequence:

1. addon capture;
2. bounded SavedVariables import and local persistence;
3. versioned calculations;
4. UI presentation and quality disclosure;
5. recommendations that consume versioned projections.

Do not send the encounter stream over Pixel Bus and do not make capture depend
on action automation.
