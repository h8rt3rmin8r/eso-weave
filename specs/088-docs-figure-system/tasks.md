# Tasks: Documentation Figure System

## Phase 1: Audit and specification

- [x] T001 Create the S088 spec-kit feature workspace on `codex/s088-docs-figure-system`.
- [x] T002 Audit all raw HTML images, Markdown images, captions, generated mdBook zoom DOM, theme assets, browser seams, print output, and offline delivery.
- [x] T003 Define requirements, research, data model, figure-system contract, quickstart, and accessibility checklist.
- [x] T004 Resolve clarification under the autopilot decision policy and pass the pre-implementation analysis gate.

## Phase 2: Test-first figure contracts

- [x] T005 Add failing-first source inventory, meaningful/decorative classification, runtime theme, and idempotence tests in `.github/scripts/docs-policy.test.mjs`.
- [x] T006 Add failing-first trigger, native-dialog, caption hierarchy, contrast, and print contract tests in `.github/scripts/docs-policy.test.mjs`.
- [x] T007 Add failing-first figure observation, receipt completeness, keyboard, pointer, focus, geometry, and caption tests in `.github/scripts/docs-render-smoke.test.mjs`.
- [x] T008 Run the focused Node suite and record the expected red state in `analysis.md`.

## Phase 3: Runtime figure system

- [x] T009 Implement idempotent meaningful-image discovery and semantic trigger conversion in `docs/theme/eso-weave.js`.
- [x] T010 Normalize mdBook diagram wrappers into the shared trigger and remove checkbox modal remnants at runtime.
- [x] T011 Implement the one native dialog, accessible label and description, close paths, focus lifecycle, and intrinsic sizing.
- [x] T012 Add persistent trigger, modal, focus, responsive geometry, reduced-motion, and print rules in `docs/theme/eso-weave.css`.
- [x] T013 Add the shared caption hierarchy with surface-specific inherited colors and strong lead-in preservation.

## Phase 4: Policy and browser evidence

- [x] T014 Implement exact figure inventory, script contract, caption CSS, print, and generated-delivery validation in `.github/scripts/docs-policy.mjs`.
- [x] T015 Adapt the S086 diagram browser seam to the superseding shared dialog while preserving its paint and geometry evidence.
- [x] T016 Add the S088 20-cell figure matrix, trusted pointer and keyboard journeys, 200 percent scale, print, and blocked-script observations in `.github/scripts/docs-render-smoke.mjs`.
- [x] T017 Complete mutation coverage and run focused policy, mdBook, generated-site, and browser tests.

## Phase 5: Documentation and governance

- [x] T018 Publish `docs/project/documentation-figure-system.md` with inventory, maintenance, accessibility, and verification guidance.
- [x] T019 Update `CHANGELOG.md`, Plan 039, and the active-plan index in chronological order.
- [x] T020 Re-run `/speckit.analyze`, resolve all findings, and mark the spec plus checklists implemented.
- [x] T021 Complete code, security, documentation, accessibility, and scope review.
- [x] T022 Run documentation, browser, spelling, UTF-8, mojibake, `git diff --check`, and full Cargo parity gates.

## Phase 6: Delivery

- [x] T023 Commit S088 as `feat(088): complete documentation figure system` with issue linkage and attribution.
- [x] T024 Push the authorized feature branch and open the official pull request closing #155 and #156.
- [x] T025 Set both project items to Status In Progress, Stage PR review, and Slice S088.
- [ ] T026 Address every first-round CI, Codex, security, and reviewer finding.
- [ ] T027 Request and address at most one authorized second Codex review round.
- [ ] T028 Confirm every review conversation resolved and every required check green.
- [ ] T029 Hand off to the maintainer for final review and merge.
