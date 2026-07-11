# Contract: Weave Settings Sections

Additive `skills` and `timing` sections on the settings document (S001 Config
Store). Backward compatible: files without them load the defaults, no schema bump.

## Shape

```json
{
  "schema_version": 1,
  "logging": { "level": "info", "file_enabled": false },
  "bindings": { "skill1": "digit1", "...": "..." },
  "timing": {
    "global_cooldown": 500,
    "d_weave": 50,
    "d_heavy": 1000,
    "d_bash": 125
  },
  "skills": {
    "slot1": { "weave_type": "light_attack", "active": true, "d_weave": null, "d_heavy": null, "d_bash": null },
    "slot6": { "weave_type": "light_attack", "active": false }
  }
}
```

## Rules

- Absent `timing` or `skills`: defaults from master specification sections 7.3 and
  7.1 are used.
- A `timing` value that is missing or out of a sensible range falls back to its
  default with a notice.
- A slot's `weave_type` is one of `light_attack`, `heavy_attack`, `bash_attack`,
  `block_casting`; an unknown value falls back to the default with a notice.
- Per-slot overrides (`d_weave`, `d_heavy`, `d_bash`) are optional; null or absent
  means use the global default. Overrides irrelevant to the slot's type are
  ignored.
- The slot's key is not stored here; it comes from the S002 `bindings` section so
  the two stay consistent.
- On save, the full resolved `skills` and `timing` sections are written.
