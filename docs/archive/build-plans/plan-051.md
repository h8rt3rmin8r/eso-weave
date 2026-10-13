# Plan 051: Data Addon Workflow Recovery

Status: Complete, Archived

## S125 (2026-10-11)

After S124 merged in PR #264 on 2026-10-07, deliver
[S125](../../../specs/125-data-addon-workflow-recovery/spec.md) for issues
[#265](https://github.com/h8rt3rmin8r/eso-weave/issues/265) through
[#271](https://github.com/h8rt3rmin8r/eso-weave/issues/271) in one official PR.

1. Complete Spec Kit specification, clarification, checklists, implementation
   plan, tasks and blocking analysis before implementation.
2. Repair surface text contrast and complete Next Step painting, preserving
   dashboard alignment and accessible controls across width, scale and theme.
3. Route both addon modules' replies through readiness-aware system chat and
   add validated, idempotent recording start and concise status/help.
4. Inspect the selected last-saved source through the existing bounded worker,
   distinguish source and store failures, and reject stale results.
5. Connect Data Details to an ordered, copyable recording/import walkthrough;
   fold maintainer collection and show content-bound catalog acknowledgment
   only beside an eligible installation.
6. Update canonical user documentation, audit the seven acceptance outcomes,
   archive completed Plan050, and run the required automated gates.
7. Automatically push and publish the official PR, address every review comment
   within at most two Codex review rounds, verify green final-head checks, and
   request owner final review and merge.

The owner explicitly authorized push and PR publication. No merge, tag,
release publication or owner field verification is authorized by this plan.
Native Encounter.log ingestion (#272) and multiple retained sessions (#273)
remain separate slices. Repository tests do not prove installed game behavior.

## S125 review corrections (2026-10-12)

The first external review identified two P2 recovery gaps. Reproduce and fix
failed-import source invalidation and stopped empty-session guidance, including
current game status and repeated start. Revalidate, push to PR #274, reply to
and resolve both threads, then request the second and final authorized review.
Finish with green checks on the final head and owner merge handoff.

## S125 merge closeout (2026-10-12)

Owner squash-merged [PR #274](https://github.com/h8rt3rmin8r/eso-weave/pull/274)
at 7939729f302611f911c39216aea0c00f5dfe3ef1, closing issues #265 through #271.
The merged tree matches the reviewed final branch. Both permitted external
review rounds completed, both findings were answered and resolved, and all nine
final-head checks passed. Tag and release publication remain separate.
