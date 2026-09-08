# Documentation Migration Ledger

The machine-readable [migration ledger](migration-ledger.json) freezes the
documentation corpus at commit
`bdc7b228b78ef535d07357e7b33a36bbade917cc`. It is the preservation authority
for S058.

## Frozen Baseline

| Inventory | Count |
| --- | ---: |
| Files beneath `docs/` | 49 |
| Orphaned website article | 1 |
| Total artifacts | 50 |
| Technical-specification H2 units | 20 |
| Legacy build plans | 27 |
| Safety invariants | 6 |

Each artifact row records its original Git blob, disposition, destination,
authority, and preservation evidence. Specification units may map to multiple
canonical destinations. Their normalized required excerpts, plus the six safety
excerpts, make removal of a named destination's substantive contract fail policy.
The suite compares inventory rows with an independently frozen path-and-blob
manifest, so changing a count cannot hide an omission or substitution.

## Lifecycle Result

- Published and future bundled documentation lives only under `docs/src/`.
- Current maintainer records live under `docs/project/`.
- Plans 001 through 027 and the website announcement live under `docs/archive/`.
- The post-baseline lifecycle preserves every completed plan with concrete
  delivery evidence and names plan 031 as the sole active S061 entry. The policy
  requires the matching plan file and index row for every state and rejects
  simultaneous current and archive copies.
- The old monolithic specification is removed only after all 20 units and six
  safety invariants have named canonical destinations.

No legacy plan is deleted. All 27 are Complete and Archived. Plan 008 corrected
the bite signal delivered during plan 007, but plan 007 remains useful history.
Ultimate field verification remains independently tracked by issue #77 and does
not reopen plans 025 or 026.

## Historical Exceptions

Live references use the new paths. A retired literal may remain only when the
JSON ledger names the exact file, literal, occurrence count, and provenance
reason. Current exceptions are limited to release-era statements in
`CHANGELOG.md`; the exception mechanism accepts no wildcard.

## Validation

Run the fixture suite and complete documentation policy from the repository
root:

```text
node --test .github/scripts/docs-policy.test.mjs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

The checks reject incomplete or duplicate inventory, erased preservation
excerpts, unsafe deletion, dangling destinations, unshaped delivery evidence,
undeclared plan lifecycle state, publication or search-boundary leaks, stale live
paths, inexact history exceptions, invalid text encoding or punctuation, and a
root README longer than 120 lines.
