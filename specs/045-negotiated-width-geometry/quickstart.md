# Quickstart: Negotiated Width-Aware Pixel Geometry

## Focused protocol loop

1. Run the pixel-bus geometry and header tests:

   ```powershell
   cargo test --test pixelbus layout
   ```

2. Run addon contract and manifest tests:

   ```powershell
   cargo test --test beacon
   ```

3. Run settings and display tests:

   ```powershell
   cargo test --test app_settings
   cargo test --test pixelbus_display
   ```

## Full merge gate

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

## Expected evidence

- A 1024-pixel client at block size 32 publishes at least 32 columns, enough for
  three header cells and all 21 payload cells on row zero.
- Exact boundary widths wrap only the first cell that cannot fit.
- Corrupt or incompatible headers produce no payload events.
- A legacy magenta status cell selects the 16-column compatibility path.
- Steady samples prepare one extent; acquisition and growth prepare at most two.
- Settings text distinguishes negotiated, legacy, unavailable, and waiting.

## Deferred field proof

Do not close #44 from automated evidence. After a release, validate the matrix
there across Windows modes, display changes, all supported block sizes, and the
available X11/Proton path.
