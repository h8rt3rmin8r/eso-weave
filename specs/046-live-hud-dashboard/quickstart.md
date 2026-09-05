# Quickstart: Validate S046

## Focused model checks

```powershell
cargo test --test app_view_model resource
cargo test --test app_view_model model_projects_runtime
```

Expected: percent boundaries, configured Low, Dormant, Unavailable, and beacon
signal states are exhaustive and truthful.

## Rendered-frame checks

```powershell
cargo test --test app_ui_sizing dashboard
cargo test --test app_ui_sizing log_pane
```

Expected: the narrow view stacks, the wide view uses columns, meter geometry is
stable, and the log never overlaps dashboard or Skills.

## Full merge gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

## Text hygiene

```powershell
$forbiddenText = [string][char]0x2014 + '|' + [string][char]0x2013 + '|' + [string][char]0xfffd
rg -n $forbiddenText specs/046-live-hud-dashboard src/app tests README.md CHANGELOG.md docs
```

Expected: no output from the hygiene scan.

## Maintainer review

Inspect the pull-request screenshots or local release build at normal, narrow,
and high-DPI scale. Confirm the hierarchy reads as a compact HUD, Skills is
unchanged, the appropriate addon action is visually primary, and no state relies
on color alone.
