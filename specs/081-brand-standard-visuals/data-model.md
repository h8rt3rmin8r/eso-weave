# Data Model: Brand Standard Visuals

## ApprovedBrandAsset

- `name`: Full-color banner, Badged mark, or Badge-less glyph
- `authority_path`: canonical repository file
- `published_path`: local mdBook source copy
- `bytes`: exactly equal across authority and published copy
- `intended_use`: wordmark, universal icon, or ink-surface lockup
- `allowed_surfaces`: dark, light, or dark only
- `minimum_size`: documented CSS pixel threshold

## SurfaceExample

- `label`: Dark ink surface or Light surface
- `surface_class`: scoped theme-independent presentation class
- `assets`: allowed approved assets only
- `rule`: visible explanation of why those assets are valid

## PaletteToken

- `theme`: Dark or Light
- `role`: visible semantic role
- `hex`: visible uppercase hexadecimal value
- `use`: visible purpose
- `chip_color`: exact match to `hex`
- `accessible_name`: role plus hexadecimal value and `color swatch`
- `boundary`: visible independently of fill and documentation theme

## CompatibilityOutput

- `examples`: `eso-weave-logo-clear.png` and `eso-weave-logo-white.png`
- `status`: generated compatibility output
- `authority`: never an editable master or preferred download

## Relationships and invariants

- One published asset maps to exactly one authority and must match its bytes.
- One palette row maps to exactly one chip and one visible hexadecimal value.
- A chip's fill and accessible name must match its row.
- The glyph belongs only to a dark surface example.
- Public and bundled documentation derive from the same Markdown, CSS, and local assets.
- No entity is fetched or generated at page runtime.
