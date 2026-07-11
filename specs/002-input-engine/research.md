# Research: Input Engine

Phase 0 decisions. No open NEEDS CLARIFICATION items remain; the spec and its
Clarifications resolved them.

## Core classification and hand-off

**Decision**: A platform-agnostic `InputEngine` owns the binding table, a focused
flag and a suspended flag (atomics), a set of currently-held bound keys, and the
sending half of a bounded channel to the worker. `classify(&self, event)` returns
a `Decision` (suppress or pass) and, on a new key-down of a bound active key,
pushes one `Action` onto the channel with a non-blocking `try_send`. The OS
adapter calls `classify` from the hook and suppresses when told to.

**Rationale**: This puts every safety-critical decision in one small, synchronous,
non-blocking function that is fully unit-testable with the mock, satisfying
constitution Principles II and III. `try_send` on a bounded `sync_channel` never
blocks; a full channel drops the action with a warning (FR-023), honoring the hard
"no blocking on the hook thread" rule.

**Alternatives considered**: Doing classification inside each backend (rejected:
duplicates the safety logic per platform and makes it hard to test). An unbounded
channel (rejected: unbounded growth under a stuck worker; the spec wants a bounded
non-blocking hand-off).

## Key-down, key-up, and auto-repeat

**Decision**: A bound key while focused suppresses both transitions. A held-keys
set records keys currently down; a key-down hands off an action only if the key
was not already held (so auto-repeat downs suppress without re-handing off), and a
key-up clears the held state.

**Rationale**: Directly implements FR-020 and keeps a stray key-up from reaching
the game.

## Recursion breaking

**Decision**: Synthesized input carries an origin marker. On Windows the marker is
the operating system's injected-input flag (`LLKHF_INJECTED` in the hook struct),
which `SendInput` sets automatically; the adapter maps injected events to
self-originated. On Linux the marker is structural: the backend grabs only the
physical keyboard device for reading and synthesizes through a separate uinput
virtual device, so it never reads its own output. The core treats self-originated
events as pass (never suppressed, never handed off).

**Rationale**: Uses each platform's idiomatic mechanism (the spec names
injected-input flagging for Windows) while exposing a single per-event origin to
the core, so the recursion-break test runs against the mock.

**Alternatives considered**: A custom injected-key sentinel sequence (rejected:
fragile and unnecessary given both platforms provide a clean distinction).

## Focus determination

**Decision**: The backend feeds the core's focused flag. Windows compares
`GetForegroundWindow` against the ESO window (matched by window title). Linux
queries the X11 active window (works on X11 and XWayland). On a pure-Wayland
session where the active window cannot be determined, focus cannot be confirmed
and interception stays off; this is a documented limitation consistent with the
master specification's Wayland best-effort posture and open item R3.

**Rationale**: Keeps focus-scoping (FR-001, FR-005) authoritative in the core
while each backend supplies the platform focus signal.

## Windows backend

**Decision**: Use `windows-sys` for `SetWindowsHookExW(WH_KEYBOARD_LL)`,
`SendInput`, `GetForegroundWindow`, and `timeBeginPeriod`/`timeEndPeriod` (raised
timer resolution for the worker lifetime). The hook callback reads the key and the
injected flag, calls `classify`, and returns a non-zero value to suppress or
`CallNextHookEx` to pass.

**Rationale**: `windows-sys` is a thin raw-FFI binding with a small build
footprint. The spec names exactly these Win32 facilities.

**Alternatives considered**: The higher-level `windows` crate (rejected: heavier
for a thin adapter); `winapi` (rejected: effectively unmaintained).

## Linux backend

**Decision**: Use the `evdev` crate to grab the physical keyboard
(`EVIOCGRAB`) for reading and to create a uinput virtual device for synthesis.
Startup surfaces a clear error when the process lacks permission (input group or
udev rule), which packaging documents.

**Rationale**: Matches the spec's evdev-plus-uinput approach that operates below
the display server. Declared as a Linux-only dependency so Windows builds never
compile it.

**Verification note**: This backend cannot run on the Windows build host. It is
type-checked with `cargo check --target x86_64-unknown-linux-gnu` where that
target is available, and its runtime behavior is validated manually on Linux. Any
residual gap is reported at the pre-push halt.

## Bindings persistence

**Decision**: Add an additive `bindings` field to `Settings` that serializes the
action-to-key table. Absent in older files, it defaults to the section 6.4
defaults, so no schema version bump is needed. A persisted table that is
conflicting or names an unknown action or key falls back to the affected defaults
with a notice.

**Rationale**: Reuses the S001 Config Store contract and its notice mechanism
(FR-016, FR-017, FR-021) without breaking existing settings files.

**Alternatives considered**: A schema version bump with a migration (rejected:
unnecessary for a purely additive field).
