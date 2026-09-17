# Plan 045: Local Extension Surface

Status: Complete, Archived

Sequence:

1. S104 implemented issue #175 by selecting the embedded HTTP and MCP stack and
   fixing the local listener, authentication, lifecycle, canonical state,
   database query, parity, resource-limit, and compatibility contracts.
2. S105 implemented issue #176 by adding the persisted opt-in combined service
   lifecycle, discovery, status, authentication, and shared runtime host.
3. S106 implemented issue #177 by projecting every canonical current
   player-state observation through the HTTP adapter and adding maintained
   source-inventory coverage.
4. S107 implemented issue #178 by adding canonical resources to the mounted MCP
   adapter and proving transport parity against the same snapshot authority.
5. S108 implemented issue #179 with one bounded read-only SQLite query service
   and parity across HTTP and MCP.
6. S109 implemented issue #180 by publishing user and integrator documentation
   and closing the sequence with cross-surface end-to-end tests.

No production service shipped in S104. Every implementation slice retained the
loopback-only, authenticated, observation-only, bounded, and adapter-parity
boundaries accepted in ADR 0002. The sequence completed when
[PR #218](https://github.com/h8rt3rmin8r/eso-weave/pull/218) merged, closing issue
#180 and enabling epic #174 to close.
