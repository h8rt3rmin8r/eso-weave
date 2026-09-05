# Data Model: Roll-Dodge Safety

## RollDodgeState

Runtime-only enum owned by the pixel-bus protocol:

- `Unknown`: no valid current evidence; generated weaves are gated
- `Inactive`: positive evidence that no roll dodge is active
- `Active`: ability 28549 effect gained is inside its bounded active window

Transitions:

```text
startup -> Unknown
Unknown|Inactive -> Active (effect gained)
Active|Unknown -> Inactive (effect faded)
Active -> Inactive (1,500 ms watchdog)
any -> Unknown (death, deactivation, invalid sample, signal loss, process exit)
Unknown -> Inactive (completed activation baseline)
Unknown -> Inactive (in-place resurrection)
```

Duplicate observations are idempotent.

## B23 wire value

```text
G = 0xF9
R = 0x20 Unknown | 0x80 Inactive | 0xE0 Active
B = 255 - R
```

B23 exists only in negotiated protocol version 3. Versions 1 and 2 retain their
22-cell and 23-cell payload extents and never sample B23.

## Roll watchdog

The addon stores a nullable monotonic deadline. Effect gained sets it to current
game time plus 1,500 ms. Effect faded, death, deactivation, activation, and player
alive clear it. The fast tick changes Active to Inactive when the deadline is reached.

## Roll gate

InputEngine owns an atomic roll gate that defaults closed. It is closed for
Unknown and Active and open only for Inactive. WeaveEngine holds the typed state
for worker revalidation and presentation. RealSink receives a clone of the atomic
gate with the life gate and cancels new generated down events when either closes.
The engine records global cooldown only after the sink reports that at least one
generated down event was successfully emitted.

## RollDodgeView

| State | Text | Role | Generated weave |
| --- | --- | --- | --- |
| Unknown | Not detected | Warning | Blocked |
| Inactive | Inactive | Muted | Allowed by this gate |
| Active | Active | Warning | Blocked |
| Inactive game override | Game not active | Muted | Blocked by runtime |
