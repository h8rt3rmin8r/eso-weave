# Migration Ledger Contract

The authoritative machine-readable ledger is
`docs/project/migration-ledger.json`. A readable summary and lifecycle guide live
beside it.

## Required cardinalities

- `baselineCommit`: exactly `bdc7b228b78ef535d07357e7b33a36bbade917cc`
- `documents`: 50 unique baseline paths
- `specificationUnits`: 20 unique H2 units in source order
- `plans`: 27 unique IDs, `001` through `027`
- `safetyCrosswalk`: the six constitutional safety invariants
- `postBaselinePlans`: contiguous plan lifecycle entries beginning at 028, with
  at most one Active entry

## Disposition rules

- Retain: source and destination may be identical.
- Move: one exact destination exists and the source is absent.
- Split: every destination exists and the source is absent.
- Archive: one destination under `docs/archive` exists and the source is absent.
- Delete: source is absent, replacement exists, and evidence explains why no
  authority was lost.

Unknown dispositions, empty evidence, duplicate rows, and dangling destinations
are invalid.

Specification units may map to multiple destinations. Every mapping and safety
crosswalk row carries a normalized required excerpt, and the complete excerpt
inventory is independently frozen by the policy checker. Plan completion
evidence must identify a pull request, commit, closed issue, or release.

Post-baseline Active entries require an issue, spec package, current plan file,
and Active index row. Archived entries require a substantive archive file, a
Complete and Archived index row, and concrete delivery evidence. A later entry
may become Active after all earlier entries are archived.

## History rule

Historical reference exceptions identify exact file and literal pairs. They do
not exempt other occurrences of that literal from the stale-live-reference gate.
