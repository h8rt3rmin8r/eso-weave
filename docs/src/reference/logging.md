# Logging

**Application Log** shows recent ESO Weave diagnostic events for troubleshooting.
**File Logging** optionally saves those events in a monthly file. Neither
records fights or enables the ESO Weave Data addon. The word "live" in older
guides meant application events arriving now, not live encounter recording.

For fights, use [Encounter Capture](../features/encounter-capture.md) and import
ESO's saved addon data into **File > Encounter History**. ESO's own native
combat-log files are a separate provisional ingestion route, described in
[Encounter Ingestion](../development/encounter-ingestion.md).

ESO Weave uses structured logging with runtime selection of OFF, ERROR, WARN,
INFO, DEBUG, or TRACE.

- The optional file sink can be enabled or disabled while the application runs.
  It writes monthly `YYYY-MM.log` files beneath the platform data directory.
  Each line includes UTC time, level, target, and message.
- The in-memory ring buffer is always active and feeds the Application Log independently
  of file logging.
- Input contents are not logged above DEBUG, and keystrokes are never logged while
  ESO Weave is suspended.
- Frequently changing resource observations use TRACE so DEBUG remains useful for
  other diagnosis.

The global level filters subsequent events before they reach either sink. The
ring retains the newest 1000 captured events and evicts the oldest. A file write
failure does not remove an event already placed in the ring.

## Application Log

Open **View > Application Log**. In Settings, **Application Logging** controls
the captured level and optional file output. The level selector changes and persists the global
captured level, so it affects both the Application Log ring and optional file sink.

Use INFO for ordinary lifecycle diagnosis, DEBUG for a bounded reproduction, and
TRACE only when detailed observations are necessary. OFF prevents subsequent
events from entering either sink. The panel autoscrolls while it is at the bottom
and can be resized without covering controls.

## File paths

| Platform | Monthly file |
| --- | --- |
| Windows | `%APPDATA%\eso-weave\logs\YYYY-MM.log` |
| Linux | `$XDG_STATE_HOME/eso-weave/logs/YYYY-MM.log`, falling back to the platform configuration root when no XDG state directory is available |

The directory and month file are created lazily after file logging is enabled and
an eligible event arrives. File-sink failure is otherwise silent in the current
UI, so the Application Log remains the first diagnostic surface.

## Privacy when sharing logs

Normal logs describe categorical state changes rather than every evaluation.
Input contents are unavailable above DEBUG and are suppressed while ESO Weave is
suspended. Paths and operational context can still identify a local environment.
Review the bounded relevant section before sharing it and omit unrelated personal
paths or activity.

See [Troubleshooting](../getting-started/troubleshooting.md#use-the-application-log) for a
status-first diagnosis sequence.
