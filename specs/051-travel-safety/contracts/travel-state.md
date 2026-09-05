# Travel-State Contract

- Protocol version: 4
- Payload length: 25 blocks
- Block: B24
- States: `Unknown`, `Inactive`, `Pending`
- Legacy layouts: v1 22 blocks, v2 23 blocks, v3 24 blocks
- Invalid marker, malformed value, unsupported layout, and signal loss decode as `Unknown`.
- Addon contract version: 18
- Consumers: input hook, weave worker and sink, fishing controller, auto-potion controller, application diagnostics, System and State UI.
