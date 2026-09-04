# Data Model: Delivery Pipeline Governance

## AtomicIssue

Represents one independently closeable repository outcome.

| Field | Type | Rules |
|---|---|---|
| number | positive integer | Unique within the repository |
| state | open or closed | Closed only when this issue's own acceptance criteria are met |
| type | one repository label | Exactly one of bug, feature, task, epic, or chore |
| milestone | repository milestone | Reflects the release that contains or is expected to contain the outcome |
| priority | one repository label | Exactly one active priority label |
| effort | one repository label | Exactly one effort label |
| area | one repository label | Exactly one area label |
| verificationRequired | boolean | True only while issue-level field or release proof is an entry gate |
| parent | issue number or null | Epic parent for a child outcome |
| blockedBy | issue numbers | Native ordering dependencies |

### Invariants

- An actionable non-epic issue has one lifecycle and one independently testable outcome.
- Implementation and release verification are separate issues when their completion times differ.
- An epic is complete only after all required children are complete.
- Closed implementation issues do not retain `needs: verification` unless their own acceptance criteria still require evidence, in which case they should not be closed.

## PullRequestLinkageDecision

Represents the deterministic outcome of the CI policy.

| Field | Type | Rules |
|---|---|---|
| author | string | Treated as untrusted event data |
| labels | string collection | Treated as untrusted event data |
| closingIssues | unique positive integers | Parsed only from supported closing references |
| exempt | boolean | True only for a documented exemption |
| reason | string | Actionable pass, fail, or exemption explanation |

### State transitions

```text
received -> exempt
received -> linked
received -> rejected
```

- `exempt` when author is Dependabot or an exemption label is present.
- `linked` when at least one valid closing reference is found.
- `rejected` otherwise.

## DeliveryProjectItem

Represents one issue in the GitHub Project.

| Field | Type | Rules |
|---|---|---|
| issueNumber | positive integer | One item per repository issue |
| stage | Stage | Required |
| slice | string or empty | Known `SNNN` identifier only; never inferred |

### Stage

Ordered values:

1. Backlog
2. Ready
3. Specced
4. In progress
5. PR review
6. Release verification
7. Done

### Mapping rules

- Closed issue -> Done.
- Open issue with `needs: verification` -> Release verification.
- Current slice issues before pull request -> In progress.
- Current slice issues after pull request -> PR review.
- Fully specified next issue with no blocker -> Ready or Specced according to artifact state.
- Other open issue -> Backlog unless active epic progress justifies In progress.

## AuditFinding

Represents one evidence-backed audit disposition.

| Field | Type | Rules |
|---|---|---|
| issueNumber | positive integer | Required |
| milestoneFinding | correct, corrected, or unresolved | Required |
| verificationFinding | correct, corrected, or unresolved | Required |
| evidence | string collection | Merge, tag, release, state, or field-verification evidence |
| action | string or none | Metadata mutation or explicit no-change result |

### Invariants

- No milestone correction is made from title similarity alone.
- No release verification is marked complete without actual release or field evidence.
- Every issue receives one audit disposition even when no change is required.
