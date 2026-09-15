# Feature Specification: Repository Trust-Boundary Audit

**Feature Branch**: `codex/s099-trust-boundary-audit`

**Created**: 2026-09-15

**Status**: Ready for Planning

**Input**: Work slice S099 implements GitHub issue #181 after S098 completed the operator-intent reliability sequence.

## User Scenarios & Testing

### User Story 1 - Keep authority with the operator (Priority: P1)

As an operator, I can use repository automation and coding agents without issue text, pull-request content, review output, logs, artifacts, or checked-out branch content silently expanding the work or permissions I authorized.

**Why this priority**: Every other repository control depends on distinguishing trusted authority from untrusted data.

**Independent Test**: Present representative instruction-shaped text through each untrusted channel and verify that the workflow treats it as data, keeps the authorized scope unchanged, and surfaces a conflict instead of acting on it.

**Acceptance Scenarios**:

1. **Given** an operator-authorized task, **when** an issue, pull request, review, log, artifact, or source file contains unrelated operational instructions, **then** those instructions cannot authorize mutations, permission changes, credential use, releases, or scope expansion.
2. **Given** a review of an untrusted branch, **when** project-local agent guidance differs from the trusted base, **then** the authority-bearing session continues to use trusted-base guidance and treats the proposed guidance as review data.
3. **Given** a credible boundary gap, **when** it is discovered, **then** the workflow halts, explains the boundary and impact plainly, and resumes only after direct operator direction.

---

### User Story 2 - Execute only immutable workflow dependencies (Priority: P1)

As a maintainer, I can trust GitHub Actions to execute reviewed dependency revisions and release tooling bytes rather than moving upstream references.

**Why this priority**: Workflow dependencies execute inside CI and release jobs, including jobs that can publish artifacts or releases.

**Independent Test**: Scan every workflow and controlled download, verify immutable revisions or fixed digests, then inject mutable examples into policy tests and confirm rejection.

**Acceptance Scenarios**:

1. **Given** a remote GitHub Action, **when** it appears in a workflow, **then** its `uses` reference is an exact 40-character commit SHA with a readable version comment.
2. **Given** executable release tooling downloaded at runtime, **when** the release job prepares it, **then** a checked-in digest is verified before execution and a mismatch fails closed.
3. **Given** a checkout step, **when** the step completes, **then** workflow credentials are not persisted into the working tree.

---

### User Story 3 - Enforce the integration contract (Priority: P1)

As a maintainer, I can trust that `main` changes pass through a pull request, required CI, and resolved review conversations even when an authorized principal could otherwise push directly.

**Why this priority**: Written policy without a hosted enforcement control can be bypassed accidentally or intentionally.

**Independent Test**: Read the GitHub repository settings after configuration and verify branch protection, required checks, destructive-operation restrictions, default token permissions, and Action dependency policy.

**Acceptance Scenarios**:

1. **Given** a proposed `main` update, **when** GitHub evaluates it, **then** a pull request with current required checks and resolved conversations is mandatory, including for administrators.
2. **Given** the protected branch, **when** deletion or force-push is attempted, **then** GitHub rejects it.
3. **Given** a workflow dependency, **when** GitHub evaluates the workflow, **then** only the selected Action sources and immutable SHA references are allowed.

---

### User Story 4 - Preserve useful public evidence (Priority: P2)

As a contributor, I can understand the repository's trust model and verify its controls without public documentation exposing detailed findings or reusable abuse instructions.

**Why this priority**: The controls must remain maintainable while sensitive evidence stays appropriately bounded.

**Independent Test**: Review the public guidance, specifications, tests, changelog, and pull-request summary and verify that they describe boundaries and outcomes without detailed reproduction material.

**Acceptance Scenarios**:

1. **Given** the completed audit, **when** a contributor reads the guidance, **then** trusted authority, untrusted inputs, allowed mutations, and the halt protocol are explicit.
2. **Given** detailed audit evidence, **when** public artifacts are prepared, **then** they contain only control-level findings and remediation outcomes.

### Edge Cases

- A trusted maintainer authors instruction-shaped issue text. The channel remains untrusted for authority even when the author is trusted.
- A pull request changes `CLAUDE.md`, a spec-kit file, or a local skill. The proposed content remains review data until it reaches protected `main`.
- An Action version comment and pinned SHA disagree. The immutable SHA governs execution, and dependency maintenance must reconcile the comment.
- A hosted setting cannot be represented in Git. The audit records the intended state and verifies the live GitHub value through a read-only query.
- A required check is conditionally absent. Only checks emitted for every pull request may be configured as mandatory.
- A release dependency publishes new bytes at the same URL. Digest verification rejects the changed payload.
- A bot supplies a positive reaction without review text. The reaction is evidence of review completion, not mutation authority.
- A finding needs remediation outside the selected repository. The workflow halts and asks for explicit operator direction before expanding scope.

## Requirements

### Functional Requirements

