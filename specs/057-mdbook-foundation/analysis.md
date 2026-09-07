# Cross-Artifact Analysis

## Pre-Implementation

Status: PASS

- The specification, research, data model, contract, plan, tasks, and checklists
  consistently select `docs/src` as the published source and
  `target/docs-site/html` as the ignored Pages artifact.
- Exact mdBook 0.5.4 and mdbook-linkcheck2 0.13.0 pins appear in every toolchain
  artifact; action dependencies are required to use verified immutable SHAs.
- Reader, contributor, and deployment stories map to independent automated or
  bounded manual evidence.
- Source navigation, document links, generated resources, accessibility basics,
  Pages subpath behavior, and workflow permissions each have an explicit task
  and acceptance gate.
- Pull-request checking and main-only deployment are consistently separated, and
  no workflow credential may self-enable Pages.
- Pinned workflow governance is satisfied by a required dated changelog decision.
- #80 through #84 implementation and the orphaned website article remain outside
  S057, avoiding competing documentation authorities.
- No unresolved clarification or constitution conflict remains.

## Post-Implementation

Status: PASS

- The implemented source tree, exact tool pins, ignored artifact location, and
  Pages subpath match the specification, plan, contract, ADR, and contributor
  guidance.
- Twenty dependency-free fixture tests cover positive delivery plus missing,
  duplicate, escaping, case-mismatched, orphaned, fragment, CSS-resource,
  contrast, skip-link, and workflow-escalation failures.
- Test-first red evidence was captured before the checker existed: the initial
  Node fixture run failed with `ERR_MODULE_NOT_FOUND`, then the minimal policy
  implementation brought the same suite green before site integration.
- mdBook test/build, linkcheck2, and generated-site policy checks pass with the
  exact mdBook 0.5.4 and mdbook-linkcheck2 0.13.0 installations.
- A Pages-shaped local mount verified local Inter loading, branded narrow and
  wide rendering, search, light-theme switching, nested navigation, and the
  skip-to-content target without a remote runtime request.
- Normal text and link contrast exceed 4.5:1 in light and dark presentation.
  Light focus contrast is at least 6.56:1 across page, panel, and raised surfaces.
- The workflow parser proves exact read-only top-level/build permissions, exact
  deploy-only Pages/OIDC writes, main guards on configure/upload/deploy, the
  checked-build dependency, immutable action pins, and the protected environment.
- Repository Pages uses the Actions source and its `github-pages` environment
  has one custom deployment branch policy for `main`.
- Full format, strict all-target Clippy, and locked repository tests pass. Text
  hygiene, YAML parsing, JavaScript syntax, whitespace, secret-pattern, and
  ignored-output gates pass.
- The accepted yanked transitive build-only crate warning is recorded in research
  and the ADR; it does not enter the shipped or runtime dependency graph.
- The diff leaves legacy migration, complete reader content, desktop embedding,
  and package verification to #80 through #84 as required.
