# Quickstart: Documentation Site Foundation

1. Install exact tools:
   `cargo install mdbook --version '=0.5.4' --locked` and
   `cargo install mdbook-linkcheck2 --version '=0.13.0' --locked`.
2. Run `node --test .github/scripts/docs-policy.test.mjs`.
3. Run `mdbook test docs`.
4. Run `mdbook build docs`.
5. Run `node .github/scripts/docs-policy.mjs docs target/docs-site/html`.
6. Preview with `mdbook serve docs --open`.
7. Confirm the generated tree remains ignored by Git.
8. On pull requests, confirm the docs workflow validates but has no upload or
   deploy job.
9. After merge, confirm the main workflow deploys beneath `/eso-weave/`.
