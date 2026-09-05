# Contract: Atomic Issue Lifecycle

## Outcome boundary

One actionable issue owns one independently closeable outcome. If two outcomes can be merged, released, verified, or rolled back at different times, they require separate issues.

An epic may group multiple outcomes only by linking child issues. The epic body summarizes scope and progress; it does not replace child issue acceptance criteria.

## Implementation and verification

Implementation completion means the code and repository acceptance criteria have merged. Release verification means a released build has produced the required real-world evidence. When those events can differ, use this sequence:

```text
implementation issue -> merge and close
release verification issue -> release evidence -> close
epic -> all required children closed -> close
```

The `needs: verification` label appears only on the issue currently waiting for issue-level field evidence.

## Pull request grouping

One pull request may close several issues when all outcomes share one coherent implementation and validation path. Each issue remains atomic and receives its own complete closing keyword.

## Post-merge housekeeping

After merge:

1. Confirm every referenced issue closed automatically.
2. Update Project Stage to Done for closed issues.
3. Keep release-verification children open until evidence exists.
4. Close an epic only after every required child outcome is complete.
5. Synchronize local `main` and remove the merged feature branch.
