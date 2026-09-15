# Contract: Raw Source Observation v2

```json
{
  "session_id": "session-...",
  "encounter_id": "encounter-...",
  "sequence": 1,
  "monotonic_ms": 0,
  "api_version": 101050,
  "source_kind": "callback",
  "source_id": "EVENT_PLAYER_COMBAT_STATE",
  "source_code": 2,
  "source_version": 1,
  "argument_count": 2,
  "return_count": 0,
  "values": [
    {"position": 1, "value_type": "number", "sign": 1, "significand": "2", "exponent": 0},
    {"position": 2, "value_type": "boolean", "boolean": true}
  ]
}
```

Structural objects reject unknown fields. Raw scalar values are open in value,
not shape. Callback values include the event code delivered at position one.
API-sample values list inputs first and outputs second, separated by the declared
counts. Source version 1 is the reviewed signature contract for S094. The v1
source contract permits at most 256 tagged values per observation. A larger
callback is rejected whole and declared as `record-limit` loss.

Exact strings are retained locally. Diagnostic text may identify a field or loss
reason but never reproduce a raw value.

Every compatibility projection resolves to its retained raw observation except
`encounter-start` when raw sequence 1 was rejected whole. That boundary keeps
source sequence 1 and the raw-loss range proves the observation is unavailable.
