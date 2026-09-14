# Research: Data Addon Lifecycle UI

## R1. Evidence boundary

**Decision**: Treat filesystem installation, managed ownership, compatibility,
runtime availability, configured enablement, current-session load, reload need,
and each module's activity as independent facts with provenance.

**Rationale**: S092 intentionally selected no desktop-to-addon command or live
data bridge. ESO process presence proves only process presence. Installed files
prove neither enablement nor load.

## R2. SavedVariables evidence

**Decision**: Do not parse SavedVariables for S093 lifecycle presentation.
Enabled, loaded, catalog, and encounter facts remain unconfirmed. A future
version-compatible, bounded, stable reader may report historical evidence, but
it could never confirm current-session state.

**Rationale**: ESO flushes SavedVariables only on supported lifecycle
boundaries, so disk content can lag in-memory state. Parsing an optional payload
of up to 128 MiB during startup adds cost and hostile-input surface without
answering the live readiness question in issue #185.

## R3. Configured enablement

**Decision**: Display configured enablement as unconfirmed in S093.

**Rationale**: The repository has no supported, cross-platform,
current-account source whose semantics are reliable enough to claim enabled or
disabled. The UI explains how to verify it in ESO's Add-Ons menu.

## R4. Lifecycle orchestration

**Decision**: Install, Update, and Repair call `data_addon::install`; Uninstall
calls `data_addon::uninstall`. The caller retains reload-required outcomes and
refreshes status after completion.

**Rationale**: S092 already provides exact inventory, marker, link, race,
atomic replacement, rollback, and neighbor protection.

## R5. Refresh architecture

**Decision**: Keep one cached lightweight lifecycle observation shared by the
main interface and Catalog Update workflow. Refresh at startup and after
lifecycle completion, not during paint.

**Rationale**: This keeps filesystem work out of the renderer and leaves the
optional SavedVariables payload unread.

## R6. Reload retention

**Decision**: Retain a successful operation's reload requirement while ESO is
running or runtime is unknown. Clear it after defensible stopped evidence, never
on a timer.

## R7. UI composition

**Decision**: Place the data-addon lifecycle row immediately after PixelBeacon
and show subordinate evidence text for enablement, loading, runtime, reload,
catalog, and encounter. Keep PixelBeacon Signal independent.

## R8. Catalog Update ownership

**Decision**: Remove install and uninstall mutations from Catalog Update. Keep
capture wait, build, and module-local clear guidance there.
