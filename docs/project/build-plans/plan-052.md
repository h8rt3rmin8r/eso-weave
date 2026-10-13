# Plan 052: Retained Recording Sessions

Status: Active

## S126 (2026-10-13)

After S125 merged in PR #274 on 2026-10-12, deliver
[S126](../../../specs/126-retained-recording-sessions/spec.md) for
[issue #273](https://github.com/h8rt3rmin8r/eso-weave/issues/273).

1. Complete specification, clarification, requirements checklists, research,
   implementation plan, tasks and blocking analysis.
2. Preserve terminal recording identities and exact evidence through bounded
   multi-session retention, supported migration, start and explicit resume.
3. Enforce whole-spool budgets and deliberate selected-session discard.
4. Validate all source sessions, import the selected environment atomically,
   and preserve durable history grouping through deletion and reimport.
5. Update last-saved source summaries, player guidance, canonical docs, Plan051
   archive, indexes, migration ledger and dated decisions.
6. Run full repository gates, commit, automatically push and publish the
   official PR, address every review within at most two rounds, and hand off
   green final-head checks for owner review and merge.

Push and official PR publication are explicitly authorized. No merge, tag,
release or owner field-verification task is authorized by this plan.
Native completed-file import remains separately tracked by issue #272.

Publication: [PR #275](https://github.com/h8rt3rmin8r/eso-weave/pull/275).
Both authorized review rounds have completed. Round 2 identified a migration
rollback gap; the correction preserves schema1-4 store bytes on rejection.
All nine checks passed on corrected source head
3e8ef8cbdf8954572a65cdeb9ae4d5bbe3a716a0. The review thread is answered and
resolved; the receipt commit changes records only and its exact-head checks
are verified live before owner handoff. Plan052 remains Active until owner merge.
