# S130 Data Model

Date: 2026-10-15. Supported producer profile: normal/separate native log v15. Deferred profiles: #296/#297.

- Native parser state: versioned bounded current run, metadata, pending combat, exact tail bytes, physical/run-local counters, uncertainty and verified recovery state. Idle EOF is not termination. State validation precedes output after restore.
- Native canonical evidence: schema1 remains unchanged/readable. New schema2 separates logical identity from physical source positions and removes lifetime source ceilings while preserving exact original lines and provenance. Conflicting equivalent identities must not overwrite good data.
- Source checkpoint: canonical root/environment, source key, generation witness, cursor, parser state/version and revision. A checkpoint is accepted only with its actual source witness and revision.
- Ingest transaction: writer-owned validated connection; completed fights, identity aliases, supported projections and checkpoint revision commit together after current native root token is admitted under the shared control lock.
- Suppression: durable native logical identities/content equivalence prevent automatic catchup undoing supported deletion. Deletion does not erase progress/suppression.
- Derived processing key: actual source/raw hash, algorithm version and compatible Catalog semantic identity. Raw evidence is immutable; derivations are replaceable and computed without selected UI state.
- Service observation: dormant/waiting/processing/recoverable failure, controlled diagnostics, actual ingest counts and explicitly unknown logging activity. Never publish paths/raw records or infer activity from old files.

Invariants: source bytes never change; no post-stop admitted commit; Live/PTS attribution follows actual root; read-only databases stay discoverable independently of collection; unsupported evidence never becomes addon/API parity; schema upgrades occur only in successful write transactions.
