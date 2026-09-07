# Content Coverage Checklist: Documentation Completeness

**Purpose**: Account for every user and developer topic required by issue #81 before S059 can close.

**Created**: 2026-09-07

**Feature**: [spec.md](../spec.md)

## Installation and First Launch

- [x] CHK001 Windows MSI download, checksum verification, install, first launch, shortcuts, upgrade, removal, and relevant data paths are documented.
- [x] CHK002 Linux `.deb`, AppImage, and tarball download, checksum verification, install, launch, update, and removal procedures are documented separately.
- [x] CHK003 Linux input-group and udev alternatives, sign-out requirements, `/dev/uinput`, X11, XWayland, and unsupported Wayland capture conditions are documented with diagnosis steps.
- [x] CHK004 First launch connects Game Installation, Game State, focus, AddOns discovery, PixelBeacon installation, ESO addon enablement, `/reloadui`, restart requirements, and a safe signal smoke test.
- [x] CHK005 Steam, ESO Store, Epic Games, and Steam Proton discovery are covered, including Not detected, Multiple installs detected, Unknown, Launcher open, Inactive, and Active outcomes.

## Interface, Status, and Accessibility

- [x] CHK006 File, View, Settings, Exit, Live Log, disclosure, dashboard, Skills, and lifecycle controls are described as user tasks rather than only layout geometry.
- [x] CHK007 Every Live HUD state is covered: Health, Stamina, Magicka, Ultimate, Game Context, Combat, Movement, Roll Dodge, Life State, Weapon Bar, and the composed Quickslot readout.
- [x] CHK008 Every System and State value is covered: Game and provider summary, Travel, World State, ESO Weave, PixelBeacon Status, PixelBeacon Signal, Fishing, and Auto Potion.
- [x] CHK009 Status documentation gives exact visible text, meaning, automation effect, and recovery for dormant, unavailable, unknown, warning, error, ready, and active families.
- [x] CHK010 Health, Stamina, and Magicka percentages, Low state, quarter landmarks, zero, unavailable, and inactive-game behavior are documented.
- [x] CHK011 Ultimate current and maximum, active-bar cost, threshold marker, Ready state, special-bar behavior, protocol compatibility, and display-only boundary are documented.
- [x] CHK012 Quickslot documentation distinguishes no signal, legacy addon, corrupt signal, unsupported API, invalid selection, inconsistent facts, empty, every non-potion kind, potion availability, and independent cooldown.
- [x] CHK013 Responsive card behavior, text truncation access, keyboard focus, System and State collapse behavior, and persistent layout choices remain discoverable without dominating the task guide.

## Weaving and Input Safety

- [x] CHK014 Skills 1 through 5, Ultimate, and Synergy defaults, enablement, weave selector, delay override, effective delay, and cooldown display are documented.
- [x] CHK015 Light Attack, Heavy Attack, Bash Attack, and Block Casting sequences have clear ordered examples.
- [x] CHK016 Front and back timing profiles, unknown-bar fallback, weapon-class presets, Auto Timing from Weapon, and bar swaps have a worked example.
- [x] CHK017 Global cooldown, dropped requests, per-slot overrides, latency factor, additive latency adjustment, cap, unavailable telemetry fallback, and unaffected delay fields are documented accurately.
- [x] CHK018 The input decision order covers game activity, focus, injected origin, binding, life, world, travel, roll dodge, game context, suspension, queue handoff, mid-sequence closure, held-button release, and no replay.
- [x] CHK019 All ten keybindings, their defaults, rebinding flow, conflict behavior, suspend-exempt toggles, and focused-window scope are documented.

## Fishing

- [x] CHK020 Fishing prerequisites cover bait, current PixelBeacon, overlay visibility, focus, fishing-hole alignment, and the rule against casting manually before start.
- [x] CHK021 Casting, Fishing waiting for a bite, Reeling in, Recasting, Idle, no cast detected, signal lost, game not active, game unfocused, player unavailable, world unavailable, and travel pending are documented.
- [x] CHK022 The event-driven cast, confirmation, bite, reel, recast, timeout, stop, focus-loss, lifecycle-loss, and recovery transitions are complete and do not imply replay.
- [x] CHK023 Arm Timeout, Reel Delay, Recast Delay, and the configured interact key have defaults, bounds, effect timing, and truthful UI or config availability.
- [x] CHK024 Menu deferral, operator-initiated first cast behavior, bait-consumption bite authority, and rejected alternative signals are explained.

## Auto Potion

- [x] CHK025 Per-resource enablement, independent thresholds, OR behavior, default 35 percent threshold, valid 0 through 100 range, Quickslot Key, retry interval, and non-persisted request are documented.
- [x] CHK026 The ordered trigger contract includes request, activity, focus, heartbeat, suspension, Game Context, life, world, travel, sprint, resource watches and freshness, potion classification and availability, cooldown, retry, and low-resource comparison.
- [x] CHK027 Off, both Dormant reasons, every Blocked reason, Ready, and all Triggered resource variants map to exact visible text and recovery advice.
- [x] CHK028 Unknown resource, quickslot, cooldown, life, world, and travel evidence is explicitly fail-closed, while Unknown movement does not invent Sprinting.
- [x] CHK029 A worked example demonstrates the at-or-below comparison, usable potion, ready cooldown, retry floor, one key press and release, and current-reading evaluation after recovery.
- [x] CHK030 The manual states that Auto Potion does not choose a potion, switch quickslots, or infer which resources the selected potion restores.

