# Contract: Documentation Syntax Highlighting

## Source contract

The documentation corpus contains exactly:

| Classification | Identifier | Count | Runtime treatment |
| --- | --- | ---: | --- |
| Prompt-free command | `bash` | 12 | Local command grammar on bundled Highlight.js |
| PowerShell | `powershell` | 3 | Local PowerShell grammar on bundled Highlight.js |
| Deliberately plain | `text` | 8 | Bundled plaintext, exact allowlist only |

Every fence must close and carry one identifier. `console`, `sh`, unlabeled fences, unknown identifiers, additional plain blocks, and changed plain content fail policy.

## Runtime contract

The local script must:

1. detect the existing `globalThis.hljs` runtime;
2. register project-local command and PowerShell grammars when absent;
3. re-highlight only generated `language-bash` and `language-powershell` nodes;
4. mark processed nodes so a repeated invocation is a no-op;
5. retain the source language class and exact text content; and
6. make no network request and add no runtime dependency.

Command observations require executable, option, value, and punctuation roles where those forms occur. PowerShell observations require cmdlet, switch, variable, string, number, comment, keyword, and punctuation roles where present.

## Generated contract

- Each source fence maps in order to one `pre > code.language-IDENTIFIER` block.
- The output contains one hashed bundled Highlight.js runtime, the bundled theme palettes, and one hashed ESO Weave extension script and stylesheet.
- Release embedding requires these same generated local files.
- Raw HTML is not required to contain token spans because tokenization is a client-side mutation.

## Browser matrix

Cases:

- real multiline Bash command;
- real multiline PowerShell command;
- real Troubleshooting plain block; and
- generated-page JSON probe using the bundled grammar.

Each case runs in navy, light, coal, ayu, and rust at 320 and 1280 CSS pixels, producing 40 unique observations.

Every observation must prove semantic nesting, exact text, a selectable range, code-local horizontal overflow behavior, and no page-level overflow. Meaningful cases require the expected token classes and at least 4.5:1 contrast for each observed token. Plain requires zero token spans.

## Failure behavior

Any inventory, language, digest, asset, grammar, class, token, contrast, text, selection, semantics, or overflow mismatch fails with the case, page, theme, and viewport needed to reproduce it.
