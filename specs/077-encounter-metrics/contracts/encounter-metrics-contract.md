# Contract: Versioned Encounter Metrics

## Library operation

`project_encounter(request)` accepts raw store, catalog, and output paths plus a
complete session and encounter identity. It returns the typed projection only
after canonical JSON has been atomically published. Paths must be distinct.

## Command

```text
catalog-compiler encounter-project \
  --store PATH \
  --catalog PATH \
  --output PATH \
  --session ID \
  --encounter ID
```

Success prints the projection as pretty JSON and exits zero. Failure writes a
redacted error to stderr, exits 2, and publishes nothing.

## Algorithm version `s069-v1`

- Duration: validated elapsed `ended_monotonic_ms`; never subtract the raw clock
  origin in `started_monotonic_ms`.
- DPS: local-player outgoing damage divided by duration seconds.
- HPS: saturated amount-minus-overflow local-player healing divided by duration.
- Damage share: local-player damage by positive ability ID divided by total.
- Uptime: clipped merged interval union by positive effect ability ID divided by duration.
- Casts: positive ability IDs in ascending authoritative sequence.
- Player source: combat source type 1.

All sums use checked integer arithmetic. Overflow rejects the projection.

## Quality and catalog receipt

Every valid discontinuity is copied into every encounter-wide result and sets
quality to `degraded`; no loss yields `complete`. The catalog channel and API
version must exactly match the capture. Effect IDs may resolve as catalog effect
or ability entities. Unknown IDs remain visible.

## Publication

Canonical JSON is compact UTF-8 with no BOM and one trailing newline. Publication
uses a verified sibling temporary file and atomic persistence. Link-like output,
directory, input alias, or failed replacement preserves the prior output.

No raw/catalog mutation, upload, telemetry, retention, UI, recommendation,
personal identity, or action authority is exposed.
