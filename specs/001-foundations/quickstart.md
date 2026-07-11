# Quickstart: Foundations

How to build the crate and validate the feature end to end. This is a run and
validation guide; implementation lives in the source tree and `tasks.md`.

## Prerequisites

- The pinned toolchain from `rust-toolchain.toml` (Rust 1.96.0 with `rustfmt` and
  `clippy`). `rustup` selects it automatically in the repo root.

## Build and quality gate

```sh
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all four succeed with zero failures and zero warnings (SC-001).

## Run the binary

```sh
cargo run
```

Expected: the process initializes logging, loads settings (creating defaults in
memory if none exist), emits an INFO startup event, and exits 0. With the default
settings the file sink is off, so no log file is written yet.

## Validation scenarios

These map to the spec user stories and are covered by the integration tests under
`tests/`. See `contracts/public-api.md` for signatures and `data-model.md` for
shapes.

1. Settings round-trip (US1, SC-002): save a Settings value to a temporary config
   directory, load it back, and confirm equality. Confirm the file on disk has no
   byte order mark, LF endings, and a trailing newline.
2. Corruption fallback (US1, SC-003): write an invalid settings file, load, and
   confirm defaults are returned, a `.invalid` file now exists (with a numeric
   discriminator if one already existed), and a corrupt_config notice is present.
3. Forward migration and unknown keys (US1): load a file with an older
   schema_version and an unrecognized top-level key; confirm a migrated notice and
   an unknown_keys notice, and that recognized settings survive.
4. Runtime level change (US2, SC-004): initialize logging at INFO, emit a DEBUG
   event and confirm it is absent, raise the level to DEBUG, emit again, and
   confirm it is now captured.
5. Ring buffer independence (US2, SC-006): with the file sink disabled, emit events
   and confirm `recent()` returns them; confirm capacity eviction keeps only the
   most recent when the buffer overflows.
6. Monthly file sink (US2): enable the file sink against a temporary log directory,
   emit an event, and confirm a `YYYY-MM.log` file appears with a well-formed line
   (UTC RFC-3339 timestamp, level, target, message).
7. Privacy (US2, SC-005): emit an event that models input-related activity at INFO
   and confirm the specific input content is absent from the captured event.
8. Quality gate (US3, SC-001): the commands under Build and quality gate all pass on
   a clean checkout.
