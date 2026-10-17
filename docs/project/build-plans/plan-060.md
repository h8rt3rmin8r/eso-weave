# Plan060 - v0.19.1 Patch Release Candidate

Created 2026-10-17. Status: Active, In Progress.

## S134 (2026-10-17)

After S133 owner merge #311 and documentary merge #313, prepare
[candidate #315](https://github.com/h8rt3rmin8r/eso-weave/issues/315) under
[release coordinator #314](https://github.com/h8rt3rmin8r/eso-weave/issues/314).

1. Complete the Spec-Kit specify, clarify, release checklist, plan, tasks and
   blocking analyze sequence for [S134](../../../specs/134-v0191-release/spec.md).
2. Write concise Highlights for all five S133 fixes and preserve full release history.
3. Validate and commit preparation, then inspect and execute the governed local
   no-tag/no-push/no-publish v0.19.1 rollover on a clean release branch.
4. Pass existing release-note/package-policy, Rust safety/regression, documentation,
   rendering, production-build and text gates; preserve independent addon versions.
5. Publish the official PR, resolve findings and establish passing final-head CI.
6. Stop before owner merge, tagging or publication. Only candidate #315 closes on
   merge; #314 retains later publication and asset-integrity evidence.

No dependency, runtime, package-matrix, workflow or field-testing change is planned.
The latest published release remains v0.19.0 until separately authorized publication.
