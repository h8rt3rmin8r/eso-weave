# Recovery and Loss Checklist

**Purpose**: Ensure continuous capture never hides a gap or pressure failure
**Created**: 2026-09-15

- [x] CHK001 Mid-combat activation records exact current authority and an unknown prefix.
- [x] CHK002 No unknown prefix is converted into a fabricated sequence range or count.
- [x] CHK003 Reload and relog recovery use only last durably flushed authority.
- [x] CHK004 Active single interruption finalizes partial and stops.
- [x] CHK005 Active continuous interruption finalizes partial and may resume waiting.
- [x] CHK006 Waiting interruptions remain visible without inventing an encounter.
- [x] CHK007 Desktop exit has no effect on addon authority.
- [x] CHK008 Pre-flush game or operating-system crash loss is documented as unknowable.
- [x] CHK009 Hard-failed sessions never auto-retry on load.
- [x] CHK010 Aggregate bytes, events, raw observations, encounters, and markers are bounded.
- [x] CHK011 Current and outer terminal reserves cannot be consumed by regular data.
- [x] CHK012 No record or marker is silently evicted, overwritten, or rolled.
- [x] CHK013 Only terminal members are imported as encounter evidence.
- [x] CHK014 Partial encounter evidence remains distinguishable from session failure.
