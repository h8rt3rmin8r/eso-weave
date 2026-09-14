# Quickstart: Verify Data Addon Lifecycle UI

1. Open System and State and verify ESO Weave Data immediately follows
   PixelBeacon.
2. With no package installed, verify Install is the only mutation offered.
3. Install while ESO is running or runtime is unknown and verify reload guidance
   appears without claiming enabled, loaded, or collecting.
4. Drift one managed package file, verify Update and Repair are offered, then
   confirm Repair restores the exact embedded package.
5. Replace the target with an unmanaged directory and verify every mutation is
   disabled and manual remediation is explicit.
6. Confirm uninstall uses a data-addon-specific dialog and leaves PixelBeacon
   and neighboring addons unchanged.
7. Open Catalog Update and verify capture/build controls remain while lifecycle
   mutation controls do not.
8. Run deterministic screenshot capture for both themes and viewport families.

## Required gates

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
node .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
```
