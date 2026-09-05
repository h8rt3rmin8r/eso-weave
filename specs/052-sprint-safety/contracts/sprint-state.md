# Sprint-State Contract

- Protocol version: 4
- Payload length: 25 blocks
- Block: B9
- Marker: `0x43`
- `0x20`: `OnFoot`
- `0x60`: `Mounted`
- `0xA0`: `Sprinting` (keyboard-mode on foot only)
- `0xE0`: reserved mounted sprint, decoded as `Unknown`
- Entry debounce: 200 ms
- Ambiguous exit debounce: 200 ms
- Stale-positive watchdog: 1,500 ms
- Addon contract version: 19
- Consumer: auto-potion only
- Invalid marker, checksum, unsupported mounted sprint, signal loss, and absent block decode as `Unknown`.
