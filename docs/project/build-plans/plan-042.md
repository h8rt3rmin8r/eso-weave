# Plan 042: Persistent Data Addon Subsystem

Status: Active

Sequence:

1. S092 completed the permanent two-addon topology, isolated Catalog and
   Encounter modules inside ESO Weave Data, selected a provisional encounter
   ingestion direction, and recorded the desktop-to-addon command no-go.
2. S093 completed issue #185 by adding first-class, evidence-scoped ESO Weave
   Data lifecycle and status controls directly beneath PixelBeacon.
3. S094 implements child issue #198 as the first bounded tranche of #186. It
   establishes a lossless raw envelope for already selected callbacks and API
   observations, explicit loss, and non-destructive v1 compatibility. Issue #186
   remains open for source expansion and pure Rust renormalization.
4. Issue #183 follows sufficient #186 progress and ingestion qualification to implement
   approved encounter operating modes without assuming an unproven transport.
5. Issue #190 remains independent Release verification and gates native-log
   platform claims or implementation until Windows and Linux or Proton receipts
   exist.

Epic #182 coordinates these slices. It remains In progress without a slice code
until its children and independent verification dependencies are reconciled.
