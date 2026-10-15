# Automatic Catalog Collection

PixelBeacon combines the existing screen-signal publisher with a bounded local
Catalog collector. After the player and game APIs initialize, enabled Catalog
maintenance collects definition evidence during ordinary play. It works whether
native encounter logging is off or on, and whether native processing is disabled.

The collector starts and resumes automatically. Normal load, progress,
completion, deferral and recovery produce no chat announcements, dialogs or
player commands. The application exposes the independently persisted **Disable
Catalog maintenance** preference. Desktop consumption stops immediately when
disabled. The already-loaded Lua collector receives the package-owned preference
at its next natural addon load. There is no immediate desktop-to-Lua command
channel and no requested or forced reload.

## Identity and source coverage

The installed package contains exactly five files: PixelBeacon.txt,
StartupConfig.lua, EsoWeaveData.lua, PixelBeacon.lua and Catalog.lua. The generated
startup configuration supplies the Catalog optout and Live/PTS identity from the
actual installation context. Game version, API version, locale, platform,
megaserver and character scope come from public game APIs. Settings selection
does not relabel a capture. Live and PTS evidence cannot be mixed.

The collector now supplies twelve source groups: player skills, crafted
abilities, ability metadata, effects/status, items/gear, item sets, champion
skills, consumables, Mundus effects, companions/races/classes, combat statistics
and constants. Names/descriptions and virtual icon references remain attached to
actual source IDs. Skill progressions, known ranks and base/morph variants have
separate typed identities and relationships. Shared script and classification
entities retain every supported parent relationship.

These views are bounded or opportunistic, with unknown coverage when a public
API view is unavailable. Current inventory, player buffs, active Mundus, current
race and active companion do not imply global game coverage. The S129 source audit records the exact API paths and remaining gaps. Localized text stays local
user-generated evidence; virtual icon paths do not supply binary image assets.

## Work, interruption and completion

Collection keeps the previous completed Catalog snapshot intact while the next
generation runs. In-progress records, adapter cursors, duplicate/relationship
tracking, merge-sort buffers and serialization state exist only in memory.
Saved maintenance metadata reports an attempt, phase and checkpoint; it is not a
claim that an incomplete payload can be imported.

Combat and player deactivation defer work. A supported reactivation resumes the
same in-memory generation when its identity is unchanged. Character, locale or
version changes restart the bounded scan with the new identity. A natural addon
reload starts a new bounded scan automatically, preserving the last completed
snapshot loaded from the current manifest's file throughout. Relevant source changes coalesce; repeated known-item
stack/durability and known-effect duration/stack events do not continuously
rescan unchanged definitions.

Each update issues at most 64 bounded steps and stops issuing work after its
4 ms budget or 4,096 serialized-byte budget. Final sorting is an incremental
merge sort. JSON escaping and Adler-32 checksums advance across update ticks;
individual concatenations are bounded by the 64 KiB record/chunk limit. The
collector never performs a whole-generation sort, serialization or checksum in
one callback. It also limits records to 500,000, source/payload bytes to 64 MiB,
strings/chunks to 64 KiB, chunks to 1,024, distinct parents per entity to 4,096,
and saved completed generations to one. A bounded in-memory generation and
identity index are additional temporary state.

Only a complete generation replaces the completed snapshot, in one table
assignment. Identical payload and provenance/coverage semantics preserve its
identity. Scan, encoding or resource failure preserves prior completed data and
retries after 30 seconds. Unsupported saved roots or Catalog schema versions are
preserved without rewriting them; bounded internal diagnostics describe their
unavailable state. These details are available to application diagnostics, not
new player-facing maintenance controls.

## Game-managed save and desktop handoff

The PixelBeacon.txt manifest retains the global table EsoWeaveDataSaved and its
root schema. ESO names the current disk file SavedVariables/PixelBeacon.lua after
the manifest, independently of that global name. This follows the
[addon developer's filename explanation](https://www.esoui.com/forums/showthread.php?page=2&t=8411)
and [SavedVariables guide](https://esoui.com/forums/showthread.php?p=49082).

The legacy SavedVariables/EsoWeaveData.lua stays on disk and remains readable by
the desktop Catalog and encounter importers. The new addon does not load that
other manifest's file or its Encounter.lua recorder. When no current save exists,
Lua starts a new bounded generation. Desktop accumulation separately reconciles
compatible old and current facts and keeps previously verified data. It never
copies, rewrites, or merges the game's SavedVariables files.

A changed completed generation can request priority through the documented
AddOnManager:RequestAddOnSavedVariablesPrioritySave API. Requests are coalesced
and attempted at most once per 30 seconds. A request changes save-queue priority;
it does not acknowledge a disk write or promise a flush latency. ESO controls
when SavedVariables become available on disk. Natural save opportunities feed
the [background Catalog service](catalog-updates.md), which detects
stable completed content, assembles compatible evidence, verifies an immutable
SQLite generation and activates it automatically.

## Import contract and retained data

The importer never executes Lua. It projects the Catalog subtree from the shared
SavedVariables grammar, requires a complete envelope, checks a recognized
collector checksum, validates contiguous JSON-line chunks and Adler-32 metadata,
checks typed source IDs and every parent, and validates the normalized Catalog
model. The shared-file parser additionally bounds input to 128 MiB, strings,
nesting, tokens and table entries. Invalid, partial, oversized, aliased or
unsupported source leaves existing staged and active data intact.

Recognized legacy version-1 captures remain readable. The legacy single-parent
field remains accepted; new captures include the complete distinct parents
array. No player collection, build, import, review, retry or clear action is
required. Uninstalling the managed package preserves SavedVariables, immutable
Catalogs, source evidence and retained encounter history.

Repository Lua doubles and stable-file tests verify these contracts. They do not
prove exhaustive ESO visibility, installed game behavior or game save timing.
