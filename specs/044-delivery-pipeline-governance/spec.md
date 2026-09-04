# Feature Specification: Delivery Pipeline Governance

**Feature Branch**: `codex/044-delivery-pipeline-governance`

**Created**: 2026-09-03

**Status**: Ready for Planning

**Input**: Deliver issues #45, #46, and #47 as one governance slice that makes issue scope, pull request closure, historical metadata, and the GitHub Project delivery pipeline durable.

## User Scenarios & Testing

### User Story 1 - Close atomic work predictably (Priority: P1)

As a maintainer, I want every implementation pull request to identify the discrete issues it closes so that a merge records real progress without manual reconstruction.

**Why this priority**: Reliable issue closure is the foundation for milestones, progress reporting, and downstream Project automation.

**Independent Test**: Submit representative pull request bodies to the policy checker and verify that supported GitHub closing keywords pass, descriptive references fail, and documented exemptions pass.

**Acceptance Scenarios**:

1. **Given** a normal pull request targeting `main`, **When** its body contains `Closes #45`, **Then** the issue-link check passes.
2. **Given** a pull request that closes several issues, **When** each issue has its own complete closing reference, **Then** the check reports every detected issue.
3. **Given** a normal pull request with only `Related to #45`, **When** the check runs, **Then** it fails with corrective guidance.
4. **Given** an administrative or dependency pull request covered by a documented exemption, **When** the check runs, **Then** the exemption and reason are reported.

---

### User Story 2 - See the delivery pipeline at a glance (Priority: P2)

As a project owner, I want one minimal GitHub Project with a truthful delivery stage for every issue so that I can see backlog, active work, review, release verification, and completed work without opening each issue.

**Why this priority**: A visible pipeline turns otherwise dense issue and milestone metadata into an operational view of current progress.

**Independent Test**: Open the project table and board views, confirm all repository issues are present, and verify representative open, verification, and closed issues occupy the prescribed stages.

**Acceptance Scenarios**:

1. **Given** the repository issue inventory, **When** the delivery project is opened, **Then** every issue is present exactly once.
2. **Given** a closed issue, **When** its project item is inspected, **Then** its Stage is `Done`.
3. **Given** an issue waiting for release verification, **When** its project item is inspected, **Then** its Stage is `Release verification`.
4. **Given** the project board, **When** work moves through the pipeline, **Then** columns follow the defined Stage order.

---

### User Story 3 - Trust historical progress metadata (Priority: P3)

As a maintainer reviewing project history, I want milestones and verification labels to reflect actual delivery evidence so that historical progress is not distorted by stale or speculative metadata.

**Why this priority**: Historical accuracy supports planning, but it depends on the closure and pipeline conventions established by the first two stories.

**Independent Test**: Run the documented audit against every repository issue and compare milestone assignment and verification labels with merge, release, and issue-state evidence.

**Acceptance Scenarios**:

1. **Given** all repository issues, **When** the audit is completed, **Then** each issue has a recorded audit outcome and every discovered mismatch is corrected or explicitly documented.
2. **Given** a closed implementation issue, **When** no further issue-level field proof is required, **Then** it does not retain `needs: verification`.
3. **Given** an open issue whose completion depends on unavailable field or release evidence, **When** the label audit runs, **Then** `needs: verification` remains with a documented reason.

### Edge Cases

- A pull request can close multiple atomic issues, but every issue requires a complete GitHub closing reference.
- A dependency update or project-wide administrative pull request may legitimately have no issue, but only through a narrow, visible exemption.
- Text resembling a closing reference without a supported GitHub keyword must not pass.
- Pull request text is untrusted input and must never be interpolated into executable script source.
- A launcher, release, or verification issue can remain open after implementation has merged; implementation and field verification must use separate issue lifecycles.
- Epics remain open while any required child outcome remains open.
- Missing release evidence must be represented as a verification stage, not guessed as completed.
- The GitHub Project may expose default fields that duplicate repository labels or milestones; redundant custom fields must not be added.

## Requirements

### Functional Requirements

