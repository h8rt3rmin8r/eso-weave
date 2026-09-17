# Quickstart: Verify the Documentation Visualization Audit

1. Run the documentation policy fixture tests.
2. Run the complete policy against source and generated documentation.
3. Count Markdown destinations in `docs/src/SUMMARY.md` and confirm the manifest contains the same 49 unique paths.
4. Select one page from each top-level section and reproduce its page decision.
5. Select one approved and one rejected candidate and reproduce all four warrant gates.
6. Open every approved follow-up issue and compare its reader task, destination, style, authority, accessibility, offline, and update contract with the manifest.
7. Confirm the four existing diagram assets and every other published asset are byte-unchanged in the scoped diff.
8. Build and test mdBook, then run typo, trust, encoding, and diff checks.
