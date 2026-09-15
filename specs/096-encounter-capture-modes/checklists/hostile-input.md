# Hostile Input Checklist

**Purpose**: Preserve the SavedVariables and SQLite trust boundaries
**Created**: 2026-09-15

- [x] CHK001 The shared file is read once through the existing stable bounded parser.
- [x] CHK002 Wrapper, session, member, interruption, and failure structures reject unknown fields.
- [x] CHK003 Modes, states, dispositions, and reasons use closed enums.
- [x] CHK004 Identifiers have strict syntax and a small display-safe byte bound.
- [x] CHK005 Ordinals and marker sequences are positive, unique, contiguous, and checked.
- [x] CHK006 Session identity, channel, mode, counts, and nested records are congruent.
- [x] CHK007 Aggregate arithmetic is checked and recomputed from nested evidence.
- [x] CHK008 Every terminal record validates and replays before storage begins.
- [x] CHK009 One transaction owns the complete batch preflight and insertion.
- [x] CHK010 Identity, ordinal, revision, or canonical-content collisions reject atomically.
- [x] CHK011 Growing session snapshots preserve prior references and markers as prefixes.
- [x] CHK012 Legacy canonical bytes and hashes are never rewritten during migration.
- [x] CHK013 Errors and UI do not echo untrusted identifiers, raw values, or paths.
- [x] CHK014 Malformed active state cannot become terminal imported evidence.
