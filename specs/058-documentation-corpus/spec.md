# Feature Specification: Documentation Corpus Reorganization

**Slice**: S058

**Issue**: #80

**Status**: Implemented

**Created**: 2026-09-07

## User Scenarios

### User Story 1: Find one authoritative explanation (Priority: P1)

As a reader, I can enter the documentation through a concise repository README
and navigate task-oriented user guidance or deeper technical concepts without
encountering a parallel manual or a monolithic specification.

**Independent test**: Build the book and follow every landing-page and root
README route while confirming each published Markdown page appears exactly once
in navigation and search.

### User Story 2: Preserve every current statement during migration (Priority: P1)

As a maintainer, I can use a checked migration ledger to identify the disposition
and authoritative destination of every pre-S058 documentation file and every
top-level technical-specification section.

**Independent test**: Compare the frozen pre-migration inventory with the ledger
and prove that every entry has one checked disposition, a destination, and
preservation evidence before the monolith is removed.

### User Story 3: Distinguish published, current-project, and historical records (Priority: P2)

As a contributor, I can tell from paths and index pages whether material is
reader-facing, an active maintainer record, or an immutable historical record,
and only reader-facing material enters mdBook navigation and search.

**Independent test**: Inspect the three trees and generated site, then prove no
project or archive page is published and no published page sits outside
`docs/src`.

### User Story 4: Trust build-plan status (Priority: P2)

As a maintainer, I can see an evidence-backed final disposition for all 27 legacy
build plans and one current S058 plan without interpreting stale status labels
inside historical records.

**Independent test**: Validate the archived-plan index against the 27 files,
their implemented spec packages and merge or release evidence, plus the active
plan index against S058.

## Edge Cases

- Historical spec-kit records contain paths to documents that move in S058.
- A legacy plan has unchecked delivery-ritual tasks despite merged implementation.
- A plan's implementation was corrected later but remains historically useful.
- The root README and technical specification contain overlapping explanations
  intended for different reader depths.
- A process document is publicly readable in Git but must not enter site search.
- The existing Ultimate article is marketing prose, not canonical product
  documentation, and currently has no publishing system.
- A relative link works in GitHub but fails beneath the `/eso-weave/` Pages base.
- The migration changes the constitution's former single-file architecture of
  record and build-plan paths without weakening spec-driven development.

## Functional Requirements

- **FR-001**: The repository MUST expose `docs/README.md` as the lifecycle map
  for published, project, and archived documentation.
- **FR-002**: Reader-facing content MUST live only under `docs/src` and MUST be
  the only Markdown included in mdBook navigation and search.
- **FR-003**: Current maintainer workflow and active planning MUST live under
  `docs/project`, outside the published source tree.
- **FR-004**: Completed or superseded build plans MUST live under
  `docs/archive/build-plans`, outside the published source tree.
- **FR-005**: A checked migration ledger MUST account for all 49 files present
  beneath `docs` before S058 and the existing website Ultimate article, for 50
  baseline artifacts total.
- **FR-006**: The ledger MUST separately map all 20 level-two units in the
  technical specification, including sections 1 through 17 and Appendix A, to
  named authoritative destinations.
- **FR-007**: The technical specification MUST be split into audience-oriented
  pages before `docs/ESO-Weave-Specification.md` is removed.
- **FR-008**: The existing root README manual MUST be preserved in canonical
  published pages before the README is reduced to a concise project,
  installation, documentation, disclaimer, and license entry point.
- **FR-009**: Maintainer documents MUST move to `docs/project`, and all affected
  live repository references MUST resolve to their new paths.
- **FR-010**: All 27 pre-S058 build plans MUST receive an evidence-backed
  Complete plus Archived disposition; none may remain falsely Active.
- **FR-011**: S058 MUST receive one active build plan in the current project plan
  index so the constitution's sequencing contract remains live after migration.
- **FR-012**: The brand standard MUST remain an authoritative public developer
  reference and its asset references MUST remain valid.
- **FR-013**: The existing Ultimate article MUST leave the orphaned website tree,
  be preserved as a historical marketing record, and name one canonical
  published Ultimate feature page as its replacement.
- **FR-014**: Historical content MUST be retained unless the ledger names a
  preserved replacement and provides evidence that deletion loses no authority.
- **FR-015**: Automated policy tests MUST reject an incomplete ledger, stale
  live legacy path, misplaced lifecycle document, incomplete archived-plan
  index, bloated parallel root manual, or accidental project/archive publication.
- **FR-016**: Existing mdBook source, generated-site, accessibility, offline
  resource, and workflow-permission policies MUST remain green.
- **FR-017**: The constitution and autopilot guidance MUST be updated together
  to name the canonical corpus and new active-plan location without weakening
  the full spec-kit sequence or safety gates.
- **FR-018**: Changes to governance or pinned process artifacts MUST receive a
  dated decision entry in `CHANGELOG.md`.
- **FR-019**: Every moved or added text file MUST remain UTF-8 without BOM, use
  LF endings, and contain no forbidden dash characters or mojibake.
- **FR-020**: S058 MUST NOT author all missing feature/process documentation,
  embed the site in the executable, modify runtime behavior, or perform package
  and live-game verification owned by #81, #82, #84, or #77.

## Key Entities

- **Published page**: Canonical reader-facing Markdown under `docs/src`, present
  exactly once in `SUMMARY.md`.
- **Project record**: Current contributor process or active plan under
  `docs/project`, intentionally excluded from mdBook.
- **Archive record**: Completed historical plan under `docs/archive`, preserved
  for traceability and excluded from mdBook.
- **Migration ledger entry**: Original path or specification section, chosen
  disposition, destination, authority state, and evidence.
- **Plan disposition**: Complete implementation state plus Archived lifecycle
  state, supported by spec, commit, issue, pull request, or release evidence.

## Success Criteria

- **SC-001**: The ledger accounts for all 50 frozen baseline artifacts, all 20
  level-two specification units, and all required safety invariants with no
  duplicate or unchecked disposition.
- **SC-002**: Exactly 27 legacy plan files appear in the archive and exactly 27
  Complete/Archived rows appear in its index; the current plan index contains
  S058 as its sole Active plan.
- **SC-003**: No live repository reference to a removed legacy documentation path
  remains outside the migration ledger's explicit original-path records.
- **SC-004**: Every published Markdown page is navigable exactly once, all local
  links and fragments resolve with exact case, and no project/archive Markdown
  enters generated search or navigation.
- **SC-005**: The root README is at most 120 lines and routes detailed guidance
  into the canonical site rather than restating it.
- **SC-006**: Documentation policy fixtures, mdBook test/build, link checking,
  generated-site checks, text hygiene, and full Cargo CI parity all pass.

## Assumptions and Autopilot Decisions

- The 27 legacy plans are preserved rather than deleted because each records
  rationale that remains useful even when later work corrected part of it.
- A new active plan 028 is created for S058. Archiving every plan without a
  replacement would violate the current constitution during this slice.
- Existing technical and README prose is migrated at its current fidelity;
  issue #81 owns filling coverage gaps and performing exhaustive area-owner
  content review.
- The Ultimate article moves to `docs/archive/website` as a historical marketing
  announcement; its durable explanation moves once into the canonical site.
- Governance changes are a constitution MAJOR amendment to 2.0.0 because they
  retire the single-file architecture mandate while preserving all required
  spec-driven and safety gates.

## Dependencies and Exclusions

- Depends on the merged S057 mdBook foundation and issue #79.
- Closes issue #80 only.
- Leaves #81, #82, #84, #77, and epic #83 open.
