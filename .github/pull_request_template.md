## Summary

<!-- Describe the user or maintainer outcome and why this change belongs here. -->

<!--
Every issue requires its own complete GitHub closing reference. Example:
Closes #123
Closes #124

Do not use "Closes #123, #124" because the second number will not close.
For an exceptional issue-free administration change, explain why and ask a
maintainer to apply the skip: issue-link label.
-->

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
