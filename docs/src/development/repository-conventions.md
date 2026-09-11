# Repository Conventions

ESO Weave is an Apache-2.0 single-crate Rust project. Operating-system backends
remain modules within that crate, and correctness-bearing logic is isolated behind
traits for deterministic testing.

Feature work uses GitHub issues and numbered `specs/NNN-name/` spec-kit packets
containing at least `spec.md`, `plan.md`, and `tasks.md`. Current maintainer
workflow and planning records live outside the published documentation tree so
they are not included in Pages or the future bundled manual.

## Work-slice references

Published provenance uses only an uppercase `S` followed by exactly three
digits. For example, [S043](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/043-auto-potion-restoration/spec.md),
[S060](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/060-safety-boundaries/spec.md),
[S067](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/067-death-recovery-safety/spec.md),
and [S069](https://github.com/h8rt3rmin8r/eso-weave/blob/main/specs/069-encounter-model/spec.md)
link compact labels to their specifications.

Long test and source symbols are implementation evidence, not work-slice
references. Published developer pages describe the behavior and link to the
relevant source file. The source tree and the unpublished
[evidence manifest](https://github.com/h8rt3rmin8r/eso-weave/blob/main/docs/project/content-coverage.json)
retain all exact identifiers and stable representative relationships,
respectively. Fenced code samples may show literal syntax that does not follow
the prose convention because policy treats that boundary narrowly.

The lowercase encounter-metrics algorithm value shown in Architecture and
Encounter Data and Metrics is a persisted runtime identifier, not provenance.
It remains exact only on those pages.

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
