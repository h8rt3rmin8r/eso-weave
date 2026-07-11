# View-Model and Wiring Requirements Checklist: GUI

**Purpose**: Validate that the requirements governing the testable application
view-model and the subsystem wiring are complete, unambiguous, consistent, and
measurable before planning and implementation, given the egui rendering itself is
validated manually.
**Created**: 2026-07-11
**Feature**: [spec.md](../spec.md)

## Testability Seam

- [x] CHK001 Is the separation of the correctness-bearing logic (display-state derivation, UI-intent handling, settings-to-config mapping, reader-event routing) from the egui rendering stated as a requirement? [Completeness, Spec FR-016, Clarifications]
- [x] CHK002 Is it clear that the view-model is unit-testable against stubbed subsystems and a crafted configuration, independent of any live window? [Clarity, Spec FR-016, SC-007]
- [x] CHK003 Is the manual-validation expectation for the un-unit-testable rendering documented? [Coverage, Spec Assumptions, SC-007]

## Status Region Controls

- [x] CHK004 Are the application-state indicator values (Running/Suspended) and the suspend-toggle button's action-reflecting label specified? [Completeness, Spec FR-003]
- [x] CHK005 Is it specified that the suspend control drives the input engine's suspend state and that suspend-exempt toggles remain active while suspended? [Consistency, Spec FR-003, Edge Cases]
- [x] CHK006 Are the fishing-state indicator and the go-fish/stop toggle (driving the fishing controller's enable/disable) specified with action-reflecting labels? [Completeness, Spec FR-004]

## PixelBeacon Status Light and Buttons

- [x] CHK007 Is the status-light color rule (green only when installed and current, red otherwise) stated unambiguously? [Clarity, Spec FR-005]
- [x] CHK008 Are all four exact tooltip conditions enumerated (installed and current, installed but outdated, not installed, AddOns directory not found)? [Completeness, Spec FR-005, Clarifications]
- [x] CHK009 Is the Install-or-update behavior and the Uninstall single-confirmation-and-disabled-when-not-installed rule specified? [Completeness, Spec FR-006]
- [x] CHK010 Is the unmanaged-uninstall refusal and the AddOns-not-found handling surfaced at the UI level? [Edge Case, Spec Edge Cases]

## Skills Region

- [x] CHK011 Are the per-slot row contents specified (label including Ultimate/Synergy, active checkbox, weave-type dropdown limited to the fixed four, per-slot delay override)? [Completeness, Spec FR-007]
- [x] CHK012 Is it specified that editing active, weave type, or override updates the corresponding weave-configuration slot (and that a cleared override uses the global default)? [Clarity, Spec FR-008, US3]

## Live Log Viewer

- [x] CHK013 Is the ring-buffer source and its independence from file logging specified? [Completeness, Spec FR-009]
- [x] CHK014 Are the per-level colors enumerated (ERROR red, WARN amber, INFO neutral, DEBUG dim, TRACE dimmer)? [Clarity, Spec FR-010]
- [x] CHK015 Are the pause-scroll autoscroll behavior and the panel-local level filter specified? [Completeness, Spec FR-011]
- [x] CHK016 Is the View-toggle attach/detach behavior, including releasing the panel's resources on detach, specified? [Coverage, Spec FR-002, US4]

## Settings Surface

- [x] CHK017 Does the settings requirement enumerate every section-10.3 category to be edited and persisted? [Completeness, Spec FR-012]
- [x] CHK018 Is the load/apply/persist round-trip specified (open loads from config, apply writes back and saves, reopen shows applied values)? [Clarity, Spec FR-013, US5]
- [x] CHK019 Is the fallback-with-notice behavior for an invalid settings field specified, consistent with the existing loaders? [Edge Case, Spec FR-013, Clarifications]
- [x] CHK020 Are theme (dark default, light) and always-on-top specified as taking effect and persisting? [Completeness, Spec FR-012, FR-013]

## Reader-Event Routing and Worker Loop

- [x] CHK021 Is the reader-event routing table complete and unambiguous for every event kind (Latency, SignalLost, FishingStarted, BiteDetected, FishingStopped, Heartbeat)? [Completeness, Spec FR-014]
- [x] CHK022 Is the SignalLost dual action (clear weave latency and disable the fishing controller) stated so it aligns with the fishing safety surface? [Consistency, Spec FR-014, Constitution II]
- [x] CHK023 Is the worker loop required to pump the reader and drive fishing ticks without blocking the UI thread? [Completeness, Spec FR-015]
- [x] CHK024 Is the routing function required to be pure/testable against stubs, separate from the timer-driven worker loop? [Clarity, Spec FR-016, US6]

## Consistency and Traceability

- [x] CHK025 Do the status/config edits made from the window stay consistent with the existing subsystem operations (input suspend, fishing enable/disable, beacon lifecycle, weave config, settings loaders) rather than introducing parallel behavior? [Consistency, Spec Assumptions]
- [x] CHK026 Does every functional requirement have at least one acceptance scenario or measurable success criterion? [Traceability, Spec FR/US/SC]

## Notes

- This checklist tests the quality of the requirements, not the implementation.
- All items were evaluated against the spec as written and pass; it is retained as
  the definition-of-done reference for the pre-push review.
- Because the egui rendering is not unit-testable in this environment, the
  view-model separation (CHK001, CHK002) is the load-bearing requirement that keeps
  the slice verifiable; it MUST remain covered by tests, and the rendering is signed
  off via the manual checklist referenced in the plan.
