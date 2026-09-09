# Research: Documentation Release Verification

## Decision 1: Verify one immutable release

**Decision**: Bind all evidence to v0.15.0, release commit `2ec90787e5b817897736d85a6c75c25baf365cba`, and workflow run 34302551069.

**Rationale**: Issue #84 requires released-package evidence. Mixing local or later builds would make checksums, payloads, and behavior non-reproducible.

## Decision 2: Use native package forms

**Decision**: Install the MSI on Windows 11, install the deb on Ubuntu 24.04 under WSLg, and run the released AppImage as portable Linux evidence.

**Rationale**: These paths cover the required installed package on each platform and the required Linux portable package without rebuilding source.

## Decision 3: Separate evidence strengths

**Decision**: Label evidence as direct UI observation, process and network inspection, package inspection, browser inspection, or repository-contract support.

**Rationale**: A repository test cannot prove an MSI installed correctly, while a visual spot-check cannot prove exact checksums or socket ownership. The receipt must not blur those claims.

## Decision 4: Scope offline testing

**Decision**: Use process-scoped or environment-scoped isolation and browser request inspection rather than disconnecting the host network.

**Rationale**: Global disconnection would interrupt unrelated user work and the GitHub delivery session. The target claim is that the packaged guide needs only its loopback origin.

## Decision 5: Preserve defect boundaries

**Decision**: Do not repair package or runtime defects inside #84. File a linked implementation issue, retain #84 in Release verification, and rerun against a fixed release.

**Rationale**: Project governance deliberately separates implementation from release verification because they close at different times.

## Alternatives Rejected

- Test a local release build: rejected because it is not the downloadable artifact under review.
- Inspect package contents only: rejected because it does not exercise the UI, browser handoff, or listener lifecycle.
- Treat CI packaging success as field verification: rejected because CI proves construction, not installed interaction.
- Disconnect the entire workstation: rejected because isolation must not disrupt unrelated user activity.
- Amend governance in S065: rejected because issue #104 owns that separate constitution change.
