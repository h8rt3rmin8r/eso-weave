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

function initializeDocumentationFigures() {
  if (document.documentElement.dataset.docsFigureSystem === "ready") return;

  const images = [...new Set([
    ...document.querySelectorAll(".docs-screenshot > img"),
    ...document.querySelectorAll(".docs-flow-diagram .checkbox-img + img"),
    ...document.querySelectorAll(".brand-surface__assets > img"),
  ])].filter((image) => image.getAttribute("alt")?.trim());
  if (images.length === 0) return;

  const dialog = document.createElement("dialog");
  dialog.className = "docs-figure-dialog";
  dialog.setAttribute("data-docs-figure-dialog", "true");

  const panel = document.createElement("div");
  panel.className = "docs-figure-dialog__panel";

  const header = document.createElement("div");
  header.className = "docs-figure-dialog__header";

  const title = document.createElement("p");
  title.className = "docs-figure-dialog__title";

  const closeButton = document.createElement("button");
  closeButton.className = "docs-figure-dialog__close";
  closeButton.type = "button";
  closeButton.textContent = "Close";
  closeButton.setAttribute("aria-label", "Close expanded image");

  const expandedImage = document.createElement("img");
  expandedImage.className = "docs-figure-dialog__image";
  expandedImage.alt = "";
  expandedImage.setAttribute("aria-hidden", "true");

  const caption = document.createElement("div");
  caption.className = "docs-figure-dialog__caption";
  caption.id = "docs-figure-dialog-caption";
  caption.hidden = true;

  header.append(title, closeButton);
  panel.append(header, expandedImage, caption);
  dialog.append(panel);
  document.body.append(dialog);

  let activeTrigger = null;

  function syncDialogViewport() {
    const viewport = globalThis.visualViewport;
    if (!viewport) return;
    dialog.style.setProperty("--docs-figure-viewport-width", `${viewport.width}px`);
    dialog.style.setProperty("--docs-figure-viewport-height", `${viewport.height}px`);
    dialog.style.setProperty("--docs-figure-viewport-left", `${viewport.offsetLeft}px`);
    dialog.style.setProperty("--docs-figure-viewport-top", `${viewport.offsetTop}px`);
  }

  function closeDialog() {
    if (dialog.open) dialog.close();
  }

  function openDialog(trigger, sourceImage, alternative, sourceCaption) {
    activeTrigger = trigger;
    dialog.setAttribute("aria-label", alternative);
    title.textContent = alternative;
    expandedImage.src = sourceImage.currentSrc || sourceImage.src;

    caption.replaceChildren();
    if (sourceCaption) {
      caption.append(...[...sourceCaption.childNodes].map((node) => node.cloneNode(true)));
      caption.hidden = false;
      dialog.setAttribute("aria-describedby", caption.id);
    } else {
      caption.hidden = true;
      dialog.removeAttribute("aria-describedby");
    }

    syncDialogViewport();
    dialog.showModal();
    closeButton.focus({ preventScroll: true });
  }

  closeButton.addEventListener("click", closeDialog);
  dialog.addEventListener("keydown", (event) => {
    if (event.key === "Tab") {
      event.preventDefault();
      closeButton.focus({ preventScroll: true });
      return;
    }
    if (event.key !== "Escape") return;
    event.preventDefault();
    closeDialog();
  });
  dialog.addEventListener("click", (event) => {
    if (event.target !== dialog) return;
    const bounds = panel.getBoundingClientRect();
    const outsidePanel = event.clientX < bounds.left || event.clientX > bounds.right
      || event.clientY < bounds.top || event.clientY > bounds.bottom;
    if (outsidePanel) closeDialog();
  });
  dialog.addEventListener("close", () => {
    expandedImage.removeAttribute("src");
    caption.replaceChildren();
    caption.hidden = true;
    dialog.removeAttribute("aria-describedby");
    const trigger = activeTrigger;
    activeTrigger = null;
    trigger?.focus({ preventScroll: true });
  });
  globalThis.visualViewport?.addEventListener("resize", syncDialogViewport);
  globalThis.visualViewport?.addEventListener("scroll", syncDialogViewport);

  for (const image of images) {
    const alternative = image.getAttribute("alt").trim();
    const figure = image.closest("figure");
    const sourceCaption = figure?.querySelector(":scope > figcaption") ?? null;
    const trigger = document.createElement("button");
    trigger.className = "docs-figure-trigger";
    trigger.type = "button";
    trigger.setAttribute("data-docs-figure-trigger", "true");
    trigger.setAttribute("aria-label", `Expand image: ${alternative}`);

    for (const className of image.classList) {
      if (className.startsWith("brand-asset--")) trigger.classList.add(className);
    }

    if (image.closest(".docs-flow-diagram")) {
      const generatedLabel = image.closest(".checkbox-label");
      if (generatedLabel) {
        image.remove();
        generatedLabel.replaceWith(trigger);
      } else {
        image.replaceWith(trigger);
      }
    } else {
      image.replaceWith(trigger);
    }

    image.alt = "";
    image.setAttribute("aria-hidden", "true");
    trigger.append(image);
    trigger.addEventListener("click", () => openDialog(trigger, image, alternative, sourceCaption));
    trigger.addEventListener("keydown", (event) => {
      if (event.key !== "Enter" && event.key !== " ") return;
      event.preventDefault();
      openDialog(trigger, image, alternative, sourceCaption);
    });
  }

  document.documentElement.dataset.docsFigureSystem = "ready";
}

initializeDocumentationFigures();
