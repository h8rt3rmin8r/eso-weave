# Data Model: Persistent Auto Potion Request

## SessionState v4

| Field | Type | Default when absent | Authority |
| --- | --- | --- | --- |
| `schema_version` | unsigned integer | current version | Session document format |
| `suspended` | Boolean | `false` | Requested application suspension |
| `fishing` | Boolean | `false` | Requested Fishing enablement |
| `auto_potion` | Boolean | `false` | Requested Auto Potion enablement |
| `api_version` | object | empty cache | Derived API-version cache |
| `window` | object or absent | absent | Last window geometry |

### Invariants

- `auto_potion` represents request only. It does not prove eligibility or action.
- Absence in versions 1 through 3 maps to `false`.
- A present non-Boolean value invalidates the complete session document.
- Current saves always include the Boolean and schema version 4.
- Unrelated valid fields preserve their exact modeled values during load/save.

## AutoPotionController

The existing controller remains the runtime authority.

| State | Persisted | Restored |
| --- | --- | --- |
| Requested enablement | Yes | Through `set_enabled` |
| Effective state or blocker | No | Recomputed from fail-closed runtime state |
| Game, focus, signal, context, life, world, travel, movement evidence | No | Fresh runtime observations only |
| Resource and quickslot evidence | No | Fresh Pixel Bus observations only |
| Retry history and trigger cause | No | Fresh process state only |

## State transitions

```text
legacy or missing state
  -> request false
  -> controller Off

valid v4 request true
  -> set_enabled(true)
  -> Dormant or Blocked under startup defaults
  -> Ready or Triggered only after ordinary fresh evidence satisfies all gates

valid v4 request false
  -> set_enabled(false)
  -> Off

malformed state
  -> complete SessionState default plus notice
  -> request false
  -> controller Off
```
