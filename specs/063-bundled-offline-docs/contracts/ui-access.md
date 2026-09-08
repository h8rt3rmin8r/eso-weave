# Contract: Documentation Action

1. The main menu presents Help > Documentation with stable accessible text and help copy.
2. Activation asks the application-owned service for its stable root URL.
3. First activation starts the loopback service; later activations reuse it.
4. The platform opener receives only `http://127.0.0.1:<port>/eso-weave/`.
5. Opening is non-interactive and creates no Windows console window.
6. Start or launch failure becomes a visible notice and does not exit, freeze, or alter configuration.
7. Application shutdown owns and completes service shutdown.
