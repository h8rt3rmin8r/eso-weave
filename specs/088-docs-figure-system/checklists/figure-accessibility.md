# Figure Accessibility Checklist: Documentation Figure System

**Purpose**: Protect interaction, modal, caption, geometry, offline, and print behavior for issues #155 and #156.
**Created**: 2026-09-11
**Feature**: [spec.md](../spec.md)

## Inventory and enhancement

- [x] CHK001 Policy accounts for 20 meaningful placements and one decorative wordmark.
- [x] CHK002 Every meaningful image becomes exactly one native button with a persistent visible affordance.
- [x] CHK003 The decorative wordmark stays empty-alt and non-interactive.
- [x] CHK004 Repeated initialization creates no duplicate trigger or dialog.
- [x] CHK005 Generated mdBook checkboxes and expanded clones do not remain after enhancement.

## Modal and focus

- [x] CHK006 Pointer, Enter, and Space open the same shared native modal.
- [x] CHK007 The close button receives initial focus and is visibly obvious.
- [x] CHK008 Escape, close button, and backdrop each close the modal.
- [x] CHK009 Native modal state makes the background inert and contains sequential focus.
- [x] CHK010 Closing returns focus to the exact invoking trigger.
- [x] CHK011 The active dialog exposes one accessible image name and an optional caption description.

## Geometry and captions

- [x] CHK012 Portrait and landscape images preserve aspect ratio and never upscale past intrinsic dimensions.
- [x] CHK013 Modal images remain within narrow and wide viewports.
- [x] CHK014 All 13 captions share the intentional `0.9em` body-relative hierarchy and at least `1.5` line height.
- [x] CHK015 Caption contrast is at least 4.5:1 in every supported theme.
- [x] CHK016 Captions remain wrapped, unclipped, and associated at 200 percent zoom.
- [x] CHK017 Brand caption strong lead-ins retain useful emphasis without whole-caption italics.

## Delivery and evidence

- [x] CHK018 No remote asset, dependency, CDN, browser download, or second viewer is introduced.
- [x] CHK019 Static no-JavaScript figures and print output remain readable without interactive chrome.
- [x] CHK020 Source policy, generated policy, mutation tests, and the 20-cell browser matrix pass.
- [x] CHK021 Documentation, UTF-8, mojibake, text-hygiene, and repository gates pass.
