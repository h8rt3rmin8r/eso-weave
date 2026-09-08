# Plan 032: Settings Runtime Parity

Status: Active

Sequence:

1. Define and test one effective Fishing configuration shared by persistence
   and the running controller.
2. Stop requested Fishing work safely when its configuration generation changes,
   then require the operator to start Fishing explicitly.
3. Deliver scalar Pixel Bus reader edits through a wakeable, latest-value worker
   channel while preserving startup block geometry.
4. Expose Fishing Interact Key and align numeric bounds, help, diagnostics, and
   canonical documentation with the per-setting application contract.
5. Close the deferred documentation record with runtime, UI, safety, and policy
   evidence.

This slice closes issue #95. It does not include live block-geometry switching,
embedded documentation expansion, release work, or field verification.
