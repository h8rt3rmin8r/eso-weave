# Data Model: Documentation Visualization Audit

## Audit manifest

- `schema_version`: integer `1`
- `issue`: integer `170`
- `summary_path`: exact publication authority
- `audited_at`: ISO date
- `pages`: ordered page records
- `candidates`: serious candidate records
- `clusters`: overlap decisions
- `form_considerations`: required category evidence

## Page record

- `path`: unique `docs/src/*.md` destination from `SUMMARY.md`
- `title`: visible page title
- `section`: `root`, `getting-started`, `features`, `concepts`, `reference`, or `development`
- `current_medium`: concise description of present prose, tables, and figures
- `decision`: `no_candidate`, `candidate`, or `covered_existing`
- `rationale`: nonempty reproducible decision
- `candidate_ids`: zero or more references to serious candidates

Every summary destination appears exactly once. Candidate references must resolve. A `candidate` decision has at least one reference; `no_candidate` has none.

## Visualization candidate

- `id`: stable kebab-case identifier
- `title`: maintainer-facing topic name
- `destination`: authoritative published page
- `supporting_pages`: other audited pages consolidated into the topic
- `reader_question`: one concrete trace, compare, locate, predict, diagnose, or explain task
- `entities`: four or more meaningful elements when that structural condition is used
- `relationships`: named branch, handoff, state, ownership, dependency, hierarchy, comparison, spatial, or quantitative relationships
- `structural_load`: exact qualifying condition and evidence
- `current_burden`: current backtracking, ambiguity, or mental-integration cost
- `gates`: independent booleans and explanations for reader task, structural load, payoff, and durability
- `style`: selected form or evaluated form for rejection
- `style_rationale`: why it fits the reader task
- `alternatives`: why prose, table, and relevant competing visual styles are weaker or better
- `authority`: maintained sources that determine content
- `text_equivalent`: complete adjacent prose or table plan
- `offline_delivery`: identical local delivery plan
- `update_trigger`: source changes that require refresh
- `decision`: `approved` or `rejected`
- `decision_rationale`: final warrant result
- `issue`: unique number and URL for approvals; null for rejections

## Candidate cluster

- `id`: stable identifier
- `candidate_id`: chosen serious candidate
- `pages`: audited pages consolidated into it
- `rationale`: why one destination prevents duplication

## Form consideration

- `form`: one of `relational`, `temporal`, `spatial`, `hierarchical`, `comparative`, `diagnostic`, or `quantitative`
- `candidate_ids`: candidates that supplied the evidence
- `outcome`: concise approval or rejection synthesis

All seven forms occur exactly once.
