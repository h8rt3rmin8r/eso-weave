# Addon Package Contract

## Exact inventory

The shipped ESO Weave addon source inventory is exactly:

```text
addon/PixelBeacon/
addon/EsoWeaveData/
├── EsoWeaveData.txt
├── EsoWeaveData.lua
├── Catalog.lua
└── Encounter.lua
```

The manifest loads the bootstrap first, then Catalog, then Encounter, and
declares only `EsoWeaveDataSaved`.

## Managed lifecycle

- Status is `NotInstalled`, `ManagedUpToDate`, `ManagedVersionMismatch`, or
  `Unmanaged`.
- The installed directory and every expected file must be non-link regular
  filesystem objects.
- The directory contains exactly the four expected files.
- Replacement and removal require the exact managed-marker line.
- Install and update stage all new bytes and rollback the complete prior managed
  file set if any commit step fails.
- Uninstall removes only the four verified files and their now-empty directory.
- No operation follows links or writes outside the resolved AddOns directory.
- PixelBeacon and neighboring addon bytes are invariant across every operation.

## Module activation

- Bootstrap initializes only the shared versioned root.
- Catalog owns `/ewcollect`, its combat/deactivation watchers, and its update
  namespace. Its high-frequency update exists only while running.
- Encounter owns `/ewencounter`, its combat/deactivation watchers, its capture
  event namespace, and its sampling update namespace. Capture handlers exist
  only while capturing.
- Module namespaces never reuse the package name alone for overlapping event
  registrations.

## Module-local clear

- `/ewcollect clear confirm` is accepted only while catalog collection is not
  running or paused and removes the catalog subtree; absence is its idle value.
- `/ewencounter clear confirm` retains its current guards and replaces only the
  encounter subtree with its idle value.
- The desktop exposes no delete operation for the shared SavedVariables file.
