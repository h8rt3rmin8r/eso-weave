# Research: Documentation Corpus Reorganization

## Decision 1: Freeze the actual S058 baseline

**Decision**: Use merge commit `bdc7b228b78ef535d07357e7b33a36bbade917cc`
as the baseline. It contains 49 files under `docs`, one website article, 20 H2
units in the monolith, and 27 legacy build plans.

**Rationale**: Issue #80 described 25 plans before plans 026 and 027 existed.
Migrating the live repository requires current evidence rather than the stale
issue-time count.

## Decision 2: Enforce three documentation lifecycles by path

**Decision**: `docs/src` is published and later bundled, `docs/project` contains
current maintainer records, and `docs/archive` contains completed history.
`docs/README.md` explains these boundaries. Book configuration and theme assets
remain infrastructure at the `docs` root.

**Rationale**: Path ownership is visible, testable, and already compatible with
mdBook's single `src` setting.

## Decision 3: Split for preservation, defer completeness

**Decision**: Split the existing specification and detailed README into user,
concept, reference, and development pages at their current factual fidelity.
Record known gaps for #81 instead of expanding S058 into a comprehensive rewrite.

**Rationale**: Issue #80 owns information architecture and preservation. Issue
#81 owns exhaustive feature and logic coverage against current code.

## Decision 4: Amend the constitution to 2.0.0

**Decision**: Replace the exact monolith mandate with a structured canonical
documentation corpus, issue-backed numbered slices, and active build plans under
`docs/project/build-plans`.

**Rationale**: Retiring the file named by Principle I is a backward-incompatible
governance change. A MAJOR amendment makes the new authority explicit while
retaining the full spec-kit sequence, analyze gate, CI parity, and safety law.

## Decision 5: Preserve plans as completed archive records

**Decision**: Move plans 001 through 027 to the archive, mark each Complete and
Archived in an evidence table, and delete none. Keep plan 028 as the sole current
plan during this slice.

**Rationale**: Merged specs, commits, issues, pull requests, and releases prove
delivery. Unchecked ritual boxes in older spec packages do not outweigh merged
history. Plan 007 remains useful even though plan 008 corrected its bite signal.

## Decision 6: Publish the brand contract, archive the article

**Decision**: Move the brand standard into the published development track.
Move the orphaned Ultimate article to `docs/archive/website` and migrate its
durable explanation into one canonical feature page.

**Rationale**: Product design is useful contributor reference material. The blog
source is historical marketing content without an active website generator and
must not become a competing manual.

## Decision 7: Preserve historical statements narrowly

**Decision**: Rewrite all live operational references. Retain old paths in
immutable changelog statements only when the ledger identifies the exact
historical occurrence and rationale.

**Rationale**: Rewriting history to pretend a new path existed in an old release
damages provenance. Blanket exclusions would hide real broken references.

## Decision 8: Extend the dependency-free policy suite

**Decision**: Add a machine-readable ledger and corpus validator to the existing
Node standard-library policy script and fixtures. Commit a frozen baseline
inventory instead of requiring full Git history in documentation CI.

**Rationale**: This keeps pull-request checks fast, deterministic, and free of a
new dependency graph or workflow permission change.

## Alternatives Rejected

- Keep the monolith as a redirect: preserves a competing authority.
- Archive the monolith without splitting: preserves bytes but not usable
  canonical destinations.
- Delete old plans: loses decision history without a redundant source.
- Keep all plans Active: repeats the stale status defect identified by #80.
- Retain the Ultimate article in place: leaves an orphaned future publication
  surface competing with the canonical page.
- Depend on shallow CI Git history for the ledger: makes validation environment
  sensitive and would require an unrelated workflow checkout change.
