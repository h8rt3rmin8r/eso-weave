# Tasks: Collision-Safe Status Rows

## Phase 1: Specification and Analysis

- [x] T001 Create the S116 branch and spec-kit workspace from issue #231
- [x] T002 Inspect the shared row primitive, both reported surfaces, existing UI tests, and egui_kittest paint APIs
- [x] T003 Resolve scope, alignment, truncation, accessibility, text scaling, and modal-layer clarifications
- [x] T004 Produce requirements, collision-safety checklist, research, data model, contract, plan, and quickstart
- [x] T005 Pass the pre-implementation analysis gate with no critical conflict

## Phase 2: Test-First Failure

- [x] T006 Extend dashboard row geometry with row and label rectangles
- [x] T007 Add a reusable row-scoped semantic and painted-text collision oracle
- [x] T008 Enumerate all shipping dashboard titles and add default/enlarged text fixtures
- [x] T009 Add System and State plus Data Details surface fixtures across themes, widths, and addon states
- [x] T010 Run the focused suite and record the expected pre-fix failure

## Phase 3: Implementation

- [x] T011 Measure shared title columns from the exact current title font
- [x] T012 Bound group widths against row gaps, action reserves, and minimum status width
- [x] T013 Pass the shared group width through Live HUD, System and State, and Data Details
- [x] T014 Clip title, value, and interaction child painters to their exact cells
- [x] T015 Truncate constrained titles while preserving complete accessible names and tooltips
- [x] T016 Re-run the focused suite to green and refactor duplicated fixture/layout code

## Phase 4: Records and Verification

- [x] T017 Update the `[Unreleased]` changelog for S116
- [x] T018 Complete the collision-safety checklist and post-implementation analysis
- [x] T019 Run focused UI tests, fmt, clippy, all locked tests, and the locked release build
- [x] T020 Run diff, encoding, LF, mojibake, and forbidden-dash checks
- [x] T021 Review the complete diff for accessibility, responsive layout, and unrelated changes

## Phase 5: Local Delivery Boundary

- [x] T022 Commit as `feat(116): prevent status text overlap` with issue linkage and attribution
- [x] T023 Present the pre-push breakdown and exact push command
- [x] T024 After explicit authorization, push and open `S116: Prevent status text overlap` with `Closes #231`
- [x] T025 Process required hosted checks and every review thread without requesting an unauthorized second Codex round
- [x] T026 Stop for the operator's final review and merge ritual when checks and reviews are satisfied
