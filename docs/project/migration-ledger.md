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
  delivery evidence. Plans 028 through 042 are archived after their named pull
  requests or release completed. Plan 044 completed the diagram-legibility
  sequence, Plan 045 completed the local extension surface, Plan 046 completed
  the Ultimate Auto Potion resource watch, and Plan 047 completed the full
  documentation visualization audit. Plan 048 completed all four approved
  figures through merged S112, S121 and S122, preserving the intervening S120
  recovery and unused S113-S115 reservations in its archive. Plan 049 completed
  S123 in PR #256 and v0.17.3 published on 2026-10-07. Plan 050 completed
  S124 dependency maintenance and v0.17.4 candidate preparation in PR #264
  on 2026-10-07. Plan 051 completed S125 data-addon workflow recovery for
  issues #265 through #271 in PR #274 on 2026-10-12. Plan 052 completed
  S126 retained recording sessions / issue #273 in PR #275 on 2026-10-13.
  Plan 053 completed S127 native import / issue #272 in owner-merged PR #276
  on 2026-10-13. Plan 054 completed S128 candidate preparation / issue #279 in owner-merged
  PR #278 on 2026-10-13. Separately authorized v0.18.0 publication completed
  #277 that day. Plan 054 is archived. Plan 055 completed S129 in owner-merged PR #295 on 2026-10-15 and is archived. Plan 056 completed S130, #289/#290/#292-#294, in owner-merged PR #298 on 2026-10-15 and is archived. Plan 057 completed S131/#296/#297 in owner-merged PR #299 on 2026-10-15 at b28d34e and is archived. Plan 058 completed S132/#302 v0.19.0 candidate preparation in owner-merged PR #303 on 2026-10-15 at 629bc92, including #284, and is archived. Plan059 now directs owner-selected S133/#306-#310; the implementation remains unmerged until owner review. Separately authorized v0.19.0 publication and the five-asset integrity audit completed #301 at 20:40:52 UTC; release workflow 38801670245 passed. #304 records documentary closeout.
  Independent release verification does not reactivate a completed plan. The
  policy requires the matching plan file and index row for
  every state and rejects simultaneous current and archive copies.
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
