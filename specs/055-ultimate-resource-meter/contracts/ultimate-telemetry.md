# Contract: Ultimate Telemetry and Presentation

## Producer contract

- PixelBeacon is the sole producer.
- B25 through B28 exist only in negotiated protocol v5.
- Each scalar reserves two green markers for its high bit; red is the low byte
  and blue is its complement checksum.
- Current/max are sampled from the same `GetUnitPower` call.
- Front/back costs are sampled together with explicit hotbar categories. The
  high-frequency path samples the pool only; slot and hotbar events plus the
  one-second backstop refresh costs.
- While a special or temporary hotbar is active, both costs publish unavailable
  without changing the established weapon-bar signal.
- Invalid APIs, nil values, negative values, zero max, and zero costs publish the
  unavailable sentinel for the affected complete value.
- Activation rebaseline writes all four blocks before world Active is written.

## Transport contract

- A value is valid only when one field-specific marker and its checksum pass tolerance.
- 511 decodes to Unknown; 0 through 510 decode exactly to `Points(value)`.
- Protocol versions 1 through 4 do not sample or infer Ultimate.
- One changed aggregate emits one `PixelBusEvent::Ultimate`.
- Signal loss emits one unknown aggregate if a known value was cached.

## Consumer contract

- Routing stores Ultimate for presentation only.
- No Ultimate field enters `ResourceSet`, `ResourceWatch`, Auto Potion, weaving,
  input gates, timing, fishing, or cooldown behavior.
- The selected cost follows typed `ActiveBar`, not a display string.
- Unknown bar or selected cost hides threshold and Ready.
- Ready is exact `current >= selected_cost` and is never inferred from percentages.

## Visual contract

- Meter order is Health, Stamina, Magicka, Ultimate.
- All meter tracks contain subtle 25, 50, and 75 percent landmarks.
- The cost threshold is stronger, paints above landmarks, and protrudes three points.
- Ultimate readout is exact `current/maximum` when available.
- A fixed Ready allocation is immediately right of the numeric allocation.
- Ready is green, does not appear left of numbers, and never changes geometry.
- Accessibility exposes all known numeric and selection facts without color reliance.
