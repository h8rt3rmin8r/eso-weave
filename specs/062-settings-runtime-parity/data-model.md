# Data Model: Settings Runtime Parity

## EffectiveFishingConfig

Existing `FishingConfig` value with:

- `arm_timeout_ms`: inclusive 0 through 60,000.
- `reel_delay_ms`: inclusive 0 through 60,000.
- `recast_delay_ms`: inclusive 0 through 60,000.
- `interact_key`: one value from `Key::ALL`.

The same sanitized value is written to settings and installed in the controller.

## FishingConfigurationTransition

Inputs:

- current configuration;
- next effective configuration;
- requested flag;
- current state, pending deadline, and recovery state.

Rules:

- Equal configurations produce no state transition.
- A changed configuration replaces the value atomically.
- If requested or active, the transition clears the request, state, pending deadline, and recovery state without sink calls and records `SettingsChanged`.
- If already disabled and unrequested, only the configuration changes.

## LiveReaderConfig

Complete message value containing:

- `tolerance`;
- `interval_fishing_ms`;
- `interval_idle_ms`.

It deliberately excludes:

- `block_px`, retained from process startup;
- `heartbeat_timeout_ms`, an internal constant/configuration detail.

## ReaderConfigUpdatePort

- Sender owned by `AppModel`.
- Receiver owned by the existing Pixel Bus worker.
- Unbounded non-blocking sends.
- Worker drains available messages after wake; the newest complete value wins.
- Disconnect is terminal for live delivery but not for persisted restart recovery.

## ReaderApplicationTransition

- Interval-only update changes future poll selection and preserves observations.
- Tolerance update changes future decoding and invalidates cached safety observations before sampling.
- Running block size and heartbeat timeout remain equal to startup values.

## StagedGeometry

- Persisted requested `block_px`.
- Running `runtime_block_px` captured at startup.
- Managed add-on deployment outcome.
- Activation requires ESO reload or relog and ESO Weave restart.
