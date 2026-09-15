# Quickstart: Verify S100

Run from the repository root.

## Focused contract checks

```powershell
cargo test --locked --test native_bindings
cargo test --locked --test pixelbus native_binding
cargo test --locked --test beacon binding
```

These checks cover the portable model, every modifier mask, fixed RGB encoding, malformed and transposed cells, legacy compatibility, addon publisher behavior, and the no-mutation policy.

## Complete repository gates

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
cargo build --release --locked
node .github/scripts/docs-policy.mjs
node .github/scripts/trust-policy.mjs
node .github/scripts/check-encoding.mjs
node .github/scripts/check-spelling.mjs
node .github/scripts/check-links.mjs
mdbook test docs
mdbook build docs
```

## Spec-kit gate

```powershell
& .specify/scripts/powershell/check-prerequisites.ps1 -Json -RequireTasks -IncludeTasks
```

Confirm `analysis.md` contains no unresolved finding and all implementation tasks are checked.

## Manual in-client follow-up

After installing PixelBeacon 22, verify one action at a time with a default key, a rebound keyboard key, a modifier chord, a mouse button, no binding, duplicate identical slots, two distinct slots, and an unsupported gamepad binding. S100 exposes evidence only, so automation behavior and desktop settings must remain unchanged.
