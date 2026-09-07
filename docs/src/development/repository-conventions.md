# Repository Conventions

ESO Weave is an Apache-2.0 single-crate Rust project. Operating-system backends
remain modules within that crate, and correctness-bearing logic is isolated behind
traits for deterministic testing.

Feature work uses GitHub issues and numbered `specs/NNN-name/` spec-kit packets
containing at least `spec.md`, `plan.md`, and `tasks.md`. Current maintainer
workflow and planning records live outside the published documentation tree so
they are not included in Pages or the future bundled manual.

The repository contains:

```text
.github/        CI workflows and repository policy checks
.specify/       Constitution, spec-kit scripts, and templates
addon/          Embedded PixelBeacon source
assets/         Application and package artwork
docs/src/       Canonical published documentation
packaging/      Windows and Linux package metadata
specs/          Per-slice specification records
src/            Rust application and platform modules
tests/          Integration tests
```

All text files use UTF-8 without a byte order mark and LF line endings. Project
governance also prohibits en and em dash characters.
