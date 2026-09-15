# Contract: Atomic Session Import

## Pipeline

1. Resolve and read the shared SavedVariables file through the existing bounded,
   stable, no-follow boundary.
2. Parse the root once and select only the encounter module.
3. Dispatch legacy singleton or S096 controller by explicit version keys.
4. Validate controller invariants and every terminal member, including replay.
5. Canonicalize members and the small session snapshot without raw-value diagnostics.
6. Open or migrate the store, run full collision/prefix preflight, and write all
   new rows in one immediate transaction.
7. Return imported and already-present counts. Never modify the source file.

## Collision rules

- Same encounter identity plus same canonical hash: already present.
- Same encounter identity plus different hash: reject whole batch.
- Same session ordinal plus different identity or hash: reject whole batch.
- Same session revision plus different snapshot bytes: reject whole batch.
- Later snapshot missing or changing a prior prefix fact: reject whole batch.

## Compatibility

- Legacy captures import as mode `single`. A normal one-record session is
  ordinal 1; an older valid store that reused a session identity receives stable
  deterministic ordinals during read or migration without changing canonical bytes.
- Store schemas v1-v3 migrate to v4 only on a write path.
- Migration preserves source hash, content hash, canonical format, and canonical bytes.
- Read-only listing of older stores synthesizes single mode and stable
  deterministic per-session ordinals (normally ordinal 1) without migration.

## Presentation

The desktop may show validated mode, disposition, ordinal, interruption count,
failure class, and import counts under a `Last saved capture state` label. It
cannot show a toggle, claim live state, or echo raw values or unvalidated IDs.
