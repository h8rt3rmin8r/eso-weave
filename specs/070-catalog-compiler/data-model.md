# Data Model: Deterministic SQLite Catalog

## CatalogBundle input

The compiler accepts one JSON object no larger than 64 MiB. Arrays are bounded
to a combined 500,000 records and strings to 64 KiB before database insertion.
Unknown fields are rejected.

| Field | Type | Rule |
| --- | --- | --- |
| `schema_version` | integer | Exact value 1 |
| `release` | CatalogReleaseInput | One immutable live or PTS identity |
| `source_snapshots` | array | At least one immutable source |
| `source_records` | array | Content-hashed records referencing snapshots |
| `coverage` | array | Every S068 category appears once or is synthesized as Unknown |
| `entities` | array | Stable positive numeric IDs and constrained kinds |
| `attributes` | array | Typed values, one authoritative value per entity/name |
| `relations` | array | Typed edges between known entities |
| `aliases` | array | Alias or supersession edges between known entities |
| `localized_text` | array | Locale, kind, value, source version and provenance |
| `icon_references` | array | Virtual path metadata only |
| `icon_assets` | array | Hash and transformation metadata, never implicit bytes |

## Durable database tables

### `catalog_release`

Exactly one row. It records schema and catalog versions, channel, game and API
versions, explicit creation time, locales, tool version, input hash, source-set
hash, and the semantic content SHA-256. The content hash column itself is
excluded from the canonical projection to avoid recursion.

### `schema_migration`

Records every applied forward migration. Schema 1 contains one row. SQLite
`user_version` must equal the newest migration and the application-supported
schema.

### `source_snapshot`

Primary key `snapshot_id`. Includes family, channel, game/API version, locale,
immutable revision, raw SHA-256, URI, acquired time, license scope, acquisition
method, and redistribution class. Recommended file sources require a 64-digit
lowercase SHA-256.

### `source_record`

Primary key `record_id`, foreign key to `source_snapshot`. Includes category,
source key, canonical content SHA-256, acquisition method, and import result.

### `coverage`

Composite key `(category, snapshot_id, locale, scope)`. Completeness is one of
Exhaustive, Bounded, Opportunistic, or Unknown. `limits_text` is mandatory for
every value except a proven Exhaustive claim. Compiler validation prevents a
claim stronger than the matching S068 source category.

### `entity`

Composite key `(kind, stable_id, channel, api_version)`. IDs are positive signed
64-bit values. `kind` comes from the complete catalog vocabulary.
`observed_only` preserves unknown observations without pretending that metadata
exists.

### `entity_source`

Many-to-many provenance between entities and source records. Every entity must
have at least one row before commit.

### `entity_attribute`

Composite key `(entity identity, name)`. Exactly one of integer, real, text,
boolean, or canonical JSON value is present. Each attribute references the
source record that supplied it. Iterator positions require an explicit
`version-scoped-order` name.

### `entity_relation`

Composite key `(relation_kind, from identity, to identity)`. Both ends and the
source record must exist. The relation vocabulary includes the S068 and S069
ability, skill-line, progression, morph, effect-source, set membership, pet
owner kind, crafted/perfected, and category relationships.

### `entity_alias`

Composite key `(alias_kind, from identity, to identity)`. It preserves renamed,
morphed, perfected, retired, and version-superseded identities without changing
the original stable ID.

### `localized_text`

Composite key `(entity identity, locale, text_kind)`. Includes source version,
optional normalized search value, redistribution class, and source record.
Identity never depends on localized text.

### `icon_reference`

Composite key `(entity identity, virtual_path)`. Includes availability and
source provenance. A virtual path never implies local bytes.

### `icon_asset`

Composite key `(content_sha256, transformation_id)`. Includes media type,
dimensions, origin, attribution, availability, and redistribution class. S070
stores metadata only and the baseline contains no third-party asset row.

## Typed runtime model

`CatalogAccess` is either Available with a read-only `CatalogReader` or Empty
with a `CatalogDiagnostic`. `CatalogReader` exposes `release()` and
`entity(kind, stable_id)` plus typed wrappers such as `ability(stable_id)`.
Connections never expose mutable access or SQL strings to consumers.

## Invariants

1. All semantic tables use primary keys and foreign keys with no orphan rows.
2. Entity channel and API version equal the single catalog release.
3. Every fact and relationship has source-record provenance.
4. Canonical projection ordering is explicit for every table.
5. Input wall-clock reads, unordered map iteration, and SQLite-generated
   timestamps are prohibited.
6. Live and PTS are immutable provenance classes, never aliases.
7. Unknown coverage and observed-only entities remain queryable.
8. Runtime checksum verification covers every semantic table.
9. Schema migration is forward-only; runtime rejects any other `user_version`.
10. The distributable baseline contains no prohibited art or user-owned text.
