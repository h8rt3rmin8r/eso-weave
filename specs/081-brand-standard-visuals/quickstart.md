# Quickstart: Verify Brand Standard Visuals

## Focused policy tests

```text
node --test .github/scripts/docs-policy.test.mjs
```

Expected: every test passes, including S081 source, asset-byte, palette, CSS, generated-output, and mutation cases.

## Build and policy

```text
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

Expected: examples, local links, source policy, generated output, and all three asset checks pass.

## Repository CI parity

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo build --release --locked
typos docs/src docs/README.md README.md
git diff --check
```

Expected: every command passes without changing application behavior.

## Visual and accessibility inspection

Inspect the generated Brand Standard at 320, 768, and 1440 CSS pixels in light and navy themes. Confirm cards reflow without horizontal page overflow, dark and light surface labels remain readable, images stay contained, the glyph never appears in the light card, all chips have visible boundaries, and chip accessible names match visible roles and hex values.

## Asset identity and text hygiene

Confirm all three authority and published-copy pairs have identical SHA-256 values. Confirm every changed text file is UTF-8 without BOM, uses LF, and contains no replacement characters, mojibake markers, en-dashes, or em-dashes.
