# Responsible Use

The ESO Weave desktop runs beside the game, never inside its process. It does not
read or write game process memory and does not inspect or alter network traffic.
Input interception and synthesis are scoped to the focused ESO window.

This project is published for educational purposes only. It exists as a study
in cross-platform input handling, screen-signal protocols, and game-adjacent
tooling architecture. It is not affiliated with, endorsed by, or supported by
ZeniMax Online Studios, ZeniMax Media Inc., Bethesda Softworks, or Microsoft.
The Elder Scrolls® and The Elder Scrolls Online are trademarks or registered
trademarks of ZeniMax Media Inc.

Automating gameplay input may violate the Terms of Service of The Elder
Scrolls Online. Using this software with a live game account is done entirely
at your own risk. You are solely responsible for reviewing and complying with
all agreements that govern your account, and you accept all consequences of
your use of this software, up to and including permanent account suspension.

The author assumes no liability for any account action, data loss, or other
damages arising from the use or misuse of this software. This software is
provided "AS IS", without warranty of any kind, express or implied, in
accordance with the Apache License, Version 2.0 under which it is distributed.

## Supported boundary

ESO Weave supports Windows 10 and 11 x64 and Linux x64. macOS, multi-account or
multi-client orchestration, game memory access, and packet manipulation are
outside project scope.

PixelBeacon ships only inside ESO Weave. It is not published to addon indexes.
The single managed PixelBeacon package publishes the existing color-block signals
and collects bounded Catalog definitions automatically through documented public
addon APIs outside combat. Eligible collection starts and resumes without a chat
command. The desktop verifies compatible saved evidence before activating an
accumulated Catalog generation. Collection and maintenance stay local and do not
drive gameplay or use Pixel Bus for bulk records.

Settings provides independent **Disable Catalog maintenance** and **Disable native
encounter log processing** preferences. Data services are dormant when a compatible
combined addon is absent. Disabling Catalog maintenance stops desktop work at once;
the staged addon preference applies at the next natural addon load. The desktop
has no live SavedVariables, clipboard, generated-input, or Pixel Bus command path.

The combined package does not load or resume the former ESO Weave Data encounter
recorder. Its retained SavedVariables recordings remain importable, and completed
supported native encounter-log files have a separate inspection and import path.
**Last saved source** inspects the selected environment's retained addon file
without importing or writing it. Changing the selected folder or environment
discards older pending results. A disk file can lag current activity and does not
prove addon enablement, loading, or recording. Historical recorder procedures are
identified in [Automatic Native Processing](../features/encounter-capture.md).

## Privacy and network behavior

ESO Weave reads local configuration, process and window state, supported keyboard
events, and displayed pixels at the ESO client origin. PixelBeacon reads supported
ESO addon API values and renders them as local color blocks. The project does not
read game process memory, inspect packet contents, send gameplay telemetry, or
require an online account service.

The automatic Catalog collector records versioned, bounded observations in the
user's local SavedVariables unless Catalog maintenance is disabled. The desktop treats that file as hostile data, never
executes it, never uploads it, and keeps user-collected localized text local.

Historical encounter recordings contain bounded sessions of one or more fights
in the game's saved addon data (SavedVariables). Every scalar value delivered by
a selected recorder callback was retained exactly unless a hard bound or
unsupported runtime value caused declared whole-observation loss. The current
desktop importer treats the retained shared file as hostile data,
verifies the outer and encounter schemas plus loss declarations, and keeps
accepted raw history in a user-owned store separate from the shipped catalog.
It never uploads the capture, and logs, receipts, public fixtures, and default UI
summaries do not reproduce raw payload values.

The desktop performs a best-effort background startup check against the official
`esoui/esoui` client/API sources for the selected Live or PTS environment. It
compares the observed API with the combined addon's reviewed support declaration;
it does not claim compatibility by stamping a newer number into installed files.
Failure never blocks startup, current support stays unknown, and remembered
unsupported-version guidance remains visible. Package download
and GitHub documentation access naturally use their respective services.

Logs and settings stay in the platform paths described in
[Configuration](../reference/configuration.md) and [Logging](../reference/logging.md).
Review a log before sharing it, because paths and operational context may still
identify the local environment even though input contents are suppressed under
the documented rules.

The optional [Local API and MCP service](../reference/local-api-and-mcp.md) is
off by default, loopback-only, bearer-authenticated, and read-only. A connected
trusted local client can still read current player state and application-owned
database data. ESO Weave does not upload that data, host an agent, or authorize
gameplay input through the service. Protect the copied credential and leave the
service disabled when no trusted local client needs it.
