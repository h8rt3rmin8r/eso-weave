# Logging

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

The Live Log has its own display filter. Changing that filter does not change the
persisted global logging level.
