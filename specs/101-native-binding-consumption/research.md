# Research: Native Binding Consumption

## Windows interception and synthesis

**Decision**: Install `WH_KEYBOARD_LL` and `WH_MOUSE_LL` on the existing interception thread, keep its message loop, classify only `HC_ACTION`, and use injected flags for recursion breaking. Continue using `SendInput` for serialized keyboard and mouse output.

**Rationale**: Microsoft documents that the low-level mouse hook runs on the installing thread, requires a message loop, may suppress handled events with a nonzero return, and reports injected input. `SendInput` inserts a serialized array but does not reset current keyboard state, confirming that physical modifier compatibility must be handled by the application.

**Sources**:

- https://learn.microsoft.com/en-us/windows/win32/winmsg/lowlevelmouseproc
- https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowshookexw
- https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput
- https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input

**Alternatives rejected**: Raw Input lacks the existing synchronous suppress-or-pass boundary. Releasing active modifiers before synthesis violates physical ownership.

## Linux device and wheel semantics

**Decision**: Retain evdev grabs and uinput recursion separation, expand to one keyboard and one pointer device, multiplex them with `libc::poll`, and advertise the union of key and relative-axis capabilities. Treat buttons as `EV_KEY` and wheel directions as signed `REL_WHEEL` pulses.

**Rationale**: The Linux input protocol uses `EV_KEY` for keys and buttons, `EV_REL` for relative axes, and `SYN` packets for grouped changes. Kernel guidance identifies `BTN_LEFT`, later mouse buttons, and `REL_WHEEL` as the correct pointer codes.

**Sources**:

- https://kernel.org/doc/html/latest/input/event-codes.html
- https://kernel.org/doc/html/latest/input/uinput.html
- https://docs.rs/evdev/0.13.2/evdev/struct.Device.html
- https://docs.rs/evdev/0.13.2/evdev/uinput/struct.VirtualDevice.html

**Alternatives rejected**: Keyboard-only capture cannot observe mouse-bound skills. Reading virtual output reintroduces recursion. Wheel holds conflict with relative-axis semantics.

## Modifier ownership

**Decision**: Exact-match physical modifiers at trigger admission, preflight every chord, and wrap each generated primary-down with only its missing modifiers. Temporary generated modifiers release immediately after primary-down; held primaries remain tracked through matching up.

**Rationale**: Keeping temporary modifiers only around the activation edge permits sequences containing different chord modifiers without synthesizing an up for a physical modifier. A plan that would require neutralizing a physical modifier is rejected before the original trigger is suppressed.

**Alternatives rejected**: Holding a generated modifier through a heavy attack can contaminate the skill chord. Releasing a physical modifier violates issue #188. Ignoring extra modifiers makes exact chords ambiguous.

## Authority and migration

**Decision**: Put native combat facts in the existing weave authorization generation, copy a complete plan into each queued action, and remove migrated combat entries from desktop persistence. Keep only F1, F2, and F3 in `BindingTable`.

**Rationale**: Existing gates already invalidate queued and running work through one epoch. An immutable queued plan prevents mixing evidence generations. Retaining hidden combat keys would preserve the duplicate authority this issue removes.

**Alternatives rejected**: Live per-step lookups can combine old and new bindings. Combat fallback violates fail-closed acceptance. Moving desktop toggles into ESO evidence removes independent safety controls.
