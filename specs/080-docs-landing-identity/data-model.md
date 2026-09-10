# Data Model: Documentation Landing Identity

## LandingIdentity

- `banner_source`: canonical repository path `assets/eso-weave-banner.png`
- `published_banner`: mdBook path `docs/src/assets/brand/eso-weave-banner.png`
- `banner_bytes`: byte-for-byte equal to `banner_source`
- `visible_heading`: `Documentation`
- `accessible_heading`: `ESO Weave Documentation`
- `image_alternative`: empty because the heading is the text equivalent

## DocumentationSnapshot

- `handle`: exact `Cargo.toml` package name
- `applies_to`: package version rendered with a `v` prefix
- `release_date`: ISO date from the matching changelog release heading
- `repository_url`: exact `Cargo.toml` package repository and clickable destination
- `disclosure`: visible statement that values are a build-time snapshot

## MetadataAuthority

| Displayed field | Authority | Extraction rule |
| --- | --- | --- |
| Handle | `Cargo.toml` | `[package].name` string |
| Applies to | `Cargo.toml` | `[package].version` string, displayed as `vX.Y.Z` |
| Repository | `Cargo.toml` | `[package].repository` URL |
| Released | `CHANGELOG.md` | `## [X.Y.Z] - YYYY-MM-DD` for package version |

## Relationships

- One `LandingIdentity` owns one local `DocumentationSnapshot` presentation.
- One snapshot must equal exactly one set of extracted authorities.
- Public Pages and bundled offline output both derive from the same landing page,
  banner asset, and stylesheet.

## Invariants

- No displayed metadata value is fetched at runtime.
- No second repository metadata authority is introduced.
- The banner copy is identical to the approved source bytes.
- The identity has exactly one H1 and no duplicate visible product name.
- The metadata appears before introductory product prose.
- Missing release-date authority is a policy error, not an inferred date.
