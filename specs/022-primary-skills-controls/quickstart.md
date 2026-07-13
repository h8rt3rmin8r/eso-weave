# Quickstart: Primary and Skills Panel Controls (Manual Validation)

The visual behavior cannot be exercised headlessly, so it is validated against the
running app (use the PowerShell CopyFromScreen capture workaround to inspect the
window). The model Update intent and the strings hygiene tests are covered by
`cargo test`.

## Automated checks

```
cargo test --all --locked
```

Confirm:

- `update_beacon_intent_reinstalls` (new): install, apply Update, still installed
  and current.
- The strings hygiene/coverage tests still pass with the `Delay (ms)` header and
  the new Update tooltip.

## Manual validation (running app)

1. **Update button**: with the addon not installed, confirm Update is greyed and
   not pressable. Install the addon, then confirm Update is enabled; press it and
   confirm the addon remains installed and current. (US1, FR-001..FR-003.)
2. **Weapon Bar alignment**: confirm the Weapon Bar title lines up in the same
   column as Status, Fishing, and Pixel Beacon, and its state aligns with theirs.
   (US2, FR-004.)
3. **Weave dropdown width**: change a skill's Weave selection through Light Attack,
   Heavy Attack, Bash Attack, and Block Casting, and confirm the resting field
   width does not change and the rows below do not shift. (US3, FR-005.)
4. **Log level dropdown**: change the live-log level and confirm the field width
   does not change. (US3, FR-006.)
5. **Delay header and fields**: confirm the column header reads Delay (ms). For a
   row with Override off, confirm the delay shows the inherited value in a greyed,
   read-only, right-aligned four-digit field. Toggle Override on and confirm the
   field becomes editable at the same width and appearance (no row shift), edit the
   value, and confirm it applies. (US4, FR-007..FR-011.)
6. **Delay editing**: type into a delay field and confirm the value is not clobbered
   mid-edit; confirm non-digits are rejected, four digits max, and that 0 is
   accepted. (FR-011.)

## Regression

7. Confirm Install and Uninstall still behave as before (Uninstall still prompts to
   confirm; only a managed folder is removed). (FR-012.)
