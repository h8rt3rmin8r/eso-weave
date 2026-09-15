# Quickstart: Verify Stale HUD Retention

## Automated verification

```powershell
cargo test --test app_settings --locked
cargo test --test app_view_model --locked
cargo test --test app_strings --locked
cargo test --test app_ui_sizing --locked
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave
```

Run the repository documentation policy, render-smoke, mdBook, spelling, spec prerequisite, diff, encoding, and forbidden-dash gates after the focused tests.

## Manual interface pass

1. Open Settings and locate `Stale retention` under Appearance.
2. Enter `0`, `120`, and `999` directly, use increment and decrement adjustment, and confirm the value remains within bounds.
3. With a positive interval, establish visible resources and player-state values in ESO.
4. Alt-tab to ESO Weave. Confirm the values remain readable and a visible `HUD Freshness` row states that focus was lost and reports age.
5. Return focus before expiry. Confirm the stale row disappears and newly observed values replace the snapshot.
6. Repeat for ESO process inactivity and PixelBeacon signal loss.
7. Wait through a short configured interval and confirm the existing dormant or unavailable values replace the retained snapshot.
8. Set the interval to zero and confirm every covered loss clears immediately.
9. During every stale case, confirm weaving passes through or blocks as designed, Fishing stops or pauses, and Auto Potion remains dormant or blocked.
10. Restart ESO Weave and confirm no live player-state values are restored.
