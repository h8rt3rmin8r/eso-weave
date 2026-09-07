# Tasks: mdBook Documentation Foundation

## Phase 1: Spec-kit

- [x] T001 Synchronize released `main`, create the S057 branch, and move #79 to
  active project state.
- [x] T002 Complete parallel repository, tooling, workflow, accessibility, and
  validation discovery.
- [x] T003 Create the feature specification and requirements checklist.
- [x] T004 Resolve clarification through the autopilot decision policy.
- [x] T005 Complete research, data model, site contract, quickstart, plan, and
  delivery checklist.
- [x] T006 Run and pass pre-implementation cross-artifact analysis.

## Phase 2: Test-first foundation

- [x] T007 Add failing fixture tests for navigation, case, offline-resource, and
  workflow permission failures.
- [x] T008 Add the minimal `docs/src` navigation skeleton and custom 404 page.
- [x] T009 Add exact mdBook/linkcheck configuration and branded local styling.
- [x] T010 Implement the dependency-free repository site-policy checker.
- [x] T011 Add the mdBook ADR, contributor commands, plan 027, and changelog
  records, including the dated pinned-workflow decision.
- [x] T012 Add an immutable, least-privilege docs build and Pages workflow.
- [x] T013 Configure GitHub Pages for Actions and restrict its deployment
  environment to `main`.

## Phase 3: Validation

- [x] T014 Run policy fixture tests and confirm every negative case is covered.
- [x] T015 Run exact-version mdBook test/build and the generated-site policy check.
- [x] T016 Verify nested output, local search, 404, brand assets, edit links,
  contrast, focus, reduced motion, and 320-pixel reflow evidence.
- [x] T017 Verify generated output is ignored and the diff excludes #80 through
  #84 implementation.
- [x] T018 Run UTF-8, BOM, LF, punctuation, mojibake, secret, and whitespace gates.
- [x] T019 Run Cargo format, strict all-target Clippy, and the complete locked
  test suite in the foreground.
- [x] T020 Complete post-implementation cross-artifact analysis.

## Phase 4: Review and delivery

- [x] T021 Complete parallel code, security, and documentation/accessibility
  reviews, resolving findings at 80 percent confidence.
- [x] T022 Commit as `feat(057): establish mdbook documentation foundation` with
  attribution.
- [ ] T023 Push and open an official pull request with `Closes #79`.
- [ ] T024 Move #79 to PR review and monitor every hosted check and review.
- [ ] T025 Resolve every first-round finding and failed check.
- [ ] T026 Trigger at most one explicitly authorized second `@Codex review` and
  resolve every result.
- [ ] T027 Confirm green checks, no unresolved threads, and merge readiness before
  requesting the final merge ritual.
