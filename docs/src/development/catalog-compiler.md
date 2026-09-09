# Catalog Compiler and Runtime

ESO Weave ships a minimal immutable SQLite catalog and provides a deliberate
maintainer command for producing later versions. Catalog construction never runs
from `build.rs`, ordinary compilation, application startup, or a network source.

The repository baseline proves the schema, verification, runtime, and package
path. It contains project-authored metadata, 15 explicit Unknown coverage rows,
no game entities, no user-collected localized prose, and no third-party image
bytes. Later reviewed sources can add normalized facts without changing this
rights boundary.

## Build contract

The compiler accepts a strict normalized JSON bundle. It rejects unknown fields,
inputs over 64 MiB, more than 500,000 combined records, strings over 64 KiB,
duplicate stable identities, orphan relationships, unsupported completeness
claims, invalid hashes, and mismatched live or PTS channels. Input is data only:
no Lua, script, hook, expression, network request, or source-provided path is
executed.

Run a live build explicitly:

```console
cargo run --locked --bin catalog-compiler -- build --input INPUT.json --output target/catalog/live/catalog.sqlite --report target/catalog/live/build-report.json --channel live
```

PTS uses a separate output and `--channel pts`. A PTS bundle cannot be published
through a live request.

The compiler creates the complete schema and rows in one transaction, applies
foreign keys and constraints, calculates the semantic SHA-256 over a canonical
ordered projection, closes and syncs the candidate, then reopens it read-only.
Publication requires schema compatibility, `integrity_check`,
`foreign_key_check`, and semantic checksum agreement.

## Determinism and review evidence

The semantic SHA-256 is the cross-platform content authority. It includes every
semantic table and excludes only its own stored checksum cell. The artifact
SHA-256 identifies exact SQLite file bytes. Identical inputs are byte-identical
under the locked bundled SQLite toolchain, but reviewers should compare semantic
hashes when SQLite versions differ.

Each build report records:

- input, source-set, semantic, and artifact SHA-256 values;
- schema, catalog, game, API, and channel identities;
- integrity and foreign-key results;
- grouped record counts and min/max stable IDs; and
- stable added, removed, and changed keys for entities, localized text,
  relations, coverage, and icon references.

Verify an artifact without changing it:

```console
cargo run --locked --bin catalog-compiler -- verify --catalog catalog/catalog.sqlite
```

Compare two verified artifacts:

```console
cargo run --locked --bin catalog-compiler -- diff --old OLD.sqlite --new NEW.sqlite --output diff.json
```

Approval consists of reviewing the immutable sources, coverage truth, compiler
report, semantic diff, rights classification, and exact artifact hash before the
database enters a package.

## Publication and rollback

A candidate is built in the destination directory and never modifies the open or
existing database. Before replacement, a valid destination is copied to
`catalog.sqlite.rollback`, synced, hashed, and described by
`catalog.sqlite.rollback.json`. The verified candidate then replaces the
destination atomically. A parse, constraint, verification, sync, backup, or
replacement failure leaves the destination unchanged and writes a failure report
when `--report` was supplied.

Stop the application before a manual rollback. Confirm the manifest hash, copy
the rollback file over the destination, and run `verify` before restarting. User
update orchestration remains future issue #118 work.

## Schema and provenance

Schema version 1 uses a normalized entity, attribute, relation, and alias core.
The constrained entity-kind vocabulary covers skills, abilities, progressions,
effects, gear, sets, traits, enchantments, champion skills, Mundus effects,
consumables, classes, races, companions, and API constants. This avoids freezing
empty guessed tables before approved source records establish specialized
shapes.

Every fact resolves through a source record to an immutable source snapshot.
Entity identity combines kind, stable numeric source ID, channel, and API
version. Iterator positions can appear only as a version-scoped ordering
attribute. Unknown observed IDs remain valid entities, and later catalogs can
resolve them without changing raw encounter data.

Coverage is explicit per category, snapshot, locale, and scope. The compiler
cannot strengthen a claim beyond the S068 source contract and fills absent
categories with Unknown coverage.

## Runtime and package paths

The application opens one version-bound read-only handle at startup. UI code
receives typed release and entity results, never SQL strings. Missing, corrupt,
checksum-invalid, or newer-schema databases yield an empty catalog service, a
Catalog warning in System and State, and a warning log. Unrelated features keep
working.

The packaged relative path is `catalog/catalog.sqlite`:

- MSI: beside `eso-weave.exe` under the installation directory;
- portable Linux archive: beside the executable under its archive root;
- Debian: `/usr/share/eso-weave/catalog/catalog.sqlite`; and
- AppImage: `usr/share/eso-weave/catalog/catalog.sqlite` inside AppDir.

An open handle is never swapped in place. A controlled application restart or a
future explicit reopen boundary observes a newly published version.
