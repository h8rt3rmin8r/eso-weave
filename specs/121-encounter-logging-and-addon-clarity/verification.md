# S121 Execution Evidence

## Spec-kit analyze gate (2026-10-05)

Installed `/speckit-specify`, `/speckit-clarify`, `/speckit-checklist`, `/speckit-plan`, `/speckit-tasks` instructions were executed in order with the repository prerequisite/setup scripts. Optional agent-context hook is satisfied by the plan-reference update. No mandatory extension hook exists. All requirement checklists pass.

| Requirement | Tasks | Result |
| --- | --- | --- |
| FR-001-002 complete audit | T003, T016 | Covered |
| FR-003-005 explanations/setup/vocabulary | T005-007, T009, T011-012 | Covered |
| FR-006 recording workflow/states | T008-012 | Covered |
| FR-007 import/results/loss | T010-012 | Covered |
| FR-008 destructive scope | T006, T008-011 | Covered |
| FR-009 aligned help | T007, T012, T016 | Covered |
| FR-010 accessible lineage | T013-014 | Covered |
| FR-011 policy/layout/authorities | T013, T015, T017 | Covered |
| FR-012 unchanged capabilities/security | T005, T008, T010, T013, T017 | Covered |
| FR-013 autopilot/checks/review | T001-002, T004, T017-019 | Covered |

Analyze findings: none remaining. Thirteen functional requirements, nineteen tasks, 100% task coverage, zero ambiguities, duplication, critical or constitution issues. Three independently testable stories and their error cases are covered. All tasks map to requirements or process gates. Implementation may proceed. Analyze itself was read-only; this receipt records its outcome afterward.

## Implementation and repository checks

