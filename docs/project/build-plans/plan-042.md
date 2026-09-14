# Plan 042: Persistent Data Addon Subsystem

Status: Active

Sequence:

1. S092 completed the permanent two-addon topology, isolated Catalog and
   Encounter modules inside ESO Weave Data, selected a provisional encounter
   ingestion direction, and recorded the desktop-to-addon command no-go.
2. S093 implements issue #185 by adding first-class, evidence-scoped ESO Weave
   Data lifecycle and status controls directly beneath PixelBeacon.
3. A later work slice decomposes and implements issue #186 for lossless
   subscribed-event capture while preserving boundedness and privacy decisions.
4. Issue #183 follows #186 and sufficient ingestion qualification to implement
   approved encounter operating modes without assuming an unproven transport.
5. Issue #190 remains independent Release verification and gates native-log
   platform claims or implementation until Windows and Linux or Proton receipts
   exist.

Epic #182 coordinates these slices. It remains In progress without a slice code
until its children and independent verification dependencies are reconciled.