- **FR-001**: The repository MUST define one actionable issue as one independently closeable outcome with one lifecycle.
- **FR-002**: Implementation and release or field verification MUST use separate issues when they can complete at different times.
- **FR-003**: Multi-outcome initiatives MUST be represented as epics with ordered child issues and explicit dependencies where sequencing matters.
- **FR-004**: The pull request template MUST require a GitHub closing reference and explain the full-keyword requirement for multiple issues.
- **FR-005**: A CI check MUST flag pull requests targeting `main` that contain no supported same-repository closing reference.
- **FR-006**: The CI check MUST accept GitHub-supported `close`, `fix`, and `resolve` keyword variants, optional colons, and case differences.
- **FR-006a**: The CI check MUST ignore closing-reference examples inside HTML comments.
- **FR-007**: Exemptions MUST be limited to Dependabot, the `dependencies` label, or the documented `skip: issue-link` label.
- **FR-008**: The CI implementation MUST treat pull request bodies, authors, branches, and labels as untrusted input, use read-only permissions, and avoid `pull_request_target`.
- **FR-008a**: The CI implementation MUST enforce the policy from the trusted base commit after the one-time S044 bootstrap.
- **FR-009**: The issue-link policy MUST have dependency-free automated tests covering pass, fail, multi-issue, capitalization, colon, and exemption cases.
- **FR-010**: The repository MUST document atomic issue scope, implementation-versus-verification lifecycles, pull request linkage, exemptions, and post-merge housekeeping.
- **FR-011**: A public GitHub Project named `ESO Weave Delivery` MUST be linked to the repository.
- **FR-012**: The Project MUST contain a `Stage` single-select field ordered as `Backlog`, `Ready`, `Specced`, `In progress`, `PR review`, `Release verification`, and `Done`.
- **FR-013**: The Project MUST contain a `Slice` text field for known spec-kit slice identifiers.
- **FR-014**: The Project MUST provide a table view and a board view grouped by Stage without duplicating Priority, Effort, Area, or Milestone as custom fields.
- **FR-015**: Every repository issue MUST be added to the Project exactly once and assigned a Stage based on its actual lifecycle state.
- **FR-016**: Every repository issue MUST be audited against merge, release, milestone, and issue-state evidence for milestone chronology and `needs: verification` accuracy.
- **FR-017**: Audit corrections MUST preserve truthful uncertainty instead of inferring release or field evidence that does not exist.
- **FR-018**: The slice MUST record its workflow and governance decisions in the changelog and chronological build-plan index.
- **FR-019**: The slice MUST NOT modify Rust product behavior or cut a release.

### Key Entities

- **Atomic issue**: One independently closeable repository outcome, including its type, milestone, labels, parent, dependencies, and verification needs.
- **Pull request linkage decision**: The detected closing issue numbers or the documented exemption that determines whether CI passes.
- **Delivery project item**: A GitHub issue represented once in the delivery Project with Stage and optional Slice values.
- **Audit finding**: Evidence and disposition for an issue's milestone chronology and verification-label state.
- **Epic**: A parent issue that summarizes progress while child issues own discrete implementation or verification outcomes.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Automated policy tests pass for all documented keyword and exemption cases, and a pull request with no closing reference fails with actionable guidance.
- **SC-002**: 100 percent of repository issues are represented exactly once in `ESO Weave Delivery` with a non-empty Stage.
- **SC-003**: 100 percent of closed project items are in `Done`, and 100 percent of open issues carrying `needs: verification` are in `Release verification`.
- **SC-004**: 100 percent of repository issues receive a milestone and verification-label audit disposition, with every identified mismatch corrected or documented.
- **SC-005**: Issues #45, #46, and #47 meet their acceptance criteria and are linked for automatic closure by the S044 pull request.
- **SC-006**: Repository CI, formatting, linting, tests, and documentation checks remain green without product-code changes.

## Assumptions

- GitHub's default-branch closing-keyword behavior is the authoritative issue-closure mechanism.
- The issue-link check validates syntax and policy presence; maintainers remain responsible for confirming that referenced issues are semantically correct and atomic.
- `skip: issue-link` is exceptional and must be visible in the pull request labels and rationale.
- Historical issues can be assigned `Done` without fabricating a slice code when no reliable slice mapping exists.
- Release-verification issues remain open until real release and field evidence is available.
- GitHub CLI and the GitHub web interface may both be needed because Project view configuration is not fully exposed by supported CLI commands.
