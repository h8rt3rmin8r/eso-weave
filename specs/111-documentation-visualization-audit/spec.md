# Feature Specification: Documentation Visualization Audit

**Feature Branch**: `codex/s111-documentation-visualization-audit`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S111 implements GitHub issue #170 after Plan 046 completed through S110.

## Clarifications

### Session 2026-09-17

- Q: What is the authoritative published-page inventory? -> A: Every Markdown destination linked by `docs/src/SUMMARY.md`, including section landing pages and the documentation root. The current exact inventory is 49 pages.
- Q: What constitutes a serious visualization candidate? -> A: A topic that passes the named-reader-task gate and at least one structural-load condition strongly enough to warrant recording all four issue-defined gates. Every page still receives an explicit page-level decision even when it produces no serious candidate.
- Q: Does an approved candidate create artwork in S111? -> A: No. S111 records the evidence and creates one atomic implementation issue per approved candidate. Artwork, page integration, and visual verification belong to those later issues.
- Q: How are overlapping topics handled? -> A: Cluster them around one authoritative destination and reader question. Supporting pages reference that candidate instead of proposing duplicate graphics.
- Q: Which media are permitted? -> A: The audit may recommend static local SVG diagrams, generated charts, or maintained annotated captures only when the source is durable, the public and bundled documentation can ship identical local bytes, and a complete adjacent text equivalent is practical.
- Q: How is audit completeness kept from drifting? -> A: A machine-readable manifest joins exactly to `SUMMARY.md`, while documentation policy tests reject missing or extra pages, incomplete candidate evidence, duplicate approvals, and approved candidates without linked implementation issues.
- Q: How are existing figures treated? -> A: Existing diagrams, screenshots, illustrations, and brand samples count as current coverage. S111 may reject a new candidate because an existing figure already serves the reader task, but it does not reconstruct or modify those assets.

## User Scenarios and Testing

### User Story 1 - Reproduce every page decision (Priority: P1)

A maintainer can select any published page and find a complete audit entry that explains whether it needs a new visualization and why.

**Why this priority**: Complete and reproducible coverage is the central outcome of issue #170. A list of only interesting pages would hide omissions and selection bias.

**Independent Test**: Compare the machine-readable audit page paths with every Markdown destination in `SUMMARY.md`, then inspect any row and trace its decision to a serious candidate or a concrete prose, table, existing-figure, or low-load rationale.

**Acceptance Scenarios**:

1. **Given** the published navigation contains 49 Markdown pages, **when** the audit validator runs, **then** it finds exactly 49 unique page records with no missing or extra path.
2. **Given** a page has no serious candidate, **when** its record is inspected, **then** it states why prose, tables, existing figures, or simple navigation remain the better medium.
3. **Given** several pages discuss one system, **when** their records are compared, **then** they point to one clustered candidate or explain distinct reader tasks.

### User Story 2 - Evaluate serious candidates consistently (Priority: P1)

A documentation reviewer can reproduce each approval or rejection from the same visualization warrant rather than from a graphics quota.

**Why this priority**: The audit is useful only if its decisions are evidence-based, comparable, and durable.

**Independent Test**: Select any serious candidate and verify a named reader question, structural-load evidence, current comprehension burden, medium comparison, authority, accessible equivalent, offline approach, update trigger, and decision rationale.

**Acceptance Scenarios**:

1. **Given** a candidate is approved, **when** its evidence is reviewed, **then** all four warrant gates pass and the selected graphic style is justified against prose, tables, and plausible alternative styles.
2. **Given** a candidate is rejected, **when** its record is reviewed, **then** at least one failed gate or superior existing medium is explicit.
3. **Given** the issue requests broad visual-form consideration, **when** the audit summary is read, **then** relational, temporal, spatial, hierarchical, comparative, diagnostic, and quantitative forms are each considered without requiring an approval in every category.

### User Story 3 - Hand off approved work atomically (Priority: P2)

A future contributor can implement any approved visualization from one focused GitHub issue without rediscovering its reader task, authority, accessibility contract, or dependencies.

**Why this priority**: Approved candidates have no delivery value if their implementation boundary remains ambiguous or bundled together.

**Independent Test**: For every approved manifest candidate, open its unique GitHub issue and verify the issue carries the same destination, reader question, style, source authority, text equivalent, offline boundary, update trigger, acceptance criteria, and relationship to #170.

**Acceptance Scenarios**:

1. **Given** an approved candidate, **when** S111 completes, **then** exactly one open atomic implementation issue is linked from its manifest record.
2. **Given** a rejected candidate, **when** S111 completes, **then** it has no implementation issue created by the audit.
3. **Given** approved candidates share a dependency or authoritative topic, **when** their issues are inspected, **then** that relationship is recorded without combining their distinct reader tasks.

### Edge Cases

- A section landing page with only navigation still receives an explicit no-candidate decision.
- A long page with many tables is not automatically approved when exact lookup remains the reader task.
- A topic spread across several pages may yield one candidate at its authoritative page rather than repeated copies.
- An existing figure may fully cover a topic even if supporting pages contain no image themselves.
- Volatile values without a deterministic generator fail durability even when a chart would look useful.
- A screenshot candidate fails when the reader task is conceptual rather than spatial.
- A quantitative topic remains tabular when exact values matter more than trends or distribution.
- An approved issue number cannot be reused by a second candidate.
- A future `SUMMARY.md` page makes policy fail until the audit is deliberately refreshed.

