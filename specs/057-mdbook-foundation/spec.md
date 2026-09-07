# Feature Specification: mdBook Documentation Foundation

**Slice**: S057

**Issue**: #79

**Status**: Implemented

**Created**: 2026-09-07

## User Scenarios

### User Story 1: Read a coherent documentation shell (Priority: P1)

As a reader, I can open a branded documentation site, navigate its initial
audience sections, search locally, follow deep links, and recover from a missing
page on narrow or wide screens in light or dark mode.

**Independent test**: Build the book and inspect the landing page, a nested page,
search artifacts, previous/next navigation, and the custom 404 page entirely
offline.

### User Story 2: Validate documentation changes locally (Priority: P1)

As a contributor, I can install exact tool versions and run one documented check
sequence that rejects broken local links, missing chapters, unlisted pages,
case mistakes, remote runtime assets, and malformed generated output.

**Independent test**: Run the fixture tests and site check, then prove negative
fixtures fail without changing the source tree.

### User Story 3: Publish safely from main (Priority: P2)

As a maintainer, I can review documentation builds on pull requests while only a
trusted `main` run may upload and deploy the exact validated artifact to GitHub
Pages.

**Independent test**: Inspect and exercise the workflow policy so pull requests
receive read-only permissions and never upload or deploy a Pages artifact.

## Edge Cases

- A chapter is linked from `SUMMARY.md` with the wrong case on Windows.
- A Markdown page exists but is omitted from navigation.
- A nested link or fragment targets a missing page or heading.
- A style, script, image, or font attempts a remote runtime request.
- The site is hosted beneath `/eso-weave/` instead of at the domain root.
- The Pages site or protected deployment environment has not yet been enabled.
- A pull request modifies documentation but must never obtain deployment
  credentials or publish an artifact.

## Functional Requirements

- **FR-001**: The repository MUST pin mdBook 0.5.4 and
  mdbook-linkcheck2 0.13.0 exactly for local and hosted checks.
- **FR-002**: The book MUST use `docs/src` as its sole published Markdown source
  tree and MUST leave the existing mixed documentation corpus and `website/`
  article unmigrated for #80.
- **FR-003**: The initial navigation MUST contain a landing page and reserved
  getting-started, feature, concept, reference, and development sections without
  presenting placeholder prose as complete product documentation.
- **FR-004**: The site MUST reuse ESO Weave brand colors and mark, support light
  and dark presentation, retain visible keyboard focus, reflow at 320 CSS pixels,
  and honor reduced-motion preferences.
- **FR-005**: Search, syntax highlighting, deep links, previous/next navigation,
  repository links, and source edit links MUST work without third-party runtime
  services.
- **FR-006**: The build MUST use `create-missing = false`, the `/eso-weave/`
  Pages base path, a custom 404 source, and an ignored output location.
- **FR-007**: Local checks MUST reject missing or duplicate navigation entries,
  unlisted published Markdown, escaping or case-mismatched paths, broken local
  links and fragments, and remote runtime assets.
- **FR-008**: Pull requests MUST build and check the book with read-only contents
  permission and MUST NOT configure, upload, or deploy Pages.
- **FR-009**: Trusted `main` pushes and manual `main` runs MUST upload the exact
  validated output and deploy it through the protected `github-pages`
  environment.
- **FR-010**: Only the deploy job MAY receive `pages: write` and
  `id-token: write`; action dependencies MUST use immutable commit SHAs.
- **FR-011**: GitHub Pages MUST be configured to use Actions, with deployments
  restricted to `main`, without placing an elevated token in the workflow.
- **FR-012**: Contributor guidance MUST document exact install, check, build,
  serve, and output paths.
- **FR-013**: The mdBook choice, alternatives, governance impact, version pins,
  accepted tradeoffs, and upgrade policy MUST be recorded in an ADR.
- **FR-014**: Generated HTML and caches MUST remain out of version control.
- **FR-015**: The pinned workflow change MUST have a dated changelog decision.
- **FR-016**: S057 MUST NOT migrate the legacy corpus, complete the full manual,
  embed the site in the binary, or claim installed-package verification.

## Key Entities

- **Book source**: The ordered, repository-owned Markdown and static assets under
  `docs/src`.
- **Toolchain contract**: Exact mdBook and link-checker versions used locally and
  in CI.
- **Validated site artifact**: The HTML renderer output that has passed source,
  link, offline-resource, and structural checks.
- **Deployment boundary**: The main-only job and protected environment allowed
  to publish the artifact.

## Success Criteria

- **SC-001**: One documented command sequence builds and validates the site on a
  clean Windows or Linux checkout.
- **SC-002**: Automated negative fixtures prove at least missing navigation,
  bad case, broken local targets, and remote runtime dependencies are rejected.
- **SC-003**: The generated artifact contains the landing page, every reserved
  section, nested navigation, local search files, local brand assets, and a
  subpath-safe 404 page.
- **SC-004**: Static accessibility checks pass for language, headings, image
  alternatives, focus styling, reduced motion, and contrast tokens without
  claiming full WCAG conformance.
- **SC-005**: Hosted pull-request checks pass without a Pages upload or deploy,
  and the workflow grants write/OIDC permissions only to the main-only deploy
  job.
- **SC-006**: No file owned by #80, #81, #82, or #84 is migrated or implemented.

## Assumptions

- GitHub-hosted runners provide Node for the repository's dependency-free policy
  checks; no Node package manifest or runtime site dependency is introduced.
- mdbook-linkcheck2 checks document links without following external web links;
  a repository policy checker covers navigation completeness and offline assets.
- Real Pages deployment occurs after the maintainer merges the pull request;
  pre-merge evidence proves the exact artifact and permission boundary.
