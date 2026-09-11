# Analysis: Documentation Diagram Rendering Compatibility

## Pre-implementation gate

- Issue #154 is bounded to four existing static SVG figures and closes independently from #155.
- Public Chromium evidence does not reproduce a blank normal image, but it proves expanded-state distortion and browser-dependent intrinsic sizing.
- Source and generated identity, paint semantics, delivery media type, themes, viewports, and expanded state all have explicit test seams.
- A host-browser smoke is proportional because it adds no dependency graph or runtime behavior.
- Failure to reproduce the original report is documented evidence, not a verification blocker.

Result: PASS. No unresolved clarification or constitution violation remains.

## Test-first receipt

Before implementation, the focused Node suite failed four S086 contracts as
expected:

- the four SVG roots lacked required intrinsic dimensions;
- generated figures did not prove zoom semantics, decorative clone behavior, or
  byte identity;
- diagram CSS did not distinguish primary and expanded sizing; and
- the browser smoke module did not exist.

Result: RED. The failures matched the intended implementation seams.

## Final verification receipt

- Focused policy and browser helper suite: 113 passed, 0 failed.
- `mdbook test docs` and `mdbook build docs`: passed.
- Generated documentation policy: passed.
- Chrome 152.0.7977.83 browser smoke: all 32 matrix observations passed and
  emitted `ESO_WEAVE_DIAGRAM_SMOKE_PASS_V1`.
- Pages delivery evidence: all four public assets returned 200 as
  `image/svg+xml` on 2026-09-11.
- Accessibility inspection: the generated figure exposes one meaningfully named
  primary image, while the expanded clone is decorative.
- `typos`, `cargo fmt`, `cargo clippy`, `cargo test`, and `git diff --check`:
  passed.
- Independent code, security, and accessibility review findings were resolved,
  including bounded CDP deadlines, browser cleanup, realpath containment, direct
  testing of actual generated pages, and a restrictive server content security
  policy.

Result: GREEN. S086 satisfies the finite compatibility contract without adding
a package dependency or changing application behavior.
