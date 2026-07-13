# Build Plan 005: API Version Automation, UI Fixes, and Specification Rewrite

Plan: 005
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plans 001 through 004 delivered the functional product, brand and UX
polish, a GUI overhaul, weapon-bar-aware timing, and fishing reliability
(through v0.4.3). This plan advances the product on four fronts: it automates the
one remaining manual reliability chore, corrects two UI defects, brings the
architecture of record up to date, and closes a correctness gap in the fishing
documentation.

It traces to the master specification's PixelBeacon companion addon (section 9,
open item R4), the graphical user interface (section 10), and the fishing module
(section 8). Slice 018 touches a safety-critical surface: it writes to the
on-disk addon manifest, and every such write is gated by the managed-marker line,
extending the guarantee that already governs uninstall. Adding an outbound
HTTPS version check introduces a networked dependency, recorded as a dated
decision in `CHANGELOG.md`; it stays within the bounded scope of the constitution
(no game-memory access, no network interception, nothing inside the game).
Slice 019 is a self-contained GUI correctness fix. Slice 020 supersedes the
master specification with a v0.2.0 rewrite and corrects the fishing README.

## Slices

### Slice 018: ESO API Version Check Automation

Scope: automate detection and upkeep of the ESO client API version so the
PixelBeacon manifest never falls behind and the game never flags the addon Out of
Date. Introduce a global default API version constant so a manifest can always be
rendered without any network access. Add a version-source seam that issues a web
request to a stable public source and parses the current live API version, with a
real HTTP implementation behind the seam and a mock for tests; the source is
selected during planning and recorded in the feature `plan.md`. Persist the last
known API version in `state.json` (derived runtime state, never in `config.json`).
On startup, off the GUI thread, resolve the install state, perform the check, and
persist the discovered version; if the addon is installed and its on-disk manifest
carries the managed marker, rewrite only the `## APIVersion:` line in place when
it differs; if the addon is not installed, cache the version for a later install.
Rendering resolves in the order fresh fetch, then stored value, then default, and
never blocks or panics. The install path renders the manifest with the best known
version. Every manifest write is gated by the managed-marker check, so the app
never writes a manifest it does not own. All parsing, rendering, rewriting, and
resolution logic lands in tested pure helpers; the safety-critical marker gate is
tested. Feature under `specs/018-<name>/`.

### Slice 019: GUI Hover Reflow and Settings Modal Fixes

Scope: fix two GUI defects. First, buttons grow on hover and reflow the whole
window; the cause is a per-state stroke-width difference in the egui theme
feeding the widget inner margin. Equalize the size-affecting inputs across all
widget states and set the interaction expansion explicitly to zero so hover
changes color but never size. Second, the settings modal fills only part of its
width and its scrollbar floats away from the edge; the modal scroll area inherits
horizontal auto-shrink. Disable auto-shrink so the scroll area fills the modal
width and the scrollbar sits at the far right edge, matching the log panel that
already renders correctly. The correctness surface is visual; verification is
observational against the running app, and the egui layer stays thin. Feature
under `specs/019-<name>/`.

### Slice 020: Specification Rewrite and Fishing Documentation Fix

Scope: supersede the master specification with a full rewrite at v0.2.0 that
documents the system as built, in an authoritative and declarative voice with
expanded mermaid diagrams (system architecture, thread and ownership model, input
interception flow, weave sequence, fishing state machine, pixel-bus block
protocol, beacon install and verify and uninstall lifecycle, API version check
flow, GUI layout, and the config and state persistence model). The rewrite lands
as `docs/ESO-Weave-Specification-v0.2.0.md`; every repository reference is
repointed from v0.1.0 to v0.2.0, the superseded file is removed, and the standing
autopilot authorization is re-affirmed against the new document, recorded as a
dated decision. Separately, correct the fishing README: add fishing bait as a
prerequisite and as an explicit step in the usage sequence, because without bait
selected the cast fails and fishing never starts, and add a matching
troubleshooting check. Documentation only; no source changes. Feature under
`specs/020-<name>/`.
