# Quickstart: Verify Auto-potion Restoration

**Feature**: [spec.md](spec.md) | **Date**: 2026-09-03

## Automated gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo doc --no-deps --document-private-items
```

Confirm tests cover every effective state, exact input ordering, retry suppression, deterministic resource cause, signal-loss request preservation, heartbeat recovery, and normalized UI text.

## Headless state matrix

1. Start from requested Off and confirm zero input.
2. Enable the request while game inactive, unfocused, beacon unavailable, suspended, and gated in turn.
3. Exercise no resource watches, all watched readings unavailable, and fresh readings above threshold.
4. Exercise every S042 quickslot classification and potion availability plus Ready, Remaining, and Unknown cooldown.
5. Supply one eligible low resource and verify one Down/Up pair and Triggered cause.
6. Evaluate within the retry interval and verify no further input.
7. Lose and recover the beacon and game while confirming the request remains selected.

## Post-release real-client receipt

After a fresh release containing S043 is available:

1. Install the release and current PixelBeacon addon, then reload the ESO UI.
2. Enable one watched resource and auto-potion with a usable potion selected and cooldown ready.
3. Cross the Health threshold and confirm one attempt, Triggered status, then retry suppression.
4. Repeat independently for Magicka and Stamina.
5. Confirm no input for empty, non-potion, depleted, blocked, cooldown, menu, focus, suspended, and signal-loss cases.
6. Confirm signal recovery preserves the request and permits a later eligible trigger.
7. Attach the chronological receipt to issue #25 and only then evaluate closing it.

Do not claim this receipt from headless tests or before the release is available.
