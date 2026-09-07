# Project Governance

ESO Weave uses atomic issues, milestones, spec-kit slices, pull request closing
references, and the public
[ESO Weave Delivery](https://github.com/users/h8rt3rmin8r/projects/2) Project as
one delivery record.

## Issue lifecycle

One actionable issue owns one independently closeable outcome. If implementation
and release verification can finish at different times, they use separate
issues. A coordinator epic groups those issues and closes only after every
required child outcome is complete.

The `needs: verification` label belongs only on the issue currently waiting for
field, platform, or release evidence. A merged implementation issue must not
remain open solely to wait for a different verification lifecycle.

## Pull request closure

Every normal pull request targeting `main` includes at least one complete GitHub
closing reference:

```text
Closes #123
Closes #124
```

Repeat the keyword for every issue. `Related to #123` and `Closes #123, #124`
do not satisfy the contract. CI accepts GitHub's close, fix, and resolve keyword
families with case differences and an optional colon.

Dependabot and `dependencies` pull requests are exempt. A maintainer may apply
`skip: issue-link` to rare release or repository-administration pull requests
that genuinely have no atomic issue; the pull request must explain why.

After merge, confirm every reference closed, move closed items to Done, retain
open verification siblings, update parent epics, synchronize `main`, and remove
the merged branch.

## Delivery Project

The Project has two custom fields:

- **Stage**: Backlog, Ready, Specced, In progress, PR review, Release
  verification, or Done.
- **Slice**: a known identifier such as `S044`; unknown historical mappings stay
  blank.

The delivery table is grouped by Milestone and keeps native assignee, label,
parent, and sub-issue progress visible. The delivery board is grouped by Stage.
Priority, effort, area, type, and milestone remain issue metadata rather than
duplicate Project fields. GitHub's undeletable default Status field is unused.

Stage rules are conservative:

- Closed issues are Done.
- Open issues carrying `needs: verification` are Release verification.
- A fully defined, unblocked next outcome is Ready or Specced.
- Active work is In progress until its pull request opens, then PR review.
- Other open work remains Backlog unless an active epic accurately represents
  ongoing child progress.

## Historical metadata audit

Audit date: 2026-09-03. Scope: every repository issue through #47, comprising 38
issues because numbers #31 through #39 are pull requests.

Evidence included issue state and labels, linked and merged pull requests,
release publication times, annotated tags, milestone descriptions, native
parent relationships, and current verification gates.

### Milestone dispositions

| Milestone | Issues | Evidence and disposition |
|---|---|---|
| v0.7.0 | #4 to #7 | Closed immediately before the v0.7.0 release; correct. |
| v0.8.0 | #1, #8 | Shipped in the v0.8.0 tag and closed after release confirmation; correct. |
| v0.8.1 | #12 to #14 | Shipped in the v0.8.1 patch and closed after release confirmation; correct. |
| v0.9.0 | #2, #3, #9, #10, #16 | Shipped in the v0.9.0 tag; correct. |
| v0.10.0 | #11, #18, #19 | Closed before the v0.10.0 publication and included in that release; correct. |
| v0.11.0 | #20 | Closed before the v0.11.0 publication and included in that release; correct. |
| Post-v0.11.0 | #15 | Specification refresh completed after v0.11.0; correct. |
| v0.12.0 | #21 to #26, #30, #40 to #47 | Current unreleased runtime and governance work; correct. |
| v0.13.0 | #27 to #29 | Planned HUD dashboard work; correct. |
| Future telemetry | #17 | Deferred pending a reliable sprint observable; correct. |

No milestone correction was required. PR #31 merged S041 for #22 and #23, PR
#38 merged S042 for #24, and PR #39 merged S043 for #25. The latter two issues
were manually closed after merge because their former issue boundaries mixed
implementation with release verification. Issues #40 and #41 now own those
separate verification lifecycles.

### Verification-label dispositions

- Retained on #17, #40, #41, and #44. Each is open and explicitly blocked on
  field, release, or platform evidence.
- Removed from closed implementation issue #22. Its repository acceptance path
  completed in PR #31, and retaining a temporary verification label on a closed
  implementation issue contradicted the new lifecycle contract.
- Absent and correct on #1 to #16, #18 to #21, #23 to #30, #42, #43, and #45 to
  #47. These issues are either complete from repository evidence, active without
  a field-evidence entry gate, or coordinators whose child states carry the gate.

The audit therefore covered all 38 issues and produced one evidence-backed label
correction with no milestone changes.

## Current atomic-scope audit

Every open issue has one lifecycle after the earlier runtime work was split:

- #17 owns the deferred sprint observable.
- #21, #26, #27, and #30 are coordinators whose native children own the work.
- #28 and #29 own separate dashboard layout and resource-meter outcomes.
- #40, #41, and #44 own release or field verification only.
- #42 defines the negotiated geometry contract, #43 implements it, and #44
  verifies the released behavior in dependency order.
- #45, #46, and #47 separately own history, enforcement, and Project state.

No open implementation issue combines a later release-verification lifecycle.
