# S123 Candidate Evidence

Date: 2026-10-06. Baseline: protected main `56d97ae` (S122 owner squash merge).
Scope: issue #255, candidate preparation only. No installed/game checks.

## Specification Gates

Specify, clarify, requirements checklist, plan and tasks completed in order.
Both checklists pass 12/12. Read-only analyze passed before implementation:
zero critical/high/medium findings, 8/8 functional requirements and 5/5 success
criteria covered by the 14 tasks, no unresolved clarification or constitution
conflict. Required Phase 0 research agents worked read-only on rollover and
archive authority. The optional agent-context hook updated the managed block.

| Requirement | Task coverage |
| --- | --- |
| FR-001 / SC-001 | T003, T006, T007 |
| FR-002 / SC-002 | T004, T006, T007 |
| FR-003 / SC-003 | T008-T010 |
| FR-004 | T001, T009 |
| FR-005 | T002, T006, T011 |
| FR-006 | T002, T004-T007, T011, T014 |
| FR-007 / SC-004 | T012-T014 |
| FR-008 / SC-005 | T001, T003, T012, T014 |

Analysis metrics: 13 requirements/criteria, 14 tasks, 100 percent coverage;
zero ambiguity, duplication, critical findings or unmapped tasks. No mandatory
analysis hook exists. Proceeded to implementation under the owner's autopilot.

## Baseline and Tool Authority

- Local and remote `v0.17.3` tag absent; hosted release endpoint returns 404.
- Installed cargo-release 1.1.2; effective configuration keeps verification
  enabled and publication/tag/push disabled. Closed stdin uses `--no-confirm`.
- Existing Unreleased notes gate rejects missing Highlights with exit 4 before
  implementation (`target/s123-notes-red.log`).
- Archive chronology: PR #225 (September 17), #252 (October 5), #253 and #254
  (October 6). S113-S115 remain unused reservations.

## Candidate Checks

Preparatory gates passed: release-note extractor and contract fixtures,
167 documentation/brand/render-policy tests, 13 trust-policy tests, repository
trust policy, Debian validator fixtures, spelling, brand adoption, mdBook
examples/build/link checks and generated-site policy. Whitespace and 24 changed
files' strict UTF-8/no-BOM/LF/punctuation/mojibake checks passed. Current Plan048
references now resolve to the archive, including historical spec paths.

The inspected dry run and executed governed rollover produced `ee37d38`
(`release: v0.17.3`). Cargo-release selected UTC date 2026-10-07 while the
operator's local date was October 6. Changelog and bundled snapshot retain that
same generated date. Exactly six identity files changed; Cargo.lock changed
only the root package version, with no dependency changes. Main-to-candidate
changelog diff is additive, preserving the full S120-S122 and earlier history.

Post-rollover gates passed:

- Format, all-target/all-feature Clippy with warnings denied, all 1039 locked
  Rust tests and release-profile application build with bundled documentation.
- Candidate note/section extraction, fresh Unreleased, three Highlights within
  budget (76 whitespace tokens), synchronized identity and generated-site policy.
- Rebuilt book/link checks and complete headless browser gates: 64 diagram
  paint cells, eight topology probes, four fallback-font probes, four authority
  zoom/no-script probes and retained syntax, figure and table coverage. All five
  success sentinels pass, with zero reported failures.

No runtime source, addon, service contract, dependency, pinned tool, release
configuration or workflow changed. This evidence covers repository/build
behavior, not installation or real-game operation.

## Hosted Integration

Official [PR #256](https://github.com/h8rt3rmin8r/eso-weave/pull/256) was published
at `2cc0ad3` and issue #255 moved to PR Review with Slice S123. First automatic
Codex review completed on 2026-10-07 and returned one P2
[archive-table finding](https://github.com/h8rt3rmin8r/eso-weave/pull/256#discussion_r4202113906).

Removed the blank line that separated Plan048 from the archive table. GitHub's
GFM renderer confirms Plan047 and Plan048 appear in the same actual HTML table;
the existing documentation policy passes after correction. No runtime,
dependency or identity change followed the local gates.

The corrected implementation is `778966a`. Its finding was answered and
resolved, and all nine checks passed with one expected PR deployment skip.
PR-ref open CodeQL alerts: 0. Final owner merge and later publication remain
separate operations; no third review round may be requested.

The second and final review completed at 2026-10-07 01:29:12 UTC for
`778966a783fb52514357cb6c37a830bcc7160dbe`, with
[no major issues](https://github.com/h8rt3rmin8r/eso-weave/pull/256#issuecomment-6028918404)
and a thumbs-up on the PR body. The
[final request](https://github.com/h8rt3rmin8r/eso-weave/pull/256#issuecomment-6028897113)
followed the answered/resolved first-round finding. Exactly two rounds were
used: automatic opening review and one manual re-review. Zero unresolved
threads remain; no third request will be sent.

Reviewed-implementation CI evidence:

- [CI run 37557161043](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37557161043):
  Windows/Linux tests and bundled-documentation release builds, dependency review.
- [Documentation run 37557160962](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37557160962):
  build, policy and rendering gates; PR deployment intentionally skipped.
- [CodeQL run 37557161040](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37557161040):
  Rust analysis and CodeQL check.
- [Issue linkage run 37557160933](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37557160933):
  issue closure linkage and proposed policy; protected-base trust passed separately.

This final receipt only closes tasks and records completed gate/review evidence.
It changes no externally reviewed implementation. Checks on its own final head
will be awaited before the owner handoff and recorded in the PR body. Owner
merge remains pending; no tag/release, package or installed/game check occurred.
