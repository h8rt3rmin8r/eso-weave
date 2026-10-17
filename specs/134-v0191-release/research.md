# S134 Research

## Release identity and local rollover

Decision: reuse release.toml unchanged, using cargo release 0.19.1 dry-run and
then --execute --no-confirm on a clean preparation commit.
Rationale: the configuration accepts this release branch and explicitly disables
tagging, pushing and publication. It synchronizes exactly six identity files:
Cargo.toml, Cargo.lock, CHANGELOG.md, README.md, docs/src/README.md and the S073
capture-request.json fixture. The importer records CARGO_PKG_VERSION and tuple
validation rejects a mismatch, so the fixture is a meaningful contract surface.
Alternatives: hand edits risk omissions; allow-dirty would hide unrelated work;
a global version replacement would corrupt historical evidence.

## Validation and evidence

Decision: retain existing release-note/packaging, Rust, policy, production-build
and browser gates. Exercise the current missing-Highlights rejection before
adding notes. Do not add tests that merely mirror a bumped constant.
Rationale: existing checks already validate notes, documentation identity,
collector tuples, safety surfaces and all 13 named S133 regression cases.
The production build requires mdBook 0.5.4 and linkcheck2 0.13.0. Run the ten
release-profile documentation contracts as well as the ordinary locked suite.
Alternatives: source-only policy omits generated/embedded/rendered guidance;
local Windows testing cannot replace hosted Linux package contract evidence.

## Delivery boundaries

Decision: close candidate #315 only on owner merge; keep coordinator #314 and
milestone v0.19.1 open for later separately authorized publication and asset audit.
Rationale: candidate identity and downloadable artifact evidence finish at different
times. A dated bundled snapshot identifies the candidate without establishing
published assets. No installed/game check or new field-testing issue is needed.
Alternatives: one mixed issue would falsely close publication on source merge.

## Sources and research disposition

Protected-main authorities: release.toml; docs/project/releasing.md;
.github/workflows/ci.yml and docs.yml; build.rs; src/collector/import.rs;
src/catalog_pipeline/mod.rs; scripts/release-notes.sh; S132 quickstart and receipt.
The installed planning workflow dispatched a read-only release audit, which found
no blocker and confirmed the six-surface inventory and existing gates. All planning
unknowns are resolved; no dependency, pipeline or package-matrix change is needed.
