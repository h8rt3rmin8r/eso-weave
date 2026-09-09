# Data Model: Bounded ESO Discovery Exporter

## CollectorEnvelope

- `schema_version`: fixed `1`
- `collector_version`: positive integer
- `collector_checksum`: lowercase SHA-256 of embedded collector source with the
  checksum literal normalized to 64 zeroes
- `status`: `complete`, `paused`, `cancelled`, or `failed`
- `channel`: `live` or `pts`
- `game_version`, `api_version`, `locale`, `platform`, `megaserver`
- `scope_key`: pseudonymous constant or user-approved opaque value
- `started_at`, `finished_at`: game-provided timestamps
- `selected_categories`: unique supported category names
- `coverage`: category-keyed bounded declarations
- `warnings`: bounded strings
- `cancellation_reason`: optional bounded string
- `checkpoint`: current adapter and cursor without completion authority
- `chunks`: ordered `CollectorChunk` values

Only `complete` envelopes may enter staging. Other states remain parseable
evidence for resume, retry, and diagnostics.

## CollectorChunk

- `sequence`: contiguous one-based integer
- `record_count`: number of LF-delimited JSON records
- `byte_count`: UTF-8 payload length
- `checksum`: eight-character lowercase Adler-32
- `payload`: canonical JSON records separated by LF, with no trailing LF

## CollectorRecord

- `category`: one of the five S071 categories
- `kind`: an S070 entity kind allowed for that category
- `stable_id`: positive integer
- `source_key`: deterministic category/kind/id identity
- `parent`: optional typed stable entity and relation name
- `attributes`: sorted scalar attributes, including source indexes only as data
- `name`, `description`: optional local strings
- `icon_path`: optional virtual path, never bytes

Record order is `(category, kind, stable_id, source_key)`. Duplicate source keys
or conflicting typed entity identities are invalid.

## CoverageDeclaration

- `category`: supported category
- `completeness`: fixed `bounded`
- `scope`: non-empty visibility description
- `record_count`: exact accepted record count
- `limitations`: non-empty reason global exhaustiveness is not claimed

## Staged CatalogBundle Mapping

- Envelope provenance becomes one `SourceSnapshotInput` with family `collector`.
- Each record becomes one `SourceRecordInput` hashed over canonical JSON.
- Typed identities become `EntityInput` values and source associations merge.
- Parents become `RelationInput` values.
- Scalar attributes become `AttributeInput` values.
- Local names/descriptions become `LocalizedTextInput` with redistribution
  `user-generated-only`.
- Virtual paths become reference-only `IconReferenceInput` values.
- Selected category declarations become `CoverageInput`; all other S070
  categories become `Unknown` for the same snapshot.
- No icon assets or aliases are manufactured.

## ImportReceipt

- status, capture SHA-256, staged SHA-256
- schema and collector versions
- channel, game/API versions, locale, scope key
- record and chunk counts
- selected categories, warnings, and coverage summary

The receipt is returned to the caller and contains no input/output path or
personal account/character name.

## State Transitions

```text
idle -> running -> complete
          |  |\-> failed
          |  \-> cancelled
          \----> paused -> running
```

Only `running -> complete` finalizes sorted chunks and exact coverage counts.
No desktop state transition changes the active SQLite catalog.
