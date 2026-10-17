# Plan060 - v0.19.1 Patch Release Candidate

Created 2026-10-17. Status: Complete, Archived.

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

## Candidate Review (2026-10-17)

[PR #316](https://github.com/h8rt3rmin8r/eso-weave/pull/316) contains the governed
v0.19.1 candidate. All 1,219 local Rust tests, 13 S133 cases, ten release-profile
documentation tests, 184 policy tests, notes/packaging-policy, production-build,
render and text gates pass. Automatic Codex review completed without findings on
reviewed head 659c78b, and all nine hosted checks pass on that candidate.
The final documentary commit changes delivery records only; its exact head and
check results are recorded in the PR conversation before owner handoff.

This plan completes only after owner merge of the candidate. #315 is still open;
#314 and milestone v0.19.1 remain open through later publication and asset audit.
No merge, tag, package publication or field-testing task is authorized in this run.

## Owner Merge and Publication Authority (2026-10-17)

The owner merged [PR #316](https://github.com/h8rt3rmin8r/eso-weave/pull/316)
at 05:57:27 UTC as `6a3c95220120978a6decc87bc8e1ce7a63de5742`. Its tree
exactly matches reviewed head `760d41ee1cde9c1fb15c4b7f94187b94578552a4`.
Candidate #315 is closed and Done; local main was synchronized and the merged
candidate branch was removed locally, with the deleted remote ref pruned.

The owner separately authorized tag/publication and the release-asset audit.
Exact-main [CI](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008127095),
[documentation](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008127068)
and [CodeQL](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008127097)
passed before tagging. Annotated tag object
`4cf66b776d9dca01482207267dfa8210ed6d05b2`, created at 06:08:47 UTC,
was pushed and peels to the exact owner merge above.

## Publication and Asset Audit (2026-10-17)

[Release workflow 39008830108](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/39008830108)
passed and [v0.19.1](https://github.com/h8rt3rmin8r/eso-weave/releases/tag/v0.19.1)
published as latest stable at 06:25:02 UTC. All five downloaded sizes and SHA-256
digests match GitHub metadata; all four packages match SHA256SUMS. Release notes
match the governed excerpt. Read-only MSI metadata identifies ESO Weave 0.19.1;
Debian metadata identifies eso-weave 0.19.1-1, amd64 with complete required fields.
AppImage and tarball binaries identify x86_64 ELF; the tarball has the exact
expected inventory and its catalog/README/license match tagged source.

Coordinator #314 is Closed/Done and milestone v0.19.1 is closed. #317 records this
separate documentary archive through an owner-reviewed PR. Full sizes and hashes
are in the [S134 receipt](../../../specs/134-v0191-release/verification.md).
No package installation, application execution or game behavior was tested or
claimed. Published tag and assets remain unchanged.
