# Contract: Pixel Bus Live Update

1. The GUI publishes one complete sanitized live subset after a meaningful change.
2. Publishing never blocks the GUI.
3. The worker waits for either its next deadline or an update.
4. On update, the worker drains immediately available updates and retains the newest complete value.
5. The worker merges that value into both reader decoding configuration and poll selection without altering startup block size or heartbeat timeout.
6. If tolerance changed, the worker closes and routes tolerance-dependent safety observations before collecting a fresh sample.
7. A disconnected receiver ends live update delivery cleanly; persisted configuration remains the next-start recovery path and diagnostics disclose the failure.
