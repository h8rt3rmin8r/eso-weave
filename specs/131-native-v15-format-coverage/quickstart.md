# S131 Validation Guide

All fixtures are synthetic strings derived from pinned static producer writers,
not captured game output. No game session or settings change is required.

1. Run the S131 format tests for normal/inline and both verbose profiles. Compare
   semantic quantities across modes and exact original raw/context lines. Cover
   repeated unit labels, optional synergy/remove IDs, interrupted casts,
   scribing, null metadata, equipment aliases and literal string punctuation.
2. Split input at every byte and checkpoint/resume at meaningful boundaries,
   including inside UTF-8, TAB labels, brackets, inline tails and CRLF. Compare
   fights, positions, canonical bytes, IDs and projection to a single feed.
3. Change profiles inside a run/fight. Reject malformed known labels and
   incomplete tails; retain unknown records; recover only on a valid BEGIN_LOG.
   Exercise existing line/token/depth/context/fight limits and corrupt state.
4. Run schema 1/2 canonical and identity regressions, copied/extracted evidence,
   controlled collisions, deletion suppression and rollback/restart tests.
5. Feed supported profiles through automatic catchup and live append, including
   failed commit/cancellation and authenticated HTTP/MCP reads. Preserve atomic
   fight/progress state and explicit unavailable metrics.
6. Run cargo fmt, clippy and all locked Rust tests, documentation policy,
   spelling, mdBook examples/build/browser checks and release embedded-guide
   checks through the verified hidden launcher. Check UTF-8/no BOM, LF, prose
   punctuation, mojibake, JSON and git diff hygiene.
7. Publish the authorized PR, resolve all findings within two Codex review
   rounds, verify hosted checks on its final head and stop for owner merge.
