# S123 Research

## Release rollover

**Decision**: Use installed cargo-release 1.1.2 and unchanged `release.toml`, after a clean preparatory commit. Inspect `cargo release 0.17.3` before executing with `--execute --no-confirm` through the hidden launcher.

**Rationale**: Effective configuration confirms `verify=true`, branch restriction and `publish=false`, `tag=false`, `push=false`. Closed non-interactive stdin requires the supported confirmation flag; it bypasses no validation. Five cardinality-checked replacement blocks synchronize changelog, badge, documentation version/date and capture fixture; Cargo handles package/lock identity.

**Alternatives considered**: Manual metadata edits risk drift; older S053/S056/S064/S091 publication sequencing is superseded by the current canonical release procedure. No helper, pin or workflow change is needed.

## Archive lifecycle

**Decision**: Archive Plan048 and introduce active preparation-only Plan049. Update both indexes, machine/prose migration ledger, visualization audit status and four historical spec path references.

**Rationale**: Existing documentation policy requires contiguous plan IDs, one final active entry, valid destinations, matching index rows and concrete archive evidence. The ledger currently lacks S121/S122 deliveries. Direct Git history confirms S112 PR #225 on September 17, S120 PR #252 on October 5, and S121 PR #253/S122 PR #254 on October 6. S121 approval remains dated October 5. S113-S115 remain unused historical reservations.

**Alternatives considered**: Updating only human-readable indexes leaves machine authority stale. Deleting reservation history would lose provenance. Existing generic policy tests cover transitions; no new implementation-mirroring tests are appropriate.

## Evidence and authority

**Decision**: Use existing release-note tests, documentation policy/rendering, Rust gates and release-profile build; inspect final-head CI and all external findings. Issue #255 closes candidate preparation on owner merge.

**Rationale**: Candidate readiness differs from publication and installed behavior. Existing toolchain, dependencies, runtime contracts and package inventory remain authoritative. No duplicate local/remote v0.17.3 tag or hosted release exists at kickoff.

**Alternatives considered**: Installed/game checks and tag publication exceed the accepted scope. Leaving a generic publication issue open would misrepresent this preparation issue's lifecycle. No unresolved clarification remains.
