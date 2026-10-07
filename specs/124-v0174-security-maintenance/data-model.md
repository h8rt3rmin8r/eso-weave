# S124 Data Model

No persisted product model changes.

A scoped-update record contains package/action name, previous and selected target version, official source identity, source PR/head and compatibility outcome. Dependency lock entries retain registry checksum and dependency edges. The candidate record binds v0.17.4 to one source head, the three issues, six identity surfaces, automated results and review dispositions. Plan records retain chronological ID, status, destination, spec path and concrete evidence links.

Transitions: scoped -> resolved -> checked -> reviewed; candidate prepared -> official PR -> owner merge -> separately authorized publication. Original dependency PRs become superseded only after replacement merge. Plan049 becomes Complete/Archived and Plan050 becomes In Progress/Active. Existing credential, cursor, capture and storage schemas do not migrate.
