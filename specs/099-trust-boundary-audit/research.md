# Research: Repository Trust-Boundary Audit

## Decision 1: Use protected `main` as the project-policy trust anchor

**Decision**: Agent guidance from protected `main` may constrain an authorized task. Proposed branch guidance is review data until merge and cannot expand authority.

**Rationale**: Trusting the active checkout lets a pull request rewrite the rules used to review itself. Ignoring repository guidance entirely loses legitimate project constraints. A protected base provides a reviewable middle boundary while the runtime's higher-priority operator and platform rules remain authoritative.

**Alternatives considered**:

- Trust every checked-out `CLAUDE.md` and skill: rejected because branch content is untrusted before merge.
- Ignore all repository guidance: rejected because the project constitution and safety gates are required constraints.
- Sign every guidance file separately: rejected as disproportionate while GitHub branch protection and immutable history already provide a trust anchor.

## Decision 2: Remove unrelated high-risk local skills and scan metadata

**Decision**: Remove project-local skills dedicated to offensive bypass or evasion and reject comparable trigger descriptions through the trust-policy checker.

**Rationale**: ESO Weave does not require those capabilities. Auto-discoverable high-risk guidance expands prompt and tool risk without supporting the product. Retaining the broader development skill library remains useful and proportionate.

**Alternatives considered**:

- Remove all local skills: rejected because many are deliberate project development aids.
- Keep the skills but add warnings: rejected because auto-discovery can occur before a warning is considered.
- Maintain only a manual filename denylist: rejected because the same category can be reintroduced under a different directory name.

## Decision 3: Pin Action commits and restrict publishers

**Decision**: Use exact 40-character commits for every remote Action, require readable version comments, and allow only GitHub-owned actions plus `Swatinem/rust-cache` in repository settings.

**Rationale**: Major and minor tags are moving references. Exact commits make reviewed workflow code stable, while publisher restriction limits future dependency expansion.

**Alternatives considered**:

- Keep major tags and rely on trusted publishers: rejected because tags can move and repository settings already expose SHA enforcement.
- Vendor Action code: rejected as a large maintenance and provenance burden.
- Allow all verified Marketplace actions: rejected because the current repository needs only one third-party publisher.

## Decision 4: Verify the moving AppImage asset by digest

**Decision**: Keep the upstream `continuous` URL but pin the accepted SHA-256 digest and verify it before execution.

**Rationale**: The upstream project does not publish a conventional versioned release for the required asset. Digest verification converts a moving URL into fail-closed content identity without vendoring the binary.

**Alternatives considered**:

- Execute the moving asset without verification: rejected because release artifacts could change without repository review.
- Vendor the AppImage builder: rejected because a large binary does not belong in this repository.
- Replace AppImage tooling in S099: rejected as unrelated packaging redesign.

## Decision 5: Protect `main` without an impossible approval rule

**Decision**: Require pull requests, strict CI, resolved conversations, linear history, administrator enforcement, and destructive-operation blocking, but require zero independent approvals at the GitHub setting level.

**Rationale**: The repository has one collaborator, who also authors and merges feature PRs. GitHub does not permit authors to approve their own PRs. A one-approval rule would deadlock maintenance. The final human review ritual remains documented and observable.

**Alternatives considered**:

- Require one approval: rejected because the current collaborator topology cannot satisfy it.
- Leave `main` unprotected: rejected because policy-only merge gates are bypassable.
- Add a bot approval as a substitute: rejected because automated review is advisory and must not become operator authority.

## Decision 6: Keep detailed evidence outside public artifacts

**Decision**: Commit control objectives, policy code, tests, and high-level outcomes only. Keep detailed finding evidence in the operator-approved session.

**Rationale**: Issue #181 explicitly requires bounded disclosure. Reviewers can evaluate the remediation without reusable abuse procedures.
