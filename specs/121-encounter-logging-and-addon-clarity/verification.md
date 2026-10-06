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
tool version or policy gate was changed. A definitive successful documentation
result remains required.

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
suite now has 13 passing tests. Reply/resolution, the second authorized Codex
round and green checks on the final head are still pending. No third review
round will be requested.
