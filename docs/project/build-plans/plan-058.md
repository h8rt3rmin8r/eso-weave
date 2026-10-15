# Plan 058: v0.19.0 Release Candidate

Status: Active

## S132 (2026-10-15)

After S129-S131 owner merges in PRs #295, #298 and #299 and housekeeping PR #300,
prepare [S132](../../../specs/132-v0190-release/spec.md) for candidate
[issue #302](https://github.com/h8rt3rmin8r/eso-weave/issues/302), under release
coordinator [#301](https://github.com/h8rt3rmin8r/eso-weave/issues/301).

1. Complete specify, clarify, release checklist, plan, tasks and blocking analysis.
2. Incorporate #284's serde_json 1.0.152, rmcp 3.5.1 and tokio-util 0.7.20 updates
   in the candidate, preserving all current-main work and other dependencies.
3. Add concise Highlights covering the complete post-v0.18.0 delivery; correct
   stale two-package release wording while preserving the package matrix.
4. Use the configured clean-tree local rollover to synchronize v0.19.0 identity.
5. Run existing candidate, documentation/render, policy, Rust and production-build gates.
6. Push and publish the official PR, resolve findings within two Codex rounds,
   establish green final-head CI and stop for owner final review and merge.

## Later Publication Boundary

The candidate PR closes #302 only on owner merge. #301 remains open until later
authorized tagging, package publication and asset-integrity audit. #284 can be
retired after equivalent dependency content is confirmed on main. No merge,
tag/publication or owner field-testing task is authorized by candidate delivery.

## Candidate Review (2026-10-15)

[PR #303](https://github.com/h8rt3rmin8r/eso-weave/pull/303) contains the governed
v0.19.0 rollover and exact #284 dependency updates. All 1,207 local Rust tests,
184 policy tests, release/documentation/render gates and ten release-profile
documentation tests pass. Codex round 1 completed cleanly on reviewed candidate
8adc718ee73835ef1e30c530d7c0f944f60bc306; one of two authorized rounds was used.
All nine hosted checks pass on that reviewed candidate, with deployment correctly
skipped. The final delivery commit updates records only; the PR conversation
records its exact head and checks before owner handoff. This plan completes only
after owner merge; #301 remains separate publication work.
