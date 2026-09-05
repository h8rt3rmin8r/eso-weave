# Contract: ESO Weave Delivery Project

## Identity

- Owner: `h8rt3rmin8r`
- Title: `ESO Weave Delivery`
- Visibility: Public
- Linked repository: `h8rt3rmin8r/eso-weave`

## Custom fields

### Stage

Single-select field with this exact order:

1. Backlog
2. Ready
3. Specced
4. In progress
5. PR review
6. Release verification
7. Done

### Slice

Text field containing a known spec-kit code such as `S044`. Leave blank when no reliable mapping exists.

Priority, Effort, Area, Type, and Milestone remain repository metadata and are not duplicated as custom fields.

## Views

- **Delivery table**: table layout for complete inventory and metadata inspection.
- **Delivery board**: board layout grouped by Stage in the defined order.

## Population

- Every issue in `h8rt3rmin8r/eso-weave` is included exactly once.
- Closed issues are Done.
- Open `needs: verification` issues are Release verification.
- S044 issues move from In progress to PR review when the pull request opens, then Done after merge.
- Other open issues use the most truthful current lifecycle state, with Backlog as the conservative default.
