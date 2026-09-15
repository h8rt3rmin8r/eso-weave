# Quickstart: Validate Persistent Auto Potion Request

## Focused red-green loop

```powershell
cargo test --locked --test app_session_state
cargo test --locked config::state::tests
```

## Full Rust validation

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave
```

## Documentation validation

```powershell
typos docs/src docs/README.md README.md specs/097-auto-potion-persistence
node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
node .github/scripts/docs-render-smoke.mjs
```

## Repository integrity

```powershell
& .\.specify\scripts\powershell\check-prerequisites.ps1 -Json -RequireTasks -IncludeTasks
git diff --check
```

Review changed text as UTF-8 without BOM or forbidden dash characters. Confirm
the configuration model still has no Auto Potion request and the diff contains
no Pixel Bus, addon, input backend, or trigger-rule changes.

## Packaged restart smoke

1. Start a release build with no prior `auto_potion` state and confirm Off.
2. Enable Auto Potion, close normally, restart, and confirm the switch remains
   requested while effective status is dormant or blocked until evidence is
   ready.
3. Disable Auto Potion, close normally, restart, and confirm Off.
4. Confirm neither restart submits input before all normal gates succeed.
