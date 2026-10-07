# Plan 049: v0.17.3 Release Preparation

Status: Complete, Archived

## S123 (2026-10-06)

Following the S122 owner merge, prepare the v0.17.3 candidate in
[S123](../../../specs/123-v0173-release/spec.md) for
[issue #255](https://github.com/h8rt3rmin8r/eso-weave/issues/255).

1. Complete Spec Kit specification, clarification, requirements checklist,
   implementation plan, tasks and blocking analysis.
2. Summarize merged S120-S122 outcomes in bounded Highlights, preserving full
   change history. Archive completed Plan048 and update planning authorities.
3. Follow the existing release procedure to roll over application identity,
   lockfile, badge, documentation snapshot and capture fixture to v0.17.3.
4. Pass existing automated gates, push and publish an official preparation PR,
   satisfy every external finding and all final-head checks within at most two
   requested Codex rounds, then request owner final review and merge.

The owner explicitly authorizes push and official PR. This plan owns candidate
preparation only and completes when the owner merges the reviewed preparation
PR. It creates no tag or downloadable package. Later publication requires
separate authority and the exact merged-head procedure in
[Releasing](../../project/releasing.md). Installed/game checks are outside S123; repository
evidence does not prove shipped behavior.

## Completion, Publication and Follow-up (2026-10-07)

1. The owner merged [PR #256](https://github.com/h8rt3rmin8r/eso-weave/pull/256)
   at 11:12:10 UTC, completing S123 and issue #255's candidate preparation.
2. Separately authorized [v0.17.3](https://github.com/h8rt3rmin8r/eso-weave/releases/tag/v0.17.3)
   published at 11:41:28 UTC with Windows MSI, Linux tarball, Debian package,
   AppImage and combined SHA256SUMS assets.
3. The owner merged [PR #260](https://github.com/h8rt3rmin8r/eso-weave/pull/260)
   at 18:19:23 UTC (`e65c00c`), repairing future Linux AppImage packaging for
   issue #259. This did not alter the published v0.17.3 tag or artifacts.
4. S124 continues under [Plan050](../../project/build-plans/plan-050.md).

The original approval and sequence above remain historical. Publication and
repository checks do not establish installed or game-session behavior.
