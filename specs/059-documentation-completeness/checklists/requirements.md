# Requirements Checklist: Documentation Completeness

**Purpose**: Verify that S059 remains a complete, testable, documentation-only implementation of GitHub issue #81.

**Created**: 2026-09-07

**Feature**: [spec.md](../spec.md)

## Scope and Authority

- [x] CHK001 Every claim is verified against the application, addon, tests, packaging, or another repository-owned source at the S059 baseline.
- [x] CHK002 The published manual covers every shipped user-visible feature, setting, status, lifecycle action, failure state, and recovery path named by issue #81.
- [x] CHK003 The developer manual covers every action-authorizing state machine, safety gate, protocol boundary, controller flow, persistence rule, and delivery mechanism named by issue #81.
- [x] CHK004 User guidance, developer contracts, implementation notes, diagnostic advice, and version-sensitive facts are visibly distinguished.
- [x] CHK005 New or revised pages have one clear audience and purpose, and the root README remains a concise landing page rather than a duplicate manual.
- [x] CHK006 Maintainer governance, autopilot instructions, active specifications, project plans, and archive records remain outside the published manual.
- [x] CHK007 S059 changes only documentation, documentation tests or policy, navigation, and documentation-owned assets.
- [x] CHK008 Runtime, UI, addon, packaging, and release behavior are not changed in this slice.
- [x] CHK009 A discovered implementation defect is tracked separately and is not converted into a false documentation guarantee.
- [x] CHK010 Work owned by issues #79, #82, #84, and #77 is not absorbed into S059.

## Reader Journeys

- [x] CHK011 A Windows reader can install, complete first launch, connect ESO and PixelBeacon, verify a healthy signal, update, uninstall, and recover from common failures.
- [x] CHK012 A Linux reader can install each supported package, satisfy input permissions, complete first launch, update, uninstall, and diagnose X11 or XWayland limitations.
- [x] CHK013 A reader can configure and verify one weaving slot, understand timing and weapon-bar behavior, and determine why an input passed through or a weave was dropped.
- [x] CHK014 A reader can configure, start, observe, stop, and recover Fishing from every displayed idle reason.
- [x] CHK015 A reader can configure Auto Potion, interpret every effective state, verify quickslot eligibility, and understand every fail-closed condition before enabling it.
- [x] CHK016 A reader can interpret every Live HUD and System and State value without relying on color alone.
- [x] CHK017 A reader can find every setting's default, accepted values, persistence, effect timing, interaction, and restart or reload requirement.
- [x] CHK018 A reader can locate logs, select an appropriate level, preserve useful evidence, and follow bounded troubleshooting steps without exposing sensitive data.
- [x] CHK019 A developer can trace physical input, generated input, observation updates, controller decisions, persistence, and release artifacts through their owning subsystem and thread.

## Traceability and Verification

- [x] CHK020 A feature-to-page matrix accounts for every user-visible feature and setting found in the application, root README, addon, and current specifications.
- [x] CHK021 A logic-to-page matrix accounts for every action-authorizing state machine, gate, protocol transition, and fail-closed outcome.
- [x] CHK022 Every matrix row names its canonical page and repository evidence, and no row is left partial without an explicit follow-up issue.
- [x] CHK023 Every new published Markdown page appears exactly once in `docs/src/SUMMARY.md` and is reachable from a relevant audience index.
- [x] CHK024 Local links, fragments, image paths, path case, generated search, and `/eso-weave/` subpath behavior pass repository documentation checks.
- [x] CHK025 The manual contains no stale S058 disclaimer that assigns completeness or source accuracy to future issue #81 work.
- [x] CHK026 `mdbook` and link checking pass with the repository-pinned versions, and generated site output is not committed.
- [x] CHK027 Existing Cargo and documentation policy checks remain green even though S059 does not change runtime code.
- [x] CHK028 UTF-8 without BOM, LF endings, trailing newlines, whitespace hygiene, mojibake checks, and the en-dash and em-dash prohibition pass.

