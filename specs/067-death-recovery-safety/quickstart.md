# Quickstart: Death Recovery Safety Verification

1. Run focused PixelBeacon contract and reader tests. Confirm each recovery wire
   value decodes fail closed and recovered Alive is last in its refresh capture.
2. Run input and weave tests. Close life authorization after admission and during
   each generated sequence family, then confirm no new Down and balanced release.
3. Run fishing tests for armed, reel, recast, and timeout states. Confirm non-Alive
   cancels deadlines and Alive alone emits nothing.
4. Run the auto-potion reproduction: trigger once, enter Dead or Recovering, let
   the old retry interval expire, route a coherent Alive capture with low health,
   and confirm the first tick emits no Q. Confirm the earliest later attempt is
   one full configured interval afterward.
5. Run routing, view-model, and documentation tests. Confirm recovery path text,
   epoch/generation diagnostics, and fail-closed signal loss.
6. Run documentation policy, link, UTF-8, punctuation, and mojibake checks.
7. Run CI parity in the foreground: formatting, Clippy with warnings denied, and
   the complete locked test suite.

Live ESO and installed-release scenarios remain in #110.
