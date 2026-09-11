# Quickstart: Documentation Figure System

## Build and focused policy

```powershell
mdbook test docs
mdbook build docs
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
node .github/scripts/docs-render-smoke.mjs --site target/docs-site/html
```

Expected browser output includes the existing S086 and S087 sentinels plus:

```text
ESO_WEAVE_FIGURE_SMOKE_PASS_V1
```

## Manual accessibility check

1. Open one generated page containing a raw HTML screenshot and one containing a Markdown flow diagram.
2. Confirm that every meaningful image has a visible `Expand image` affordance and the landing wordmark does not.
3. Tab to a figure trigger and press Enter, then repeat with Space.
4. Confirm that the native modal opens, the close button receives focus, the caption appears when present, and background controls cannot receive focus.
5. Press Escape and confirm focus returns to the exact trigger.
6. Reopen and use the close button, then reopen and select the backdrop. Confirm both paths return focus.
7. Repeat with a portrait screenshot and a brand example in navy and light themes at narrow and wide widths.
8. Confirm the expanded image is not blurry from upscaling and retains its aspect ratio.
9. Set browser zoom to 200 percent and confirm captions remain readable and attached to their figures.
10. Open print preview and confirm figures plus captions remain while modal chrome and affordances are absent.

## Full repository parity

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
git diff --check
```

All commands must pass before the S088 commit and pull request.
