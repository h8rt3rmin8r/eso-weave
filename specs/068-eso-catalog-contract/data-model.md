# Data Model: ESO Catalog Source Contract

## SourceSnapshot

| Field | Type | Rule |
| --- | --- | --- |
| `id` | string | Stable repository identifier |
| `family` | enum | `zos-api`, `stock-ui`, `collector`, `community-code`, `archive`, or `remote` |
| `channel` | enum | `live`, `pts`, or `not-applicable` |
| `game_version` | string | Required for live and PTS evidence |
| `api_version` | integer | Required for public addon API evidence |
| `locale` | string | BCP 47 or `all` when source text is not localized |
| `revision` | string | Immutable commit, attachment identity, or version |
| `sha256` | string | Required for recommended file inputs |
| `uri` | string | Immutable evidence location where possible |
| `acquired_at` | date | Evidence refresh date |
| `license_scope` | string | Technical code, content data, art, or none proven |

Moving branch names are never revisions. A source may be a freshness signal without being a recommended compiler input.

## CatalogCategory

| Field | Type | Rule |
| --- | --- | --- |
| `id` | string | Stable contract category |
| `category` | string | Human-readable content group |
| `stable_key` | string | ID or explicit composite, never an iterator index |
| `enumeration_method` | enum | `api-iterator`, `known-id`, `observed-event`, `item-link`, `constant-file`, `local-file`, or `none` |
| `visibility` | array | Global, client, account, character, unlock, locale, channel, encounter, or local-user |
| `completeness` | enum | Exhaustive, bounded, opportunistic, or unknown |
| `source_ids` | array | At least one SourceSnapshot |
| `redistribution` | enum | Allowed, attribution-required, user-generated-only, prohibited, or unresolved |
| `failure_modes` | array | Missing, stale, partial, renamed, aliased, inaccessible, corrupt, or retired |
| `validation` | array | Count, range, hash, relationship, or uniqueness checks |
| `field_verification` | string | Required when claims need game access |

## PromotionReceipt

| Field | Type | Rule |
| --- | --- | --- |
| `from_snapshot` | string | Must be PTS |
| `to_api_version` | integer | Must name a live API |
| `live_revision` | string | Immutable live evidence |
| `live_sha256` | string | Hash of the promoted or replacement source |
| `normalized_content_sha256` | string | Allows equality comparison without conflating provenance |
| `reviewer` | string | Human approval identity |
| `reviewed_at` | timestamp | Required and immutable |
| `decision` | enum | Replace, accept-equal-content, or reject |

No receipt can mutate or relabel the original PTS snapshot.

## CollectorEnvelope

| Field | Type | Rule |
| --- | --- | --- |
| `schema_version` | integer | Exact supported version |
| `snapshot_id` | string | Unique content-addressed import identity |
| `game_version` | string | Required |
| `api_version` | integer | Required |
| `channel` | enum | Live or PTS |
| `locale` | string | Required |
| `platform` | string | Required when it changes identifiers or visibility |
| `megaserver` | string | Required when account scope is material |
| `character_scope` | string | Pseudonymous local identifier or `none` |
| `acquired_at` | timestamp | Collector observation time |
| `content_sha256` | string | Hash over canonical record bytes |
| `records` | array | Bounded declarative objects only |

The parser accepts data tokens only. It rejects functions, metatables, computed
expressions, references, and executable Lua. Limits apply before allocation and
the last known-good import is replaced only after complete validation.

## Relationships

- CatalogCategory references one or more SourceSnapshots.
- CoverageClaim is represented by each category's completeness plus visibility and exact snapshots.
- PromotionReceipt relates a PTS SourceSnapshot to independent live evidence.
- CollectorEnvelope can add bounded or opportunistic observations but cannot silently upgrade a CatalogCategory's completeness.
- RedistributionDecision applies independently to identifiers, text records, virtual icon paths, and icon bytes.
