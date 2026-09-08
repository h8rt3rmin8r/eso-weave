# Quickstart: Settings Runtime Parity

## Operator check

1. Open Settings and confirm Fishing includes Arm Timeout, Reel Delay, Recast Delay, and Interact Key.
2. Change a Fishing value while Fishing is off, then enable Fishing and confirm the new value is used without restarting.
3. Change a Fishing value while Fishing is on and confirm Fishing turns off without an emitted action.
4. Change Pixel Bus tolerance or a sample interval and confirm the running reader adopts it.
5. Change Block Size and confirm the interface still reports the current running size plus the required managed redeploy, ESO reload or relog, and application restart.

## Developer validation

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked --quiet
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs --dest-dir ../target/docs-site
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

Also run repository spelling, text-hygiene, whitespace, and mojibake checks required by the autopilot protocol.
