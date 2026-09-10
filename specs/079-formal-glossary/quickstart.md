# Quickstart: S079 Formal Glossary

1. Open `docs/src/reference/glossary.md`.
2. Use the Glossary alphabet navigation to jump to a populated letter.
3. Scan H3 canonical terms under that letter.
4. Read the Aliases line for player and technical search wording.
5. Follow the Related link for the complete feature, concept, or reference page.

## Verification

```text
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
typos docs/src docs/README.md README.md
```

Inspect the generated Glossary at 320 CSS pixels and a desktop width in light and
navy themes. Tab through every alphabet link and confirm the focus outline remains
visible. Search for representative canonical terms and aliases from every family.
