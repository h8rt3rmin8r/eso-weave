# Quickstart: Persistent Data Addon Foundation

## Prerequisites

- Rust toolchain from `rust-toolchain.toml`
- Repository checkout on `codex/s092-data-addon-foundation`
- No live ESO account or custom binding experiment is required

## Package and isolation validation

1. Run the data-addon lifecycle, Lua harness, collector, and encounter tests.
2. Verify the repository addon directory contains only `PixelBeacon` and
   `EsoWeaveData`.
3. Exercise catalog start, combat pause, resume, cancel, status, and clear.
4. Exercise encounter arm, disarm, capture, stop, status, interruption recovery,
   overflow, and clear.
5. In both orders, retain one module's terminal state while clearing the other.
6. Exercise install, update, failed update rollback, unmanaged targets, linked
   targets, foreign entries, and uninstall while checking neighboring bytes.

Expected outcome: both modules preserve their existing behavior, idle modules
perform no high-frequency work, clearing is subtree-local, and every lifecycle
mutation is confined to the marker-owned `EsoWeaveData` directory.

## Decision validation

1. Review [the ingestion contract](contracts/ingestion-transport-decision.md)
   against every source cited in [research.md](research.md).
2. Confirm every unmeasured Windows or Linux/Proton row remains provisional and
   belongs to the separate verification issue.
3. Review [the command decision](contracts/command-transport-decision.md) and
   confirm that source, addon manifest, and input code introduce no custom action
   or command transport.

## CI parity

Run in the foreground:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Then run the repository documentation policy and mojibake checks used by CI.

Expected outcome: every command exits successfully, all S092 checklists are
complete, and `tasks.md` contains no unfinished task.
