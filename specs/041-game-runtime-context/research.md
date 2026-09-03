# Research: Game Runtime and Context Truth

## R1: Provider evidence is reconciled by validated root

**Decision**: Gather provider-owned candidates, normalize and validate their ESO
root, then reconcile candidates by root. Steam and Epic manifests outrank the
generic ESO uninstall entry for the same root. A generic ESO entry identifies a
standalone installation only when no stronger provider owns that root. Distinct
validated roots are Ambiguous rather than selected arbitrarily.

**Rationale**: A sanitized observation on the maintainer's Windows system found
both `Steam App 306130` and `The Elder Scrolls Online` uninstall entries pointing
at the same validated root. Treating the generic entry as standalone would
report two providers for one Steam installation. Steam's store identifies ESO as
app 306130. Microsoft's uninstall documentation describes `InstallLocation` as
installer metadata, but does not make a generic product entry proof of a sales
channel. Epic documents its `.item` manifest directory and the `AppName` field;
the platform-owned manifest plus validated ESO artifacts is sufficient without
hard-coding an anecdotal catalog id.

**Evidence**:

- [Steam app 306130](https://store.steampowered.com/app/306130/The_Elder_Scrolls_Online/)
- [Microsoft uninstall registry properties](https://learn.microsoft.com/en-us/windows/win32/msi/uninstall-registry-key)
- [Epic manifest directory and AppName](https://dev.epicgames.com/documentation/unreal-engine/academic-installation-of-unreal-engine)
- Sanitized local observation: the Steam and generic ESO uninstall entries both
  expose `InstallLocation` and resolve to one root containing the launcher and
  client artifacts.

**Alternatives considered**:

- Hard-code standalone and Epic product ids. Rejected because no authoritative
  source established stable ESO-specific ids and validated platform manifests
  already provide stronger evidence.
- Let the first provider win. Rejected because enumeration order is not product
  truth.
- Treat the AddOns directory as installation proof. Rejected because it can
  survive uninstall and is user data rather than provider evidence.

## R2: Official executable names define runtime observations

**Decision**: Treat `eso64.exe` and the legacy `eso.exe` as game processes, and
`Bethesda.net_Launcher.exe` as the ESO launcher process. The game observation
always outranks the launcher observation.

**Rationale**: ESO support names the launcher and client executables and their
default relative locations. A process snapshot can therefore classify runtime
without reading game memory. Platform launchers such as Steam and Epic are not
the ESO launcher state: they may be open for unrelated reasons.

**Evidence**:

- [ESO launcher troubleshooting](https://help.elderscrollsonline.com/app/answers/detail/a_id/3054/)
- [ESO firewall exception list](https://help.elderscrollsonline.com/app/answers/detail/a_id/23980/)
- [ESO single-instance guidance](https://help.elderscrollsonline.com/app/answers/detail/a_id/10365/)

**Alternatives considered**:

- Treat Steam or Epic Games Launcher as Launcher open. Rejected because those
  processes do not prove the ESO launcher is open.
- Call process presence Launcher ready. Rejected because presence cannot
  distinguish patching, login, maintenance, errors, or a ready Play button.

## R3: Game presence leaves the beacon manager

**Decision**: Introduce `src/game/` as the owner of installation, runtime,
focus, freshness, surface, and Game Context. Move the Steam VDF parser there.
The beacon manager consumes game presence for lifecycle reminders but no longer
owns it.

**Rationale**: `beacon::probe_game_running` exists only because addon lifecycle
was its first caller. Expanding it into provider discovery and application-wide
state would make the addon manager own a domain that outlives the addon. A game
module gives the GUI, input safety, reader, and beacon lifecycle one neutral
contract without introducing a new crate or dependency layer.

**Alternatives considered**:

- Expand `src/beacon/`. Rejected because installation and runtime remain useful
  when PixelBeacon is absent.
- Put the state in `src/app/`. Rejected because input and reader workers need the
  same state without depending on presentation code.

## R4: Poll on the existing pixel-bus worker at one-second cadence

**Decision**: Add a one-second game-presence poll to the existing pixel-bus
worker, before any early return caused by a missing sampler. Reuse that worker's
lock order to route lifecycle transitions. Do not add a process or a thread.

**Rationale**: The worker already wakes at the idle cadence, owns the reader and
all input-driving controllers, and is explicitly outside the hook thread. One
second meets the two-second convergence target and avoids independent workers
competing to clear observations.

**Alternatives considered**:

- Poll from the GUI frame. Rejected because platform and file-system work must
  not ride rendering cadence.
- Create a dedicated thread. Rejected because the existing worker already owns
  the required routing boundary.
- Probe on every 100 ms active sample. Rejected as unnecessary process and disk
  churn.

## R5: Raw observations stay independent

**Decision**: Store installation, runtime, focus, PixelBeacon freshness, and
surface observation independently. Derive Game Context with a pure precedence
function. A valid `MenuSurface::None` means gameplay only inside an authoritative
surface observation; unavailable is a separate value.

**Rationale**: The current defect is caused by using `MenuSurface::None` for both
valid gameplay and decode failure. A larger enum combining every axis would hide
the same ambiguity in a new type and multiply states.

**Alternatives considered**:

- Add `Unknown` to `MenuSurface`. Rejected because focus, runtime, and freshness
  would still be implicit.
- Store only the final display label. Rejected because safety consumers and
  diagnostics need the underlying evidence.

## R6: Runtime and focus fail closed without making weaving depend on PixelBeacon

**Decision**: Add an explicit game-active gate to the input engine and
auto-potion controller. A non-Active runtime or non-Focused focus blocks input.
PixelBeacon unavailability produces Signal unavailable in Game Context and
continues the existing feature-specific signal-loss policies. It does not make
basic weaving require the optional addon.

**Rationale**: The constitution says interception is scoped to the focused game
window and the companion remains useful without PixelBeacon. Runtime and focus
are authoritative outside-game observations, while missing PixelBeacon cannot
prove that a menu is open. Fishing and auto-potion already fail closed on signal
loss because they consume beacon values.

**Alternatives considered**:

- Gate every weave whenever PixelBeacon is unavailable. Rejected because it
  turns an optional addon into a prerequisite for the core weave engine.
- Leave runtime implicit in focus. Rejected because stale or failed focus
  evidence must not authorize input after the game exits.

## R7: Lifecycle exit clears both public and reader state

**Decision**: When runtime leaves Active, reset the reader's observation cache,
clear game-derived values in the weave engine, set focus and game-active safety
gates false, and move the shared Game Context to Not detected. Requested
auto-potion enablement remains, but effective evaluation is blocked. Existing
signal-loss reset behavior remains for issue #25.

**Rationale**: Clearing only the displayed values leaves the reader's change
detection holding the old sample. A fast restart with identical values could
then fail to republish. Resetting both layers guarantees recovery and prevents a
stale input decision.

## R8: Game Context uses a focusable help affordance

**Decision**: Rename the row to Game Context and attach one help string to both
the label/value hover responses and a keyboard-focusable information affordance.
The visible value remains present without interaction.

**Rationale**: Game Focus would imply a single operating-system boolean, while
the field also names game surfaces and signal availability. A focusable help
control makes the explanation available without a pointer.

## R9: No new third-party runtime dependency

**Decision**: Extend the existing `windows-sys` feature set for registry reads,
reuse `serde_json` for Epic manifests, reuse the VDF parser for Steam, and reuse
`x11rb` for Linux focus. Platform adapters remain thin and all reconciliation is
tested in platform-neutral code.

**Rationale**: Every required capability already has a dependency in the crate.
Keeping registry and process calls behind a small adapter avoids adding a broad
installer-discovery library for a handful of read-only values.

## R10: Platform failure is explicit

**Decision**: Windows registry, process, or foreground-query failure yields
Unknown evidence. Linux `/proc` races skip vanished entries but a failure to
enumerate the process table yields Unknown. X11 failure, including an unsupported
Wayland-only session, yields Unknown focus rather than Unfocused.

**Rationale**: Unfocused and Inactive are observations, not catch-all errors.
Explicit Unknown keeps failure visible and non-actionable.
