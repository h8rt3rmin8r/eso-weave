# Quickstart: Responsive Documentation Tables

## Prerequisites

- Repository root is the current directory.
- Node.js 24 and the pinned Rust toolchain are available.
- `mdbook` 0.5.4 and `mdbook-linkcheck2` 0.13.0 are installed.
- A supported host Chrome-compatible browser is available to the existing smoke harness.

## 1. Run focused contract tests

```text
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
```

Expected result: all policy and receipt tests pass, including the S089 inventory, accessibility, CSS, browser matrix, and special-state mutations.

## 2. Build and test the documentation

```text
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

Expected result: source policy, generated table structure, links, local assets, earlier documentation systems, and offline delivery all pass. The existing linkcheck renderer may print its known fragment-resolution warnings.

## 3. Run generated-browser evidence

```text
node .github/scripts/docs-render-smoke.mjs --site target/docs-site/html
```

Expected result: the receipt contains exactly 20 S089 named-page observations and prints all four sentinels:

```text
ESO_WEAVE_DIAGRAM_SMOKE_PASS_V1
ESO_WEAVE_SYNTAX_SMOKE_PASS_V1
ESO_WEAVE_FIGURE_SMOKE_PASS_V1
ESO_WEAVE_TABLE_SMOKE_PASS_V1
```

Inspect the receipt for keyboard table scrolling with unchanged page position, resize state changes, `visualViewport.scale` equal to 2, print cue suppression, and blocked-script containment.

## 4. Run repository parity

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected result: all commands complete successfully in the foreground.

## 5. Run text and diff hygiene

Check changed text for UTF-8 without BOM, LF endings, replacement characters, forbidden dashes, spelling, and `git diff --check`.

Expected result: zero encoding, mojibake, spelling, whitespace, or punctuation violations.
