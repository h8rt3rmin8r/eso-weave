# S123 Data Model

No runtime schema changes.

| Entity | Fields and relationships | Validation |
| --- | --- | --- |
| Candidate identity | version `0.17.3`, preparation date; package, lock, badge, bundled snapshot and capture fixture | One version/date, existing replacements and policy gates |
| Candidate history | Highlights and complete Added/Fixed/Decisions; previous releases | 1-6 bullets, at most 120 words; earlier history preserved |
| Plan lifecycle | Plan048 Complete/Archived; Plan049 In Progress/Active; spec/issue/PR references | Contiguous IDs, one active plan, valid destination/index and evidence |
| Integration receipt | candidate head, local checks, hosted checks, findings, requested rounds | All final-head required checks green, all findings answered, no unresolved threads, at most two requested rounds |

Candidate transitions: specified -> prepared locally -> official PR -> checks/reviews satisfied -> owner merge pending. Merge, tagging and publication are later operations. Plan048 completion is established by merged delivery evidence, independently of candidate publication.
