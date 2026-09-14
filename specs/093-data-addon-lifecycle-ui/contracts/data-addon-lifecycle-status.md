# Contract: Data Addon Lifecycle and Status

## Placement

`PixelBeacon Status` is followed immediately by `ESO Weave Data`. Data evidence
details precede `PixelBeacon Signal`.

## Lifecycle matrix

| Observation | Primary | Secondary | Removal | Remediation |
| --- | --- | --- | --- | --- |
| AddOns unavailable | none | none | none | configure a supported ESO AddOns location |
| missing | Install | none | none | install, then reload ESO if instructed |
| managed current | Repair | none | Uninstall | repair only when files are suspected damaged |
| managed outdated or drifted | Update | Repair | Uninstall | update or repair, then follow reload guidance |
| unmanaged | none | none | none | move or remove the exact unmanaged target manually |
| operation failed | prior safe actions | as prior | as prior | sanitized operation-specific recovery |

Every mutation rechecks ownership at execution time.

## Fact semantics

- Installed means the package directory exists in an inspected shape.
- Managed means the verified marker and exact inventory are present.
- Compatible means bytes and package version match the embedded build.
- Enabled is unconfirmed unless a supported account-specific source proves it.
- Loaded is unconfirmed because S093 has no supported same-session source and
  does not parse the optional last-flushed envelope.
- Runtime means ESO process observation only.
- Catalog and encounter remain separately unconfirmed and never claim live.
- Reload required derives from lifecycle outcome and conservative runtime evidence.

## Error contract

Errors identify the failed operation, preserve the last known observation,
avoid private absolute paths, and provide one safe next action. Unmanaged and
link errors never recommend automatic replacement.
