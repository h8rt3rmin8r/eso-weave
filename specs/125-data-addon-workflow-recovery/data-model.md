# S125 Data Model

## SavedSourceInspection

Selected PathBuf and expected Channel, filesystem modification SystemTime, optional validated CaptureStateSummary, finished/partial terminal counts, and controlled readiness/error outcome. This is ephemeral UI state, never user configuration or a new database. Last saved information never asserts live activity.

## Controlled diagnostics

SourceUnavailable, SourceInaccessible, SourceInvalid, SourceUnsupported, SourceEnvironmentMismatch and StoreInvalid distinguish repair actions without raw error/payload disclosure. Existing catalog/detail categories remain unchanged. Empty and duplicate imports are successful outcomes, not errors.

## InspectSource request/result

Request includes generation, path and expected channel. Result preserves that identity; UI accepts only the selected generation/path/channel. Opening or changing selection clears stale presentation and queues inspection when the existing worker can accept it.

## User chat

Bootstrap-owned transient bounded pending replies with module prefix and system-chat readiness retry. No SavedVariables storage, no raw observation data, no capture authority. Overflow is explicitly reported and no unbounded queue is introduced.

## Existing records and acknowledgment

Capture/session/source identity and SQLite schema remain unchanged. Candidate acknowledgment continues to bind to the selected Live SHA-256; selection/file identity changes invalidate it. No implicit clear or supplier-authentication claim.
