# Research: Documentation Figure System

## Current evidence

- The source corpus contains 17 raw HTML image placements and four Markdown image placements.
- Eleven raw HTML placements are screenshots or the synthetic PixelBeacon illustration, five are brand examples, and one is the decorative landing wordmark.
- The four Markdown placements are flow diagrams. mdBook 0.5.4 expands only these images by generating a hidden checkbox, primary image, and duplicate expanded clone.
- The corpus contains 13 captions: 11 screenshot or illustration captions and two brand-surface group captions.
- Raw HTML images retain intrinsic width and height where applicable but receive no expansion behavior from mdBook.
- mdBook's checkbox interaction supplies pointer activation and Escape handling, but it has no obvious close button, no explicit dialog semantics, no caption description in the expanded view, and no verifiable focus containment or exact focus return.
- The existing project theme already loads after `book.js`, so it can normalize generated Markdown figures and raw HTML figures after mdBook has completed its own transformation.
- The existing hidden-browser smoke already launches one local Chrome-compatible process without a shell or visible Windows child window and serves the generated site with a restrictive content security policy.

## Decision: one native dialog, not parallel viewer dependencies

Create one native HTML `dialog` per document and reuse it for every meaningful figure. Convert each recognized image into a semantic button with a visible `Expand image` affordance. For Markdown diagrams, remove mdBook's generated checkbox and clone at runtime and retain its meaningful primary image inside the common trigger.

This is an explicit deviation from retaining mdBook's built-in expansion. The built-in checkbox cannot meet the issue's close-control, dialog semantics, caption association, modal inertness, and focus lifecycle criteria. A local native dialog satisfies those requirements with less code and a smaller attack surface than patching an invalid nested-interactive label or adding GLightbox.

## Decision: accessible meaning belongs to the active control and dialog

Before enhancement, each source image keeps its existing alternative text. After enhancement, the trigger receives `Expand image: <alternative>` and its contained visual image becomes decorative so the button has one accessible name. While open, the modal is labelled by the same alternative and described by a cloned caption when present; its visual image remains decorative. Native modal behavior makes the background document inert, preventing a duplicate active image meaning.

The original caption stays in its source `figure`. Cloning its markup into the dialog provides equivalent context without moving or weakening the authored semantic association.

## Decision: intrinsic size is the upper bound

The modal image uses `width: auto`, `height: auto`, viewport maxima, and the decoded intrinsic dimensions. It may shrink to fit either axis but does not receive a forced fill width, so a small portrait image is not stretched and a large landscape image remains contained.

## Decision: shared caption hierarchy without global figure assumptions

Apply a shared `0.9em` size and `1.55` line height to screenshot and brand captions. mdBook uses a 10-pixel root and 1.6rem body copy, so the issue's suggested `0.9rem` would incorrectly render at 9 pixels. The body-relative unit computes to 14.4 pixels, preserves the intended modest reduction from 16-pixel prose, and stays above the 14-pixel minimum. Keep component-specific border, padding, and inherited surface color rules. Preserve strong brand lead-ins and do not italicize entire captions.

## Decision: layered evidence

Source policy proves the exact 20 meaningful plus one decorative inventory. Theme-contract tests prove one idempotent enhancer, native dialog lifecycle, print neutralization, hidden legacy no-script chrome, and caption CSS. Generated policy proves static figures and local assets survive mdBook. The browser layer then proves the actual post-script DOM, trusted pointer hit testing, keyboard journeys, sequential focus containment, focus return, modal state, aspect ratio, containment, source and modal caption metrics, both brand surfaces, 200 percent browser scale, print media, blocked-script rendering, and absence of duplicate modals.

The browser matrix uses five representative cases: Markdown diagram, landscape screenshot, portrait screenshot, synthetic illustration, and brand example. Each runs in navy and light at 320 and 1280 CSS pixels. Structural policy covers every placement and all five supported themes.

## Alternatives rejected

- **Retain the checkbox unchanged**: fails the explicit close-control, dialog, focus, inertness, and caption requirements.
- **Inject raw HTML images into the checkbox structure**: spreads the same accessibility gaps to more content and creates invalid nested interaction if a close button is added inside its label.
- **Add GLightbox or another viewer**: adds licenses, package management, runtime bytes, CSP surface, and offline failure modes without a benefit over native dialog.
- **Create one dialog per image**: duplicates DOM, focus state, IDs, and cleanup behavior across every placement.
- **Upscale every modal image to fill the viewport**: makes small raster screenshots blurry and violates intrinsic-size requirements.
- **Apply a global `figcaption` rule**: assumes future unrelated figure components share the same surface and visual contract.
- **Use italics for all captions**: reduces long-caption scanning comfort without evidence that size and spacing are insufficient.

## Scope handoff

S088 owns cross-corpus figure expansion, the one modal lifecycle, caption hierarchy, and representative browser evidence. Issue #127 continues to own general table responsiveness and page-level overflow remediation.
