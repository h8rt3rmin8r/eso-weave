# Quickstart: Verify Documentation Landing Identity

## Focused policy tests

```text
node --test .github/scripts/docs-policy.test.mjs
```

Expected: every test passes, including S080 identity, metadata authority, banner,
and drift cases.

## Build and policy

```text
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
```

Expected: examples, links, source policy, generated output, and local asset checks pass.

## Spelling and text hygiene

```text
typos docs/src docs/README.md README.md
git diff --check
```

Confirm every changed text file is UTF-8 without BOM, uses LF, contains no
replacement characters, and contains no en-dash or em-dash.

## Visual inspection

Inspect built `index.html` at 320, 768, and 1440 CSS pixels in light and navy
themes. Confirm the banner stays within the content column, only Documentation is
repeated visibly, metadata labels remain associated with values, focus is visible,
and the page never gains horizontal overflow.

## Asset identity

Confirm `assets/eso-weave-banner.png` and
`docs/src/assets/brand/eso-weave-banner.png` have identical bytes and that the
generated output contains the local banner.
