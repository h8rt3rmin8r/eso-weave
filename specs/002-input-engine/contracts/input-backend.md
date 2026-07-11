# Contract: InputEngine Core and InputBackend Seam

Language-neutral. The tasks phase fixes exact Rust signatures.

## InputEngine (platform-agnostic core)

- `new(bindings, channel_capacity) -> (InputEngine, ActionReceiver)`
  - Creates the engine with a binding table and a bounded hand-off channel;
    returns the receiving half for the worker.
- `classify(&self, event: KeyEvent) -> Decision`
  - The single safety-critical decision, synchronous and non-blocking:
    - Self-originated event: pass (never suppress, never hand off).
    - Not focused: pass.
    - Key not bound: pass.
    - Bound but suspended and not suspend-exempt: pass.
    - Bound and active: suppress; on a new key-down (not auto-repeat) hand off one
      action via non-blocking `try_send`, dropping with a warning if the channel
      is full.
- `set_focused(bool)`, `set_suspended(bool)`, `is_suspended() -> bool`
- `bindings(&self) -> &BindingTable`, `rebind(action, key) -> Result<(), Conflict>`
- `load_bindings(&mut self, settings) -> Vec<Notice>` and
  `store_bindings(&self, settings)` integrate with the S001 Config Store.

## InputBackend (the OS seam)

- `synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError>`
  - Emits a key transition marked so the engine recognizes it as self-originated.
- `run(self, engine: Arc<InputEngine>) -> Result<(), InputError>`
  - Installs interception, feeds the engine's focused flag, and for each event
    calls `classify` and suppresses or passes accordingly. Returns an error if
    interception cannot start (FR-019).

Implementations:

- `MockBackend`: a test double. Feeds crafted `KeyEvent`s to `classify` and
  records synthesized output, so every safety-critical behavior is exercised
  without OS hooks (FR-018).
- Windows backend (`cfg(windows)`): low-level keyboard hook plus `SendInput`;
  injected-input flag maps to self-originated; raises timer resolution for the
  worker lifetime.
- Linux backend (`cfg(target_os = "linux")`): evdev grab for reading, uinput for
  synthesis; self-origin is structural (separate devices).

## Errors

- `InputError` covers a backend that cannot start interception (for example a
  Linux permission error) and synthesis failures. Never panics on the hook path.

## Safety invariants (must hold and be tested via MockBackend)

1. A self-originated event is never suppressed and never handed off (FR-009,
   FR-010).
2. Nothing is suppressed or handed off while unfocused (FR-005).
3. `classify` performs no blocking or timed work; it only reads state, looks up
   the binding, updates held-key state, and does one `try_send` (FR-006).
