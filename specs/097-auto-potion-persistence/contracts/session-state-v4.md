# Contract: Session State Version 4

## Current shape

```json
{
  "schema_version": 4,
  "suspended": false,
  "fishing": false,
  "auto_potion": true,
  "api_version": {},
  "window": {
    "x": 120,
    "y": 64,
    "width": 820,
    "height": 900,
    "maximized": false
  }
}
```

`window` and cache members retain their existing omission rules. The
`auto_potion` field is always serialized by current code.

## Compatibility

- Versions 1 through 3 may omit `auto_potion`; omission means `false`.
- Missing files use the complete default and produce no notice.
- Invalid JSON or a present non-Boolean `auto_potion` uses the complete default
  and produces the existing invalid-session notice.
- Saving any loaded state emits schema version 4 through the model-owned current
  state snapshot.

## File integrity

- UTF-8 without BOM
- LF line endings
- Pretty JSON
- One trailing newline
