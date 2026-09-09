# Quickstart: S071 Bounded Discovery Exporter

## 1. Verify the specification gate

```powershell
& .\.specify\scripts\powershell\check-prerequisites.ps1 -Json -RequireTasks
```

Confirm `spec.md`, both checklists, `research.md`, `data-model.md`, contracts,
`plan.md`, and `tasks.md` exist and contain no unresolved clarification.

## 2. Run focused tests

```powershell
cargo test --test collector_lifecycle --locked
cargo test --test collector_import --locked
cargo test --test collector_addon --locked
```

## 3. Exercise the staged import

```powershell
cargo run --locked --bin catalog-compiler -- import-collector --input specs/071-bounded-discovery-exporter/fixtures/live.lua --output target/s071-live.json --channel live --catalog-version local-live-1
cargo run --locked --bin catalog-compiler -- build --input target/s071-live.json --output target/s071-live.sqlite --channel live
cargo run --locked --bin catalog-compiler -- verify --catalog target/s071-live.sqlite
```

The import command must stop at JSON staging. It must not replace an active
catalog. Repeat the import and compare bytes to prove determinism.

## 4. Exercise collector lifecycle in a sandbox

```powershell
cargo run --locked --bin catalog-compiler -- collector-install --addons target/s071-addons --api-version 101050
cargo run --locked --bin catalog-compiler -- collector-status --addons target/s071-addons
cargo run --locked --bin catalog-compiler -- collector-remove --addons target/s071-addons
```

Never point this exercise at a real AddOns directory unless intentionally
testing the managed lifecycle. Removal requires the on-disk managed marker.

## 5. Run the full merge gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
cargo build --release --locked --bin eso-weave --bin catalog-compiler
```

Also run repository documentation, text-hygiene, JSON-schema, and packaging
checks used by CI.
