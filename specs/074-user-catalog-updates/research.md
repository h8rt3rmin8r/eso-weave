# Research: User-Initiated Catalog Updates

## R1. Treat observed game version as evidence, not update availability

**Decision**: Extend the existing bounded startup result with explicit fresh or
offline status, then feed it into the catalog availability resolver. Do not let
the API checker own catalog policy.

**Rationale**: The GitHub Live branch can reveal game-version drift but cannot
prove that a reviewed candidate exists. Keeping the source seam and moving the
decision into one catalog model eliminates the current split authority.

## R2. Use a user-owned candidate import area instead of inventing a feed

**Decision**: Scan only `<config>/catalog/import/{live,pts}/<sha256>/` on the
background worker. Every directory must pass the complete S073 verifier before
it is summarized.

**Rationale**: No approved remote candidate endpoint, signature authority, or
release protocol exists. GitHub Actions artifacts are review evidence, not a
stable unauthenticated update service. A local inbox delivers explicit import
without expanding network authority.

## R3. Preserve the complete candidate as the installed immutable unit

**Decision**: Install to `<config>/catalog/versions/live/<sha256>/`, retaining all
nine S073 files. The runtime opens `catalog.sqlite` only after the whole candidate
verifies at its installed identity.

**Rationale**: Keeping only SQLite would discard provenance, diff, validation,
and source-policy evidence required for later diagnosis and rollback.

## R4. Commit selection through one tiny atomic record

**Decision**: `selection.json` identifies active and previous targets as either
Bundled or a Live candidate hash. Candidate directories publish without
replacement. A same-directory temporary file is synced and atomically replaces
the pointer using the established Windows `ReplaceFileW` and Unix rename pattern.

**Rationale**: Large directory replacement is difficult to make portable.
Immutable versions plus one pointer make installation restart-safe and keep the
commit boundary auditable.

## R5. Use a standard-library cross-process file lock

**Decision**: Hold an exclusive non-blocking lock on `operation.lock` for every
install, rollback, and recovery mutation. The process-local coordinator also
rejects a second worker.

**Rationale**: File locks release automatically on crash and work on Windows and
Linux without another dependency or stale PID lease. The lock file may remain;
the lock state, not file existence, owns exclusivity.

## R6. Make cancellation cooperative and the pointer commit non-interruptible

**Decision**: Check a shared atomic cancellation flag between bounded copy and
verification stages. After the new candidate passes first open, ignore
cancellation until the selection replacement and receipt are durably resolved.

**Rationale**: Interrupting an individual filesystem call is neither portable
nor safer. A short explicit commit boundary yields an unambiguous old or new
selection.

## R7. Recover by verification, never deletion of accepted versions

**Decision**: On startup, remove only clearly named staging directories while
holding the operation lock, mark an unfinished receipt as interrupted, and
re-resolve the selection. Never delete a directory referenced by active or
previous selection during automatic recovery.

**Rationale**: Accepted immutable candidates are evidence and rollback points.
Recovery should clean partial work but must not guess that accepted content is
obsolete.

## R8. Collector-assisted builds reuse S073 end to end

**Decision**: After the user enters Waiting for capture, require a later stable
file observation, parse the S071 envelope for metadata, generate an internal
local-only S073 request, use the active Live catalog as a zero-removal baseline,
then install the resulting reviewed candidate through the same path.

**Rationale**: Directly compiling the capture would bypass the source inventory,
diff, icon, and candidate verifier. A zero-removal baseline prevents a partial
capture from silently discarding previously accepted data.

## R9. Keep progress values evidence-based

**Decision**: Stages carry either no total or a monotonically increasing
completed/total pair with a unit. Candidate copying uses manifest artifact byte
totals. Parsing/build stages remain indeterminate unless their existing APIs
provide exact counts.

**Rationale**: A smooth but fabricated percentage would be misleading. Stage
transitions and elapsed time still provide useful feedback.

## R10. Keep database handles out of shared concurrent state

**Decision**: The worker verifies and first-opens the candidate, then reports a
terminal identity. The GUI thread resolves and opens the now-selected catalog at
the next frame boundary. Shutdown waits only for the update worker's short commit
boundary, not for a shared SQLite handle.

**Rationale**: `CatalogAccess` owns a connection and need not become globally
locked. The selected immutable path is a stable handoff seam.

## R11. Modal behavior follows existing egui seams

**Decision**: Use `egui::Modal`, stable accessible labels, explicit focus
requests, Escape rules derived from operation cancellability, text progress, and
the existing responsive `modal_extent` pattern. Reduced motion disables animated
indeterminate affordances while preserving text.

**Rationale**: Existing settings-modal and headless AccessKit tests already prove
the project pattern. Reusing it avoids a second UI framework or untested window.

## R12. Separate integrity from origin authenticity

**Decision**: Present S073 verification as internal integrity only. Require a
user acknowledgement that the imported candidate came from a trusted reviewed
source. Do not label a candidate authenticated or officially signed.

**Rationale**: S073 manifests are hash-addressed but unsigned. Any local actor
can construct a self-consistent candidate, so claiming authenticated approval
would be false until a separate signing and distribution authority exists.

## R13. Treat receipt persistence as subordinate to selection safety

**Decision**: Attempt a durable receipt for every terminal result. If storage is
unavailable, preserve the old or newly committed selection, retain the in-memory
terminal result, and add `receipt-write-failed`.

**Rationale**: Disk exhaustion is itself a required failure case. It is
impossible to guarantee another durable write under that condition, and receipt
failure must not roll back an otherwise unambiguous atomic selection.
