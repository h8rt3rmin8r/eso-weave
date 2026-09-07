# Contract: Terminology and Search

## Purpose

The published corpus uses canonical product terms while including natural
player, platform, and developer vocabulary that readers are likely to search.
Aliases aid discovery and never replace interface labels or protocol names.

## Rules

- Canonical terms use the same capitalization as the interface or protocol.
- Each required alias appears naturally on its target page or in the glossary
  with a direct link to that page.
- An alias must be explained in context, not hidden in metadata or a keyword
  dump.
- Ambiguous terms identify the intended ESO Weave meaning.
- Safety terms never use `ready`, `active`, `available`, and `known`
  interchangeably.
- Search coverage is checked by exact normalized phrases and destination paths.
- Singular and plural forms do not require duplicate prose when search stemming
  already resolves them.

## Required Search Map

| Canonical term | Required aliases or related queries | Canonical target |
| --- | --- | --- |
| ESO Weave | combat weaving companion, desktop companion | `docs/src/README.md` |
| Weaving | weave, animation cancel, light attack weave, LA weave | `docs/src/features/weaving.md` |
| Light Attack | LA, left click, primary click | `docs/src/features/weaving.md` |
| Heavy Attack | HA, hold attack | `docs/src/features/weaving.md` |
| Bash Attack | bash weave, interrupt, block bash | `docs/src/features/weaving.md` |
| Block Casting | block cast, cast while blocking | `docs/src/features/weaving.md` |
| Global Cooldown | GCD, cooldown window | `docs/src/features/weaving.md` |
| Weave Delay | d_weave, attack delay, skill delay | `docs/src/features/weaving.md` |
| Weapon Bar | front bar, primary bar, back bar, backup bar, bar swap | `docs/src/features/weaving.md` |
| Latency Adaptation | ping adjustment, latency scaling, k factor | `docs/src/features/weaving.md` |
| Input Safety | key interception, key suppression, input hook, pass through | `docs/src/concepts/input-safety.md` |
| Synthesized Input | generated input, injected input, SendInput, uinput | `docs/src/concepts/input-safety.md` |
| Suspension | suspend, pause automation, F1 | `docs/src/concepts/input-safety.md` |
| Menu Gate | chat protection, typing protection, text field, native menu | `docs/src/concepts/action-authorization.md` |
| Game Context | gameplay detection, menu state, focused game | `docs/src/concepts/game-observation.md` |
| Life State | alive, dead, reincarnating, player death | `docs/src/concepts/game-observation.md` |
| World State | loading screen, world transition, player activation | `docs/src/concepts/game-observation.md` |
| Travel | teleport, wayshrine, recall, zone change, jump pending | `docs/src/concepts/game-observation.md` |
| Roll Dodge | dodge roll, dodge state, roll gate | `docs/src/concepts/game-observation.md` |
| Sprinting | sprint, movement state, mounted movement | `docs/src/concepts/game-observation.md` |
| Unknown | unavailable, missing evidence, invalid signal, fail closed | `docs/src/concepts/action-authorization.md` |
| Fishing | auto fishing, cast, bite, reel, recast | `docs/src/features/fishing.md` |
| Interact Key | use key, action key, E key | `docs/src/features/fishing.md` |
| No Cast Detected | bait missing, arm timeout, cast timeout | `docs/src/features/fishing.md` |
| Signal Lost | beacon missing, heartbeat timeout, overlay hidden | `docs/src/getting-started/troubleshooting.md` |
| Auto Potion | auto pot, potion trigger, automatic quickslot | `docs/src/features/auto-potion.md` |
| Resource Watch | health threshold, magicka threshold, stamina threshold, low resource | `docs/src/features/auto-potion.md` |
| Quickslot | potion slot, consumable wheel, Q key | `docs/src/features/auto-potion.md` |
| Retry Interval | retry floor, repeat protection, potion delay | `docs/src/features/auto-potion.md` |
| PixelBeacon | addon, pixel addon, telemetry overlay, color blocks | `docs/src/features/pixelbeacon.md` |
| Pixel Bus | pixelbus, screen telemetry, color protocol | `docs/src/reference/pixel-bus-protocol.md` |
| Layout Header | H0, H1, H2, geometry negotiation, protocol version | `docs/src/reference/pixel-bus-protocol.md` |
| Payload Block | beacon block, B0 through B28, marker, checksum | `docs/src/reference/pixel-bus-protocol.md` |
| Heartbeat | beacon freshness, status block, B0 | `docs/src/reference/pixel-bus-protocol.md` |
| Capture Tolerance | color tolerance, compositor drift, decode tolerance | `docs/src/reference/pixel-bus-protocol.md` |
| Ultimate | ult, Ultimate points, Ultimate resource | `docs/src/features/ultimate-resource.md` |
| Ultimate Cost | cast cost, front-bar cost, back-bar cost, readiness tick | `docs/src/features/ultimate-resource.md` |
| Ready | enough Ultimate, cast-ready, readiness indicator | `docs/src/features/ultimate-resource.md` |
| Configuration | settings file, config.json, preferences | `docs/src/reference/configuration.md` |
| Session State | state.json, saved window position, suspend persistence | `docs/src/reference/configuration.md` |
| Invalid Configuration | corrupt config, reset to defaults, .invalid file | `docs/src/reference/configuration.md` |
| Live Log | log viewer, in-memory log, ring buffer | `docs/src/reference/logging.md` |
| File Logging | log file, monthly log, debug log, trace log | `docs/src/reference/logging.md` |
| Windows Input | keyboard hook, WH_KEYBOARD_LL, SendInput | `docs/src/concepts/scope-and-platform.md` |
| Linux Input | evdev, uinput, input group, udev rule | `docs/src/concepts/scope-and-platform.md` |
| XWayland | Wayland, X11 compatibility, game capture | `docs/src/concepts/scope-and-platform.md` |
| PixelBeacon Status | install addon, update addon, unmanaged addon | `docs/src/features/pixelbeacon.md` |
| API Version | APIVersion, outdated addon, ESO client version | `docs/src/features/pixelbeacon.md` |
| Release Package | MSI, deb, AppImage, tarball | `docs/src/getting-started/installation.md` |
| Checksum | SHA256, SHA256SUMS, download verification | `docs/src/getting-started/installation.md` |
| Troubleshooting | not working, no input, signal unavailable, game not active | `docs/src/getting-started/troubleshooting.md` |
| Startup Failure | failed to start, startup panic, error dialog | `docs/src/getting-started/troubleshooting.md` |
| Test Strategy | unit tests, mock backend, headless UI, contract tests | `docs/src/development/test-strategy.md` |
| Release Pipeline | release workflow, tag build, publish release | `docs/src/development/release-and-packaging.md` |

