# Status Truthfulness Checklist

- [x] Installed does not imply enabled
- [x] Enabled does not imply loaded
- [x] ESO running does not imply addon loaded
- [x] Loaded does not imply either module is collecting
- [x] Catalog and encounter activity remain separate
- [x] Optional SavedVariables disk evidence is not read for lifecycle status
- [x] Unsupported live facts remain unconfirmed rather than inferred from disk
- [x] Reload state is retained conservatively and not timer-cleared
- [x] Unknown runtime remains distinct from stopped runtime
- [x] Failures retain last known status and add a diagnostic
