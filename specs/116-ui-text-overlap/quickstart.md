# Quickstart: Verify Collision-Safe Status Rows

## Automated

```powershell
cargo test --test app_ui_sizing
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave
git diff --check
```

## Visual evidence

At the reported Windows layout, expand System and State and open Data Details.
Confirm every title remains separate from its marker, value, and actions. Repeat
with both themes and enlarged text, then attach before and after captures to the
pull request. Installed verification stays open until a fixed release is tested.
