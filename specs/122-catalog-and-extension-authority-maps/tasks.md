# Tasks: S122 Catalog and Local Extension Authority Maps

## Phase 1: Setup

- [x] T001 Run specify/clarify/checklist and issue traceability in specs/122-catalog-and-extension-authority-maps/spec.md.
- [x] T002 Research both current authority chains in specs/122-catalog-and-extension-authority-maps/research.md.
- [x] T003 Generate design, contracts and validation guide in specs/122-catalog-and-extension-authority-maps/plan.md and contracts/figures.md.

## Phase 2: Foundation

- [x] T004 Run blocking cross-artifact analyze on specs/122-catalog-and-extension-authority-maps/{spec,plan,tasks}.md.
- [x] T005 Add failing figure/equivalent/gate and zoom receipt mutations in .github/scripts/docs-policy.test.mjs and docs-render-smoke.test.mjs (FR-005/006/007, SC-003/004).

## Phase 3: User Story 1

Goal: Accurate catalog transformations and explicit selection/rollback.
Independent test: Every arrow/qualifier matches contracts/figures.md and nearby prose.

- [x] T006 [US1] Add annotated catalog-evidence-lifecycle.svg in docs/src/assets/diagrams/ (FR-001/002).
- [x] T007 [US1] Add placement/full equivalent in docs/src/development/catalog-candidate-pipeline.md and supporting links (FR-001/002/005, SC-001).

## Phase 4: User Story 2

Goal: Shared local authorities with discovery and framing distinctions.
Independent test: Both adapters/current code match each depicted authority boundary.

- [x] T008 [US2] Add annotated local-extension-authority-map.svg in docs/src/assets/diagrams/ (FR-003/004).
- [x] T009 [US2] Add placement/full equivalent, retaining operations table in docs/src/reference/local-api-and-mcp.md (FR-003/004/005, SC-001).

## Phase 5: User Story 3

Goal: Govern offline, accessible, maintainable figures through shared coverage.
Independent test: Policy mutations and generated browser matrix cover both maps.

- [x] T010 [US3] Extend source/generated finite asset/gate/equivalent policy in .github/scripts/docs-policy.mjs (FR-005/007, SC-004).
- [x] T011 [US3] Extend paint/topology/zoom/no-script receipts in .github/scripts/docs-render-smoke.mjs (FR-006/007, SC-002/003).
- [x] T012 [US3] Update maintained inventories/source authorities in docs/project/content-coverage.json, documentation-figure-system.md, diagram-rendering-compatibility.md and documentation-visualization-audit.md (FR-007/008).

## Phase 6: Polish and Integration

- [x] T013 Record chronological S122 regrouping/completion in docs/project/build-plans/ and CHANGELOG.md (FR-008/009).
- [x] T014 Run all quickstart automated checks and text/diff hygiene; record evidence in specs/122-catalog-and-extension-authority-maps/verification.md (FR-009, SC-001/002/003/004).
- [x] T015 Commit/push official feature PR, handle every finding and CI with at most two review rounds; record evidence in specs/122-catalog-and-extension-authority-maps/verification.md (FR-009).

## Dependencies and Strategy

T001-T004 precede implementation. T005 establishes red before T006-T011.
US1 and US2 assets/prose can be prepared independently after shared foundation;
shared policy/render files are edited sequentially. US3 integrates both, followed
by inventories, full automated checks and publication. US1 alone is a viable
documentation increment, but the owner authorizes both in this slice.
Parallel example: catalog and local-authority source research are independent;
asset/prose work does not need a second agent. Final merge/release excluded.
