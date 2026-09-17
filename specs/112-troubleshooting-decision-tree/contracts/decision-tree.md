# Contract: Troubleshooting Decision Tree

## Placement

The canonical page contains exactly one local reference inside one `.docs-flow-diagram` figure:

```text
../assets/diagrams/troubleshooting-decision-tree.svg
```

The required meaningful alternative is:

```text
Troubleshooting decision tree routes the first failing observation to startup, game, PixelBeacon, input, encounter, or feature evidence
```

The figure appears under `Shared diagnostic flow` and before `Shared diagnostic flow text equivalent`. The existing fenced tree and all symptom sections remain present.

## Semantic SVG Root

The asset root has:

- namespace `http://www.w3.org/2000/svg`;
- `width="400"`, `height="1520"`, and `viewBox="0 0 400 1520"`;
- `role="img"` and `focusable="false"`;
- `aria-labelledby="troubleshooting-decision-tree-title troubleshooting-decision-tree-desc"`;
- `data-flow-direction="top-down"`;
- matching `<title>` and `<desc>` elements;
- one opaque 400 by 1520 ink canvas.

## Required Visible Labels

- First failing observation
- Startup evidence
- Game observation
- PixelBeacon evidence
- Input and bindings
- Encounter evidence
- Feature status and Live Log

Every visible text element uses at least 14 SVG units. Evidence cards include the next evidence categories, not a diagnosis or promised fix.

## Text-Equivalent Anchors

Source policy requires these canonical phrases to remain visible on the page:

- `The indented text is the complete decision tree`
- `Game discovery and runtime`
- `PixelBeacon Signal is missing or lost`
- `Native binding evidence is unavailable`
- `Encounter capture is waiting, interrupted, or failed`
- `Startup failure`
- `inspect the feature-specific status and Live Log`

## Authority and Update Triggers

Review the figure when any of these change:

- a symptom family or its first evidence check;
- a public status name used by the troubleshooting page;
- the shared diagnostic sequence or one of the five symptom sections;
- startup surfacing in `src/startup/mod.rs`;
- game presence, focus, or context in `src/game/mod.rs`;
- managed PixelBeacon lifecycle in `src/beacon/mod.rs`;
- signal or freshness authority in `src/pixelbus/mod.rs`;
- input or binding authority in `src/input/mod.rs` or `src/input/bindings.rs`;
- encounter import or validation in `src/encounter/mod.rs`, `src/encounter/validate.rs`, or `src/app/encounter_history.rs`.

## Publication Gates

- Source and generated diagram policy pass.
- Finite figure inventory reports 21 meaningful placements, including five diagrams.
- Rendering receipt contains 40 unique diagram observations.
- Layout receipt contains five unique diagram observations.
- Browser figure, no-script, print, zoom, theme, and narrow-layout evidence passes.
- The generated SVG is byte-identical to source and served as `image/svg+xml`.
- No remote or active content appears in the SVG or generated figure.
