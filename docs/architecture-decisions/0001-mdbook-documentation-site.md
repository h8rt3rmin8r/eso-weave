# ADR 0001: mdBook Documentation Site

Date: 2026-09-07

Status: Accepted

## Context

ESO Weave needs one repository-owned Markdown corpus that can publish to GitHub
Pages and later ship inside the desktop executable. The current documentation is
mixed across a monolithic specification, maintainer records, chronological plans,
the root README, and one unintegrated website article. Issue #79 owns the build,
validation, theme shell, and public delivery foundation. Issues #80 and #81 own
corpus migration and content completeness.

The output must support local search, hierarchical navigation, deep links,
repository edit links, light and dark presentation, and offline runtime use. It
must not add a hosted search service, analytics, CDN asset, or Node application
dependency graph.

## Decision

Use mdBook 0.5.4 with its locked dependency graph. Store published source under
`docs/src`, configure `/eso-weave/` as the hosted base path, and write generated
output beneath the ignored `target/docs-site` directory.

Use mdbook-linkcheck2 0.13.0 with external-link following disabled and warnings
treated as errors. Add a dependency-free Node checker for navigation completeness,
exact path case, generated local resources, brand/accessibility CSS requirements,
and Pages workflow policy. Node is a contributor and CI check only; it is not a
site runtime or package-managed application dependency.

Apply the ESO Weave mark, bundled Inter faces, and color tokens with one
additional stylesheet. Add one small local behavior script that inserts a
skip-to-content link without replacing mdBook templates. These selective
overrides retain upstream semantic, keyboard, search, and security improvements.

Use one GitHub Actions workflow for validation and delivery. Pull requests build
and check with read-only contents permission. Only a trusted main run configures
and uploads the exact checked artifact. A separate main-only deploy job targets
the protected `github-pages` environment and alone receives `pages: write` and
`id-token: write`. Every action uses a full immutable commit SHA.

## Alternatives

- VitePress and Starlight provide polished documentation features but add Node,
  a package manager, a JavaScript framework, and their update lifecycle.
- Zola is a strong Rust static-site generator but would require more assembly for
  documentation navigation and client-side search.
- A temporary book under `website/` would conflict with #80's required canonical
  tree and force an immediate move with URL churn.
- A full copied mdBook theme would couple the project to upstream templates and
  increase accessibility maintenance.
- mdBook alone does not reject every broken link, orphaned page, case mismatch,
  or remote runtime resource.

## Consequences

- Contributors install two exact Cargo tools and use the existing dependency-free
  Node policy convention.
- The site republishes the canonical Inter Regular and SemiBold font files with
  their existing SIL Open Font License so brand typography remains deterministic
  offline.
- Hosted builds compile the pinned tools, favoring reproducibility over the speed
  of mutable prebuilt installers.
- The link checker's locked build graph currently warns about one yanked
  transitive crate. It is not a shipped or runtime dependency, the exact install
  succeeds, and the warning must be reconsidered with the next tool upgrade.
- Existing documents remain outside the published tree until #80 inventories and
  migrates them without loss.
- Pages must use GitHub Actions as its source, and its deployment environment must
  restrict branches to `main`.
- Tool upgrades require a reviewed version change, fixture/build evidence, and a
  dated changelog decision when the pinned workflow changes.

## References

- [mdBook documentation](https://rust-lang.github.io/mdBook/)
- [mdBook 0.5.4](https://github.com/rust-lang/mdBook/releases/tag/v0.5.4)
- [mdbook-linkcheck2 0.13.0](https://github.com/marxin/mdbook-linkcheck2/releases/tag/v0.13.0)
- [GitHub Pages custom workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)
- [GitHub Actions secure use](https://docs.github.com/en/actions/reference/security/secure-use)
