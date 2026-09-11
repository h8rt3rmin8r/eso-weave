# Quickstart: v0.16.0 Release

1. Confirm all post-v0.15.1 merges have detailed Unreleased entries.
2. Generate and inspect the three v0.16.0 Highlights.
3. Run documentation policy, Rust checks, text hygiene, and scope checks.
4. Run a cargo-release dry run and inspect every governed identity surface.
5. Publish and merge the reviewed preparation pull request with green checks.
6. On clean `main`, repeat the dry run and execute `cargo release 0.16.0 --execute`.
7. Monitor the tag workflow until verification, Windows, Linux, and release jobs
   are green.
8. Inspect the public release notes, five required artifacts, and checksums.
9. Keep installed and live-game verification issues open for later evidence.
