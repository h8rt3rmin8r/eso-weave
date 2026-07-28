# Tasks: Master Specification Refresh

**Input**: [plan.md](plan.md), [spec.md](spec.md)

**Tests**: no behavior changes, so there is nothing new to test. The full suite is
run to prove exactly that (T012).

## Phase 1: Setup

- [ ] T001 Confirm a clean tree and a green baseline in the foreground

## Phase 2: The document (US1, US2, US3)

- [ ] T002 [US1] Write `docs/ESO-Weave-Specification.md` in full: header block with version 1.0.0 and the note that the document version is independent of the application version, rebuilt table of contents, and every section reconciled against the shipping source
- [ ] T003 [US1] Reconcile the overview, scope, and terminology: three capabilities, not two; auto-potion, the menu gate, quickslot, and resource watches defined
- [ ] T004 [US1] Reconcile the input engine section: all three application toggles in the keybinding table with `F3` for auto-potion, and the menu gate described as part of the interception decision
- [ ] T005 [US1] Reconcile the PixelBeacon section: twenty blocks in one table that renders as one table, the two-row grid, the marker registry rule, display detection, and which signals act versus which are observable only
- [ ] T006 [US1] Give auto-potion its own top-level section with the ordered trigger rule, the unknown-is-never-permissive rule, and the safety requirements
- [ ] T007 [US1] Reconcile the interface and configuration sections: the status region's full row set, the skills grid's Cooldown column, the settings modal maximum, and the `potion` config section
- [ ] T008 [US3] Reduce to four diagrams (architecture, interception decision, fishing state machine, pixel-bus flow), none adjacent, converting the other six to tables or lists per plan.md D3
- [ ] T009 [US2] Remove the open-items section and the owed-validation appendix, and rewrite every hedging or self-dating construction

## Phase 3: References

- [ ] T010 Delete the old document and update all references to `docs/ESO-Weave-Specification.md` across `.specify/memory/constitution.md`, `CLAUDE.md`, `README.md`, `CHANGELOG.md`, `docs/build-autopilot.md`, `docs/plans/**`, and `specs/**`

## Phase 4: Verify and record

- [ ] T011 Verify the acceptance checks: zero hits for the old filename, zero hits for the hedging vocabulary, at most four diagrams with none adjacent, and every table-of-contents anchor resolving to a heading. Also confirm by diff that `src/`, `addon/`, and `tests/` are untouched (FR-014); a green suite would show only that what changed still works, which is the weaker claim.
- [ ] T012 Add the `[Unreleased]` changelog entry with dated decisions for the rename (D1), the independent document version (D2), and the diagram budget (D3), then run the full merge gate in the foreground

## Dependencies

T002 blocks T003 through T009, which are all edits to the same file and are done
as one authoring pass. T010 depends on T002. T011 depends on T010.

## Analyze gate

To be run before implementation.
