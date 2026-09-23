# Evidence: BrandBuilder 2.0.1 Density Correction

## Baseline

- Branch: `codex/s119-brandbuilder-2-0-1-density`
- Base: `73c27348f43fcd36e14abce5f6422fbada5c6297` (`v0.17.1`)
- Tracker: [issue #241](https://github.com/h8rt3rmin8r/eso-weave/issues/241)
- Upstream correction commit: `7547446ae95e5a9987294aade759f7b595c0adb6`
- Published bundle source revision: `801ed912aaa8bbc66c12d8a97b5487fcddb0d9da`
- Existing package: `eso-weave-brand-1.0.0-bb2.0.0.zip`
- Existing package SHA-256: `b37ac1459666ae33d772229bd5247699c2c845971eabed267ff85465b68a1ba1`
- Existing theme values: 44 by 44 interaction size, 12 by 12 item spacing, and 12 by 6 button padding.

## Specification gate

- Prerequisite check resolved `specs/119-brandbuilder-2-0-1-density` with research, data model, contract, quickstart, and tasks present.
- Requirements checklist: 16 of 16 complete.
- Brand conformance checklist: 7 of 7 complete.
- Analyze gate: PASS with complete task coverage and no constitution conflict.

## Red evidence

On 2026-09-22, the focused Rust regression failed as expected because the existing theme returned `[44.0 44.0]` instead of `[28.0 28.0]`.

The Node policy suite also failed as expected because production still required egui adapter 1.0.0 and compiler 2.0.0, and no native density contract was enforced.

## Upstream artifact

- Official release: [BrandBuilder v2.0.1](https://github.com/ShruggieTech/shruggie-brand/releases/tag/v2.0.1), published 2026-09-23 01:34 UTC.
- Canonical package: `https://brand.shruggie.tech/eso-weave/downloads/eso-weave-brand-1.0.0-bb2.0.1.zip`.
- SHA-256: `1b1ba26e57472573d31e2dc7dd20ac2f30a6eb89e7fb8d1e1c9b0ea0fe46c595`. The website bytes and GitHub release asset bytes are identical and match the release's `SHA256SUMS` entry.
- The package `enforcement/bundle.json` declares `publication.status=release`, tag `v2.0.1`, compiler 2.0.1, egui adapter 1.0.1, and source revision `801ed912aaa8bbc66c12d8a97b5487fcddb0d9da`.
- All 342 package manifest entries match their recorded lengths and SHA-256 checksums.
- The retained `enforcement/distributions/shruggie-brandbuilder-2.0.1.skill` matches the official package SHA-256 `5d712a07bab9f535207f1a4b81704ae8790274907919d1e38a83f8e4aa395742`.
- Every locally consumed brand artifact with a package-relative source remains byte-identical to the official 2.0.1 package. The external Geist Mono license notice is separately pinned by the existing local inventory.
- The pre-release candidate used for red-green work had a different source revision and recovery digest. Both were replaced with the official release facts before verification.

## Green evidence

- Brand policy: 7 of 7 Node tests pass; repository adoption validation passes with the official archive and recovery hashes.
- Focused application sizing: 50 of 50 tests pass, including the 28-point fine-pointer response floor and symmetric dashboard-card geometry.
- Formatting and lint: `cargo fmt --all -- --check` and `cargo clippy --all-targets --all-features -- -D warnings` pass.
- Documentation: 162 Node policy and render tests pass; `mdbook test docs`, `mdbook build docs`, site policy, and browser render smoke pass. The site policy was updated from its stale 2.0.0 phrase to the official 2.0.1 kit and now enforces the native density guidance.
- `cargo test --all --locked` passes every unit, integration, and documentation test.
- `cargo build --release --locked --bin eso-weave` passes.
- `git diff --check` and the strict UTF-8, no-BOM, LF-only, mojibake, replacement-character, and forbidden-dash audit pass for every changed text file and the S119 packet.
