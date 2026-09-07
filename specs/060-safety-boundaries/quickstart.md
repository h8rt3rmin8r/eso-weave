# Quickstart: Safety Boundaries

## Focused red-green cycle

1. Add failing S060 tests for queue epochs, mid-sequence focus/suspend/menu closure, held-input release, and Fishing suspension.
2. Implement the shared authorization epoch and typed projections.
3. Add failing PixelBeacon target-shape, writer-refusal, UI action-matrix, and stale-intent tests.
4. Implement ownership inspection, guarded writers, distinct UI projection, and in-place managed update.
5. Update canonical docs and S059 coverage-policy records.

Focused commands:

```powershell
cargo test --locked --test input_engine s060_
cargo test --locked --test weave_engine s060_
cargo test --locked --test fishing s060_
cargo test --locked --test beacon s060_
cargo test --locked --test app_view_model s060_
```

## Full validation

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all --locked
node --test .github/scripts/docs-policy.test.mjs
mdbook test docs
mdbook build docs
node .github/scripts/docs-policy.mjs docs target/docs-site/html
typos docs/src docs/README.md README.md
git diff --check
```

## Manual review points

- No code on the interception callback blocks, sleeps, or synthesizes.
- A close-reopen cycle cannot revive previously queued work.
- Cancelling a running weave still releases held generated input.
- Unsuspending Fishing never causes an automatic cast.
- Every PixelBeacon writer refuses unproven ownership without changing target bytes.
- Unmanaged status gives persistent manual-resolution guidance and no lifecycle action.
