# Linux Key Capability Checklist

- [x] `Key::ALL` contains every variant exactly once.
- [x] All 13 application keys round-trip through Linux mappings.
- [x] E and F3 are explicitly covered as shipped defaults.
- [x] Both supported mouse buttons are advertised.
- [x] Physical-only key codes are retained in the virtual capability union.
- [x] An early app-only virtual device is upgraded before grab.
- [x] Only key events cross the key-only virtual forwarding boundary.
- [x] A forwarded-key emission failure is never ignored.
- [x] Tests require no real Linux input device.