## State Vocabulary

The following terms have distinct meanings and must remain distinct:

| Term | Meaning |
| --- | --- |
| Observed | A current source positively supplied a valid value |
| Fresh | The heartbeat or observation remains within its validity window |
| Unknown | No valid value currently authorizes a conclusion |
| Unavailable | A specific field cannot currently be decoded or supplied |
| Active | A named runtime or protocol state, not a synonym for enabled |
| Requested | The operator asked for a feature, even if it is dormant or blocked |
| Ready | Every non-trigger prerequisite holds but the trigger condition does not |
| Triggered | The current evaluation submitted an attempt |
| Gated | A safety authority prevents interception or synthesis at a named boundary |
| Pass through | The original physical input continues to ESO unchanged |
| Dropped | Queued or attempted work is discarded and is not replayed |

## Validation

The terminology check must fail when:

- a canonical target does not exist;
- a required canonical term is absent from its target;
- none of an alias group's phrases occurs on the target or in a directly linked
  glossary entry;
- interface capitalization drifts for Game Context, Life State, World State,
  Weapon Bar, PixelBeacon Status, Auto Potion, or Live Log;
- `Unknown` is described as positive authorization;
- a deprecated source name, archived page, or project record becomes a competing
  search destination; or
- keyword-only prose is introduced solely to satisfy the check.

