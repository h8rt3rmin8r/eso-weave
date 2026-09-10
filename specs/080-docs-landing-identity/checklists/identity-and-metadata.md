# Identity and Metadata Checklist: Documentation Landing Identity

**Purpose**: Protect accessible project identity and authoritative static metadata
**Created**: 2026-09-10
**Feature**: [spec.md](../spec.md)

## Identity

- [x] Full-color banner is the required primary visual
- [x] Square and monochrome substitutes are excluded
- [x] Visible product-name repetition is prohibited
- [x] Accessible H1 retains the full page name
- [x] Decorative image treatment avoids duplicate announcement

## Metadata

- [x] Handle authority is `Cargo.toml` package name
- [x] Applicability authority is `Cargo.toml` package version
- [x] Repository authority is `Cargo.toml` package repository
- [x] Release-date authority is the matching dated changelog heading
- [x] Snapshot and update semantics are disclosed in visible prose

## Delivery

- [x] Asset remains local to the mdBook source
- [x] Public and bundled builds use the same source and asset
- [x] Narrow and wide layout obligations are measurable
- [x] No remote query, runtime behavior, or release action is introduced

## Notes

The domain checklist passes. Implementation planning may use the selected contract.
