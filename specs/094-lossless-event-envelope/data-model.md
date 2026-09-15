# Data Model: Lossless Subscribed-Event Envelope

## Encounter capture v2

The existing encounter metadata and normalized `events` remain. V2 adds:

- `raw_first_sequence`
- `raw_last_sequence`
- `raw_observation_count`
- `raw_omitted_observation_count`
- optional `raw_loss`
- ordered `raw_observations`

V1 omits all raw fields and remains valid through explicit version dispatch.

## Raw source observation

| Field | Rule |
| --- | --- |
| session_id, encounter_id | Equal the parent capture identity |
| sequence | Strictly increasing raw-source authority |
| monotonic_ms | Nondecreasing within the capture |
| api_version | Equal parent source API version |
| source_kind | callback, api-sample, or lifecycle |
| source_id | Stable project-owned source token |
| source_code | Callback event code when applicable |
| source_version | Positive project-owned signature version |
| argument_count | Number of input values |
| return_count | Number of output values |
| values | Contiguous one-based tagged values, inputs then outputs |

## Tagged raw value

Every descriptor has `position` and `value_type`.

- `nil`: no value field.
- `boolean`: one boolean field.
- `string`: one exact UTF-8 string field, never truncated.
- `number`: `sign` (-1 or 1), decimal integer `significand`, and signed binary
  `exponent`. The represented value is `sign * significand * 2^exponent`.
  Zero uses significand `0`; sign distinguishes negative zero.

## Raw loss

One range is sufficient because raw regular insertion stops after the first
unretainable observation. Fields are `missing_sequence_from`,
`missing_sequence_to`, and a controlled reason token: `record-limit`,
`byte-limit`, `string-limit`, `unsupported-value`, or `callback-failed`.
One observation contains at most 256 tagged values. A larger future callback is
omitted as one whole observation with `record-limit` evidence.

## Compatibility projection

Each v2 normalized event adds:

- `source_sequence`: the primary raw observation used to create it.
- `projection_ordinal`: contiguous zero-based order among facts from that source.

The required `encounter-start` boundary may reference raw sequence 1 when that
whole initial callback is inside the declared raw-loss range. This narrow case
keeps the partial envelope importable without fabricating raw evidence.

V1 normalized events omit both fields. Current metric algorithms continue to use
the normalized event sequence and payload.

## Store schema v2

`raw_encounters` retains the immutable canonical blob and indexed metadata and
adds `canonical_format_version`. V1 rows use format 1; v2 rows use format 2.
Capture schema and canonical format are validated per row. The singleton store
metadata records the latest writable schema and format.
