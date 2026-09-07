# Plan 028: Documentation Corpus Reorganization

Status: Complete, Archived

Sequence:

1. Freeze and check a migration ledger for every pre-S058 documentation artifact
   and every top-level technical specification unit.
2. Establish the published, current-project, and historical archive boundaries,
   then classify all maintainer records, plans, and the orphaned website article.
3. Split the technical specification and root README manual into canonical,
   audience-oriented pages without losing requirements or safety invariants.
4. Move current process records and completed plans, update live repository
   references, and replace the former single-file architecture authority with the
   canonical documentation corpus.
5. Verify navigation, links, search scope, generated output, text hygiene, and CI
   parity before removing redundant source documents.

This slice closed issue #80 in PR #88. It did not fill every documentation
coverage gap, embed the site in application packages, or perform package and
live-game verification owned by issues #81, #82, #84, and #77.
