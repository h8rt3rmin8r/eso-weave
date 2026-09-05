# Quickstart: Life State Safety

1. Install or update the managed PixelBeacon after building S048, then run
   `/reloadui` in ESO.
2. With the game active, confirm Live HUD shows Life state as Alive.
3. Toggle auto-potion and fishing on, then enter a dead state. Confirm their
   visible state names the life blocker and no key is synthesized.
4. Press a configured weave key while dead. Confirm the original key passes
   through and no weave sequence runs.
5. Reincarnate. Confirm Reincarnating remains blocked, then Alive returns without
   replaying any action that was due while blocked.
6. Collapse System and State by activating its header. Confirm Skills moves up,
   restart the application, and confirm the preference is retained.

Validation commands:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

The pull request closes #53, #54, #55, and #58. It must not close or implement
the separate world-transition, roll-dodge, sprint, or effect-database issues.

