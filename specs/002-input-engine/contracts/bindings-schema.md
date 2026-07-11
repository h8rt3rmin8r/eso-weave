# Contract: Bindings Settings Section

The additive `bindings` section on the settings document (S001 Config Store). It
is backward compatible: settings files written before this slice omit it and load
the default bindings, with no schema version bump.

## Shape

```json
{
  "schema_version": 1,
  "logging": { "level": "info", "file_enabled": false },
  "bindings": {
    "skill1": "digit1",
    "skill2": "digit2",
    "skill3": "digit3",
    "skill4": "digit4",
    "skill5": "digit5",
    "ultimate": "r",
    "synergy": "x",
    "toggle_suspend": "f1",
    "toggle_fishing": "f2"
  }
}
```

## Rules

- Keys of the object are action names; values are platform-neutral key
  identifiers.
- Absent section: the default table (master specification section 6.4) is used.
- The suspend-exempt flag is a property of the action (toggle_suspend and
  toggle_fishing are exempt), not stored per entry.
- A value that names an unknown key, or a table that maps two actions to the same
  key, is a conflict: the affected actions fall back to their defaults and a
  notice is raised (FR-017). The rest of the table still loads.
- Unknown action keys in the object are ignored with a notice, consistent with the
  Config Store unknown-key rule.
- On save, the full resolved table is written.
