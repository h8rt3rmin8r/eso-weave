# Glossary

- **Weave:** Fitting a basic attack, block, or bash into the same
  global-cooldown window as a skill activation.
- **GCD:** ESO's 1000 ms global cooldown for skill activations.
- **Skill slot:** One of seven automatable inputs: skills 1 through 5, Ultimate,
  and Synergy.
- **Weave type:** Light Attack (`LA`), Heavy Attack (`HA`), Bash Attack (`BA`),
  or Block Casting (`BL`).
- **Weapon bar:** ESO's front or back weapon set. The active bar and each bar's
  weapon class can affect heavy attack timing and Ultimate readiness.
- **PixelBeacon:** The companion addon that publishes game observations as
  on-screen colors.
- **Pixel bus:** The block protocol PixelBeacon renders and ESO Weave samples.
- **Beacon block:** One solid-color square at a protocol-defined physical-pixel
  position.
- **Marker:** A block's green channel, used to identify the signal it carries.
- **Menu gate:** Suppression of interception and synthesis while a native menu
  or text field is open.
- **Quickslot:** ESO's active consumable slot.
- **Resource watch:** Independent Auto Potion enablement and threshold for
  Health, Magicka, or Stamina.
- **Interact key:** The in-game interaction binding, `E` by default, used by
  fishing to cast, reel, and recast.
- **Managed marker:** `## X-ESO-Weave-Managed: true` in the PixelBeacon manifest.
  This line gates install-over-existing, update, removal, API refresh, and
  block-size redeploy. An existing target without proven ownership is Unmanaged
  and remains unchanged.

## Search vocabulary

These player terms and abbreviations lead to their canonical explanations.

- **Weaving:** animation cancel, LA weave. See [Weaving](../features/weaving.md).

- **Light Attack:** left click. See [Weaving](../features/weaving.md).

- **Heavy Attack:** hold attack. See [Weaving](../features/weaving.md).

- **Block Casting:** cast while blocking. See [Weaving](../features/weaving.md).

- **Weave Delay:** `d_weave`, skill delay. See [Weaving](../features/weaving.md).

- **Weapon Bar:** primary bar, backup bar. See [Weaving](../features/weaving.md).

- **Input Safety:** key interception, key suppression, input hook. See [Input Safety](../concepts/input-safety.md).

- **Synthesized Input:** `SendInput`. See [Input Safety](../concepts/input-safety.md).

- **Suspension:** pause automation, F1. See [Input Safety](../concepts/input-safety.md).

- **Menu Gate:** text field, native menu. See [Action Authorization](../concepts/action-authorization.md).

- **Game Context:** menu state. See [Game Observation](../concepts/game-observation.md).

- **Life State:** player death. See [Game Observation](../concepts/game-observation.md).

- **Travel:** teleport, wayshrine, zone change, jump pending. See [Game Observation](../concepts/game-observation.md).

- **Sprinting:** movement state, mounted movement. See [Game Observation](../concepts/game-observation.md).

- **Unknown:** missing evidence, invalid signal, fail closed. See [Action Authorization](../concepts/action-authorization.md).

- **Fishing:** auto fishing. See [Fishing](../features/fishing.md).

- **No Cast Detected:** bait missing, cast timeout. See [Fishing](../features/fishing.md).

- **Signal Lost:** beacon missing, overlay hidden. See [Troubleshooting](../getting-started/troubleshooting.md).

- **Auto Potion:** potion trigger, automatic quickslot. See [Auto Potion](../features/auto-potion.md).

- **Resource Watch:** health threshold, magicka threshold, low resource. See [Auto Potion](../features/auto-potion.md).

- **Quickslot:** potion slot, Q key. See [Auto Potion](../features/auto-potion.md).

- **Retry Interval:** repeat protection, potion delay. See [Auto Potion](../features/auto-potion.md).

- **Layout Header:** geometry negotiation. See [Pixel Bus Protocol](pixel-bus-protocol.md).

- **Payload Block:** beacon block, B0 through B28. See [Pixel Bus Protocol](pixel-bus-protocol.md).

- **Heartbeat:** beacon freshness, status block. See [Pixel Bus Protocol](pixel-bus-protocol.md).

- **Capture Tolerance:** color tolerance, decode tolerance. See [Pixel Bus Protocol](pixel-bus-protocol.md).

- **Ultimate Cost:** front-bar cost, back-bar cost. See [Ultimate Resource](../features/ultimate-resource.md).

- **Ready:** readiness indicator. See [Ultimate Resource](../features/ultimate-resource.md).

- **Configuration:** settings file, `config.json`. See [Configuration](configuration.md).

- **Session State:** `state.json`. See [Configuration](configuration.md).

- **Invalid Configuration:** `.invalid` file. See [Configuration](configuration.md).

- **Live Log:** log viewer, in-memory log. See [Logging](logging.md).

- **File Logging:** debug log, trace log. See [Logging](logging.md).

- **Windows Input:** `WH_KEYBOARD_LL`, `SendInput`. See [Scope and Platform](../concepts/scope-and-platform.md).

- **Linux Input:** input group, udev rule. See [Scope and Platform](../concepts/scope-and-platform.md).

- **XWayland:** X11 compatibility, game capture. See [Scope and Platform](../concepts/scope-and-platform.md).

- **API Version:** `APIVersion`. See [PixelBeacon](../features/pixelbeacon.md).

- **Checksum:** `SHA256SUMS`. See [Installation](../getting-started/installation.md).

- **Troubleshooting:** not working, no input, signal unavailable. See [Troubleshooting](../getting-started/troubleshooting.md).

- **Startup Failure:** startup panic, error dialog. See [Troubleshooting](../getting-started/troubleshooting.md).

- **Test Strategy:** mock backend. See [Test Strategy](../development/test-strategy.md).

- **Release Pipeline:** publish release. See [Release and Packaging](../development/release-and-packaging.md).
