# Plan 043: Operator Intent Reliability

Status: Active

Sequence:

1. S097 completed issue #172 by persisting the requested Auto Potion toggle in
   session-state schema 4, restoring it through the existing fail-closed
   controller, converging UI and F3 persistence, and replacing obsolete
   session-only documentation without changing action eligibility.
2. S098 completed issue #171 with one bounded, process-local stale HUD
   presentation snapshot. Rendered values may be retained, while current game
   evidence remains the only authority for automation and synthesized input.
3. S099 completed issue #181 by making protected `main` the project-policy
   trust anchor, keeping collaboration and proposed branch content untrusted,
   enforcing immutable workflow dependencies and verified release tooling, and
   applying hosted pull-request and Actions controls. Its mandatory operator
   halt protocol was exercised before each authorized remediation phase.
4. S100 implements issue #206, the first child of epic #188, by publishing and
   decoding fixed-size read-only native binding evidence for eleven ESO actions.
   It deliberately leaves controller behavior and duplicate settings unchanged.
5. Issue #207 will consume valid native combat chords with platform-safe input
   ownership. Issue #208 will migrate Fishing and Auto Potion, then remove the
   remaining duplicate settings and guidance.

Release-verification issues #110, #129, #131, and #190 remain independent of
this repository-verifiable sequence. They do not become complete without their
named installed or platform evidence.
