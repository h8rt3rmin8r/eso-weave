# Data Model: Foundations

Entities derived from the spec. Types are described in language-neutral terms;
concrete Rust types are an implementation detail of the tasks phase.

## Settings

The persisted user configuration. Serialized as the settings JSON file.

| Field | Type | Notes |
| --- | --- | --- |
| schema_version | integer | Current known value is 1. Gates forward migration. |
| logging | LoggingPrefs | The only settings category in this slice. |

Validation and rules:

- schema_version MUST be present. A value below the current version triggers
  forward migration; a value above the current version is loaded on a best effort
  with a warning notice (never a crash).
- Only user settings appear here. No timestamps, host metadata, session
  identifiers, or derived tables (FR-006).
- Unknown top-level keys are captured, a warning notice is raised, and they are not
  treated as an error (FR-008, FR-016).
- Default form: schema_version = 1, logging = LoggingPrefs default.

## LoggingPrefs

Operator-chosen logging preferences that legitimately belong to user settings.

| Field | Type | Notes |
| --- | --- | --- |
| level | LevelName | One of off, error, warn, info, debug, trace. Default info. |
| file_enabled | boolean | Whether the monthly file sink is on. Default false. |

## LevelName

Enumeration: off, error, warn, info, debug, trace. Serialized as a lowercase
string. Any other string is a validation error surfaced as a notice, and the field
falls back to its default.

## LogEvent

One structured record. Always flows to the ring buffer; also to the file when the
sink is enabled.

| Field | Type | Notes |
| --- | --- | --- |
| timestamp | UTC instant | Rendered RFC-3339 / ISO-8601 in output (FR-014). |
| level | LevelName | The event level (off is never an event level, only a filter). |
| target | string | The source module or subsystem. |
| message | string | The rendered message. Never contains input contents above debug (FR-015). |

## LogConfig (runtime state, not persisted directly)

Governs the live logging facility. Initialized from Settings.logging at startup;
runtime changes are persisted only on the normal settings save (FR-017).

| Field | Type | Notes |
| --- | --- | --- |
| level | reload handle over LevelName | Runtime-adjustable; effective immediately (FR-011). |
| file_enabled | boolean flag | Runtime-toggleable (FR-013). |
| buffer | bounded ring of LogEvent | Always on; capacity 1000; oldest evicted first (FR-012). |
| input_suppressed | boolean flag | Suppression control for future input events (FR-015). |

## Notice

A non-fatal condition surfaced to the caller from a Config Store operation.

| Field | Type | Notes |
| --- | --- | --- |
| kind | enumeration | For example corrupt_config, unknown_keys, migrated, unwritable. |
| message | string | Human-readable summary. Emitted as a warn-level log event (FR-018). |

## State transitions

- Settings schema migration: on load, if schema_version < current, apply ordered
  upgrade steps until it equals current, recording a migrated notice. In this slice
  the only known version is 1, so migration is the identity path plus the framework
  for later versions.
- Corrupt file: parse failure moves the file to a preserved name and yields default
  Settings plus a corrupt_config notice (FR-009).
- Unwritable location: read or write failure yields default Settings (in memory)
  plus an unwritable notice; startup still succeeds (FR-010).
