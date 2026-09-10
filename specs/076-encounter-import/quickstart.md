# Quickstart: S076 Encounter Import Verification

## Prerequisites

- Rust 1.96 toolchain
- A terminal S075 SavedVariables fixture
- A temporary directory outside the repository for the encounter store and
  backup

The production command operates only on an explicit input file and explicit
store path. It does not find game files automatically.

## Import

```powershell
cargo run --bin catalog-compiler -- encounter-import `
  --input .\tests\fixtures\encounter\valid-complete.lua `
  --store $env:TEMP\eso-weave-encounters.sqlite `
  --channel live
```

Expected output is a JSON receipt with `outcome`, source and canonical hashes,
channel, terminal status, identity, and counts. It contains no event payloads.

## List

```powershell
cargo run --bin catalog-compiler -- encounter-list `
  --store $env:TEMP\eso-weave-encounters.sqlite
```

The list is deterministic and contains metadata only.

## Back Up

```powershell
cargo run --bin catalog-compiler -- encounter-backup `
  --store $env:TEMP\eso-weave-encounters.sqlite `
  --output $env:TEMP\eso-weave-encounters.backup.sqlite
```

Verify the reported digest against the final snapshot and run SQLite integrity
checking in the automated test suite.

## Delete Explicitly

```powershell
cargo run --bin catalog-compiler -- encounter-delete `
  --store $env:TEMP\eso-weave-encounters.sqlite `
  --session session-1788912000-1000 `
  --encounter encounter-1788912000-1
```

Use `--all` instead of the two identity arguments only when deliberately clearing
the entire raw store.

## Required Merge Gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Also run the repository mdBook, link, documentation policy, spelling, JSON,
whitespace, UTF-8 without BOM, forbidden-dash, and mojibake checks documented in
the active build plan.
