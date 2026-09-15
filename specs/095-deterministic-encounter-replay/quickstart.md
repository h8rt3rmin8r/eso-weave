# Quickstart: Verify S095

1. Run the encounter addon differential tests and confirm production Lua emits a
   version-1 normalization profile and Rust independently verifies every source.
2. Mutate a supplied projection and confirm import rejects it without showing
   expected or actual payload values.
3. Mark raw loss and confirm the capture imports as partial with replay
   indeterminate and degraded metric quality.
4. Import schema-v1 and addon-v2 schema-v2 fixtures and confirm compatibility
   bytes, hashes, metrics, and history remain stable.
5. Validate the machine subscription contract against production Lua and both
   pinned API snapshots.
6. Run formatting, clippy, all tests, release build, mdBook tests/build, docs
   policy, scoped text checks, UTF-8/BOM/mojibake scan, and `git diff --check`.
