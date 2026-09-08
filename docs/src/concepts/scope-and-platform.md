# Scope and Platform Model

Windows Input uses a keyboard hook (`WH_KEYBOARD_LL`) plus `SendInput`. These
terms distinguish it from the Linux `evdev` and `uinput` path.

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

| Boundary | Windows 10 and 11 x64 | Linux x64 with X11 | Linux x64 with Wayland |
| --- | --- | --- | --- |
| Installation discovery | Steam and generic uninstall Registry entries, plus Epic manifests | Native and Flatpak Steam library metadata for Proton | Same as Linux X11 |
| Process observation | Tool Help process snapshot | `/proc` process names | Same as Linux X11 |
| Focus | Foreground window process or title, depending on subsystem | X11 active-window title | Requires an XWayland game window; pure Wayland yields Unknown |
| Physical input | `WH_KEYBOARD_LL` | evdev device grab | evdev device grab below the display server |
| Synthesized input | `SendInput` | uinput virtual device | uinput virtual device |
| Screen sampling | GDI capture of the composited desktop | X11 capture | Requires an XWayland surface |
| Window-position restore | Virtual-screen bounds keep a restored window reachable | Placement is left to the window manager | Placement is left to the window manager |
| Packages | x64 MSI | x86_64 `.deb`, AppImage, tarball | Same as Linux X11 |

The application identifies the ESO window per platform and activates
interception only while that window has keyboard focus. Linux input requires
membership in the `input` group or equivalent udev permissions. Pure Wayland can
provide evdev and uinput access but cannot supply the X11 focus and capture facts
that authorize operation, so an XWayland ESO surface is required.

The advertised Linux uinput keys include every supported application mapping,
including `E` and `F3`. The virtual device also advertises every key reported by
the selected physical keyboard, preserving ordinary key pass-through after the
grab.

PixelBeacon lifecycle writes are ownership gated on every platform. An existing
target that cannot be proven managed is reported as Unmanaged, offers no
lifecycle buttons, and remains unchanged. Managed updates replace embedded files
in place, while removal still requires the managed marker.

macOS, Linux aarch64, pure Wayland capture, direct process-memory reading, and
network-traffic inspection are outside the supported product scope.

See [Responsible Use](../getting-started/responsible-use.md) for the project and
account-safety boundary, and
[Release and Packaging](../development/release-and-packaging.md) for the artifact
pipeline.
