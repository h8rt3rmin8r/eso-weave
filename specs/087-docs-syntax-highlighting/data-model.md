# Data Model: Documentation Syntax Highlighting

## FenceRecord

- `page`: source-relative Markdown path
- `line`: one-based opening-fence line
- `language`: normalized identifier
- `content`: exact decoded source content between fences
- `sha256`: lowercase content digest
- `classification`: command, PowerShell, structured, or plain

Invariant: every source fence closes, has one approved identifier, and occurs once in the complete inventory.

## PlainException

- `page`
- `sha256`
- `rationale`

Invariant: the exception matches one current `text` fence exactly, carries a substantive rationale, and cannot authorize any other content.

## LocalGrammar

- `name`: project-local Highlight.js registration name
- `sourceLanguage`: `bash` or `powershell`
- `tokenRoles`: expected semantic roles
- `selector`: owned generated code nodes
- `marker`: idempotence marker

Invariant: registration reuses `globalThis.hljs`, touches only owned classes, runs once per node, and preserves exact text.

## SyntaxObservation

- `caseId`
- `surface`: generated-loopback
- `language`
- `theme`: navy, light, coal, ayu, or rust
- `viewportWidth`: 320 or 1280
- `sourceText`, `renderedText`, `selectedText`, `copyText`
- `semantic`
- `selectable`
- `codeOverflow`
- `pageContained`
- `tokenClasses`
- `tokenContrasts`
- `result`

Invariant: meaningful cases have their required roles and all observed token contrasts are at least 4.5:1; plain has no token spans; every case preserves source, rendered, selected, and copied text exactly, plus semantics and containment.

## RenderingReceipt

- `schemaVersion`
- existing S086 diagram evidence
- `syntaxObservations`
- `syntaxFailures`
- `syntaxSentinel`

Invariant: S087 success requires all 40 unique syntax matrix cells while preserving the complete S086 receipt contract.
