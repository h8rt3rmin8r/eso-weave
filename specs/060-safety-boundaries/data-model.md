# Data Model: Safety Boundaries

## SynthesisAuthorization

Shared, input-owned atomic state.

Fields:

- game inactive gate
- focus gate
- suspension gate
- menu gate
- life gate
- roll-dodge gate
- world gate
- travel gate
- monotonic invalidation epoch

Rules:

- Defaults are safe for startup: synthesis remains closed until required evidence opens its gates.
- Every safe-to-unsafe transition advances the epoch.
- Reopening a gate does not restore an earlier epoch.
- Reads never wait on a controller mutex.

## QueuedAction

Fields:

- action
- authorization epoch captured at classification

Rules:

- Application toggles remain deliverable so the operator can recover.
- Generated weave actions require current gate approval and exact epoch equality.
- A rejected queued action is discarded and never replayed.

## WeaveAdmission

Fields:

- admitted epoch
- cancellation flag
- generated keys currently held
- generated mouse buttons currently held
- whether a Down event was emitted

Rules:

- Any relevant closed gate or epoch mismatch makes cancellation sticky for the sequence.
- Cancellation rejects new Down events.
- Cancellation permits only matching Up events for held generated input.

## FishingAuthorizationState

Fields:

- requested enabled
- effective Fishing state
- optional deadline
- optional stop reason
- shared authorization projection

Suspension transitions:

```text
active/requested + suspend -> Disabled/requested/Suspended/no deadline
Disabled/requested + resume -> Disabled/requested/Suspended/no output
Disabled/requested + fresh FishingStarted + open gates -> Waiting/requested/no output
Disabled/requested + explicit off then on + open gates -> Armed/requested/cast
```

## PixelBeaconOwnership

States:

- Absent: target entry does not exist.
- ManagedCurrent: real directory, readable manifest, exact marker, current version.
- ManagedOutdated: real directory, readable manifest, exact marker, version differs or is missing.
- Unmanaged: any existing target that does not prove the managed contract.

Rules:

- A file, symbolic link, junction-like link, missing manifest, unreadable manifest, invalid UTF-8 manifest, or marker-free manifest is Unmanaged.
- Status inspection does not create or modify filesystem entries.

## PixelBeaconMutation

Operations:

- Install: Absent only.
- Refresh/Update: ManagedCurrent or ManagedOutdated only.
- Redeploy: ManagedCurrent or ManagedOutdated only.
- API Update: ManagedCurrent or ManagedOutdated only.
- Uninstall: ManagedCurrent or ManagedOutdated only.

Every operation re-evaluates ownership immediately before its first mutation.
