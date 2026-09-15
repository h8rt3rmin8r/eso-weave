# Plan 042: Persistent Data Addon Subsystem

Status: Active

Sequence:

1. S092 completed the permanent two-addon topology, isolated Catalog and
   Encounter modules inside ESO Weave Data, selected a provisional encounter
   ingestion direction, and recorded the desktop-to-addon command no-go.
2. S093 completed issue #185 by adding first-class, evidence-scoped ESO Weave
   Data lifecycle and status controls directly beneath PixelBeacon.
3. S094 completed child issue #198 as the first bounded tranche of #186. It
   established a lossless raw envelope for already selected callbacks and API
   observations, explicit loss, and non-destructive v1 compatibility.
4. S095 completed child issue #200 as the final #186 tranche. It independently
   replays complete current captures, rejects divergent projections, preserves
   explicit partial and legacy outcomes, and closes the reviewed include and
   exclude decision surface without adding a source family. The official review
   surface is [pull request #201](https://github.com/h8rt3rmin8r/eso-weave/pull/201).
5. S096 implements issue #183 as the final epic child. It adds exactly single
   and continuous-until-disabled encounter modes, a bounded ordered session
   spool, atomic terminal-record import, and historical desktop status. Capture
   control remains user-initiated inside ESO because S092 approved no desktop
   command ingress. The official review surface is
   [pull request #202](https://github.com/h8rt3rmin8r/eso-weave/pull/202).
6. Issue #190 remains independent Release verification and gates native-log
   platform claims or implementation until Windows and Linux or Proton receipts
   exist.

Epic #182 coordinates these slices. It remains In progress without a slice code
until S096 is reviewed and merged. Issue #190 remains independently owned and
does not block the SavedVariables-backed mode implementation.
