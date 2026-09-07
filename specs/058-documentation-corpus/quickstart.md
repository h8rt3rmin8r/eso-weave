# Quickstart: Validate S058

From the repository root:

```text
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
git diff --check
```

Then verify:

1. `docs/README.md` maps published, project, and archive lifecycles.
2. `docs/project/migration-ledger.json` passes the corpus policy.
3. The root README points to the canonical Pages site and stays within 120 lines.
4. Plans 001 through 027 are archived and plan 028 is the only current plan.
5. The generated search index contains no project or archive sentinel text.
6. No removed live path remains outside an explicit ledger history record.
