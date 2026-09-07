# Documentation Site Contract

## Toolchain

- mdBook: `=0.5.4`, installed with `--locked`
- mdbook-linkcheck2: `=0.13.0`, installed with `--locked`
- Policy runtime: dependency-free Node standard library

## Local Commands

```text
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
mdbook serve docs --open
```

## Renderer

- Source: `docs/src`
- Output: `target/docs-site/html`
- Base path: `/eso-weave/`
- Missing page: `404.md` to `404.html`
- External web links are not followed during builds.
- Search and static runtime resources are local.

## Workflow

- Pull request: checkout, install, fixture test, mdBook test, build, policy check.
- Main: the same checks, then configure and upload the checked artifact.
- Deploy: main-only, protected `github-pages` environment, exact uploaded artifact.
- Global permission: `contents: read`.
- Deploy-only permissions: `pages: write`, `id-token: write`.
