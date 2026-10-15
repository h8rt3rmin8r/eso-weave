# S130 Planning Gate: Required Producer Grammar

Date: 2026-10-15. Status: resolved by direct owner scope revision before implementation.

## Completed Work

- Created the exact owner-selected branch and spec-kit feature directory for S130.
- Completed specification, five scope clarifications and requirement-quality checklists.
- Captured all 83 acceptance criteria from #289, #290 and #292-#294 without narrowing them or marking implementation complete.
- Researched native formats/identity/checkpoints, application runtime/storage/consumers, and full UI/documentation/asset retirement.
- Recorded a draft implementation plan and proposed compatible transaction/cancellation/identity design.

## Blocking Finding P0-001

Issue #289 and specification FR-002 require normal/separate, normal/inline, verbose/separate and verbose/inline native processing without asking players to change serialization.

Retained representative fixtures and the current parser cover normal/separate version 15. Current reviewed first-party stock help and API declarations establish the two independent format options, but do not implement or fully define their serialized record syntax. The additional public parsers/samples and producer-owned uploader source inspection did not resolve exact verbose/inline expansion.

Inventing synthetic layouts would test our own assumptions rather than establish that the parser reads ESO output. Shipping only normal/separate would materially narrow the approved scope and cannot be substituted autonomously.

Resolution requires existing authoritative producer grammar or sanitized representative verbose/inline evidence sufficient to declare the missing layouts, or direct owner agreement to revise the format scope and affected acceptance criteria. No new owner game/field-testing task is requested.

## Workflow State

`/speckit-plan` Phase 0 has not passed. Phase 1 contracts/data model, task generation, `/speckit-analyze` and implementation have not run and are not represented as complete. The existing plan template was replaced with an explicitly blocked draft, not a passing gate receipt.

No Rust/product implementation, schema change, commit, push, pull request or Codex review round has occurred. All five GitHub issues remain open. No release or merge was performed. The feature pointer and uncommitted planning artifacts remain available on the S130 branch for resumption.

## Validation Actually Performed

Feature artifacts passed UTF-8 strict decoding, no-BOM/LF/punctuation/mojibake checks. `git diff --check` passed. No Rust tests or feature validation were run because implementation has not begun; prior S129 results are not reused as S130 evidence.

See [native research](research-native.md), [runtime research](research-runtime.md), [surface research](research-surfaces.md), [acceptance inventory](issue-acceptance.md) and [draft plan](plan.md).

## Owner resolution (2026-10-15)

The owner authorized S130 normal/separate v15, with normal/inline tracked in #296 and verbose/separate plus verbose/inline in #297. The original blocker and workflow state above are historical pre-resolution evidence. Specs and affected GitHub acceptance criteria were revised under that authorization. Resume plan, tasks, blocking analysis and implementation without another publication or review authorization request.
