# Plan 033: Bundled Offline Documentation

Status: Active

Sequence:

1. Generate the canonical mdBook during every production Cargo build and emit
   a deterministic immutable asset manifest.
2. Serve only that allow-listed corpus through a bounded loopback-only GET and
   HEAD service owned by the application lifetime.
3. Add Help > Documentation with native browser opening, stable reuse, and
   visible non-fatal failure handling.
4. Exercise the exact documentation toolchain and production embedding path on
   Windows and Linux CI and release jobs.
5. Document the offline access, trust boundary, release prerequisite, test
   evidence, and measured payload cost.

This slice closes issue #82. It does not include release field verification,
release creation, a webview, remote fallback, or application-state endpoints.
