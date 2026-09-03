## Summary

<!-- Describe the user or maintainer outcome and why this change belongs here. -->

Closes #

## Slice and documentation

<!-- Name the spec-kit slice for feature work, or explain why no slice is needed. -->

- [ ] Relevant spec-kit artifacts and public documentation are updated.
- [ ] `CHANGELOG.md` records the change and any pinned-artifact decision.

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all --locked`

<!-- Record platforms, results, and any intentionally unavailable field checks. -->

## Safety and risk

- [ ] Input recursion, focused-window scoping, hook-thread behavior, and managed
      addon deletion remain covered when affected.
- [ ] Unknown or unavailable game evidence remains non-actionable.
- [ ] No pinned artifact changed without a dated changelog decision.

<!-- Describe remaining platform, data-integrity, accessibility, or release risk. -->
