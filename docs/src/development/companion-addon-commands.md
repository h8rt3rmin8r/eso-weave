# Companion-to-Addon Configuration

PixelBeacon publishes the existing screen signals and collects Catalog
definitions automatically. The supported real-time desktop-to-addon command
vocabulary remains empty. The desktop does not type a collection command,
synthesize a chat or gameplay binding, modify live SavedVariables, or force an
addon reload.

## Persisted startup preference

The package-owned StartupConfig.lua representation contains only the Catalog
disable boolean and the actual installation context's Live/PTS channel. The
package manager renders and validates that bounded representation. It does not
accept arbitrary Lua supplied through application settings.

Desktop Catalog disable cancels desktop scheduling and blocks stale activation
immediately. Already-loaded Lua applies the persisted value at the next natural
addon load or game session. The application must keep these two facts distinct;
writing configuration does not acknowledge a live Lua change. A stale result
cannot re-enable disabled desktop maintenance. Re-enable reconciles retained
progress automatically.

Catalog and native log processing have independent preferences. Neither changes
weaving, fishing, auto-potion, beacon signals or game-native logging. Combined
package availability supplies shared eligibility, evaluated for each actual
installation context. Preference values survive temporary absence.

## Public game boundary

The documented ESO addon API supplies no supported inbound desktop socket,
file-watch, clipboard or URL command transport. Generated custom bindings and
slash-command typing provide no supported delivery acknowledgement and can
interfere with contextual input. Such mechanisms are not used for maintenance.
The ordinary native binding-discovery feature keeps its existing input contract.

The automatic collector uses public read APIs and supported game-managed save
opportunities. A priority-save request changes queue priority; it does not prove
that bytes reached disk. See [Automatic Catalog Collection](discovery-collector.md)
for bounded scanning, completed generations and restart behavior.

## Historical recording compatibility

The combined package does not load the old Encounter.lua recorder or register
its /ewencounter control workflow. It also does not register /ewcollect.
Previously saved Catalog and encounter evidence remain in the legacy
SavedVariables/EsoWeaveData.lua file with its EsoWeaveDataSaved root. Current
PixelBeacon.txt reads and saves that global table in SavedVariables/PixelBeacon.lua;
the global name cannot override the manifest-derived filename. New Lua does not
load the legacy file. The desktop reconciles both Catalog sources into its own
accumulation and keeps legacy encounter import/history access. Neither source
file is copied or rewritten, and source fixtures preserve old recorder formats.

The retained /pbquickslot diagnostic belongs to the existing beacon signal
feature. It does not start maintenance or supply a desktop-to-addon transport.