## Requirements

### Functional Requirements

- **FR-001**: The audit MUST inventory every unique Markdown page linked from `docs/src/SUMMARY.md` and no unpublished page.
- **FR-002**: Every page record MUST name its title, section, audit decision, rationale, current medium, and zero or more serious candidate identifiers.
- **FR-003**: Every serious candidate MUST identify a concrete reader question, meaningful entities, relationship types, structural-load evidence, and current comprehension burden.
- **FR-004**: Every candidate MUST evaluate named-reader-task, structural-load, comprehension-payoff, and durability gates independently.
- **FR-005**: Every candidate MUST select and justify a graphic style or rejection, compare the choice with prose, tables, and relevant alternative graphics, and avoid decorative rationale.
- **FR-006**: Every candidate MUST identify an authoritative maintenance source, complete adjacent text-equivalent approach, identical offline-delivery approach, and expected update trigger.
- **FR-007**: The audit MUST include representative rejected candidates demonstrating that simple sequences, exact lookup tables, existing coverage, volatile data, and low structural load do not qualify.
- **FR-008**: The audit MUST explicitly consider relational, temporal, spatial, hierarchical, comparative, diagnostic, and quantitative forms without requiring an approved candidate in each category.
- **FR-009**: Overlapping candidates MUST be clustered around one authoritative destination, with supporting pages referencing the same candidate.
- **FR-010**: Every approved candidate MUST link exactly one separately created atomic GitHub implementation issue, and rejected candidates MUST link none.
- **FR-011**: Each follow-up issue MUST preserve the manifest reader question, destination, style, maintenance source, accessible equivalent, offline boundary, update trigger, dependencies, and acceptance criteria.
- **FR-012**: S111 MUST NOT implement, reconstruct, or modify documentation graphics, figure-system behavior, browser rendering, or published prose beyond links necessary to maintain the audit record.
- **FR-013**: Documentation policy MUST reject incomplete page coverage, path drift, duplicate records, unknown candidate references, incomplete warrant evidence, duplicate issue links, or approvals without valid issue metadata.
- **FR-014**: The human audit record MUST summarize scope, method, approvals, representative rejections, clustering decisions, form coverage, maintenance expectations, and follow-up issues.
- **FR-015**: Audit records and generated issue bodies MUST use UTF-8 without BOM, LF line endings, valid links, no mojibake, and no forbidden dash characters.
- **FR-016**: S111 MUST archive Plan 046 with PR #219 delivery evidence, establish Plan 047, update the migration lifecycle ledger, and record the audit in the `[Unreleased]` changelog.
- **FR-017**: Existing documentation policy, mdBook, render, figure, table, syntax, link, trust, and repository tests MUST remain passing without weakening assertions.
- **FR-018**: S111 MUST publish an official pull request that closes #170, process every review comment, invoke no more than the authorized second Codex review, and preserve operator merge authority.

### Key Entities

- **Page record**: One exact published Markdown destination, its present medium, page-level decision, rationale, and serious candidate references.
- **Visualization candidate**: One named reader task and proposed or rejected visual response evaluated against the complete warrant.
- **Candidate cluster**: A set of overlapping page topics consolidated at one authoritative destination.
- **Form consideration**: Evidence that one requested visual category was evaluated even if no candidate using it was approved.
- **Implementation issue**: One atomic GitHub issue created only for an approved candidate and carrying the complete handoff contract.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Policy proves a one-to-one join between all 49 current `SUMMARY.md` Markdown destinations and 49 page records.
- **SC-002**: Every serious candidate contains all four warrant gates and every required maintenance, accessibility, offline, alternative, and update field.
- **SC-003**: All seven required form categories have explicit consideration evidence, with representative approvals and rejections demonstrating no quota.
- **SC-004**: Every approved candidate has one unique, open, atomic GitHub issue and every rejected candidate has none.
- **SC-005**: A reviewer can select any published page and reproduce its decision using only the audit record and linked source authority.
- **SC-006**: Documentation unit tests, mdBook test/build, rendered-site policy, trust policy, typo scan, UTF-8, LF, mojibake, forbidden-dash, and diff checks pass.

## Assumptions

- `docs/src/SUMMARY.md` remains the publication authority for Markdown pages.
- Existing source inventory and figure manifests remain authoritative signals, not automatic visualization approvals.
- Follow-up issue creation is an intended deliverable of issue #170 and is authorized within this S111 work slice.
- Approved issue implementation will use the existing local, accessible, offline figure system unless its own spec proves a narrower compatible mechanism.

## Out of Scope

- Creating or editing SVGs, screenshots, charts, CSS, JavaScript, or figure interactions.
- Reconstructing the four existing flow diagrams completed under issue #168.
- Adding remote assets, browser dependencies, telemetry, or public/bundled documentation divergence.
- Performing installed-game verification for issues #110, #129, #131, or #190.
