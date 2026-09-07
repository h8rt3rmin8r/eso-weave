# Logging

File Logging means the monthly log file used for debug or trace diagnosis.

ESO Weave uses structured logging with runtime selection of OFF, ERROR, WARN,
INFO, DEBUG, or TRACE.

- The optional file sink can be enabled or disabled while the application runs.
  It writes monthly `YYYY-MM.log` files beneath the platform data directory.
  Each line includes UTC time, level, target, and message.
- The in-memory ring buffer is always active and feeds the Live Log independently
  of file logging.
- Input contents are not logged above DEBUG, and keystrokes are never logged while
  ESO Weave is suspended.
- Frequently changing resource observations use TRACE so DEBUG remains useful for
  other diagnosis.

The global level filters subsequent events before they reach either sink. The
ring retains the newest 1000 captured events and evicts the oldest. A file write
failure does not remove an event already placed in the ring.

## Live Log

Open **View > Live Log**. The level selector currently changes and persists the
global captured level, so it affects both the Live Log ring and optional file
sink. This corrects older display-only documentation; the remaining in-app and
source wording is tracked in
[issue #96](https://github.com/h8rt3rmin8r/eso-weave/issues/96).

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
UI, so the Live Log remains the first diagnostic surface.

## Privacy when sharing logs

Normal logs describe categorical state changes rather than every evaluation.
Input contents are unavailable above DEBUG and are suppressed while ESO Weave is
suspended. Paths and operational context can still identify a local environment.
Review the bounded relevant section before sharing it and omit unrelated personal
paths or activity.

See [Troubleshooting](../getting-started/troubleshooting.md#use-the-live-log) for a
status-first diagnosis sequence.
