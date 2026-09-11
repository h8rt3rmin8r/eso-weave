# Quickstart: Documentation Diagram Rendering Compatibility

## Focused red and green tests

```powershell
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
```

Record the expected pre-implementation failures in `analysis.md`, then rerun after implementation.

## Build and browser smoke

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
node .github/scripts/docs-policy.mjs
mdbook-linkcheck2 --book-dir docs
typos docs/src docs/README.md README.md
```

## Repository gates

```powershell
git diff --check
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

## Visual evidence

Review the generated manual in the supported matrix and reconcile `docs/project/diagram-rendering-compatibility.md`. Verification observations inform compatibility evidence but never block unrelated project progress.
