# Plan 030: Safety Boundaries

Status: Active

Sequence:

1. Extend input-owned authorization gates with an invalidation epoch for stale
   queued and running weave work.
2. Publish focus, suspension, and menu closure before controller locks while
   preserving physical pass-through and held generated-input releases.
3. Cancel suspended Fishing work without replay, retain the requested toggle,
   and require a fresh manual observation or explicit restart.
4. Classify every existing unproven PixelBeacon target as unmanaged and enforce
   ownership inside every lifecycle writer.
5. Present unmanaged content without lifecycle actions, refresh managed content
   in place, and prove both safety matrices through tests and documentation.

This slice closes issues #92 and #94. It does not include Linux key capability,
Settings application timing, unrelated UI copy, release, or field verification.
