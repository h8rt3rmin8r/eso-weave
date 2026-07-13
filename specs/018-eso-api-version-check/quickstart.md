# Quickstart: ESO API Version Check Automation

Validation guide proving the feature end to end. Implementation details live in
`data-model.md`, `contracts/api-check.md`, and `tasks.md`.

## Prerequisites

- Rust 1.96 toolchain (`rust-toolchain.toml` pins it).
- Network access for the one-time `cargo` fetch of the new `ureq` dependency.

## Build and unit tests

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: green. The new tests cover, with a mock `GameVersionSource` and a
`tempfile` AddOns root:

- `parse_commit_message_version` on `"12.0.6"`, `"12.0.0 Season Zero Pt.2"`, and
  malformed inputs.
- `rewrite_api_version` primary/keep/drop token rule and preservation of all other
  lines and the managed marker.
- `run_check` resolution order (`max(stored, default)`), never-downgrade, and the
  marker gate refusing an unmanaged manifest.
- `SessionState` round-trips with and without the `api_version` section (v1 to v2
  migration).

## Behavior checks (manual)

1. **Offline fallback**: disconnect the network and launch the app. Expected: the
   window appears immediately; no error dialog; logs show the check failed and the
   app proceeded. Any manifest written carries a valid APIVersion.
2. **In-place upkeep**: install the addon, then hand-edit the on-disk
   `PixelBeacon.txt` `## APIVersion:` primary token to an older value (for example
   `101040`), keeping the managed marker. Relaunch. Expected: the primary token is
   restored to the compiled default (`101050`), all other lines unchanged.
3. **Managed-marker gate**: with the addon folder present, delete the managed
   marker line from `PixelBeacon.txt` and set an old APIVersion. Relaunch.
   Expected: the file is left untouched (the app refuses to write an unmanaged
   manifest).
4. **No downgrade / no churn**: set the primary token to a value higher than the
   default (for example `101060`). Relaunch. Expected: no write; file unchanged.
5. **Bump detection**: with the live game version ahead of `DEFAULT_GAME_VERSION`
   (verifiable when a real patch has shipped, or by pointing a mock source at a
   newer version in a test), confirm the live log shows a plain-language notice to
   update ESO Weave, and `state.json` records the newer `last_seen_game_version`.

## Success signals

- `state.json` contains an `api_version` object after a run.
- The installed manifest's `## APIVersion:` primary token equals the resolved
  effective version.
- The app never blocks on startup and never crashes on a network or parse failure.