## PixelBeacon and Pixel Bus

- [x] CHK031 AddOns discovery, Live and PTS environments, manual override, existing-directory requirement, embedded install, update, managed removal, and reload or relog behavior are documented.
- [x] CHK032 Not Installed, Unmanaged, Installed current, Installed outdated, AddOns folder not found, signal never observed, signal detected, and signal lost are distinguished from one another.
- [x] CHK033 The managed-marker write and removal boundary is described without promising behavior that the current implementation does not guarantee.
- [x] CHK034 Overlay origin, physical block size, footprint, obstruction risk, non-movable geometry, block-size redeploy, `/reloadui`, and application restart sequence are documented.
- [x] CHK035 API-version baseline, startup network check, monotonic update behavior, offline fallback, and non-blocking failure behavior are documented.
- [x] CHK036 `/pbquickslot`, watch mode, bounded output, stop command, and omitted localized data are documented as diagnostics rather than normal setup.
- [x] CHK037 Protocol geometry, header authority, version negotiation, legacy bounds, payload registry, marker and checksum validation, tolerance, freshness, corruption, signal loss, and display reconciliation remain complete.

## Settings, Persistence, Logging, and Troubleshooting

- [x] CHK038 Theme and Always on Top include choices, defaults, persistence, and live effect.
- [x] CHK039 Combat Timing includes front Global Cooldown, Light Attack Delay, Heavy Attack Delay, Bash Delay, Auto Timing, back-bar delays, Adapt to Latency, and Latency Factor.
- [x] CHK040 Fishing settings include every visible timing control and clearly identify any stored option that is not exposed by the modal.
- [x] CHK041 PixelBeacon and Bus settings include AddOns override, environment, block size choices, tolerance, fast interval, idle interval, defaults, bounds, and effect timing.
- [x] CHK042 Auto Potion settings include all three watch toggles and thresholds, Quickslot Key, Minimum Retry Interval, defaults, bounds, and interactions.
- [x] CHK043 Logging settings include OFF, ERROR, WARN, INFO, DEBUG, TRACE, Write Log to File, Live Log behavior, and the true relationship between its dropdown and the persisted capture level.
- [x] CHK044 Main-window skill controls, System and State disclosure, Live Log height, window geometry, suspend intent, Fishing intent, and Auto Potion request are included in the persisted-versus-session matrix.
- [x] CHK045 Configuration paths, data paths, log paths, schema migration, unknown fields, invalid values, `.invalid` preservation, safe fallback, coalesced writes, and final geometry flush are documented.
- [x] CHK046 Common-failure guidance covers startup failure, inactive interception, Linux permission, provider ambiguity, AddOns discovery, stale addon, missing or obstructed signal, loading, focus loss, config corruption, fishing stops, Auto Potion blockers, and absent cooldown telemetry.
- [x] CHK047 Troubleshooting starts with visible status and bounded checks, then uses logs and advanced protocol diagnostics, and never recommends unsafe deletion or broad permission changes.
- [x] CHK048 Privacy and network guidance states what is read, written, captured, logged, and queried; what never leaves the machine; and the exact best-effort network exception.

## Developer Contracts

- [x] CHK049 Subsystem ownership and the Main, Interception, Weave worker, Pixel bus worker, and API version check threads have an explicit data-flow and ownership model.
- [x] CHK050 Physical versus synthesized input, recursion tagging, focus scope, callback limits, bounded queues, worker timing, gate rechecks, cancellation, cleanup, and no-replay contracts are documented.
- [x] CHK051 Installation, runtime, Game Context, life, world, travel, roll dodge, and movement state machines each identify inputs, transitions, Unknown behavior, action authorization, reset, and recovery.
- [x] CHK052 Fishing and Auto Potion developer flows document controller ownership, events or tick inputs, timers, ordered decisions, side effects, and every fail-closed state.
- [x] CHK053 Pixel bus developer guidance distinguishes stable wire guarantees, backward compatibility, geometry negotiation, atomic groups, freshness, corruption handling, and version-sensitive addon details.
- [x] CHK054 Config and session migration, save scheduling, logging fan-out, deterministic seams, test layers, fixtures, platform coverage, and safety-contract tests are documented.
- [x] CHK055 Packaging and release guidance documents stable artifact production, version authority, checksums, CI gates, and tag-to-release flow without publishing maintainer-only ritual or credentials.
- [x] CHK056 Heavy-attack estimates, ESO API observations, operating-system details, and other change-prone facts are labeled version-sensitive and linked to evidence.

## Navigation and Traceability

- [x] CHK057 Every coverage item has exactly one canonical page and useful cross-links from adjacent task, concept, or reference pages.
- [x] CHK058 Reader-facing indexes use likely search terms such as addon, overlay, hotkey, potion, quickslot, signal, cooldown, focus, permission, log, update, and uninstall.
- [x] CHK059 The feature-to-page and logic-to-page matrices are complete, evidence-backed, and retained as reviewable S059 acceptance evidence.
- [x] CHK060 Any unresolved source conflict names a follow-up issue and keeps the affected documentation explicitly truthful.
