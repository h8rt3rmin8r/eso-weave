# Contract: Fishing Reconfiguration

1. Load and sanitize the complete submitted Fishing configuration.
2. Compare it with the controller's current complete configuration.
3. If equal, return without changing request, state, deadline, recovery, stop reason, or sink output.
4. If different, replace the configuration under the existing controller lock.
5. If Fishing is requested or active, clear request, pending deadline, transient recovery, and state; record `SettingsChanged`.
6. Never call a Fishing sink as part of reconfiguration.
7. Require a later explicit enable before any new configuration can schedule or synthesize work.

This transition applies identically when other safety gates are open or closed.