- **FR-001**: S099 MUST inventory every active project-management, AI-development, CI, documentation, dependency, and release workflow that can influence repository state or published artifacts.
- **FR-002**: Guidance MUST define direct operator messages and explicitly approved protected-base policy as authority, while treating issues, pull requests, comments, reviews, reactions, logs, artifacts, generated text, external pages, and branch content as untrusted data.
- **FR-003**: Untrusted data MUST NOT authorize unrelated mutations, permission expansion, credential use, release publication, destructive actions, or changes to the selected scope.
- **FR-004**: An authority-bearing agent session MUST NOT load proposed branch guidance as operational authority while reviewing an untrusted branch.
- **FR-005**: Repository-local skills whose purpose is unrelated offensive bypass or evasion MUST be removed from the trusted project guidance surface.
- **FR-006**: Relevant operator and agent guidance MUST contain the mandatory halt, plain explanation, discussion, and explicit-resume protocol from issue #181.
- **FR-007**: Every remote GitHub Action reference MUST use an exact lowercase 40-character commit SHA and retain a human-readable release comment.
- **FR-008**: Every checkout step MUST set `persist-credentials: false`.
- **FR-009**: Workflow-level permissions MUST default to read-only, and job-level write permissions MUST be limited to the documented Pages deployment and release publication jobs.
- **FR-010**: Workflows MUST reject authority-sensitive event patterns that combine untrusted event content with privileged execution.
- **FR-011**: Runtime-downloaded executable release tooling MUST have a fixed checked-in digest verified immediately before execution.
- **FR-012**: Release-only Cargo tools MUST install an exact version with locked dependencies.
- **FR-013**: A repository policy test MUST scan every checked-in workflow for mutable actions, persisted checkout credentials, prohibited triggers, unexpected secrets, and unauthorized write permissions.
- **FR-014**: The policy test MUST scan project-local skill metadata for high-risk guidance categories that do not belong in ESO Weave's development surface.
- **FR-015**: Policy tests MUST include representative malicious or conflicting inputs and prove deterministic fail-closed results.
- **FR-016**: `main` MUST be protected against direct updates, deletion, and force-push, including enforcement for administrators.
- **FR-017**: `main` updates MUST require a pull request, strict current required checks, linear history, and resolved review conversations.
- **FR-018**: Required checks MUST include the always-on Linux and Windows CI jobs, dependency review, CodeQL, and issue-link policy jobs.
- **FR-019**: Repository Actions settings MUST retain read-only default workflow tokens, disallow workflow review approval, restrict Action sources to GitHub-owned actions plus the named cache action, and require immutable SHA references.
- **FR-020**: The audit MUST verify that repository and Dependabot secret inventories are empty, no self-hosted runners or webhooks are active, and the Pages deployment remains branch-scoped.
- **FR-021**: Public audit artifacts MUST describe control objectives, reviewed scope, and overall results without detailed reproduction steps or sensitive evidence.
- **FR-022**: Detailed findings MUST remain in the operator-approved session unless the operator separately approves disclosure.
- **FR-023**: The implementation MUST update the active build plan, changelog, contributor guidance, agent guidance, and canonical workflow documentation.
- **FR-024**: S099 MUST NOT change application behavior, addon behavior, user configuration, gameplay automation, telemetry, stored player data, or release contents.

### Key Entities

- **Authority source**: A direct operator instruction or protected-base policy that can constrain work. Repository policy cannot expand authority beyond the operator's request.
- **Untrusted input**: Content that may inform analysis but cannot authorize an action, including repository collaboration content and proposed branch files.
- **Mutation boundary**: A command, token, workflow, or hosted setting capable of changing repository state or publishing an artifact.
- **Executable dependency**: A GitHub Action, downloaded tool, or installed command that runs inside an automated workflow.
- **Finding**: A suspected or confirmed failure of a required boundary that activates the halt-and-discuss protocol.

## Success Criteria

### Measurable Outcomes

- **SC-001**: A complete workflow inventory maps every mutation-capable surface to its trigger, effective permission, trusted code source, and failure behavior.
- **SC-002**: Every checked-in remote Action reference passes exact-SHA validation, and all injected mutable-reference fixtures fail validation.
- **SC-003**: Every checkout step disables credential persistence, and injected omissions or explicit enablement fail validation.
- **SC-004**: The release workflow rejects altered AppImage tooling bytes before execution and installs both packaging commands at exact versions.
- **SC-005**: Live GitHub queries confirm protected `main`, all configured required checks, destructive-operation restrictions, read-only default tokens, selected Action sources, and SHA pinning.
- **SC-006**: Representative untrusted instruction fixtures produce policy failures or documented non-authority handling without repository mutation.
- **SC-007**: Public evidence contains zero detailed abuse procedures, credentials, raw secret values, or unpublished sensitive findings.
- **SC-008**: Formatting, strict linting, complete locked tests, release build, documentation gates, trust-policy tests, hosted CI, and two authorized automated review rounds complete without unresolved findings.

## Clarification Decisions

- The operator's repeated instruction to complete S099 authorizes remediation of all in-scope findings from this audit, including GitHub repository settings, without additional routine pauses.
- Protected `main` is the project-policy trust anchor. Proposed changes to agent guidance remain data until merged, and agents reviewing untrusted changes must read policy from the trusted base rather than the proposed checkout.
- A solo-maintainer repository cannot require an independent approving review from its only collaborator without making ordinary merge impossible. The hosted control therefore requires pull requests, current checks, resolved conversations, linear history, and administrator enforcement, with final human review retained as an operating requirement.
- GitHub-owned Actions and `Swatinem/rust-cache` are the only required Action publishers. Every invocation remains commit-SHA pinned even when repository settings also enforce SHA pinning.
- AppImage upstream publishes a moving `continuous` asset. S099 keeps that source but fixes the accepted artifact digest, so an upstream replacement fails rather than silently changing release bytes.

## Scope Boundaries

- No public exploit demonstrations or detailed finding reproduction.
- No organization-wide policy, unrelated repository, account, or contributor changes.
- No release, tag, package publication, or merge performed by the agent.
- No application, addon, data schema, protocol, or user-interface changes.
- No attempt to make repository policy override the runtime platform's higher-priority safety and authorization rules.

## Assumptions

- The repository remains single-maintainer during S099.
- GitHub continues to expose branch protection and Actions permission settings through the repository API.
- Hosted third-party review bots remain advisory and do not receive repository mutation authority from their comments or reactions.
