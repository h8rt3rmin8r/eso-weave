# Contract: Diagram Rendering

## Source asset contract

Each of the four local SVG roots declares `width`, `height`, and `viewBox` with equal positive dimensions. The image contains an opaque full-canvas rectangle, readable labels, connector geometry, and multiple painted colors. Scripts, event handlers, animation, external resources, remote URLs, and runtime fonts are forbidden.

## Generated delivery contract

The mdBook build copies each source SVG byte-for-byte to its expected output path. Its page contains one `.docs-flow-diagram` with mdBook's checkbox, meaningful primary image, `.img-wrapper`, and decorative duplicate image. Both use the exact local asset path.

The primary image may use `width: 100%` inside its bounded figure. The expanded duplicate uses auto width and height plus mdBook's viewport constraints so its intrinsic ratio remains unchanged.

## Browser smoke contract

One loopback server serves the generated site under a restrictive content security policy. One directly spawned Chrome-compatible process runs headless, without a shell or visible Windows child window. It loads all four actual generated pages in navy and light at 320 and 1280 CSS pixels.

For each cell, a DevTools observation verifies the generated figure nesting and adjacency, waits for decode, and records:

- positive natural and rendered dimensions;
- computed visibility and the hidden-to-visible expanded-state transition;
- rendered ratio within a small tolerance of intrinsic ratio;
- normal wrapper or expanded viewport containment;
- at least 95 percent opaque raster coverage;
- at least four opaque RGB colors; and
- more than 1 percent pixels differing from the dominant background.

Every diagram request must return 200 and `image/svg+xml`. The smoke writes a versioned JSON receipt and a unique pass sentinel to standard output. Missing sentinel, browser failure, request mismatch, or failed observation fails the command.

## Accessibility contract

The primary image retains the established meaningful alt. The generated expanded clone is decorative (`alt=""` and hidden from accessibility APIs) because the adjacent text equivalent and primary name already communicate the content.

## Dependency and scope contract

The smoke uses only Node built-ins and a preinstalled browser selected by explicit override or known platform paths. It downloads nothing, performs no external request, and adds no production or npm dependency. General expansion interaction remains issue #155.
