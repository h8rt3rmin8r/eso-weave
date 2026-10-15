# S129 Research

Date: 2026-10-15. Baseline: 43d8c557c0b3e346fea0cbc9fc1a288af854234a.

## Package

Decision: central exact-inventory transactional PixelBeacon lifecycle with legacy Data facade. Rationale: beacon currently writes in place/removes recursively, while data_addon has bounded inventory/race/quarantine/rollback seams. Preserve the legacy saved table contract, legacy file bytes and unrelated files. Alternative of moving Lua alone leaves duplicate lifecycle and unsafe migration. Research agent: package.

## Collection

Decision: runtime in-progress work, retained completed snapshot, bounded incremental sort/serialize/checksum, automatic coalesced refresh and game priority-save request only on changed completion. Generated startup config supplies actual channel and persisted optout. Rationale: current start destroys last-good data and full finalize bypasses the tick budget. Encounter.lua auto-resumes legacy continuous state, so it must not load in combined package. Research agent: lua.

## Accumulation

Decision: verified SQLite-to-bundle accumulation with deterministic newer-evidence conflict choice and semantic coverage comparison. Rationale: current single-capture replacement and snapshot-keyed zero-removal gate reject independent first fill; test baseline manufactured from same capture masks it. Preserve true integrity/regression checks and all relationship/localized facts. Alternative unlimited removal thresholds loses evidence. Research agent: catalog.

## Runtime and preferences

Decision: typed independent settings with shared pure per-root eligibility/cancellation and periodic bounded background reconciliation. Existing complete saved data is eligible without observing a new save. Immutable activation outcome updates current consumers independent of Catalog modal. Stale results use cancellation generations. Requested, effective and next-load Lua state remain distinct. Native worker implementation is #290, while #291 supplies its seam first.

## Evidence limits

Temporary filesystem, Lua doubles and SQLite fixtures establish their tested scope only. No account/game experiment, source log mutation, forced reload, release or field-testing gate is included.

## Review corrections, 2026-10-15

The second Codex round identified that ESO derives the SavedVariables filename
from the addon manifest, not the declared global table. The current manifest
therefore uses PixelBeacon.lua; the old EsoWeaveData.lua remains untouched.
Automatic Catalog ingestion reads both in bounded legacy-first/current-second
order with independent stable-input and retry state. The historical encounter
reader retains the old input. No manifest filename override is assumed. See the
primary addon-developer explanation linked from lua-verification.md.

Windows CI also exposed in-process package recovery racing a lifecycle mutation.
The correction serializes mutation and recovery per canonical AddOns root,
retaining the existing checks for external filesystem changes.

The next hosted Windows run isolated a second path: a production bounded
inspection handle denies directory retirement while open. A deterministic
held-handle regression reproduced the same OS error 5. Public presence/status/
geometry inspections now share the per-root lease; internal mutation readers
reuse their caller's lease. Stable-read sharing and identity checks remain
unchanged, and the lease covers only bounded package I/O.
