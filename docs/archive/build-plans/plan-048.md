# Plan 048: Approved Documentation Visualizations

Status: Complete, Archived

## Owner-approved Recovery Priority (2026-10-05)

S112's initial figure delivery preceded the independent S116-S119 UI/release
work. The owner now prioritizes S120 (`120-combat-recovery-and-api-compatibility`)
for issues #248, #251, and #250 before the remaining figure sessions below.
Complete the full spec-kit sequence, automated parity, authorized push and
official PR, at most two Codex review rounds, all external findings, and green
CI before owner final review and merge. No live-game checks or release are part
of S120. Repository evidence must remain distinct from unobserved installed
behavior.

After S120, resume documentation planning. Issue #249 and issue #222 are the
proposed next coherent bundle; record any regrouping at that kickoff. Reserved
S113-S115 numbers and remaining figure issues are preserved and are not complete.

## Owner-approved S121 Regrouping (2026-10-05)

After the S120 squash merge (#252), the owner authorized S121
(`121-encounter-logging-and-addon-clarity`) for the full #249 messaging audit
and #222 encounter-lineage figure. The figure moves from the historical unused
S115 reservation into S121; that reservation is not a completed implementation.
S113/#223 and S114/#221 remain separate. No new issue arrived at the post-merge
assessment. Preserve full audit scope, automated safety/documentation gates,
official PR, two-review-round cap and owner merge. No field check or release.

## Reserved Figure Sequence

The sequence below preserves the original reservations, not the current kickoff
numbers. S115/#222 moved into merged S121; S113/#223 and S114/#221 move into
owner-authorized S122 as recorded below.

Sequence:

1. S112 implements issue #220 by adding the accessible troubleshooting decision
   tree, preserving the complete prose authority, and expanding the governed
   figure inventory and browser evidence.
2. S113 implements issue #223 by mapping shared local API and MCP authority
   ownership without changing the service contract.
3. S114 implements issue #221 by visualizing the reviewed catalog evidence
   lifecycle from bounded collection through explicit selection and rollback.
4. S115 implements issue #222 by visualizing encounter evidence lineage while
   keeping native-log ingestion provisional until issue #190 is verified.

Each slice owns one repository-backed, offline-safe, accessible SVG, its
complete text equivalent, exact update authorities, policy coverage, and
browser evidence. These slices improve comprehension only. They do not change
application, addon, automation, data, transport, or release behavior.

Field-verification issues #110, #129, #131 and #190 were closed as not planned
under the owner's direction. Closure does not prove installed behavior or live
parity. Native-log ingestion remains provisional and does not block these figures.

The official S112 implementation record is
[PR #225](https://github.com/h8rt3rmin8r/eso-weave/pull/225).

## Owner-approved S122 Regrouping (2026-10-06)

S121 merged as #253 and closed #249/#222. The live post-merge assessment found
no new issues and only #221/#223 open. The owner authorized S122
(`122-catalog-and-extension-authority-maps`) to deliver both remaining figures
with shared policy, offline, topology, narrow and 200 percent zoom coverage.
Each remains an independently reviewable figure with a complete equivalent and
source/update authorities. The unused S113/S114 reservations are historical,
not completed slices. Full spec-kit/autopilot, automatic push/official PR, all
findings, green CI and at most two review rounds apply. Final review/merge remains
with the owner; no release or field check. Plan048 awaits that integration.

## Completion and Archival (2026-10-06)

All four approved figures have merged. This completion record supersedes the
historical pending language above without rewriting the original approvals.

| Delivery date | Actual slice | Delivered issues | Merge evidence |
| --- | --- | --- | --- |
| 2026-09-17 | S112 troubleshooting figure | #220 | [PR #225](https://github.com/h8rt3rmin8r/eso-weave/pull/225), `dc28b1a` |
| 2026-10-05 | S120 recovery interruption | #248, #251, #250 | [PR #252](https://github.com/h8rt3rmin8r/eso-weave/pull/252), `729c298` |
| 2026-10-06 | S121 messaging and encounter lineage | #249, #222 | [PR #253](https://github.com/h8rt3rmin8r/eso-weave/pull/253), `2bd4fc3` |
| 2026-10-06 | S122 catalog and extension authority | #221, #223 | [PR #254](https://github.com/h8rt3rmin8r/eso-weave/pull/254), `56d97ae` |

S113-S115 remain unused historical reservations. Their figure issues were
delivered in S121/S122, not those reserved slice numbers. S121's October 5
authorization preceded its October 6 merge. S123 now directs v0.17.3 candidate
preparation in [Plan049](plan-049.md); publication and
installed behavior remain separate from this completed repository program.
