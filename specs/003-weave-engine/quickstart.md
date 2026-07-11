# Quickstart: Weave Engine

Build and validation guide. Implementation lives in `src/weave/` and `tasks.md`.

## Build and quality gate

```sh
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all succeed on the host. The Linux mouse-synthesis additions are
type-checked with `cargo check --target x86_64-unknown-linux-gnu`.

## Validation scenarios

Covered by `tests/weave_sequence.rs` and `tests/weave_engine.rs` using the
`MockSink` and virtual clock. See `contracts/weave-engine.md` and `data-model.md`.

1. Sequence correctness (US1, SC-001): for each weave type, `sequence_for` returns
   exactly the specified ordered steps (primary and secondary mouse, key, and the
   correct waits at the correct points).
2. Per-slot overrides (US4, SC-004): a slot with a d_weave override emits that
   value at the relevant wait; a slot without one uses the global default.
3. Cooldown gating (US2, SC-002, SC-006): with the virtual clock, a second action
   inside global_cooldown runs no sequence; after advancing past the window, the
   next action runs its sequence.
4. Action mapping (US1): each skill action maps to its slot; the suspend and
   fishing toggle actions run no weave.
5. Inactive pass-through (US3, SC-003): with a slot inactive, the Input Engine
   `classify` passes the key through (no suppression, no hand-off); activating the
   slot restores interception.
6. Persistence (US4, SC-005): slot and timing configuration saved to a temp config
   dir round-trips through load unchanged; a missing or out-of-range timing value
   falls back to the default with a notice.
