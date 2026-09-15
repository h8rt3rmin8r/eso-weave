# Contract: Stale Retention Setting

## JSON shape

```json
{
  "ui": {
    "stale_retention_seconds": 120
  }
}
```

The key is additive inside the existing opaque UI section.

## Load

- missing UI section or key: `120`, no notice
- integer 0 through 999: exact value, no notice
- negative, integer above 999, fraction, string, Boolean, array, object, or null: `120` plus one invalid-value notice

## Save

Save one JSON integer under `ui.stale_retention_seconds` through the existing settings form. No retained HUD values, causes, timestamps, or deadlines may be added to the file.

## Interface

Render `Stale retention` as a numeric whole-second control bounded to 0 through 999. It supports direct text entry and ordinary increment/decrement adjustment. Zero is labeled by help text as immediate clearing.
