# Quickstart: Verify BrandBuilder 2.0 Kit Adoption

## Focused conformance

```powershell
node --test .github/scripts/brand-kit-policy.test.mjs
node .github/scripts/brand-kit-policy.mjs
cargo test app::theme
cargo test --test app_ui_sizing
```

## CI parity and documentation

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
mdbook test docs
mdbook build docs
git diff --check
```

## Manual review

Open representative dark and light screens and confirm:

- cards and base surfaces use different governed roles;
- primary actions and focus are teal on dark and accessible gold on light;
- action text remains legible;
- technical metadata uses Geist Mono;
- interactive response regions remain usable at the narrow supported layout;
- status retains visible text or shape cues in addition to color.

Release installation and merge remain outside S117.
