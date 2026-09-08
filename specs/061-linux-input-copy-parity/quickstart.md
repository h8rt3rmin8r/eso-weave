# Quickstart: Validate S061

## Focused behavior

```powershell
cargo test --locked --test input_engine
cargo test --locked --test fishing
cargo test --locked --test pixelbus
cargo test --locked --test app_strings
```

Run Linux-specific unit tests in Linux CI or a Linux development environment:

```bash
cargo test --locked input::linux::tests:: -- --list
cargo test --locked input::linux::tests::
```

Expected evidence:

- all 13 application keys map and appear in Linux application capabilities;
- physical-only keyboard keys remain in the advertised union;
- forwarded key emission errors are observable and non-key metadata is excluded;
- generated input begins menu-gated and opens only after explicit gameplay evidence;
- latency and logging text contains the new semantic anchors and rejects obsolete wording.

## Full gates

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
typos docs/src docs/README.md README.md
git diff --check
```

Also confirm no mojibake or BOM appears in changed files.
