# Research: v0.16.0 Release Preparation

## Decision 1: Use v0.16.0

The 23 merged pull requests since v0.15.1 add substantial backward-compatible
catalog, encounter capture, import, analysis, recommendation, and documentation
capabilities. A minor release communicates that scope accurately.

## Decision 2: Use three outcome groups

Three Highlights map the catalog workflow, encounter workflow, and documentation
experience. They cover all detailed slice records without promoting dependency
or governance detail into user-facing notes.

## Decision 3: Repair the rollover contract

S080 made the documentation landing version and date enforced authorities after
v0.15.1. Existing cargo-release configuration updates Cargo, the root README,
and the changelog, but not those two fields. Add exact semantic-field rules with
`exactly = 1`, validate their structure in documentation policy, and inspect the
dry run.

## Decision 4: Preserve independent verification

Issues #110, #129, and #131 require installed or live-game observations. They
can run only when suitable environments are available and never block repository
progress or public artifact creation.

## Alternatives Rejected

- Manual metadata edits were rejected because they temporarily contradict the
  current Cargo and changelog authorities.
- A new release wrapper was rejected because cargo-release already supports the
  exact governed replacements.
- A patch release was rejected because it understates the feature scope.
- Delaying publication for field verification was rejected because those issues
  are explicitly nonblocking and benefit from stable public artifacts.
