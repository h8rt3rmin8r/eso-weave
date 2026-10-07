# Plan 050: Dependency Security and v0.17.4 Preparation

Status: Active

## S124 (2026-10-07)

After Plan049, v0.17.3 publication and the merged Linux packaging repair,
deliver [S124](../../../specs/124-v0174-security-maintenance/spec.md) for
[issue #261](https://github.com/h8rt3rmin8r/eso-weave/issues/261),
[issue #262](https://github.com/h8rt3rmin8r/eso-weave/issues/262) and
[issue #263](https://github.com/h8rt3rmin8r/eso-weave/issues/263).

1. Complete Spec Kit specification, clarification, checklists, implementation
   plan, tasks and blocking analysis.
2. Establish existing dependency-contract coverage, then resolve the ten scoped
   package targets and the official paired CodeQL v4.38.2 pins. Preserve local
   service authority, canonical blob encoding and exact Lua import behavior.
3. Archive completed Plan049 and update both indexes and the lifecycle ledger.
   Prepare bounded Highlights with the complete maintenance history.
4. Follow [Releasing](../releasing.md) to prepare v0.17.4 through cargo-release.
   Validate all governed identities, existing automated gates and bundled docs.
5. Automatically push and publish the official PR, satisfy every review finding
   and final-head CI check within at most two requested Codex rounds, then
   request owner final review and merge.

The owner authorizes push and official PR. This plan prepares the candidate;
tag creation and package publication are outside S124. Original update PRs
#239, #240, #247, #257 and #258 remain open until post-merge supersession.
Repository checks do not establish installed or game-session behavior.
