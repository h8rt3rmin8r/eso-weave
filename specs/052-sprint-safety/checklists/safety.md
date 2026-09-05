# Safety Checklist

- [x] No protected game functions, process memory, or network traffic are used.
- [x] No new input-hook work or polling thread is introduced.
- [x] Unsupported evidence cannot produce explicit sprint.
- [x] Stale positive evidence has a watchdog.
- [x] Auto-potion suppression does not queue or replay input.
- [x] Existing life, world, travel, focus, context, cooldown, and availability gates retain precedence.
- [x] Raw coordinates and physical input contents are not logged.
- [x] Live verification is separated from deterministic merge gates.
