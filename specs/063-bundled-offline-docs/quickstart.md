# Quickstart: Bundled Offline Documentation

## Operator check

1. Install or run a production build on Windows or Linux.
2. Disconnect external networking.
3. Choose Help > Documentation and confirm the browser opens `127.0.0.1` under `/eso-weave/`.
4. Navigate, search for `PixelBeacon`, switch theme, and load pages with fonts and images.
5. Choose Documentation again and confirm the same address is reused.
6. Exit ESO Weave and confirm the local address no longer responds.

## Developer validation

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked --quiet
cargo build --release --locked
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs --dest-dir ../target/docs-site
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

Run spelling, text-hygiene, whitespace, mojibake, generated-reference completeness, and Windows/Linux package smoke gates. Release builds require exact `mdbook 0.5.4` and `mdbook-linkcheck2 0.13.0`; ordinary debug tests use the checked fixture.
