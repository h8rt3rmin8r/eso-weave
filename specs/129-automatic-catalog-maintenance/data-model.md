# S129 Data Model

- DataMaintenancePreferences: catalog_disabled and native_processing_disabled typed booleans, default false, user settings only.
- PackagePresence: actual root/environment, compatible package version and capability set; separate managed ownership snapshot controls mutation.
- Eligibility: requested preferences plus per-context presence -> independent effective booleans; transition changes cancellation generation without resetting preferences.
- StartupConfig: bounded static catalog_disabled/channel literal, manager-owned file, applied by Lua on natural load; never live SavedVariables command.
- Collection: completed envelope retains actual channel/game/API/locale/source scope, exact supported records/parents and checksum; unfinished runtime work cannot replace it.
- Accumulated facts: entity identity + attributes/relationships/locales, semantic coverage and bounded accepted lineage; bounded absence is not deletion.
- AutomaticCatalogOutcome: Activated with verified immutable resolution or Unchanged, distinct from receipt persistence outcome.
- Processing state: stable complete input/content identities, bounded retries and cancellation; active/previous compatible generations remain accessible to current readers.

Transitions: unavailable/disabled -> dormant, eligible -> reconcile -> wait or build -> verify/open -> atomic selection -> consumer event. Failures retain last-good and back off. Disable cancels before commit; postcommit receipts cannot undo valid selection.
