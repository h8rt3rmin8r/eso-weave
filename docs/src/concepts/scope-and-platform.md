# Scope and Platform Model

ESO Weave is a single-crate Rust desktop companion with three principal
capabilities:

1. Combat weaving replaces a configured skill press with a basic attack and
   skill sequence while ESO is focused.
2. Fishing can cast, observe a bite, reel, and recast.
3. Auto Potion can use the active quickslot when a watched resource is low.

Weaving has no in-game dependency. Fishing and Auto Potion consume screen
telemetry from the embedded PixelBeacon addon. PixelBeacon renders a small grid
of solid colors in the game window, and ESO Weave samples the displayed pixels.

## Supported capabilities

- Focus-scoped key interception and input synthesis.
- Light Attack, Heavy Attack, Bash Attack, and Block Casting sequences.
- Per-skill enablement, weave type, and timing overrides.
- Weapon-bar-aware heavy attack timing and optional latency adaptation.
- Fishing through a detector abstraction.
- Auto Potion using resource, quickslot, and cooldown observations.
- Input suppression while unsafe game, lifecycle, travel, roll-dodge, or menu
  conditions are observed.
- Embedded PixelBeacon installation, update, verification, and managed removal.
- Configurable keybindings, structured logging, and a live log viewer.
- Windows MSI and Linux `.deb`, AppImage, and tarball packages.

## Platform behavior

| Platform | Input | Screen sampling |
| --- | --- | --- |
| Windows 10 and 11 x64 | `WH_KEYBOARD_LL` and `SendInput` | GDI capture of the composited desktop |
| Linux x64 with X11 | evdev grab and uinput | X11 capture |
| Linux x64 with Wayland | evdev grab and uinput below the display server | An XWayland game surface is required |

The application identifies the ESO window per platform and activates
interception only while that window has keyboard focus. Linux input requires
membership in the `input` group or equivalent udev permissions.

See [Responsible Use](../getting-started/responsible-use.md) for the project and
account-safety boundary.
