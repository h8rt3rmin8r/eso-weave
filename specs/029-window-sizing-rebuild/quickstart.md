# Quickstart: Window Sizing Model Rebuild

Validation guide for slice 029. Proves the four issue-#8 defects are fixed. This
is a desk-only UI change; no game is required.

## Prerequisites

- Repo on `main`, Rust toolchain per `rust-toolchain.toml`.

## Automated verification (CI parity)

Run in the foreground and watch to completion:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all green. The `tests/app_window_sizing.rs` suite covers:

- `measurement_stable`: first measurement not stable; two equal-within-epsilon
  frames are stable; a differing frame is not.
- `content_min_size`: boot floor until stable, then measured wins (per dimension,
  can be smaller than the floor); a later smaller stable measurement is not
  latched.
- `open_log_reserve`: equals `log_min_height + row`, strictly greater than
  `log_min_height`.
- `split_log_height`: shares a height delta by the live fraction, clamps to
  `[log_min, window - content]`, remainder to central.
- Resizability invariant: at `content_h + open_log_reserve(row)` the log max is
  strictly greater than its min.

## Manual desk validation (run the app)

```bash
cargo run
```

Reproduce issue #8's steps and confirm the fixes:

- MV-1 (defect 1, closed): with the log closed, shrink the window to its minimum.
  Expected: the last control sits near the bottom edge with no large empty band;
  the minimum tracks the content (it adjusts if a control row changes).
- MV-2 (defect 2, open): open the View -> live log toggle. Drag the log pane's top
  splitter upward. Expected: the pane grows; there is no empty reserved band above
  the log.
- MV-3 (defect 2, growth): with the log open, enlarge the window vertically.
  Expected: the extra height is shared, so the log grows by roughly its share (not
  zero) and the central area takes the rest.
- MV-4 (defect 3, height-neutral): note the window height, open the log, enlarge
  the pane, then close the log. Expected: the window returns to its original
  height with no residual empty band.
- MV-5 (compressible + resizable at minimum): with the log open, shrink the window
  toward its minimum. Expected: the log compresses toward six lines and the window
  can still shrink; at the minimum the pane still has a small drag range.

## Rollback

The change is confined to the app UI sizing model and its tests. Reverting the
commit restores the prior (v0.7.0) behavior. No persisted data shape changes, so
existing config files are unaffected either way.
