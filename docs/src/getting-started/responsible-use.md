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
features beyond the three named local addon bridges are outside project scope.

PixelBeacon ships only inside ESO Weave. It is not published to addon indexes.
The separately managed ESO Weave Collector runs only after an explicit user
command, reads documented public addon API values outside combat, and writes a
bounded local SavedVariables capture. It does not drive gameplay or communicate
over the network.

ESO Weave Encounter is a distinct third addon. It remains dormant until the user
explicitly arms one Live or PTS encounter, records bounded numeric observations
with encounter-local anonymous actors, declares capture loss, then disarms. It
stores no account, character, unit, ability, or effect names and does not use
Pixel Bus, upload data, or drive gameplay.

## Privacy and network behavior

ESO Weave reads local configuration, process and window state, supported keyboard
events, and displayed pixels at the ESO client origin. PixelBeacon reads supported
ESO addon API values and renders them as local color blocks. The project does not
read game process memory, inspect packet contents, send gameplay telemetry, or
require an online account service.

The optional collector records versioned, bounded catalog observations in the
user's local SavedVariables. The desktop treats that file as hostile data, never
executes it, never uploads it, and keeps user-collected localized text local.

The optional encounter addon records one bounded local SavedVariables envelope.
S075 does not import or upload it. A later importer must treat the file as
hostile data, verify the fixed schema and loss declarations, and keep it in a
user-owned store separate from the shipped catalog.

The desktop application performs one best-effort background startup check against
the official `esoui/esoui` live client-version source. That request is used only
as an addon API-version bump signal. Failure never blocks startup, and the
application retains a compiled and last-known local fallback. Package download
and GitHub documentation access naturally use their respective services.

Logs and settings stay in the platform paths described in
[Configuration](../reference/configuration.md) and [Logging](../reference/logging.md).
Review a log before sharing it, because paths and operational context may still
identify the local environment even though input contents are suppressed under
the documented rules.
