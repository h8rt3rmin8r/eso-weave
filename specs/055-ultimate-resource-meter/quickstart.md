# Quickstart: Validate S055

## Automated validation

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Focused development commands:

```powershell
cargo test --locked --test pixelbus ultimate
cargo test --locked --test beacon ultimate
cargo test --locked --test weave_engine ultimate
cargo test --locked --test app_view_model ultimate
cargo test --locked --test app_ui_sizing resource_meter
```

## Expected fixtures

1. Publish `current=185`, `maximum=500`, front cost `200`, and back cost `125`.
2. On Front, expect `185/500`, a 40 percent threshold, and no Ready.
3. Swap to Back, expect a 25 percent threshold and green Ready without geometry movement.
4. Corrupt one front-cost byte, expect Front threshold and Ready hidden while Back remains valid.
5. Remove the signal, expect Ultimate unavailable; restore it, expect the same values republished.
6. Render both themes and narrow/wide dashboards; expect four aligned meters and paired cards.

## Live verification boundary

Do not treat repository fixtures as proof of ESO runtime APIs or modifier behavior.
Run the separately tracked verification issue after a build containing S055 is published.
