# Quickstart: Verify Repository Trust Boundaries

## Local policy tests

```powershell
node --test .github/scripts/trust-policy.test.mjs
node .github/scripts/trust-policy.mjs .
```

## Existing repository gates

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

## Hosted policy verification

Read the effective settings after configuration:

```powershell
gh api repos/h8rt3rmin8r/eso-weave/branches/main/protection
gh api repos/h8rt3rmin8r/eso-weave/actions/permissions
gh api repos/h8rt3rmin8r/eso-weave/actions/permissions/workflow
gh api repos/h8rt3rmin8r/eso-weave/actions/permissions/selected-actions
```

Confirm that the protected branch requires the six bootstrap pull-request checks
named in [workflow-integrity.md](contracts/workflow-integrity.md), enforces
administrators, blocks deletion and force-push, and requires linear history plus
conversation resolution. After S099 merges, require `Enforce protected trust
policy` as the seventh check; GitHub cannot emit that base-owned check until the
workflow exists on protected `main`.

## Disclosure and text checks

- Inspect the complete diff for detailed finding evidence or reusable abuse instructions.
- Confirm no secret value appears in specifications, documentation, tests, logs, or the PR body.
- Confirm changed text is UTF-8 without BOM, uses LF, contains no mojibake, and contains no em dash or en dash.
