# Quickstart: Review the Local Extension Contract

1. Read the accepted architecture decision and condensed implementer contract:

   ```text
   docs/project/architecture-decisions/0002-local-extension-stack-and-contract.md
   docs/project/local-extension-contract.md
   ```

2. Compare the complete state inventory in `contracts/player-state-v1.md` with `GameObservations`, `PixelBusReader`, `AppView`, fishing, auto-potion, weave configuration, and native binding sources.

3. Compare `contracts/database-query-v1.md` with the catalog and encounter schemas. Confirm exactly two runtime database identifiers and all current table families.

4. Confirm `contracts/service-lifecycle.md` assigns one implementable value to listener, routes, authentication, discovery, runtime ownership, startup, failure, stop, and application exit.

5. Run repository documentation, link, spelling, formatting, encoding, and mojibake gates. Confirm S104 changes no production dependency and exposes no listener.

6. Review dependent issues #176 through #180 and verify that each refers to the accepted contract without duplicating transport-specific state or query logic.
