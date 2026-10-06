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
multi-client orchestration, game memory access, packet manipulation, and in-game
features beyond the three named local addon surfaces are outside project scope.

PixelBeacon ships only inside ESO Weave. It is not published to addon indexes.
The separately managed ESO Weave Data addon contains catalog and encounter
modules with isolated state and activation rules. Catalog capture runs only
after an explicit user command and reads documented public addon APIs outside
combat. Encounter capture remains dormant until the user explicitly enables
`single` or `continuous` inside ESO for a selected Live or PTS channel. It warns
that exact local values can include names and identifiers, records bounded
observations from deliberately selected sources, and declares capture loss,
interruptions, and hard failure. Single stops after one combat period;
continuous keeps one bounded session across combat gaps until the user disables
it or a reported hard failure stops it. Neither module uploads data, drives
gameplay, or uses Pixel Bus for bulk records.

The desktop cannot enable, disable, or change encounter mode. It has no custom
binding, generated-input, clipboard, Pixel Bus, or live SavedVariables command
path. **Last saved recording state** comes from the last successfully imported
saved file. Refresh and failed imports keep that older summary; it can predate
the latest disk save or selected environment. Check **Saved channel** in the
summary. ESO's disk file can also lag its current state. Use `/ewencounter status`
inside ESO for the current requested and effective capture state.

## Privacy and network behavior

ESO Weave reads local configuration, process and window state, supported keyboard
events, and displayed pixels at the ESO client origin. PixelBeacon reads supported
ESO addon API values and renders them as local color blocks. The project does not
read game process memory, inspect packet contents, send gameplay telemetry, or
require an online account service.

The optional collector records versioned, bounded catalog observations in the
user's local SavedVariables. The desktop treats that file as hostile data, never
executes it, never uploads it, and keeps user-collected localized text local.

The optional encounter module records a bounded local session of one or more
fights in the game's saved addon data (SavedVariables).
Every scalar value delivered by a selected callback is retained exactly unless
a hard bound or unsupported runtime value causes declared whole-observation
loss. The current desktop importer treats the shared file as hostile data,
verifies the outer and encounter schemas plus loss declarations, and keeps
accepted raw history in a user-owned store separate from the shipped catalog.
It never uploads the capture, and logs, receipts, public fixtures, and default UI
summaries do not reproduce raw payload values.

The desktop performs a best-effort background startup check against the official
`esoui/esoui` client/API sources for the selected Live or PTS environment. It
compares the observed API with each bundled addon's reviewed support declaration;
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