Completed on 2026-10-05 (local date), before publication:

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --all --locked` | PASS, 1,038 tests; zero failures or ignored tests |
| `node --test .github/scripts/docs-policy.test.mjs .github/scripts/docs-render-smoke.test.mjs .github/scripts/brand-kit-policy.test.mjs` | PASS, 164 tests; zero failures/skips |
| `typos docs/src docs/README.md README.md` | PASS |
| `mdbook test docs` | PASS |
| `mdbook build docs` including local link checks | PASS; pre-existing linkcheck fragment-resolution warnings remain |
| Generated-site documentation policy | PASS |
| Brand kit exact-adoption policy | PASS |
| Headless documentation browser smoke | PASS, all five sentinels, 48 paint cells and six topology observations |
| Encounter-lineage 200-percent zoom | PASS: scale 2; page/source/dialog contained; source and dialog effective text 30px; complete text equivalent present |
| `git diff --check`, UTF-8/no-BOM/mojibake/long-dash sanity | PASS |

Test-first evidence: setup/current-versus-saved fixtures failed on former labels;
addon command fixtures failed on former message text and then passed all 57
collector/data/encounter tests. Missing-source and explained-loss fixtures failed
against baseline generators. Combined loss/threshold and unknown-target fixtures
caught incorrect empty-prompt implications. Lineage and complete workflow policy
mutations failed before the new asset/guidance, then passed. A final imported-state
origin fixture failed before the UI described the last successful import and
retention across Refresh/failure, then passed the complete 12-test history suite.

Full integration first caught two catalog fixtures pinned to the old normalized
addon checksum. Both S092 synthetic capture fixtures and the S073 request revision
and SHA were synchronized; the direct pipeline test now derives its revision from
the embedded checksum. No importer verification was weakened. A stale unchanged-
collection message assertion was aligned; pipeline 14 and update 21 tests pass.

A subsequent full run reached an unrelated loopback fixture and failed with
Windows ConnectionAborted 10053; the headless browser also lost DevTools access.
The focused loopback test and the complete full suite passed with authorized
local-loopback access, and final rebuilt browser coverage passed. No product or
network policy was changed to accommodate these runner failures.

The complete matrix links desktop literal/state inventory, all 54 baseline addon
message sites plus two help continuations, generated diagnostics/recommendations,
and H01-H43 public/bundled guide dispositions. Every audited entry is replaced or
explicitly retained with scope and explanation; no open audit gap remains.

No ESO/game session, installed package, physical input or shipped behavior was
verified. No release, field-verification request or merge occurred. These are
repository fixture and documentation rendering results only.

## Hosted checks and reviews

Official non-draft [PR #253](https://github.com/h8rt3rmin8r/eso-weave/pull/253)
was published from `b59fee0`. Initial Linux and Windows CI, dependency review,
CodeQL, trust boundary and closing-issue policy checks passed. The PR-specific
open code-scanning alert query returned an empty array.

Documentation tooling installation failed twice before repository checks, while
downloading unrelated crates (`unicode-width`, then `anstream`), with crates.io
HTTP/2 framing errors. Failed-job-only retries were issued; no workflow, pinned
tool version or policy gate was changed. The third targeted attempt passed;
documentation CI also passed on the corrected `2e680c0` head.

Codex round 1 automatically ran on PR opening and completed on `b59fee0` at
2026-10-06 00:39:46 UTC. It reported one P2 finding, review comment `4190347760`
(thread `PRRT_kwDOTU-tqc6pRiLF`): longer explained loss labels wrapped inside a
fixed-height virtual list, which could clip later diagnostic rows.

Correction: only the loss-diagnostic list uses a normal bounded-height scroll
area so each explanation receives its actual wrapped height. Other numeric
virtual lists remain unchanged. A 360-point fixture imports ten declared gaps,
checks distinct wrapped row geometry and fully painted first rows, then scrolls
to verify the final row. The final fixture failed on the old implementation
because the last explanation was clipped; the corrected implementation passes.
Correction parity: formatting check and Clippy pass; `cargo test --all --locked`
passes all 1,039 tests with zero failures or ignored tests. The complete history
suite now has 13 passing tests. Reply/resolution and the second authorized Codex
round completed as recorded below. All hosted checks passed on the reviewed
implementation. The receipt-head CI gate remains required before owner handoff.
No third review round will be requested.

Codex round 2 was requested exactly once on `2e680c0`. It completed at
2026-10-06 00:51:50 UTC with no further major issues. Round 1 received a
verification-backed reply, and its sole thread was resolved. All inline review
comments and ordinary review comments were read; no unresolved finding remains.
The two-round budget is exhausted; no additional Codex/security review is
triggered by the agent. Current-head security-alert inspection returned no open
PR alerts.

## Completed implementation gates and owner boundary

All hosted checks passed on reviewed implementation
`2e680c0ad1b2f58afba57bac6d7c4c887cae6264`:

- Linux/Windows full CI and dependency review: [run 37396021211](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37396021211).
- Documentation, generated-site policy and headless browser: [run 37396021337](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37396021337).
- Rust CodeQL analysis: [run 37396021206](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37396021206).
- Protected trust policy: [run 37396018602](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37396018602).
- Closing-issue policy: [run 37396110458](https://github.com/h8rt3rmin8r/eso-weave/actions/runs/37396110458).

The initial documentation tool-download failures recovered on the third targeted
attempt of run 37394935059; the corrected implementation's independent docs run
also passed without a workflow change. The only skipped job is Pages deployment,
which is intentionally restricted to main after merge.

All nineteen agent tasks are complete through the owner-handoff boundary. This
receipt changes only the specification, tasks and evidence; implementation bytes
remain the reviewed `2e680c0` content. Final owner handoff must wait for successful
hosted checks on this documentation-only receipt commit. Review count stays at
two, with no third trigger. The owner's final review and merge remain pending;
no release or live-game/installed-package verification is included.

## Late consistency correction after the second review

The final source read found a remaining empty-result contradiction: a Ready or
Qualified report with zero questions still had an available/limited status, next
to a summary saying no questions were available. Only presentation now uses
`No review prompts for this recording` for a non-suppressed empty result;
suppressed, nonempty Ready and nonempty Qualified states keep their messages.
Analysis rules, report availability, citations and qualification are unchanged.

The existing combined-gate/unknown-target fixture was extended for empty Ready
and Qualified statuses. It failed against the prior presentation and passed with
the correction. Canonical help and the generator audit describe the empty state.
This correction is after Codex round 2 and is explicitly not externally re-reviewed;
the two-round cap remains binding. Full local parity and hosted gates must pass,
and the owner must include this presentation correction in the final review.

Late-correction local parity passes: formatting, Clippy, all 1,039 locked Rust
tests, all 164 documentation policy/renderer/brand tests, spelling, book examples
and build, generated-site policy, and all five headless browser sentinels.
Hosted checks on this correction remain a required final-handoff gate. No
additional receipt-only commit or review trigger is needed after they pass;
GitHub retains the definitive final-head check results.
