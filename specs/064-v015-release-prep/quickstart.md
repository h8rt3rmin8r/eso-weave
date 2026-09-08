# Quickstart: v0.15.0 Release Preparation

1. Confirm every merged change since v0.14.0 is represented under Unreleased.
2. Add four concise user-facing bullets under `### Highlights`.
3. Run the release-note contract suite in the repository's Bash environment.
4. Preview Unreleased as v0.15.0 with the authoritative release-note script.
5. Confirm output contains only four bullets and the immutable tagged changelog link.
6. Confirm Cargo, lockfile, and README still identify v0.14.0.
7. Confirm release machinery, tags, releases, assets, #77, and #84 are unchanged.
8. Merge the reviewed preparation PR before separately authorizing the release command.
9. After publication, verify released packages through #84 and continue #77 independently.
