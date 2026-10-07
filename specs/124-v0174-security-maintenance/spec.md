# Feature Specification: S124 Dependency Security and v0.17.4 Preparation

**Feature Branch**: `codex/s124-v0174-release`
**Created**: 2026-10-07
**Status**: Specified
**Input**: Owner-authorized S124, consolidating dependency maintenance and preparing the v0.17.4 candidate under autopilot with automatic push/PR and at most two requested Codex review rounds.
**Issues**: #261 runtime dependencies; #262 CodeQL pins; #263 release candidate/planning.

## User Scenarios & Testing

### User Story 1 - Receive current dependency security fixes (Priority: P1)

As an application user, I want the next candidate to include the known TLS security repair and pending dependency corrections without changing my established local data and access behavior.

**Why this priority**: The current rustls dependency has an open medium-severity advisory. The owner chose a consolidated maintenance slice rather than five independent update sessions.
**Independent Test**: The candidate dependency inventory includes every scoped update and excludes versions affected by GHSA-2mjx-qc3c-rqvc; existing automated behavior checks pass.

**Acceptance Scenarios**:
1. **Given** the affected TLS dependency, **When** S124 resolves the candidate dependencies, **Then** no affected rustls version remains in the resolved inventory.
2. **Given** existing credentials, query cursors, local HTTP/MCP clients and Lua imports, **When** dependencies change, **Then** their documented formats, authority rules and exact-value handling continue to pass existing automated checks.
3. **Given** a dependency incompatibility, **When** it is identified, **Then** narrowly required compatibility work is documented and checked rather than silently omitting the upgrade or weakening a gate.

### User Story 2 - Maintain reviewed scanner tooling (Priority: P2)

As a maintainer, I want both scanner actions updated together with reviewable version identity and the existing scoped permissions.

**Why this priority**: The pinned scanner update is a separate closeable outcome sharing the maintenance session.
**Independent Test**: Both reviewed action pins match their official version; repository policy and hosted scanner checks pass.

**Acceptance Scenarios**:
1. **Given** PR #247, **When** its update is consolidated, **Then** initialization and analysis reference the same official full commit identity and accurate version comments.
2. **Given** the workflow trust boundary, **When** pins change, **Then** existing triggers, job-scoped permissions and checkout credential behavior are preserved.

### User Story 3 - Review one accurate maintenance candidate (Priority: P3)

As the owner, I want a complete v0.17.4 candidate and chronological planning record, ready for final review and merge after automated checks and external reviews.

**Why this priority**: Combining candidate preparation avoids another session solely for version rollover.
**Independent Test**: Every governed candidate identity agrees; release notes are bounded; completed Plan049 is archived and active Plan050 directs S124; the official PR closes all three outcomes on owner merge.

**Acceptance Scenarios**:
1. **Given** completed S123 and the published v0.17.3 release, **When** planning advances, **Then** Plan049 preserves its actual merge/publication and follow-up packaging repair history, with all references resolved.
2. **Given** the completed maintenance changes, **When** governed rollover runs, **Then** crate, lockfile, badge, documentation identity/date, changelog and capture fixture consistently identify v0.17.4.
3. **Given** a review finding or failed check, **When** the PR is evaluated, **Then** necessary corrections are checked and all threads resolved before owner handoff, within two requested review rounds.

### Edge Cases

- A dependency target may be incompatible despite its small version change; record and resolve the actual incompatibility without broad unrelated upgrades.
- Another rustls instance may remain transitively vulnerable; inspect the complete resolved inventory, not only a direct entry.
- A future scanner tag may move; retain full commit pins and their reviewed provenance.
- A v0.17.4 tag or release may already exist; stop the identity collision rather than moving a tag.
- Existing Dependabot PRs may refresh; record exact selected revisions and close them as superseded only after owner merge of their replacement.
- Public advisory status on main may remain open while the patch is still in review; candidate dependency evidence does not claim main or a published package is fixed.

## Requirements

### Functional Requirements

- **FR-001**: The candidate MUST address the named rustls advisory across the complete resolved dependency inventory.
- **FR-002**: S124 MUST account for the ten scoped dependency upgrades from #257/#258/#239/#240, with provenance and narrowly required compatibility changes.
- **FR-003**: Existing local authority, credential generation, cursor encoding, exact Lua import behavior and all safety-critical behaviors MUST retain their automated coverage and documented contracts.
- **FR-004**: Both scanner action pins MUST adopt the reviewed #247 update while preserving the existing workflow authority and credential boundaries, with a dated changelog decision.
- **FR-005**: S124 MUST complete spec-kit design and blocking analysis before implementation, using matching branch, directory and issue identities.
- **FR-006**: Candidate preparation MUST use the existing governed release rollover for v0.17.4 with bounded Highlights and complete maintenance history.
- **FR-007**: Planning MUST archive completed Plan049, introduce chronological Plan050, and update indexes and machine/prose ledger references consistently.
- **FR-008**: Before owner handoff, every required automated gate MUST pass on the final PR head and every review finding MUST be answered/resolved, requesting at most two Codex review rounds.
- **FR-009**: S124 MUST stop at owner review/merge without tag or package publication; existing v0.17.3 assets and tag MUST remain intact.
- **FR-010**: Original dependency PRs MUST remain open until the replacement merges, with post-merge supersession recorded rather than premature closure.

### Key Entities

- **Scoped update**: Package/action name, old and selected target identity, source PR/revision and compatibility outcome.
- **Candidate**: Version, issue scope, exact source head, identity fields and automated/review evidence.
- **Build-plan record**: Chronological slice, governing issues, completed-plan evidence and active-plan links.

## Success Criteria

- **SC-001**: Zero resolved dependency versions fall in the named advisory's affected range.
- **SC-002**: All ten dependency upgrades and both scanner references are accounted for with no unreported omission.
- **SC-003**: All six governed candidate identity surfaces agree on v0.17.4; date-bearing fields agree on the preparation date, with valid bounded notes.
- **SC-004**: All mandatory automated checks pass and zero unresolved review findings remain at owner handoff.
- **SC-005**: Three independently closeable outcomes are linked to one official reviewed PR; historical/current planning links resolve.
- **SC-006**: No tag or package publication occurs during S124, and no original update PR closes before replacement merge.

## Assumptions

- S124 is candidate preparation, not authorization to publish a release tag or merge for the owner.
- Existing dependencies and product contracts are authoritative; no new product feature or schema migration is requested.
- Existing relevant automated tests are reused; add meaningful regression coverage only for demonstrated gaps or compatibility changes.
- Installed/game-session checks and verification-only issues are outside the owner's accepted workflow.

## Clarifications

### Session 2026-10-07

- Q: Combine all five dependency PRs or split newer and older upgrades? A: Combine the accepted ten package upgrades plus the two CodeQL references in S124, with exact provenance and narrowly required compatibility work.
- Q: Publish v0.17.4 during this slice? A: Prepare the candidate through the governed rollover, then stop at owner final review/merge. Later tag/publication needs separate authority.
- Q: Close original Dependabot PRs at PR creation? A: Keep them open until owner merge of the consolidated replacement; record them for post-merge housekeeping.
