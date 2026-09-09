# Quickstart: Build and Inspect a Catalog

## Build the reviewed live fixture

```powershell
cargo run --locked --bin catalog-compiler -- build --input specs/070-catalog-compiler/fixtures/minimal-live.json --output target/catalog/live/catalog.sqlite --report target/catalog/live/build-report.json --channel live
```

The command validates the bounded JSON input, builds a sibling candidate,
checks integrity, foreign keys, schema and semantic checksum through a read-only
reopen, records a diff from any existing destination, preserves that destination
as the rollback artifact, then atomically publishes the candidate.

## Verify a catalog

```powershell
cargo run --locked --bin catalog-compiler -- verify --catalog target/catalog/live/catalog.sqlite
```

The output is stable JSON containing release identity, schema version, content
hash, integrity status, and grouped entity counts.

## Compare two catalogs

```powershell
cargo run --locked --bin catalog-compiler -- diff --old ROLLBACK_FROM_MANIFEST.sqlite --new target/catalog/live/catalog.sqlite --output target/catalog/live/diff.json
```

Read the exact content-addressed rollback path from
`target/catalog/live/catalog.sqlite.rollback.json`. The diff names added,
removed, and changed entity, localized-text, relation, coverage, and
icon-reference keys in stable order.

## Build PTS separately

```powershell
cargo run --locked --bin catalog-compiler -- build --input specs/070-catalog-compiler/fixtures/minimal-pts.json --output target/catalog/pts/catalog.sqlite --report target/catalog/pts/build-report.json --channel pts
```

The explicit channel must match the bundle. Never reuse the live output path for
a PTS candidate.

## Regenerate the packaged baseline

```powershell
cargo run --locked --bin catalog-compiler -- build --input assets/catalog/baseline.json --output assets/catalog/catalog.sqlite --report target/catalog/baseline-report.json --channel live
```

Review the report, the semantic diff, and the rights audit before committing the
generated database. The baseline intentionally contains Unknown coverage,
project-authored metadata, and no third-party graphics or user-collected prose.

## Roll back manually

Stop ESO Weave, verify the rollback manifest and SHA-256, then copy the named
rollback database over the destination. Reopen through the `verify` command
before launching the application. Automatic user update orchestration belongs
to issue #118 and is outside S070.
