# Encounter Ingestion Transport Decision

## Status

Provisional pending the separate Windows and Linux/Proton operator matrix.

## Ordered candidates

1. Incremental native `Encounter.log` tailing, if completeness and latency pass.
2. Terminal native-log import after logging is disabled, if completeness passes
   but incremental cadence does not.
3. Current terminal SavedVariables import when native coverage or lifecycle is
   unsuitable.
4. A bounded hybrid only for callback-only facts, with those facts unavailable
   live unless a separately approved egress exists.

## Qualification matrix

Each claimed platform runs at least three solo or target-dummy, grouped-dungeon,
and raid or trial encounters when locally available. Runs cover anonymity on and
off, normal and supported verbose or inline formats, clean disable, reload,
relog, clean exit, controlled interruption, missing files, pre-existing files,
append, truncation, replacement, and large-file behavior.

Each receipt records:

- operating system and ESO or Proton environment;
- game, API, channel, log, addon, and application versions;
- format and anonymity settings;
- start and finish timestamps plus sanitized evidence hashes;
- record counts, unknowns, malformed lines, truncations, and replacements;
- p50, p95, maximum, and post-`END_COMBAT` availability latency;
- CPU, memory, bytes per minute, and maximum parser backlog;
- callback, native, and SavedVariables coverage comparison;
- interruption and recovery outcome.

## Truth rules

- Read only newline-terminated native records.
- File offsets and identities establish source order; timestamps do not invent
  missing source sequence.
- Replacement or truncation creates a discontinuity.
- Unknown or malformed records remain explicit.
- Game omission and ESO Weave filtering remain distinguishable.
- Missing platform evidence leaves the candidate provisional.
- Bulk data never uses PixelBus.
