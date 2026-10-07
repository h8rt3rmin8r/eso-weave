# Plan 049: v0.17.3 Release Preparation

Status: Active

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
[Releasing](../releasing.md). Installed/game checks are outside S123; repository
evidence does not prove shipped behavior.
