# Data Model: Data Addon Lifecycle UI

## DataAddonObservation

One cached immutable observation.

| Field | Values | Provenance |
| --- | --- | --- |
| filesystem | unavailable, missing, present | resolved AddOns root and exact target inspection |
| ownership | managed, unmanaged, unknown | marker and exact inventory authority |
| compatibility | current, update available, unknown | embedded package comparison |
| configured enablement | enabled, disabled, unconfirmed | unconfirmed unless supported evidence exists |
| load evidence | unconfirmed | no supported same-session source |
| reload | required, not required, unknown | retained lifecycle outcome plus runtime evidence |
| runtime | available, unavailable, unknown | existing ESO runtime observation |
| catalog activity | unconfirmed | no supported live channel |
| encounter activity | unconfirmed | no supported live channel |
| failure | none or sanitized diagnostic | inspection or operation result |

## DataAddonView

- `lifecycle_line`: installed, managed, and compatibility summary only.
- `facts`: separate enablement, load, reload, runtime, catalog, and encounter
  text with provenance labels.
- `primary_action`: Install, Update, Repair, or none.
- `uninstall_enabled`: true only for a managed target.
- `remediation`: precise next safe step.

## State transitions

```text
missing --Install--> managed current
managed outdated --Update/Repair--> managed current
managed current --Repair--> managed current
managed current/outdated --confirmed Uninstall--> missing
unmanaged --any mutation--> rejected, unchanged
operation success + ESO running/unknown --> reload required
reload required + ESO stopped --> not required
inspection/operation failure --> prior snapshot retained + failure attached
```

S093 does not parse last-flushed module values. A future bounded reader may add
historical evidence, but it must never transition a current-session state.
