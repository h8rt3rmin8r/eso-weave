# Quickstart: Placeholder-First Local Icon Cache

## Build a synthetic local generation

1. Create a user-owned directory whose relative files mirror catalog virtual
   paths, for example `esoui/art/icons/example.dds`.
2. Supply only project-created or independently obtained local PNG/DDS files.
3. Pass selected catalog virtual paths, the catalog semantic hash, the source
   root, and an application-data cache root to `build_generation`.
4. Inspect the returned immutable generation and manifest. Missing or rejected
   files resolve to the project placeholder.

S072 does not expose a network or archive-extraction command. It never edits
the source directory and does not activate a cache globally.

## Verification

Run the focused contract suite first:

```text
cargo test --locked --test icon_cache
```

Then run spec-kit and repository gates:

```text
specify check
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

## Expected evidence

- valid synthetic PNG and DDS inputs become identical stable PNG objects when
  their decoded pixels match;
- missing, unsupported, corrupt, oversize, and link-like inputs bind to the
  same verified placeholder with distinct reasons;
- repeated input produces the same manifest and generation hash;
- aliasing and cache publication failures preserve existing generations;
- manifests contain no user root and packages contain no user or game bytes.

## Completed evidence

All commands above passed on 2026-09-09. The focused suite contains 10
synthetic tests, the catalog and packaging regression set contains 23 tests,
and documentation policy contains 74 tests. Both optimized binaries built from
the locked dependency graph, whose `image_dds` feature set contains only
`ddsfile` and `image`.
