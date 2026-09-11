# Research: Documentation Diagram Rendering Compatibility

## Current evidence

- The public Architecture diagram returns HTTP 200 as `image/svg+xml` and renders in current in-app Chromium in navy and light themes.
- The four SVG roots declare only `viewBox`, so intrinsic sizing depends on browser inference.
- mdBook 0.5.4 emits a checkbox zoom wrapper with a primary image and a hidden expanded clone.
- `.docs-flow-diagram img { width: 100%; height: auto; }` applies to both images and stretches the expanded clone to the viewport ratio.
- Generated diagram asset bytes currently match their sources, but policy does not enforce that identity.
- The bundled server already maps `.svg` to `image/svg+xml`, uses same-origin CSP, and embeds release-built mdBook output.
- The repository has no browser-testing package graph. GitHub's Ubuntu runner and the local development machine provide a Chrome-compatible browser.

## Decision: explicit geometry and scoped CSS

Add intrinsic `width` and `height` equal to each viewBox and apply fill width only to the primary image. Explicitly restore auto dimensions for the expanded clone while preserving mdBook's viewport maxima.

This removes browser-dependent intrinsic sizing and the reproduced expanded-state distortion without changing diagram content or general expansion UX.

## Decision: layered regression evidence

Retain structural policy as the fast first layer, add byte identity and generated zoom-DOM checks, then run one dependency-free browser process over all matrix cells. The browser layer checks semantic geometry and rasterized paint characteristics rather than screenshot hashes.

This catches decode and blank-paint failures that string policy cannot see while avoiding a Playwright, Puppeteer, or browser-download dependency.

## Decision: finite compatibility boundary

The required automated matrix is all four pages in light and navy at 320 and 1280 CSS pixels against generated documentation. Pages response evidence and the release-built loopback surface establish the two delivery paths. Manual Edge and Firefox observations may supplement this receipt but do not become unbounded engine promises.

Failure to reproduce the original report is not a blocker. The hardened contract and finite passing matrix close the reported risk with reproducible evidence.

## Alternatives rejected

- **Mermaid or another page-time renderer**: adds script, CSP, theme, offline, and availability failure modes.
- **Inline SVG conversion**: duplicates content into pages and weakens the single-source asset contract.
- **PNG fallback**: adds a generated asset family and scaling tradeoffs without evidence that SVG delivery is unsupported.
- **Playwright or Puppeteer**: adds a package lock, browser management, and downloads for a four-asset smoke.
- **Structural policy only**: cannot prove successful browser decode and nonblank paint.
- **Pixel-golden screenshots**: brittle across moving hosted browser versions and unnecessary for semantic visibility.

## Scope handoff

S086 owns the four existing diagram figures in their current normal and expanded states. Issue #155 owns cross-corpus expansion controls, affordances, keyboard behavior, focus return, and raw-HTML figures.
