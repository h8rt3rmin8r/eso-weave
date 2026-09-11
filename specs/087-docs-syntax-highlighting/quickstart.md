# Quickstart: Documentation Syntax Highlighting

## Focused red and green tests

```powershell
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
```

Record the expected pre-implementation failures in `analysis.md`, then rerun after implementation.

## Build and runtime smoke

```powershell
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
node .github/scripts/docs-render-smoke.mjs --site target/docs-site/html
```

Set `DOCS_CHROME_BIN` to an absolute Chrome-compatible executable when automatic local discovery is unsuitable.

## Documentation gates

```powershell
node --test .github/scripts/docs-policy.test.mjs
mdbook-linkcheck2 --standalone docs
typos docs/src docs/README.md README.md
```

## Repository gates

```powershell
git diff --check
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

## Manual evidence

Open the generated Catalog Compiler, Installation, Screenshot Maintenance, and Troubleshooting pages. Confirm exact copy text, token legibility, and narrow scrolling in all five themes. These observations supplement automated repository evidence and never block unrelated progress.
