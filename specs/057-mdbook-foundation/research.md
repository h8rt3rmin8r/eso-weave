# Research: mdBook Documentation Foundation

## Decision 1: Use mdBook 0.5.4

**Decision**: Install mdBook exactly at 0.5.4 with its locked dependency graph.

**Rationale**: It is the latest official release as of 2026-09-07, supports the
repository's Rust 1.96 toolchain, produces static HTML with local search, and
avoids introducing a Node application and package-management lifecycle.

**Alternatives rejected**: VitePress and Starlight add Node framework dependency
graphs. Zola needs more custom documentation navigation and search assembly.

## Decision 2: Use `docs/src` now

**Decision**: Establish `docs/book.toml` and `docs/src` while leaving current
top-level documentation files in place.

**Rationale**: #80 already requires `docs/src` to become the only published and
binary-bundled source. Starting under `website/` would create avoidable movement,
URL churn, and competing authority in the next slice. The current website article
remains untouched for #80 to classify.

## Decision 3: Combine proven link checking with repository policy

**Decision**: Pin mdbook-linkcheck2 0.13.0 with external-link following disabled,
and add a dependency-free Node policy checker with fixture tests.

**Rationale**: mdBook does not validate all links, while link checking alone does
not reject orphaned pages, path-case drift, remote runtime resources, or unsafe
workflow permissions. Node is already used by repository policy tests and adds no
package manifest or shipped runtime.

**Pin caveat**: The link checker's locked graph emits a warning for one yanked
transitive build dependency. The exact install and checks succeed, and the crate
does not enter ESO Weave's shipped or runtime dependency graph. Keep the pin for
reproducibility and reassess the warning at the next tool upgrade.

## Decision 4: Preserve upstream templates

**Decision**: Add only a custom stylesheet, a bounded skip-link script, and local
brand assets instead of copying a complete mdBook theme.

**Rationale**: Selective overrides apply the brand without freezing upstream
semantic, keyboard, search, or security improvements into a private template.
The published asset set includes the existing Inter Regular and SemiBold files
and license so typography does not depend on fonts installed on the reader's
device.

## Decision 5: Use an ignored shared output tree

**Decision**: Build both renderers beneath `target/docs-site`, with Pages
uploading `target/docs-site/html`.

**Rationale**: The repository already ignores every `target` directory. This
keeps generated HTML out of Git without another ignore authority and gives local
and hosted builds the same artifact path.

## Decision 6: Separate checking from deployment

**Decision**: Run the same build/check job for pull requests and main. Configure
and upload Pages only on main, and grant Pages/OIDC writes only to the guarded
deploy job.

**Rationale**: Pull requests remain untrusted and read-only. The deploy job
consumes the exact checked artifact and cannot run for a pull-request ref.

## Decision 7: Pin actions immutably

**Decision**: Use full commit SHAs for checkout 7.0.1, configure-pages 6.0.0,
upload-pages-artifact 5.0.0, and deploy-pages 5.0.1.

**Rationale**: GitHub documents a full commit SHA as the only immutable action
reference. This strengthens the new workflow without expanding S057 into a
retrofit of unrelated existing workflows.

## Decision 8: Do not add a blog post

**Decision**: S057 adds no article under `website/`.

**Rationale**: This slice is delivery plumbing rather than a completed reader
feature. #80 owns classification of the existing orphaned article and #81 owns
substantive public content. Adding another article now would violate both
boundaries and create a third documentation authority.

## Primary Sources

- mdBook configuration and CI: https://rust-lang.github.io/mdBook/
- mdBook 0.5.4 release: https://github.com/rust-lang/mdBook/releases/tag/v0.5.4
- GitHub Pages custom workflows:
  https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages
- GitHub Actions secure use:
  https://docs.github.com/en/actions/reference/security/secure-use
