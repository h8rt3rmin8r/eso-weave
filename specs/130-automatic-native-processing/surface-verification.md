# S130 surface verification receipt

Date: 2026-10-15. These are repository, synthetic application, generated-site and
release-profile embedded-guide results. Installed-game/platform behavior is not
claimed. Scope and compatibility dispositions are in `surface-audit.md`.

## Tool identities

Verified binaries reported mdBook `0.5.4`, mdbook-linkcheck2 `0.13.0`, typos-cli
`1.50.1`, Node `26.5.0`, and headless Chrome `155.0.8059.40` on Windows x64.
Rust uses the repository-pinned 1.96.0 toolchain. Non-Git console tooling used the
hidden redirected foreground runner; no visible console or desktop capture was
launched.

## Results

| Verification | Result and local evidence |
| --- | --- |
| Test-first File retirement | New absence assertion failed specifically on existing Encounter History before removal (`target/s129-s130-retirement-red.log`). |
| Rendered UI and strings | `cargo test --locked --test app_ui_sizing --test app_strings --test app_encounter_history --test documentation_capture`: 55 UI + 13 string + 3 retained-domain tests passed, zero ignored; capture catalog/isolation checks passed (`target/s129-s130-ui-targeted6.log`). |
| Final source-built captures | `cargo test --locked --test documentation_capture -- --capture-to target/documentation-captures/s130-final`: 28/28 variants and manifest, with zero input and explicit eligible-package control assertion (`target/s129-s130-captures-final.log`). |
| Published screenshot curation | Seven final PNGs, six 1280x760 and one 1280x900, source/crop/hash metadata reconciled; visual inspection covered every published PNG. MSI image remained byte-identical (`target/s129-capture-receipts-final.log`). |
| Policy/render/brand unit suites | 171 tests passed, zero failed/skipped (`target/s129-s130-artifact-policy-final.log`). Includes positive/negative automatic model and removed-workflow requirements. |
| Pinned book build and links | `mdbook build docs` passed with HTML and linkcheck2 backends (`target/s129-s130-artifact-build-final.log`). The renderer reports its known source-fragment limitation; generated-site policy additionally validates actual generated targets/navigation. |
| Book examples | `mdbook test docs` passed all chapter invocations (`target/s129-s130-book-examples.log`). |
| Generated site | `node .github/scripts/docs-policy.mjs docs target/docs-site/html` passed against final assets (`target/s129-s130-artifact-generated-final.log`). Covers navigation/search, source coverage, actual tables/figures, screenshot provenance and contracts. |
| Browser diagram/figure/table/syntax rendering | All five existing pass sentinels passed. Matrix: 64 diagram observations, 8 topology observations, 40 syntax observations, 20 figure observations and 5 interaction journeys, 20 table observations, 4 fallback-font and 4 authority zoom/no-script probes; failure lists empty. Captioned raster 200-percent zoom preserves image/caption containment, 14.4-pixel caption text and association (`target/s129-s130-artifact-render-final.log`). |
| Release guide | `cargo test --release --locked --test documentation`: 10 tests passed, zero ignored, against final generated guide/theme/screenshots (`target/s129-s130-artifact-embedded-final.log`). New release-only test checks controls, supported profile/deferred links and actual embedded HTML/SVG/search absence. Existing HTTP routing/deadline/shutdown/concurrency checks retained. |
| Spelling | Pinned `typos docs/src docs/README.md README.md` passed (`target/s129-s130-final-spelling.log`). |
| Current stale-reference scan | No matches in shipped `src`, current addon, current `docs/src`, generated HTML/search/SVG output for retired viewer/menu/import/recording-command labels. `rg` exit 1 means zero matches (`target/s129-s130-final-stale-scan.log`). Release embedding independently checks the same removed workflow class. |
| Text integrity | Strict UTF-8 without BOM, LF, prohibited-dash and mojibake scan found zero errors across changed/specification text files (`target/s129-s130-text-audit-final.log`). Final receipts use the same encoding. |

## Corrected failures and verification boundaries

The old 640-pixel dashboard crop cut Skills controls mid-row. Curation now keeps
all rows at 760 pixels. The taller complete screenshot exposed caption overflow
at 200-percent browser zoom. The existing browser gate failed before the narrowly
scoped captioned-raster fit and passed afterward; it was not weakened. SVG
intrinsic/scrolled readability remains unchanged.

The capture harness has no maintenance runtime thread. Its eligible settings
scene now seeds only the compatible package presence established by the synthetic
fixture, so the image does not falsely gray a control while showing that package
installed. The harness asserts the native disable control is active in that
eligible scene without starting either background service.

Restricted execution initially denied Chrome's crash-handler pipe and local
loopback client sockets (Windows 10013). The same authorized hidden browser and
release tests passed with sandbox escalation; no product timeout, endpoint,
assertion or test gate was weakened for those environment restrictions.

Whole-repository formatting, all-target/all-feature Clippy, full locked Rust
parity, final-head hosted checks and bounded Codex review remain the primary
slice delivery gates, recorded by the primary verification receipt. This surface
receipt does not substitute for them or assert installed-game evidence.
