# Quickstart: Documentation Flow Diagrams

Run from the repository root after implementation.

## Focused policy tests

```powershell
node --test .github/scripts/docs-policy.test.mjs
```

Expected: all tests pass, including S082 source, SVG safety, accessibility, generated-output, responsive CSS, and mutation cases.

## Documentation build and policy

```powershell
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
mdbook-linkcheck2 --standalone docs
```

Expected: tests, build, policy, and local links pass with all four SVGs present.

## Text and repository checks

```powershell
node --test .github/scripts/*.test.mjs
cargo fmt --all -- --check
git diff --check
```

Expected: policy suites pass, formatting is unchanged, and the diff has no whitespace errors.

## Visual inspection

Serve `target/docs-site/html` locally. Inspect the four contracted pages at 320 and 1280 CSS pixels in light and navy themes. Confirm labels remain discernible, each diagram fits the page, text equivalents are complete, assets load locally, and disabling images loses no required meaning.

## Text hygiene

Decode every changed text file as strict UTF-8, reject BOM and carriage-return bytes, search for mojibake markers, and reject en-dash or em-dash characters.
