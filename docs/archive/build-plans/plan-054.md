# Plan 054: v0.18.0 Release Preparation

Status: Complete, Archived

## S128 (2026-10-13)

After S125-S127 owner merges in PRs #274, #275 and #276, prepare
[S128](../../../specs/128-v0180-release/spec.md) for candidate
[issue #279](https://github.com/h8rt3rmin8r/eso-weave/issues/279).

1. Complete the full spec-kit sequence and blocking analysis.
2. Archive completed Plan053 with PR #276 merge evidence, repair current links
   and synchronize plan indexes and machine/prose ledger.
3. Add compact outcome Highlights preserving complete S125-S127 change history.
4. Use the governed clean-tree local rollover to synchronize v0.18.0 identity.
5. Run all existing candidate, documentation/render, Rust and bundled build gates.
6. Automatically push and publish the official candidate PR, address every finding
   within two review rounds, verify green final-head CI and deliver owner handoff.

## Completion and Publication (2026-10-13)

The owner squash-merged [PR #278](https://github.com/h8rt3rmin8r/eso-weave/pull/278)
at d88151c3ba1b5523fc29a22f7fcd8b9cc8491289, completing candidate issue #279.
Both authorized review rounds completed; the sole finding was fixed, answered
and resolved. Final candidate and exact merged-main checks passed.

After separate owner authorization, annotated tag v0.18.0 was published on that
exact commit. [Release workflow 38396415318](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/38396415318)
passed and [v0.18.0](https://github.com/h8rt3rmin8r/eso-weave/releases/tag/v0.18.0)
published at 2026-10-13 12:31:53 UTC. All five required assets were downloaded;
package SHA-256 values matched SHA256SUMS and all asset sizes/digests matched
GitHub metadata. Parent issue #277 is closed and its project item is Done.

Archived during owner-requested post-release housekeeping. No active build plan
remains. Package installation and real-game behavior were not tested or claimed.
