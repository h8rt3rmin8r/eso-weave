# S125 Verification

Date: 2026-10-11. Local platform: Windows x64, pinned Rust toolchain.

## Regression evidence

- Before the UI repair, the actual empty-catalog heading painted at approximately
  1.228:1 contrast, essential instruction completeness failed, and the raw
  `ui.strong` guard identified the scoped bypasses. The repaired paint fixtures
  require at least 4.5:1 and include a wrong-role negative control.
- New command/worker/diagnostic/interaction fixtures were added through the
  implementation phases. The artifact plan requested test-first ordering for
  each phase; some new fixtures were completed alongside the implementation.
  Do not infer that every new contract was demonstrated failing before coding.
- The full suite caught the managed collector checksum fixtures after Catalog
  changed. The normalized checksum literal, canonical/local capture fixtures and
  capture request's exact source hash were updated together. Validation was not
  relaxed to accept stale captures.
- Final inspection found queue admission could fail while flushing succeeded.
  A full-queue fixture now requires recording to remain OFF if its enable warning
  was dropped; a subsequent explicit start succeeds after backlog delivery.
- Missing/nonnumeric version fields remain malformed input, while unsupported
  numeric versions have their own diagnostic. A deterministic permission-denied
  fixture verifies controlled guidance without leaking an injected path canary.
- Documentation fixtures exposed obsolete imported-only state assertions.
  They now require pre-import source inspection, metadata, independent refresh
  and stale-result handling, with removal fixtures preserving those contracts.

## Local gates

All required local commands finished successfully on the final implementation.

| Gate | Result |
| --- | --- |
| Format | Passed |
| Clippy, all targets/features, warnings denied | Passed |
| Complete locked Rust suite | Passed, 1,067 tests |
| Release binary with bundled documentation | Passed |
| Spelling | Passed |
| Repository and documentation policy fixtures | Passed, 191 fixtures |
| Trust and brand policies | Passed |
| mdBook examples, build and links | Passed |
| Generated-site policy and headless rendering | Passed |
| Release-note and Debian-validator fixtures | Passed |
| UTF-8/BOM, mojibake and diff checks | Passed |

Commands run in the foreground through a hidden, noninteractive launcher with
redirected output. Logs stay under ignored `target/s125-*.log` and are not
published as game evidence.

## Scope and delivery boundary

All seven issue outcomes map to [audit.md](audit.md). No dependency, addon schema,
managed-file inventory or version rollover is introduced. Native logs (#272)
and multiple retained sessions (#273) remain separate work.

These results verify repository behavior and generated/bundled documentation.
They do not prove installed ESO behavior or field parity. No owner field check
is requested. Automatic push and one official PR are authorized. External
reviews and exact-head CI must be satisfied within at most two Codex rounds;
the final boundary is owner review and merge, with no agent merge or release.

## First external review corrections

Automatic Codex review at 9d5503ea9e88d3e3068148c3b60435646e4505d2 raised two P2 findings. Both were reproduced before the fix: an import failure left one current error heading beside an older ready source summary; a valid stopped session with no records directed import instead of restart. New fixtures require failed malformed/missing/wrong-environment imports to replace both source and import status, and validate empty-session guidance through bounded inspection and the worker import result. The focused suite passes 15 tests after correction. The same controlled readiness classifier now serves inspection and empty-import recovery.

The same empty-session recovery is covered through registered in-game status and repeated-start commands for Live and PTS (45 addon tests pass). One full Windows run encountered TCP reset 10054 in the unchanged local-service HTTP fixture. Its isolated 18-test suite and the subsequent complete 1,066-test run passed without code changes; no cause was established. Final fmt, clippy, all 1,067 Rust tests and the rebuilt release binary pass after the addon-copy changes.
