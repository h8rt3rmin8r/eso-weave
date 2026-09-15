# Quickstart: Native Binding Consumption Verification

## Focused development loop

```powershell
cargo test --locked --test input_engine
cargo test --locked --test weave_sequence
cargo test --locked --test weave_engine
cargo test --locked --test config
cargo test --locked --test app_settings
```

## Full local parity

```powershell
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo build --locked --release
```

Run the repository documentation, trust-policy, spelling, link, encoding, and forbidden-dash gates defined by CI. Hosted CI must compile and test both platforms.

## Manual evidence matrix

1. Verify default and rebound single-key skills with current Attack.
2. Verify modifier plus keyboard and modifier plus mouse triggers.
3. Verify rebound Attack and Block in all four weave modes.
4. Make one requirement unbound or conflicting and confirm pass-through without generation.
5. Hold an extra modifier and confirm pass-through.
6. Lose focus or signal mid-sequence and confirm no generated control remains held.
7. Confirm F1, F2, and F3 remain configurable and independent.
