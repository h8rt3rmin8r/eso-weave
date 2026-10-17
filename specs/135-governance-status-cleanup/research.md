# S135 Research

Date: 2026-10-17. Protected baseline: f230aa8.

## Decisions

- Reframe the #17-#47 governance inventory as a 2026-09-03 historical audit. Retain its statements as facts at that date and link current readers to native issue/Project authorities. Replacing it with another static current backlog would recreate the same drift.
- Correct current migration-ledger evidence summaries and the visualization-audit current-plan pointer. Preserve archived plans, completed packets, dated release observations and source snapshots.
- Keep evidence classifications such as verification-required and synthetic_fixture.proves_live_parity=false. Add separate lifecycle context to encounter-model.json and catalog-sources.json, because issue closure cannot supply absent source or field evidence. Replacing those labels with verified/none would change technical meaning and contradict the existing policy.
- Follow the constitution amendment procedure for the obsolete Sync Impact Report follow-ups: patch 5.1.0 to 5.1.1, add a metadata-only Sync Impact Report and preserve the previous report as history. All principle text and gates remain byte-identical. A silent edit or a policy revision would obscure what changed.
- Use existing documentation policy tests and an audit receipt. No new production checker or test that mirrors prose is warranted for reversible text maintenance. Constitution IV exempts this non-Rust commit from local Cargo gates; hosted checks still gate handoff.

## Observed evidence

GitHub reports old inventory issues #17/#21/#26-#30/#40-#47 Closed/Completed. #110/#129/#131/#190/#245 are Closed/Not planned on 2026-09-23; their closure comments explicitly withhold unperformed field evidence. #237 is Closed/Not planned as superseded by #245. #77 is Closed/Completed with an earlier operator observation recorded in its closure comment; this slice does not repeat or generalize it. Encounter implementation children #132-#136 are Closed/Completed.

PR #318 merged at f230aa8 on 2026-10-17. v0.19.1 publication and its five-asset audit are recorded in the merged S134 receipt; the release milestone is closed. Live API snapshots are kept ignored under target/.

Read-only research under the installed planning command identified the ledger and current-plan-pointer gaps. No unresolved technical choice remains.
