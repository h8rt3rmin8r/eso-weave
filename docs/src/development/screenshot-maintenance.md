# Documentation Screenshot Maintenance

ESO Weave publishes a deliberately small visual set for Installation, First
Launch, Weaving, Auto Potion, and PixelBeacon. Application images come from the
test-only deterministic sandbox. They are not desktop captures and require no
running ESO client, account, physical input hook, or synthesized input backend.

The canonical asset inventory is
[`docs/project/documentation-screenshots.json`](https://github.com/h8rt3rmin8r/eso-weave/blob/main/docs/project/documentation-screenshots.json).
It records the reader question, published pages, exact alternative text and
caption, dimensions, byte size, SHA-256 digest, source receipt, crop rectangle,
and update trigger for every asset.

## Published set

| Source | Published assets | Maintenance rule |
| --- | ---: | --- |
| S083 deterministic application sandbox | 7 PNG files | Regenerate after a named UI or state-contract trigger, then retain only the planned dark-wide variants |
| Maintainer-supplied Windows Properties capture | 1 PNG | Keep the authorized bytes until the maintainer approves a replacement |
| Repository-authored synthetic PixelBeacon schematic | 1 SVG | Keep the visible synthetic label and update when overlay placement or geometry changes |

The seven application images are losslessly cropped to remove empty lower canvas.
Six dashboard captures use `[0, 0, 1280, 640]`; the Auto Potion settings capture
uses `[0, 0, 1280, 820]`. No content is scaled or rewritten during curation.

## Refresh checklist

1. Review the inventory's update triggers against the application and guidance
   changes in the current slice.
2. Generate a complete capture set into an ignored directory:

   ```powershell
   cargo test --locked --test documentation_capture -- --capture-to target/documentation-captures/refresh
   ```

3. Confirm the generated manifest contains all 28 scene, theme, and viewport
   variants and that the target reports no emitted input.
4. Review the seven dark-wide sources for truthful labels, complete controls,
   consistent state, and absence of personal data.
5. Run the curation script with the generated root and an explicitly authorized
   MSI Properties source:

   ```powershell
   ./scripts/curate-documentation-captures.ps1 `
     -CaptureRoot target/documentation-captures/refresh `
     -MsiPropertiesSource C:\approved\msi-properties.png
   ```

6. Update the inventory's source receipts, crop rectangles, published dimensions,
   byte sizes, and SHA-256 digests from the resulting files.
7. Review every page, alternative text, caption, and adjacent instruction. The
   task must remain understandable when images are unavailable and must not rely
   on color alone.
8. Build at narrow and wide widths in light and dark documentation themes. Check
   that figures remain inside the content column and comparison grids reflow to
   one column.
9. Run the documentation policy tests, site build, linkcheck, spelling check,
   text hygiene checks, and full Cargo merge gate.

Do not publish files directly from `target/`, capture live gameplay, include a
user profile path, or silently replace the maintainer-supplied image. If a new
reader question needs a visual, update the plan and inventory before adding it.
