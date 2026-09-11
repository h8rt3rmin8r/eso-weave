"use strict";

function commandGrammar(highlighter) {
  return {
    name: "ESO Weave command",
    contains: [
      highlighter.HASH_COMMENT_MODE,
      highlighter.QUOTE_STRING_MODE,
      highlighter.APOS_STRING_MODE,
      { className: "variable", begin: /\$\{?[A-Za-z_][A-Za-z0-9_]*\}?/ },
      { className: "attr", begin: /(?:^|\s)--?[A-Za-z][A-Za-z0-9-]*/ },
      { className: "number", begin: /\b\d+(?:\.\d+)?\b/ },
      { className: "string", begin: /(?:\.{1,2}[/\\]|[/\\])[A-Za-z0-9_.-]+(?:[/\\][A-Za-z0-9_.-]+)*/ },
      { className: "string", begin: /\b(?:INPUT|NEW|OLD|OUTPUT|PACKAGE|SAVED|STAGED|VERSION)(?:[A-Za-z0-9_.:/\\-]*)\b/ },
      { className: "string", begin: /\b(?:live|pts)\b/ },
      { className: "string", begin: /\b[A-Za-z0-9_.-]+(?:[/\\][A-Za-z0-9_.-]+)+\b/ },
      { className: "title", begin: /(?:^|\n)[A-Za-z0-9_.:/\\-]+/ },
      { className: "punctuation", begin: /[\\|&;=]/ },
    ],
  };
}

function powershellGrammar(highlighter) {
  return {
    name: "ESO Weave PowerShell",
    case_insensitive: true,
    keywords: {
      keyword: "begin break catch class continue data do dynamicparam else elseif end enum exit filter finally for foreach from function hidden if in param process return static switch throw trap try until using var while workflow",
      literal: "$false $null $true",
    },
    contains: [
      highlighter.HASH_COMMENT_MODE,
      highlighter.QUOTE_STRING_MODE,
      highlighter.APOS_STRING_MODE,
      { className: "variable", begin: /\$\{?[A-Za-z_][A-Za-z0-9_:]*\}?/ },
      { className: "title", begin: /(?:^|\n)(?:\.{0,2}[/\\])?[A-Za-z0-9_.-]+(?:[/\\][A-Za-z0-9_.-]+)*/ },
      { className: "string", begin: /(?:\.{1,2}[/\\]|[/\\])[A-Za-z0-9_.-]+(?:[/\\][A-Za-z0-9_.-]+)*/ },
      { className: "string", begin: /\b[A-Za-z]:\\[A-Za-z0-9_.-]+(?:\\[A-Za-z0-9_.-]+)*/ },
      { className: "string", begin: /\b(?:INPUT|NEW|OLD|OUTPUT|PACKAGE|SAVED|STAGED|VERSION)(?:[A-Za-z0-9_.:/\\-]*)\b/ },
      { className: "string", begin: /\b[A-Za-z0-9_.-]+(?:[/\\][A-Za-z0-9_.-]+)+\b/ },
      { className: "title", begin: /\b[A-Za-z]+-[A-Za-z][A-Za-z0-9-]*\b/ },
      { className: "attr", begin: /(?:^|\s)-[A-Za-z][A-Za-z0-9-]*/ },
      { className: "number", begin: /\b\d+(?:\.\d+)?\b/ },
      { className: "punctuation", begin: /[`|&;=,(){}[\]]/ },
    ],
  };
}

function highlightOwnedCode() {
  if (!globalThis.hljs) return;
  if (!globalThis.hljs.getLanguage("eso-command")) {
    globalThis.hljs.registerLanguage("eso-command", commandGrammar);
  }
  if (!globalThis.hljs.getLanguage("eso-powershell")) {
    globalThis.hljs.registerLanguage("eso-powershell", powershellGrammar);
  }
  for (const code of document.querySelectorAll("code.language-bash, code.language-powershell")) {
    if (code.dataset.esoHighlighted === "true") continue;
    const sourceLanguage = code.classList.contains("language-powershell") ? "powershell" : "bash";
    const localLanguage = sourceLanguage === "powershell" ? "eso-powershell" : "eso-command";
    const sourceText = code.textContent;
    code.esoSourceText = sourceText;
    code.textContent = sourceText;
    code.classList.remove("hljs", "bash", "powershell", `language-${sourceLanguage}`);
    code.classList.add(`language-${localLanguage}`);
    globalThis.hljs.highlightBlock(code);
    code.classList.remove(`language-${localLanguage}`);
    code.classList.add(`language-${sourceLanguage}`, localLanguage);
    code.dataset.esoHighlighted = "true";
  }
}

highlightOwnedCode();

const main = document.querySelector("main");
if (main) {
  main.id = "main-content";
  main.tabIndex = -1;
  const skipLink = document.createElement("a");
  skipLink.className = "eso-skip-link";
  skipLink.href = "#main-content";
  skipLink.textContent = "Skip to main content";
  skipLink.addEventListener("click", () => main.focus({ preventScroll: false }));
  document.body.prepend(skipLink);
}

for (const image of document.querySelectorAll(".docs-flow-diagram .img-wrapper > img")) {
  image.alt = "";
  image.setAttribute("aria-hidden", "true");
}
