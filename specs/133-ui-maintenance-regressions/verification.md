# S133 verification receipt

## Scope and evidence

Owner-selected #306-#310, implemented on codex/133-ui-maintenance-regressions from main cf0d66e87277a31eca9f2f843b1f695be1fa5928.

Before repairs, the behavioral regressions reproduced window growth, missing visibility, inline confirmation and absent PTS reconciliation. Source review identified the unconditional API suffix. The initial status fixture hit a restricted temporary-directory access error before its presentation assertion; that fixture and the full suite passed after temporary test state moved under ignored target/. Read-only installed diagnostics confirmed recurring once-per-minute warnings and an absent discovered PTS AddOns directory alongside an existing Live root. No installed package content was modified.

## Completed local checks

- cargo fmt --all -- --check.
- cargo clippy --all-targets --all-features -- -D warnings.
- cargo test --all --locked: 1,219 tests passed across 54 test summaries, zero failures. The change adds 13 named S133 regression cases (one replaces an obsolete inline-row expectation).
- Required package, ownership/race, transactional recovery, retained-data, input recursion, focus and hook-thread tests remain enabled and pass.
- 184 documentation, brand and trust policy tests passed. Repository trust scan passed all seven workflows and 54 local skills.
- mdbook test docs and mdbook build docs passed; generated-site policy and spelling passed. Local fragment checking is supplemented by the generated-site policy because linkcheck2 does not resolve Markdown fragments.
- Headless Chrome diagram, syntax, figure and table smoke checks passed every sentinel with empty failure lists.
- 28 deterministic UI scene/theme/viewport captures generated. Seven dark-wide sources were visually reviewed and curated; six PNGs changed and one remained byte-identical. Screenshot digests were refreshed, and the approved MSI Properties image stayed byte-identical.
- cargo build --release --locked --bin eso-weave verifies the release binary and bundled documentation.

All console checks used a hidden, noninteractive launcher and redirected I/O. The initial restricted run could not bind a loopback socket; the complete suite passed under normal socket permissions. Temporary test state stays below ignored target/.

## Limits and handoff

Repository, synthetic UI, package fixtures and build evidence do not establish installed-game behavior. No application/game session or owner field-testing task was used. Linux lacks a trustworthy compositor work-area rectangle through the existing seam, so it preserves current client geometry and scrolls constrained content rather than guessing a larger window. Windows uses the selected monitor's native usable rectangle with the viewport's actual scale.

Publication is a source PR for review only. Merge, tag, release and installed-addon deployment remain outside this run. Hosted exact-head CI and review state are recorded in the PR; this receipt does not predeclare their outcome.
