# Contract: Production Documentation Embedding

1. A release-profile Cargo build verifies `mdbook 0.5.4` and `mdbook-linkcheck2 0.13.0` exactly.
2. The build runs `mdbook build docs` against committed configuration and canonical sources.
3. Any tool, renderer, generation, traversal, duplicate-path, missing-index, or file-read failure stops the build.
4. The build inventories output, normalizes and sorts paths, derives media types, and emits immutable `include_bytes!` entries into `OUT_DIR`.
5. Cargo reruns generation when book configuration, source, theme, or build support changes.
6. Non-release profiles emit the same manifest API from a checked fixture without requiring mdBook.
7. Generated HTML remains ignored and is never staged.
8. CI and release jobs install exact tools before Windows and Linux release builds.
