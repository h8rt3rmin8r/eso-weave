# Runtime Application Checklist

- [x] Changed Fishing configuration is applied atomically.
- [x] Changed requested or active Fishing work stops without sink output.
- [x] Unchanged Fishing configuration is a strict state no-op.
- [x] The next explicit enable uses only the new configuration generation.
- [x] Reader updates cannot block the GUI.
- [x] A reader update wakes an old worker wait and rapid edits coalesce latest-wins.
- [x] Reader decoding and poll cadence adopt the same update boundary.
- [x] Tolerance changes close stale safety evidence before resampling.
- [x] Block size and heartbeat timeout remain unchanged at runtime.
- [x] No reader-config lock is held during sampling, subsystem locking, or input operations.
