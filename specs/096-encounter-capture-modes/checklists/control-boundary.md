# Control Boundary Checklist

**Purpose**: Prevent convenience controls from bypassing the S092 authority boundary
**Created**: 2026-09-15

- [x] CHK001 Capture enablement, disablement, and mode selection originate in ESO.
- [x] CHK002 The desktop command vocabulary remains empty.
- [x] CHK003 No custom action, binding manifest, or binding mutation is added.
- [x] CHK004 No synthesized key, slash-command typing, or native-action piggyback is added.
- [x] CHK005 No clipboard, URL, socket, packet, process-memory, or file-watch ingress is added.
- [x] CHK006 The desktop never writes live SavedVariables or owns requested capture state.
- [x] CHK007 `single` and `continuous` are the only modes; stopped is a state.
- [x] CHK008 One controller transition handles toggle-on and toggle-off semantics.
- [x] CHK009 High-frequency handlers exist only during active capture.
- [x] CHK010 Pixel Bus remains outside the bulk encounter path.
- [x] CHK011 The data addon remains unable to drive gameplay automation.
- [x] CHK012 Desktop presentation says last-saved or historical, never live control.
