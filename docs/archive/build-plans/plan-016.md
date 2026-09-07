# Build Plan 016: Responsive Live HUD Dashboard

Plan: 016
Status: active
Master specification: `docs/ESO-Weave-Specification.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

The main window currently presents setup state, automation readiness, and live
game observations as one flat list. Working resource values are plain text, and
PixelBeacon installation can appear interchangeable with its live signal.

This plan gives the pre-Skills region an explicit information hierarchy. It
adds a glanceable Live HUD, a separate System and automation section, truthful
resource meters, and one responsive layout boundary without changing Skills.

## Ordering

Slice 046 combines issues #28 and #29 because the resource meters establish the
visual language of the Live HUD while the broader dashboard decides their
placement, dormant semantics, sizing, and accessibility contract. Delivering
either issue alone would leave the same region in a temporary mixed hierarchy.

## Slice 046: Responsive Live HUD Dashboard

Feature under `specs/046-live-hud-dashboard/`.

Scope:

- group resources and live game observations under Live HUD
- group game, application, addon, fishing, and auto-potion state under System
  and automation
- separate PixelBeacon installation from live signal and select one primary
  lifecycle action
- render Health, Stamina, and Magicka with one accessible percentage-meter
  component and typed observed, low, dormant, and unavailable states
- use two columns from 880 logical points and stack Live HUD first below it
- preserve Skills, live-log containment, modal sizing, input safety, and window
  anti-ratchet behavior

Done when issues #28 and #29 close from the merged pull request, responsive and
resource-state tests pass, every review thread is resolved, and CI is green.
