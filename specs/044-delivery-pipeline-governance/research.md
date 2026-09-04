# Research: Delivery Pipeline Governance

## Decision 1: Use GitHub closing keywords as the closure contract

**Decision**: Accept `close`, `closes`, `closed`, `fix`, `fixes`, `fixed`, `resolve`, `resolves`, and `resolved`, with case differences and an optional colon before `#N`.

**Rationale**: GitHub automatically closes linked issues only when a pull request targeting the default branch is merged. Matching that native behavior avoids a second, conflicting closure system.

**Alternatives considered**:

- Accept any issue reference. Rejected because `Related to #N` does not close an issue.
- Close issues through a write-capable workflow. Rejected because native merge behavior is safer and more transparent.

**Source**: GitHub Docs, [Linking a pull request to an issue](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/linking-a-pull-request-to-an-issue).

## Decision 2: Use a dependency-free policy module and targeted CI job

**Decision**: Implement parsing and exemption logic as an ECMAScript module with Node's built-in test runner. Run it in a dedicated `pull_request` workflow on `main`.

**Rationale**: The policy is small, deterministic, fast, and locally testable. Avoiding packages removes supply-chain and lockfile maintenance for a governance check.

**Alternatives considered**:

- A marketplace issue-link action. Rejected because the required behavior is smaller than the dependency and needs repository-specific exemptions.
- An inline shell expression. Rejected because it is harder to test and creates avoidable injection risk.
- API validation of referenced issue state. Rejected for this slice because syntax enforcement is the lightweight control requested; semantic correctness remains a review responsibility.

## Decision 3: Treat all pull request metadata as untrusted

**Decision**: Pass body, author, and labels through environment variables, grant only `contents: read`, and never use `pull_request_target`.

**Rationale**: GitHub explicitly identifies pull request titles, bodies, branches, and similar context as attacker-controlled. Environment transport prevents metadata from becoming executable script source.

The workflow tests the proposed policy copy but enforces the copy checked out
from the base commit. S044 has a one-time bootstrap fallback because the base
does not contain the new policy; once merged, normal pull requests cannot weaken
the evaluator and its tests in the same change.

**Source**: GitHub Docs, [Script injections](https://docs.github.com/en/actions/concepts/security/script-injections) and [Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use).

## Decision 4: Keep exemptions narrow and visible

**Decision**: Exempt Dependabot, pull requests labeled `dependencies`, and pull requests labeled `skip: issue-link`.

**Rationale**: Dependency automation and rare repository administration can lack an atomic implementation issue. A visible label is preferable to hidden title or branch heuristics.

**Alternatives considered**:

- Exempt all documentation or workflow changes. Rejected because those changes can and usually should close issues.
- Permit arbitrary magic text in the body. Rejected because it is easy to overlook and difficult to audit.

## Decision 5: Use a minimal Project schema

**Decision**: Create one public project with a `Stage` single-select field and a `Slice` text field. Reuse issue labels and milestones for type, priority, effort, area, and release.

**Rationale**: GitHub Projects already surfaces issue metadata. Duplicate custom fields drift and make maintenance harder. Stage and Slice are the only missing cross-issue dimensions.

**Source**: GitHub Docs, [About Projects](https://docs.github.com/en/issues/planning-and-tracking-with-projects) and [Customizing the board layout](https://docs.github.com/en/issues/planning-and-tracking-with-projects/customizing-views-in-your-project/customizing-the-board-layout).

## Decision 6: Model the delivery lifecycle explicitly

**Decision**: Order stages as Backlog, Ready, Specced, In progress, PR review, Release verification, and Done.

**Rationale**: These stages distinguish definition, implementation, code review, and real-world verification. They directly address the repeated failure where merged implementation was mistaken for verified completion.

**Alternatives considered**:

- Use only To do, In progress, and Done. Rejected because it hides specification, review, and release verification.
- Add separate QA, blocked, and waiting columns. Rejected because dependencies and labels already express those concerns and the requested Project should remain minimal.

## Decision 7: Audit history from evidence, not issue prose

**Decision**: Compare each issue's current state, milestone, labels, linked merged pull requests, repository tags, and release dates. Correct only evidence-backed mismatches.

**Rationale**: Issue descriptions may have been rewritten or filed retroactively. Merge and release records are stronger chronological evidence, while unavailable field evidence must remain visibly unresolved.

## Decision 8: Use native epic hierarchy and dependencies

**Decision**: Keep independently closeable outcomes as child issues and record ordering with native sub-issues and blocked-by relationships.

**Rationale**: Native hierarchy and dependency data is visible in issues and Projects without embedding fragile checkbox state in prose.

**Source**: GitHub Docs, [Adding sub-issues](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/adding-sub-issues) and [Creating issue dependencies](https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/creating-issue-dependencies).

## Decision 9: Borrow the useful sibling convention without cloning it

**Decision**: Adopt fragcap's explicit `Closes #N` expectation and strong contributor guidance, then add ESO Weave-specific atomicity, verification, and Project stages.

**Rationale**: Fragcap demonstrates the desired closure discipline. Its conventions do not cover ESO Weave's release-verification lifecycle, so copying the files unchanged would preserve the current ambiguity.

## Decision 10: Configure Project views through supported interfaces

**Decision**: Use GitHub CLI and GraphQL for project creation, fields, items, and values. Use the GitHub interface only for view layout when no supported CLI mutation exists.

**Rationale**: External state must be reproducible where possible, but Project view mutations are not fully exposed by the standard CLI.
