# Quickstart: Evidence-Scoped Encounter Recommendations

## Automated verification

Run the focused pure policy suite:

```powershell
cargo test --test encounter_recommendations --locked
```

Expected result: deterministic ready, qualified, suppressed, empty, invalid,
boundary, tie, provenance, and isolation cases pass.

Run the encounter-history UI suite:

```powershell
cargo test --test app_encounter_history --locked
```

Expected result: the worker bundles one projection and report, observed facts stay
separate, wide and narrow rendered states expose provisional recommendations or
exact reasons, and no recommendation action control exists.

Run repository text and documentation checks:

```powershell
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
node --test .github/scripts/issue-link-policy.test.mjs
```

Expected result: policy, UTF-8, forbidden-dash, generated documentation, issue-link, and
recommendation-boundary checks pass.

Run full CI parity in the foreground:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected result: all commands exit zero.

## Manual deterministic inspection

1. Start ESO Weave with a compatible local catalog and an imported complete
   encounter.
2. Open File, Encounter History and select the encounter.
3. Confirm Observed Metrics appear before Provisional Recommendations.
4. Confirm every prompt names its provisional status and shows the encounter,
   calculation, catalog, and policy identities.
5. Inspect a short or materially incomplete fixture and confirm metrics remain
   visible while advice is suppressed with exact reasons.
6. Confirm the recommendation section offers no apply or automation control.

No live ESO session, release artifact, network service, or user-provided capture is
required. Issues #110, #129, and #131 remain separate nonblocking verification.
