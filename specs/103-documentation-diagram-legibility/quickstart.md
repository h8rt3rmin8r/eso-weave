# Quickstart: Verify Documentation Diagram Legibility

1. Run the focused receipt tests:

   ```bash
   node --test .github/scripts/docs-render-smoke.test.mjs
   ```

2. Build and validate the documentation:

   ```bash
   mdbook test docs
   mdbook build docs
   node .github/scripts/docs-policy.mjs docs target/docs-site/html
   ```

3. Run the generated-site browser evidence:

   ```bash
   node .github/scripts/docs-render-smoke.mjs --site target/docs-site/html
   ```

4. Confirm the output includes the S086 rendering sentinel, S088 figure sentinel, and the new S103 layout sentinel with no failures.

5. Inspect each normal and expanded diagram at 320 and 1280 CSS pixels in navy and light themes. Confirm expansion never makes a diagram narrower, tall assets scroll inside the bounded dialog, and every labeled branch can be followed from source to destination against the adjacent prose equivalent.
