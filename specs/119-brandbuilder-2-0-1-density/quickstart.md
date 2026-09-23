# Quickstart: Verify BrandBuilder 2.0.1 Density

1. Run `node --test .github/scripts/brand-kit-policy.test.mjs`.
2. Run the focused theme tests and confirm both themes resolve to the governed precise-pointer metrics.
3. Run `cargo test --test app_ui_sizing --locked` to preserve explicit product-owned geometry.
4. Run the complete Cargo merge gate and locked release build.
5. Run repository documentation, encoding, line-ending, mojibake, forbidden-dash, and diff checks.
6. Confirm the adoption record and recovery artifact match the immutable official 2.0.1 package.
