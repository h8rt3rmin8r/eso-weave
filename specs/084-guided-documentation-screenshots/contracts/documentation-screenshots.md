# Contract: Documentation Screenshots

## Generation

```powershell
cargo test --locked --test documentation_capture -- --capture-to target/documentation-captures/s084
```

Generation must complete with 28 PNG files and `capture-manifest.json`. S084 selects only the seven `--dark--wide.png` files named in the screenshot plan and losslessly removes empty lower canvas through `scripts/curate-documentation-captures.ps1`.

## Publication

- Application and supplied PNG files live in `docs/src/assets/screenshots/`.
- The synthetic SVG lives in `docs/src/assets/illustrations/`.
- `docs/project/documentation-screenshots.json` is the canonical inventory.
- Published Markdown references only those local paths and uses the manifest's exact alt text.

## Presentation

Each image is placed in a `figure.docs-screenshot` element with a caption. Images are block-level, height auto, and width constrained to the content column. Linked images, if introduced later, retain visible keyboard focus.

## Validation

Policy validation must fail for:

- an invalid or truncated PNG, malformed chunk structure, CRC mismatch, or missing IEND;
- a missing or empty asset;
- a dimension, byte-size, or digest mismatch;
- missing or mismatched deterministic scene and receipt filename provenance;
- a duplicate identifier or destination;
- an unrecognized source kind;
- a path outside `docs/src/assets/`;
- a missing page reference, alt text, or caption;
- an unlabeled synthetic SVG;
- missing responsive screenshot CSS; or
- missing files in generated public/bundled output.

## Safety

No capture command may call a desktop screenshot API, native-window automation, ESO, the physical input hook, the synthesized input backend, or addon lifecycle mutation. The supplied PNG is consumed only under the user's explicit publication instruction.
