# Research: Brand Standard Visuals

## Decision 1: Publish only three approved assets

**Decision**: Present `assets/eso-weave-banner.png`, `assets/brand/eso-weave-mark.svg`, and `assets/brand/eso-weave-glyph.svg` as the approved set. Describe older clear and white PNG variants as generated compatibility outputs.

**Rationale**: The brand README identifies the SVGs as masters and regeneration output as derived. The banner is already approved and published by S080.

**Alternatives considered**:

- Display every raster. Rejected because it would blur master and compatibility status.
- Redesign or regenerate assets. Rejected because issue #122 asks for presentation, not new identity work.

## Decision 2: Require byte-identical documentation copies

**Decision**: Copy the three authorities into `docs/src/assets/brand/` without editing, optimization, or re-encoding.

**Rationale**: mdBook only publishes files in its source tree. Byte comparison provides simple provenance and keeps public and bundled output identical.

## Decision 3: Use labeled inline swatches beside visible values

**Decision**: Add a `<span class="brand-swatch" role="img" aria-label="ROLE, #HEX color swatch" style="--swatch-color: #HEX"></span>` before the existing visible code value in every palette row.

**Rationale**: The role, hex, and use remain readable text, while assistive technology receives an explicit chip description. A CSS border independent of the swatch fill handles near-white and near-black colors.

**Alternatives considered**:

- Background-color the whole cell. Rejected because contrast and theme interactions become harder to govern.
- Use images. Rejected because 25 redundant files add maintenance and weak semantics.
- Generate chips with JavaScript. Rejected because offline output needs no behavior for static information.

## Decision 4: Demonstrate surfaces with scoped cards

**Decision**: Use labeled dark and light surface cards. Show the banner and badged mark on both contexts, and the glyph only on the dark card.

**Rationale**: Readers can compare valid placement without mistaking the light-surface glyph for an approved use. Scoped cards avoid global table changes reserved for issue #127.

## Decision 5: Derive reproduction rules from existing assets

**Decision**: Preserve aspect ratio, reserve clear space equal to one strand width, use 16 CSS pixels as the minimum badged-mark size, 32 CSS pixels for the glyph, and 160 CSS pixels for the banner.

**Rationale**: These practical limits match the existing mark description and prevent the transparent glyph and detailed banner from becoming illegible. They do not alter master files.

## Verification implications

- Byte comparison proves all published copies match authorities.
- Exact token maps prove all 25 current palette roles and values are represented.
- Source and generated validators prove semantics, valid surface use, and local delivery.
- CSS validation proves chip borders, intrinsic image sizing, responsive card reflow, and no reliance on color alone.
