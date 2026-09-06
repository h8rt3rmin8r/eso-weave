# Quickstart: v0.13.0 Release Preparation

1. Confirm every S048 through S052 outcome is recorded under Unreleased.
2. Add four concise user-facing bullets under `### Highlights`.
3. Run `bash scripts/release-notes.test.sh` in a compatible Bash environment.
4. Preview with `CHANGELOG_HEADING=Unreleased scripts/release-notes.sh 0.13.0 h8rt3rmin8r/eso-weave`.
5. Confirm the output contains only four bullets and the tagged full-changelog link.
6. Confirm Cargo, lockfile, and README still identify v0.12.0.
7. Merge the reviewed preparation pull request before running the separately authorized release command.
8. Validate the published build through #67 and #69.
