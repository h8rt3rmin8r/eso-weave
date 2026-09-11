# Research: Guided Documentation Screenshots

## Decisions

### Use dark-wide S083 captures

The wide 1280 by 900 canvas preserves labels and multi-column settings without shrinking text. One dark-theme set avoids visual churn while remaining readable in either manual theme because each PNG has its own opaque application background.

### Publish all seven deterministic scenes, but reuse them across pages

Each S083 scene answers a distinct issue criterion. Reusing healthy and unmanaged states on the PixelBeacon page avoids producing redundant bytes while keeping the reader path explicit.

The fixed 1280 by 900 capture canvas intentionally leaves room below shorter application layouts. A repository script crops only that empty lower canvas: six dashboard images become 1280 by 640, and the taller settings-modal image becomes 1280 by 820. The crop retains the full width and all task-relevant controls.

### Preserve the supplied MSI capture byte-for-byte

The maintainer explicitly requested initial publication as-is. It shows the real Windows Properties General tab and an emphasized Unblock control. The obscured path contains no readable account identifier. Provenance records its source hash without retaining a temporary local path.

### Use a synthetic SVG for the overlay

A live overlay capture would require ESO and could expose game or account content. A repository-authored schematic can show the top-left anchor, solid telemetry cells, and occlusion boundary while labeling itself synthetic. It uses no game art.

### Validate PNG metadata without another package

PNG width and height are big-endian integers at bytes 16 through 23 after the fixed signature and IHDR marker. Node's standard library can validate signature, dimensions, byte size, and SHA-256 without a new dependency.

### Keep prose sufficient without images

Markdown alt text names the visible state, captions explain why it matters, and adjacent steps repeat the actionable control or recovery. Images confirm recognition rather than becoming the sole instruction.

### Open the real settings modal for the Auto Potion ready capture

The original S083 ready fixture contained the configured model values but rendered only the dashboard. S084 uses the existing test-only `set_settings_open` seam and accessibility scroll action to expose the real Auto Potion settings group in that same deterministic scene. The blocked scene remains on the dashboard so its recovery reason stays visible. Production behavior is unchanged.

## Alternatives Rejected

- Publishing all 28 generated variants would add redundant package weight and maintenance surface.
- Taking live desktop or ESO screenshots would violate the isolated workflow and introduce private or game-owned pixels.
- Linking images from GitHub or another host would break bundled offline parity.
- Lossy image conversion would make provenance ambiguous and could soften small UI text.
- Adding image libraries only for metadata checks would expand the dependency surface unnecessarily.
