# Build Plan 014: Delivery Pipeline Governance

Plan: 014
Status: active
Master specification: `docs/ESO-Weave-Specification.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Earlier slices exposed a process defect: dense issues mixed implementation and
release verification, so merged pull requests did not reliably close discrete
outcomes and the owner could not see the real pipeline. The repository already
has useful labels, milestones, native epic relationships, and spec-kit records.
This plan connects them with enforceable pull request linkage and one minimal
Project view.

## Ordering

Slice 044 combines issues #45, #46, and #47 because they establish one governance
contract. Define atomic issue and closing behavior first, then audit historical
metadata against that contract, and finally project the corrected issue inventory
into table and board views. The three outcomes remain separate issues and receive
separate closing references in the pull request.

## Slice 044: Delivery Pipeline Governance

Feature under `specs/044-delivery-pipeline-governance/`.

Scope:

- define one independently closeable outcome and lifecycle per actionable issue
- separate implementation from later release or field verification
- require GitHub closing references in normal pull requests targeting `main`
- test the issue-link policy without third-party dependencies or write access
- audit every issue's milestone and verification metadata against repository
  evidence
- create a public linked Project containing every issue exactly once
- provide a milestone-grouped table and Stage-grouped delivery board
- preserve native labels, milestones, parent issues, and sub-issue progress as
  the source of project truth

Done when issues #45, #46, and #47 close from the merged pull request, the Project
and audit invariants pass, all review threads are resolved, and CI is green.
