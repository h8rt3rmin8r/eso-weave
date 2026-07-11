# Quickstart: Input Engine

Build and validation guide. Implementation lives in `src/input/` and `tasks.md`.

## Build and quality gate

```sh
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all succeed with zero failures and zero warnings on the host (Windows)
target.

## Linux backend type-check (where available)

```sh
rustup target add x86_64-unknown-linux-gnu
cargo check --target x86_64-unknown-linux-gnu
```

This type-checks the Linux backend without a Linux host. Its runtime behavior is
validated manually on Linux (evdev grab and uinput require the input group or a
udev rule).

## Validation scenarios

These map to the spec user stories and are covered by `tests/input_engine.rs`
using `MockBackend`. See `contracts/input-backend.md` and `data-model.md`.

1. Focused interception (US1, SC-001): with focus set, a bound key-down is
   suppressed and produces exactly one handed-off action; an unbound key passes
   through and hands off nothing.
2. Unfocused pass-through (US1, SC-002): with focus cleared, no key is suppressed
   or handed off.
3. Non-blocking hand-off (US1, SC-004): classification only reads state, looks up
   the binding, updates held state, and does one non-blocking send; a full channel
   drops with a warning and never blocks.
4. Recursion breaking (US2, SC-003): a self-originated event for a bound key while
   focused is not suppressed and not handed off; a subsequent real press of the
   same key is still intercepted.
5. Key transitions (US1): a bound key suppresses both down and up; auto-repeat
   downs suppress without additional hand-offs.
6. Suspend (US3, SC-005): while suspended, non-exempt bound keys pass through and
   suspend-exempt keys remain intercepted; resuming restores interception.
7. Bindings (US4, SC-006): defaults match section 6.4; a rebind to a free key is
   accepted and persists; a rebind that collides is rejected leaving both
   unchanged; a persisted conflicting or unknown entry falls back to defaults with
   a notice.
