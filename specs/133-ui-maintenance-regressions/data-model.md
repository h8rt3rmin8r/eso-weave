# Data Model

- UI preferences gain boolean log_panel_open, default false when absent/invalid, independent of height and verbosity.
- Model and GUI initialize from the same preference. Explicit toggle marks a layout save; graceful close flushes pending settings.
- Viewport constraint facts: usable screen rectangle, inner and outer rectangles and maximized flag. Growth requests are bounded, and main content can scroll when intrinsic height exceeds available client space.
- Migration schedule: last attempt instant plus last typed diagnostic identity. Unchanged failures retain retries without another warning; changed failure emits; success clears the failure and emits recovery. Root removal prunes state.
- Installed compatibility is based on bounded stable manifest bytes and current environment evidence. It does not change package ownership.
