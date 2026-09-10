# Catalog Update Safety Checklist: User-Initiated Catalog Updates

**Purpose**: Verify activation, rollback, privacy, and collector boundaries
**Created**: 2026-09-09
**Feature**: [spec.md](../spec.md)

## Authority and Channel

- [x] CHK001 Only explicit user actions can install, build, roll back, or clean up.
- [x] CHK002 PTS remains preview-only and can never become the Live selection.
- [x] CHK003 No remote feed, silent download, release, or package update is added.
- [x] CHK003A Candidate integrity is never presented as authenticated origin.

## Filesystem and Recovery

- [x] CHK004 The bundled installation directory is read-only.
- [x] CHK005 Candidate directories publish immutably on the same filesystem.
- [x] CHK006 The active pointer is bounded, canonical, and atomically replaced.
- [x] CHK007 A cross-process lock excludes concurrent mutation.
- [x] CHK008 Cancellation and crash recovery preserve the prior selection.
- [x] CHK009 Automatic recovery never deletes an accepted active or previous target.

## Verification and Privacy

- [x] CHK010 The complete S073 verifier and S070 first open run before selection.
- [x] CHK011 Receipts and UI summaries contain hashes and counts, never local paths or content.
- [x] CHK011A Receipt-write failure cannot corrupt or ambiguously roll back selection.
- [x] CHK012 Unsupported schema, corruption, wrong channel, and rights-policy failures close safely.

## Collector Boundary

- [x] CHK013 Collector lifecycle remains separately marker-gated from PixelBeacon.
- [x] CHK014 Import requires a later stable flushed capture and never executes Lua.
- [x] CHK015 Collector builds retain zero-removal baseline gates and S072 placeholders.

## Interface

- [x] CHK016 Progress is determinate only from exact totals.
- [x] CHK017 Focus, Escape, cancellation, text status, reduced motion, and narrow sizing are testable.
