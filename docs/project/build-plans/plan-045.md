# Plan 045: Local Extension Surface

Status: Active

Sequence:

1. S104 implements issue #175 by selecting the embedded HTTP and MCP stack and
   fixing the local listener, authentication, lifecycle, canonical state,
   database query, parity, resource-limit, and compatibility contracts.
2. S105 implements issue #176 by adding the persisted opt-in combined service
   lifecycle, discovery, status, authentication, and shared runtime host.
3. Issue #177 projects every canonical current player-state observation through
   the HTTP adapter and adds maintained source-inventory coverage.
4. Issue #178 adds canonical resources to the mounted MCP adapter and proves
   transport parity against the same snapshot authority.
5. Issue #179 adds one bounded read-only SQLite query service and parity across
   HTTP and MCP.
6. Issue #180 publishes user and integrator documentation and closes the sequence
   with cross-surface end-to-end tests.

No production service ships in S104. Each implementation slice must retain the
loopback-only, authenticated, observation-only, bounded, and adapter-parity
boundaries accepted in ADR 0002.
