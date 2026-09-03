# Quickstart: Validate Game Runtime and Context Truth

## Automated validation

From the repository root:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

The focused tests cover:

- provider reconciliation and invalid roots
- the game x launcher runtime matrix
- Game Context precedence
- menu decode validity and signal recovery
- game-inactive input classification
- inactive and unfocused controller pauses that preserve requested toggles
- held-key and stale menu-gate cleanup across game exit and focus loss
- downstream dormancy and reader reset
- rendered Game Context values and help affordance

## Windows field validation

1. Start ESO Weave with ESO and its launcher closed.
   - Installation names the validated provider when present.
   - Runtime is Inactive.
   - Game Context is Not detected.
   - Every game-derived field is dormant.
2. Start only `Bethesda.net_Launcher.exe`.
   - Runtime becomes Launcher open within two seconds.
   - The interface does not say Ready.
3. Start ESO.
   - Runtime becomes Active within two seconds.
   - Focus and PixelBeacon evidence determine Game Context.
4. Close the launcher while remaining in game.
   - Runtime remains Active.
5. Exercise gameplay, inventory, map, chat entry, another menu, and Alt+Tab.
   - Each context is truthful and every menu gates input.
6. Reload the UI or hide/remove PixelBeacon.
   - Runtime remains Active.
   - Game Context becomes Signal unavailable rather than Gameplay.
7. Exit ESO while leaving the launcher open, then close the launcher.
   - Runtime moves first to Launcher open and then Inactive.
   - Live fields become dormant and no input is emitted.
8. Restart ESO without restarting ESO Weave.
   - Fresh observations repopulate normally.

Capture a sanitized receipt containing provider, state transitions, and the
absence of repeated normal logs. Do not capture personal paths.

### S041 Windows receipt (2026-09-03)

- One validated Steam candidate was present.
- One generic ESO uninstall entry produced a second validated candidate at a
  distinct root.
- Provider reconciliation reported Ambiguous and exposed no installation path.
- The live process observation was Inactive with both game and launcher absent.
- Automated lifecycle matrices cover launcher-only, Active with launcher,
  Active after launcher exit, game exit to launcher-only, full exit, unknown
  probes, and restart republishing.

## Linux field validation

Repeat the lifecycle and context sequence under Steam Proton on X11. Confirm the
Steam library and compatdata roots validate, the launcher and game are detected
independently, and focus transitions are observed.

On a Wayland-only session where X11 focus evidence is unavailable, confirm focus
is Unknown and input remains safety-negative rather than being reported as
Unfocused.

## Regression validation

- Weaving still works while ESO is Active and focused without requiring
  PixelBeacon.
- Fishing and auto-potion retain their signal-loss safety behavior.
- Addon install, update, and uninstall reminders still consume the shared game
  runtime probe.
- Managed-marker uninstall protection and AddOns path containment tests remain
  unchanged and green.
