# Automatic Native Processing

ESO owns native encounter logging. When the game writes a supported log, ESO
Weave discovers and processes it automatically in the background. There is no
desktop encounter viewer, source picker, import step or recording control.

The single managed PixelBeacon package supplies screen signals and automatic
Catalog definitions. It does not load or resume the former addon encounter
recorder. Native processing and Catalog maintenance are independent services;
neither requires the other to acquire data.

## Automatic native processing

Normal and verbose log version 15, with separate or inline ability information,
are supported. The desktop discovers the actual
Live or PTS environment, catches up completed fights, and follows appended
records. A completed combat can be stored before the logging run ends. An
unfinished line or fight waits for more evidence; silence does not end a fight.
The desktop does not change ESO logging, anonymity, serialization or source files.
An old file does not establish that logging is currently active.

Format changes within a run use the same automatic processing and preserve
original records. The desktop does not require a particular serialization setting.
Unsupported formats preserve existing history and source bytes. They do not
create a player repair task or a requirement to change game serialization.

## Independent disable preferences

In Settings, **Disable Catalog maintenance** and **Disable native encounter log
processing** persist independently. Both default to automatic operation. Both
controls are gray/inactive without a compatible combined addon, and both services
are dormant. Preferences survive restart, temporary absence and reinstall.

Disabling native processing stops new work through the application cancellation
boundary. Existing data remains readable. Catalog disable stops desktop work
immediately; its Lua startup preference applies at the next natural addon load.
No game reload is forced and neither option toggles ESO's own logging feature.

## Retained data and local access

Raw addon and native history remains in the user-owned encounter database.
Automatic processing preserves ordered original evidence, canonical hashes,
source identities and explicit missing-data qualifications. Retry and restart
reuse committed progress rather than adding duplicate fights. Removing the
desktop viewer does not delete recordings or introduce automatic retention.

Supported results are available through the optional authenticated read-only
[Local API and MCP](../reference/local-api-and-mcp.md), including when collection
is disabled or the addon is absent. Native facts do not require a Catalog;
unsupported calculations remain unavailable. See [Encounter Data and
Metrics](../reference/encounter-data-and-metrics.md).

## Historical addon recorder compatibility

Older releases used a separate ESO Weave Data recorder. Historical SavedVariables
and database rows remain readable through retained domain readers and contributor
utilities. The current package does not load that recorder, register its commands,
or start recording from saved authority. These compatibility readers are not a
current player setup or import workflow.

Legacy `SavedVariables/EsoWeaveData.lua` and current `SavedVariables/PixelBeacon.lua`
are distinct files. The compatible global `EsoWeaveDataSaved` does not determine
the filename; ESO uses the addon manifest. The desktop never deletes, moves,
copies or rewrites either file to consolidate their data. Catalog accumulation
reads compatible evidence into application-owned storage.

Historical capture schemas preserve complete, partial and interrupted states,
declared loss, original ordering and source versions. Missing observations never
become fabricated zero values. Retained reader details are in
[Encounter Ingestion](../development/encounter-ingestion.md) and the historical
specifications identified by [Encounter Data and Metrics](../reference/encounter-data-and-metrics.md#implementation-history-and-detailed-contracts).
