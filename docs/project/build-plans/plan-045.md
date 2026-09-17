# Plan 045: Local Extension Surface

Status: Active

Sequence:

1. S104 implements issue #175 by selecting the embedded HTTP and MCP stack and
   fixing the local listener, authentication, lifecycle, canonical state,
   database query, parity, resource-limit, and compatibility contracts.
2. S105 implements issue #176 by adding the persisted opt-in combined service
   lifecycle, discovery, status, authentication, and shared runtime host.
3. S106 implements issue #177 by projecting every canonical current
   player-state observation through the HTTP adapter and adding maintained
   source-inventory coverage.
4. S107 implements issue #178 by adding canonical resources to the mounted MCP
   adapter and proving transport parity against the same snapshot authority.
5. S108 implements issue #179 with one bounded read-only SQLite query service
   and parity across HTTP and MCP.
6. S109 implements issue #180 by publishing user and integrator documentation
   and closing the sequence with cross-surface end-to-end tests.

No production service ships in S104. Each implementation slice must retain the
loopback-only, authenticated, observation-only, bounded, and adapter-parity
boundaries accepted in ADR 0002.
