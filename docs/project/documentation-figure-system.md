# Documentation Figure System

S088 provides one local, accessible expansion interaction and one caption hierarchy for the meaningful images used by the public and bundled documentation.

## Maintained inventory

The canonical source contains 21 image placements.

| Kind | Placements | Interactive | Caption contract |
| --- | ---: | --- | --- |
| Screenshots and synthetic illustration | 11 | Yes | One adjacent caption per figure |
| Flow diagrams | 4 | Yes | Complete adjacent prose equivalent |
| Brand examples | 5 | Yes | One group caption per brand surface |
| Landing wordmark | 1 | No | Decorative empty alternative beside the accessible heading |

Documentation policy rejects a new, removed, or reclassified placement until this finite inventory and its tests are deliberately updated. The same asset may appear on more than one page; each placement receives its own control.

## Runtime contract

Authored HTML remains static and readable without JavaScript. After mdBook's own script runs, the local ESO Weave theme recognizes only these established surfaces:

- a direct image in `.docs-screenshot`;
- the primary generated image in `.docs-flow-diagram`; and
- a direct image in `.brand-surface__assets`.

Each meaningful image becomes one native button named `Expand image: <alternative>`. Its persistent `Expand image` badge provides a visible affordance without relying on hover or color. The decorative landing wordmark remains unchanged and does not become a control.

mdBook 0.5.4 generates a hidden checkbox and duplicate clone for Markdown images. That interaction lacks an obvious close control, dialog semantics, caption association, verifiable modal inertness, and exact focus restoration. S088 therefore removes that generated wrapper at runtime after preserving the meaningful primary diagram and routes it through the same local interaction as raw HTML figures.

One native `dialog` is shared by the document. It receives the selected image's existing local source and alternative, plus a cloned description of the owning caption when present. The visual image inside the dialog is decorative because the dialog already carries the active accessible name. Native modal behavior makes the background inert and contains focus.

The close button receives focus on open. Tab and Shift+Tab stay on that sole interactive modal control. Escape, the close button, and a backdrop pointer action close the view. The close event restores focus to the exact invoking figure button. Initialization is idempotent and creates no second dialog, trigger, or listener set.

## Geometry and captions

The modal image uses intrinsic auto dimensions with viewport maxima. It may shrink to fit but never stretches beyond its natural dimensions. This preserves the four SVG diagram ratios, large landscape captures, and the smaller portrait Windows Properties image.

Screenshot and brand captions use `0.9em` with a `1.55` line height. mdBook uses a 10-pixel root and 16-pixel body copy, so `0.9em` computes to 14.4 pixels. The initially suggested `0.9rem` would compute to only 9 pixels and is intentionally not used. Borders, padding, and surface colors remain component-specific, and strong brand lead-ins remain bold without italicizing entire captions.

Print rules hide the expansion badge and dialog while leaving the source image and caption visible through a neutral button wrapper.

## Automated evidence

Source policy accounts for all 20 meaningful placements, the one decorative exception, and all 13 captions. Script and CSS mutations cover selector boundaries, native-dialog construction, idempotence, mdBook wrapper replacement, accessible name and description, close paths, sequential focus, focus return, intrinsic sizing, caption scale, caption line height, strong lead-ins, print neutralization, and no-script legacy-chrome suppression.

The dependency-free browser smoke reuses one hidden host Chrome process and the generated local site. Its 20-cell S088 matrix covers:

| Dimension | Values |
| --- | --- |
| Figures | Markdown diagram, landscape screenshot, portrait screenshot, synthetic illustration, brand banner |
| Themes | Navy, Light |
| CSS viewport widths | 320, 1280 |
| Total observations | 20 |

Each observation proves one semantic trigger, one native modal, no legacy checkbox or clone, persistent affordance, accessible label and caption association, initial close-button focus, background inertness, intrinsic aspect ratio, no upscaling, viewport containment, source and modal caption size and contrast, the matching light or dark brand surface, and no page-level overflow introduced by the figure system.

Separate trusted CDP keyboard and pointer journeys prove Enter and Space opening, forward and reverse Tab containment, Escape closing, close-button closing, backdrop hit testing, and exact focus return. Dedicated observations prove readable associated captions and contained images at 200 percent browser scale, static source images and captions with script resources blocked, and clean print media with interaction chrome hidden. The existing 32-cell diagram paint matrix and 40-cell syntax matrix continue to pass in the same browser run.

The figure system adds no dependency, remote resource, browser download, or second viewer. GitHub Pages and release builds continue to consume the same generated documentation bytes.

## Maintenance rules

1. Give every meaningful image a concise, task-relevant alternative.
2. Place screenshots and illustrations in `.docs-screenshot` with one direct caption.
3. Place static flow diagrams in `.docs-flow-diagram` and keep their complete adjacent prose equivalent.
4. Place approved identity examples in `.brand-surface__assets` with one group caption on the surface.
5. Use empty alternatives only for truly decorative images whose meaning is already supplied nearby.
6. Do not add a second modal library or per-image dialog.
7. When adding a new figure kind, update source inventory policy, a representative browser case when necessary, print behavior, and this record in the same slice.

General table responsiveness and unrelated page overflow remain owned by issue #127.
