# Quickstart: Verify Quickslot Observation Reconstruction

**Feature**: [spec.md](spec.md) | **Date**: 2026-09-03

## Automated gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Confirm protocol tests cover every B20 state, corrupt and partial blocks, legacy
samples, signal loss, cooldown transitions, and Lua/Rust agreement.

## Real-client receipt

1. Update PixelBeacon through ESO Weave and run `/reloadui`.
2. Run `/pbquickslot` once with the previously failing selected potion and attach
   the bounded output to issue #24. Name the first old-pipeline predicate whose
   value disproves the previous assumption.
3. Run `/pbquickslot watch` and exercise, chronologically: empty slot, ready
   positive-stack potion, same potion on cooldown, depleted behavior, food or
   another item, collectible, quest item, selected-slot change, contents change,
   reload/player activation, loading screen, keyboard mode, and gamepad mode.
4. Run `/pbquickslot watch` again to disable change receipts.
5. Hide or remove the addon and corrupt one captured block in a deterministic test
   fixture. Confirm both clear any previous Potion/Usable state.
6. Confirm auto-potion emits no input in this slice.

Do not include localized item names or descriptions in the receipt.
