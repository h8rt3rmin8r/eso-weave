import { spawn } from "node:child_process";
import { access, mkdtemp, readFile, realpath, rm, stat } from "node:fs/promises";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";

export const PASS_SENTINEL = "ESO_WEAVE_DIAGRAM_SMOKE_PASS_V1";
export const SYNTAX_PASS_SENTINEL = "ESO_WEAVE_SYNTAX_SMOKE_PASS_V1";
export const FIGURE_PASS_SENTINEL = "ESO_WEAVE_FIGURE_SMOKE_PASS_V1";

const DIAGRAMS = [
  { id: "S082-D01", page: "development/architecture.html", asset: "architecture-ownership.svg", alt: "Architecture ownership flow keeps physical input and observed game evidence separate until named consumers" },
  { id: "S082-D02", page: "concepts/action-authorization.html", asset: "action-authorization.svg", alt: "Action authorization flow requires every positive gate or fails closed without generated input" },
  { id: "S082-D03", page: "development/state-machines.html", asset: "safety-recovery.svg", alt: "Safety recovery flow closes gates before synchronization and reopens only after a coherent baseline" },
  { id: "S082-D04", page: "reference/pixel-bus-protocol.html", asset: "pixel-bus-validation.svg", alt: "Pixel Bus validation flow rejects invalid headers and layouts before independently decoding and publishing payload signals" },
];
const THEMES = ["navy", "light"];
const VIEWPORTS = [320, 1280];
const SYNTAX_CASES = [
  { id: "S087-COMMAND", page: "reference/encounter-data-and-metrics.html", language: "bash", selector: "code.language-bash", requiredTokens: ["hljs-title", "hljs-attr", "hljs-punctuation"] },
  { id: "S087-POWERSHELL", page: "development/screenshot-maintenance.html", language: "powershell", selector: "code.language-powershell", selectorIndex: 1, requiredTokens: ["hljs-title", "hljs-attr", "hljs-string", "hljs-punctuation"] },
  { id: "S087-JSON", page: "getting-started/troubleshooting.html", language: "json", probe: '{"schema_version": 1, "enabled": true}', requiredTokens: ["hljs-attr", "hljs-number", "hljs-literal"] },
  { id: "S087-PLAIN", page: "getting-started/troubleshooting.html", language: "text", selector: "code.language-text", requiredTokens: [] },
];
const SYNTAX_THEMES = ["navy", "light", "coal", "ayu", "rust"];
const FIGURE_CASES = [
  { id: "S088-DIAGRAM", page: "development/architecture.html", selector: "figure.docs-flow-diagram .docs-figure-trigger", alternative: DIAGRAMS[0].alt, captionRequired: false },
  { id: "S088-LANDSCAPE", page: "getting-started/first-launch.html", selector: "figure.docs-screenshot .docs-figure-trigger", alternative: "ESO Weave at first launch with ESO inactive and initial System and State information", captionRequired: true },
  { id: "S088-PORTRAIT", page: "getting-started/installation.html", selector: "figure.docs-screenshot--portrait .docs-figure-trigger", alternative: "Windows MSI Properties dialog with the Unblock checkbox selected and emphasized", captionRequired: true },
  { id: "S088-ILLUSTRATION", page: "features/pixelbeacon.html", selector: "figure.docs-screenshot--illustration .docs-figure-trigger", alternative: "Synthetic game-window diagram with PixelBeacon color blocks anchored at the top-left", captionRequired: true },
  { id: "S088-BRAND", page: "development/brand-standard.html", selector: "figure.brand-surface--dark .docs-figure-trigger", lightSelector: "figure.brand-surface--light .docs-figure-trigger", alternative: "ESO Weave full-color banner wordmark", lightAlternative: "ESO Weave full-color banner wordmark on light", captionRequired: true },
];
const FIGURE_THEMES = ["navy", "light"];

function observationKey(observation) {
  return `${observation.diagramId}|${observation.theme}|${observation.viewportWidth}|${observation.state}`;
}

export function validateObservation(observation) {
  const errors = [];
  const prefix = `${observation?.diagramId ?? "unknown"} ${observation?.theme ?? "unknown"} ${observation?.viewportWidth ?? "unknown"} ${observation?.state ?? "unknown"}`;
  if (observation?.surface !== "generated-loopback") errors.push(`${prefix}: observation surface must be generated-loopback`);
  if (!(observation?.naturalWidth > 0) || !(observation?.naturalHeight > 0) || !(observation?.renderedWidth > 0) || !(observation?.renderedHeight > 0)) {
    errors.push(`${prefix}: image decode and geometry must be positive`);
    return errors;
  }
  const naturalRatio = observation.naturalWidth / observation.naturalHeight;
  const renderedRatio = observation.renderedWidth / observation.renderedHeight;
  if (Math.abs(naturalRatio - renderedRatio) / naturalRatio > 0.015) {
    errors.push(`${prefix}: rendered aspect ratio differs from intrinsic geometry`);
  }
  if (!observation.visible) errors.push(`${prefix}: image is not visibly painted in its required state`);
  if (!observation.contained) errors.push(`${prefix}: image containment failed`);
  if (!(observation.opaqueCoverage >= 0.95)) errors.push(`${prefix}: opaque pixel coverage must be at least 95 percent`);
  if (!(observation.opaqueColorCount >= 4)) errors.push(`${prefix}: rendered image needs at least four opaque colors`);
  if (!(observation.nonBackgroundCoverage > 0.01)) errors.push(`${prefix}: rendered paint must differ meaningfully from the dominant background`);
  return errors;
}

export function validateRenderingReceipt(receipt) {
  const errors = [];
  if (receipt?.schemaVersion !== 1) errors.push("S086 rendering receipt schema is invalid");
  if (receipt?.sentinel !== PASS_SENTINEL) errors.push("S086 rendering receipt pass sentinel is missing");
  const observations = Array.isArray(receipt?.observations) ? receipt.observations : [];
  const expectedKeys = new Set();
  for (const diagram of DIAGRAMS) {
    for (const theme of THEMES) {
      for (const viewportWidth of VIEWPORTS) {
        for (const state of ["normal", "expanded"]) expectedKeys.add(`${diagram.id}|${theme}|${viewportWidth}|${state}`);
      }
    }
  }
  const actualKeys = new Set(observations.map(observationKey));
  if (observations.length !== expectedKeys.size || actualKeys.size !== expectedKeys.size || [...expectedKeys].some((key) => !actualKeys.has(key))) {
    errors.push(`S086 rendering matrix requires ${expectedKeys.size} unique observations`);
  }
  for (const observation of observations) errors.push(...validateObservation(observation));
  const requests = Array.isArray(receipt?.requests) ? receipt.requests : [];
  for (const diagram of DIAGRAMS) {
    const request = requests.find((candidate) => candidate.asset === diagram.asset);
    if (!request || request.status !== 200 || !/^image\/svg\+xml(?:\s*;|$)/iu.test(request.contentType ?? "")) {
      errors.push(`S086 ${diagram.asset} request requires status 200 and the SVG media type`);
    }
  }
  if (Array.isArray(receipt?.failures) && receipt.failures.length > 0) errors.push(...receipt.failures.map((failure) => `S086 browser: ${failure}`));
  return [...new Set(errors)];
}

function syntaxObservationKey(observation) {
  return `${observation.caseId}|${observation.theme}|${observation.viewportWidth}`;
}

export function validateSyntaxObservation(observation) {
  const errors = [];
  const prefix = `${observation?.caseId ?? "unknown"} ${observation?.theme ?? "unknown"} ${observation?.viewportWidth ?? "unknown"}`;
  if (observation?.surface !== "generated-loopback") errors.push(`${prefix}: observation surface must be generated-loopback`);
  if (!observation?.semantic) errors.push(`${prefix}: code block must retain pre and code semantics`);
  if (typeof observation?.sourceText !== "string" || observation?.renderedText !== observation.sourceText) {
    errors.push(`${prefix}: rendered text must exactly preserve source text`);
  }
  if (observation?.selectedText !== observation?.sourceText || observation?.copyText !== observation?.sourceText) {
    errors.push(`${prefix}: selected and copied text must exactly preserve source text (source ${observation?.sourceText?.length ?? "missing"}, selected ${observation?.selectedText?.length ?? "missing"}, copied ${observation?.copyText?.length ?? "missing"})`);
  }
  if (!observation?.selectable) errors.push(`${prefix}: code text must remain selectable`);
  if (!observation?.codeOverflow) errors.push(`${prefix}: code block must retain horizontal scrolling`);
  if (observation?.caseId === "S087-COMMAND" && observation?.viewportWidth === 320 && !observation?.actualOverflow) {
    errors.push(`${prefix}: the long command must produce code-local overflow at narrow width`);
  }
  if (!observation?.pageContained) errors.push(`${prefix}: syntax highlighting created page-level overflow`);
  const tokenClasses = Array.isArray(observation?.tokenClasses) ? observation.tokenClasses : [];
  const tokenContrasts = Array.isArray(observation?.tokenContrasts) ? observation.tokenContrasts : [];
  const tokenColors = Array.isArray(observation?.tokenColors) ? observation.tokenColors : [];
  if (observation?.caseId === "S087-PLAIN") {
    if (tokenClasses.length > 0 || tokenContrasts.length > 0 || tokenColors.length > 0) errors.push(`${prefix}: plain block must not contain syntax tokens`);
  } else {
    if (tokenClasses.length === 0) errors.push(`${prefix}: meaningful block requires syntax tokens`);
    const expected = SYNTAX_CASES.find((candidate) => candidate.id === observation?.caseId)?.requiredTokens ?? [];
    for (const token of expected) if (!tokenClasses.includes(token)) errors.push(`${prefix}: required token class is missing: ${token}`);
    if (tokenContrasts.length === 0 || tokenContrasts.some((ratio) => !(ratio >= 4.5))) {
      errors.push(`${prefix}: every syntax token requires at least 4.5:1 contrast`);
    }
    if (new Set(tokenColors).size < 3) errors.push(`${prefix}: meaningful syntax requires at least three distinct token colors`);
  }
  return errors;
}

export function validateSyntaxReceipt(receipt) {
  const errors = [];
  if (receipt?.syntaxSentinel !== SYNTAX_PASS_SENTINEL) errors.push("S087 syntax receipt pass sentinel is missing");
  const observations = Array.isArray(receipt?.syntaxObservations) ? receipt.syntaxObservations : [];
  const expectedKeys = new Set();
  for (const syntaxCase of SYNTAX_CASES) {
    for (const theme of SYNTAX_THEMES) {
      for (const viewportWidth of VIEWPORTS) expectedKeys.add(`${syntaxCase.id}|${theme}|${viewportWidth}`);
    }
  }
  const actualKeys = new Set(observations.map(syntaxObservationKey));
  if (observations.length !== 40 || actualKeys.size !== 40 || [...expectedKeys].some((key) => !actualKeys.has(key))) {
    errors.push("S087 syntax rendering matrix requires 40 unique observations");
  }
  for (const observation of observations) errors.push(...validateSyntaxObservation(observation));
  if (Array.isArray(receipt?.syntaxFailures) && receipt.syntaxFailures.length > 0) {
    errors.push(...receipt.syntaxFailures.map((failure) => `S087 browser: ${failure}`));
  }
  return [...new Set(errors)];
}

function figureObservationKey(observation) {
  return `${observation.caseId}|${observation.theme}|${observation.viewportWidth}`;
}

export function validateFigureObservation(observation) {
  const errors = [];
  const prefix = `${observation?.caseId ?? "unknown"} ${observation?.theme ?? "unknown"} ${observation?.viewportWidth ?? "unknown"}`;
  if (observation?.surface !== "generated-loopback") errors.push(`${prefix}: observation surface must be generated-loopback`);
  if (!observation?.triggerSemantic) errors.push(`${prefix}: figure trigger must be a native button`);
  if (observation?.triggerName !== `Expand image: ${observation?.alternative ?? ""}`) errors.push(`${prefix}: figure trigger accessible name is incorrect`);
  if (!observation?.visibleAffordance) errors.push(`${prefix}: figure trigger requires a persistent visible affordance`);
  if (observation?.dialogCount !== 1) errors.push(`${prefix}: document requires one shared dialog`);
  if (observation?.legacyModalCount !== 0) errors.push(`${prefix}: legacy or duplicate modal elements remain`);
  if (!observation?.modalOpen) errors.push(`${prefix}: figure must open as a native modal`);
  if (!observation?.closeFocused) errors.push(`${prefix}: close control must receive initial focus`);
  if (!observation?.backgroundInert) errors.push(`${prefix}: modal background must remain inert`);
  if (observation?.dialogLabel !== observation?.alternative) errors.push(`${prefix}: dialog label must equal the image alternative`);
  if (!(observation?.naturalWidth > 0) || !(observation?.naturalHeight > 0)
      || !(observation?.renderedWidth > 0) || !(observation?.renderedHeight > 0)) {
    errors.push(`${prefix}: expanded image decode and geometry must be positive`);
  } else {
    const naturalRatio = observation.naturalWidth / observation.naturalHeight;
    const renderedRatio = observation.renderedWidth / observation.renderedHeight;
    if (Math.abs(naturalRatio - renderedRatio) / naturalRatio > 0.015) errors.push(`${prefix}: expanded image aspect ratio differs from intrinsic geometry`);
  }
  if (observation?.upscaled) errors.push(`${prefix}: expanded image must not upscale beyond intrinsic dimensions`);
  if (!observation?.contained) errors.push(`${prefix}: expanded image containment failed`);
  if (observation?.captionRequired) {
    if (!observation?.captionAssociated) errors.push(`${prefix}: source and modal caption association is missing`);
    if (!(observation?.captionFontSize >= 14)) errors.push(`${prefix}: caption size must be at least 14 CSS pixels`);
    if (!(observation?.captionLineHeight >= observation?.captionFontSize * 1.5)) errors.push(`${prefix}: caption line height must be at least 1.5`);
    if (!(observation?.captionContrast >= 4.5)) errors.push(`${prefix}: caption contrast must be at least 4.5:1`);
    if (!observation?.captionContained) errors.push(`${prefix}: caption containment failed`);
    if (!(observation?.modalCaptionFontSize >= 14)) errors.push(`${prefix}: modal caption size must be at least 14 CSS pixels`);
    if (!(observation?.modalCaptionLineHeight >= observation?.modalCaptionFontSize * 1.5)) errors.push(`${prefix}: modal caption line height must be at least 1.5`);
    if (!(observation?.modalCaptionContrast >= 4.5)) errors.push(`${prefix}: modal caption contrast must be at least 4.5:1`);
    if (!observation?.modalCaptionContained) errors.push(`${prefix}: modal caption containment failed`);
  }
  if (observation?.caseId === "S088-BRAND" && observation?.brandSurface !== (observation.theme === "light" ? "light" : "dark")) {
    errors.push(`${prefix}: brand observation must cover the matching light or dark surface`);
  }
  if (!observation?.pageContained) errors.push(`${prefix}: figure system created page-level overflow`);
  return errors;
}

export function validateFigureReceipt(receipt) {
  const errors = [];
  if (receipt?.figureSentinel !== FIGURE_PASS_SENTINEL) errors.push("S088 figure receipt pass sentinel is missing");
  const observations = Array.isArray(receipt?.figureObservations) ? receipt.figureObservations : [];
  const expectedKeys = new Set();
  for (const figureCase of FIGURE_CASES) {
    for (const theme of FIGURE_THEMES) {
      for (const viewportWidth of VIEWPORTS) expectedKeys.add(`${figureCase.id}|${theme}|${viewportWidth}`);
    }
  }
  const actualKeys = new Set(observations.map(figureObservationKey));
  if (observations.length !== 20 || actualKeys.size !== 20 || [...expectedKeys].some((key) => !actualKeys.has(key))) {
    errors.push("S088 figure rendering matrix requires 20 unique observations");
  }
  for (const observation of observations) errors.push(...validateFigureObservation(observation));

  const expectedJourneys = new Set(["keyboard-enter-escape", "keyboard-space-escape", "keyboard-focus-cycle", "pointer-close-button", "pointer-backdrop"]);
  const journeys = Array.isArray(receipt?.figureJourneys) ? receipt.figureJourneys : [];
  const journeyIds = new Set(journeys.map((journey) => journey.id));
  if (journeys.length !== expectedJourneys.size || journeyIds.size !== expectedJourneys.size
      || [...expectedJourneys].some((id) => !journeyIds.has(id))) {
    errors.push("S088 figure receipt requires every pointer and keyboard journey");
  }
  for (const journey of journeys) {
    if (!journey.opened) errors.push(`S088 ${journey.id} journey did not open`);
    if (!journey.closed) errors.push(`S088 ${journey.id} journey did not close`);
    if (!journey.focusReturned) errors.push(`S088 ${journey.id} journey requires exact focus return`);
    if (journey.id === "keyboard-focus-cycle" && (!journey.tabForwardContained || !journey.tabReverseContained)) {
      errors.push(`S088 keyboard-focus-cycle journey requires forward and reverse sequential focus containment: ${JSON.stringify(journey)}`);
    }
  }
  const zoom = receipt?.figureZoom;
  if (!(zoom?.scale >= 1.99) || !zoom?.sourceCaptionWrapped || !zoom?.sourceCaptionContained
      || !zoom?.modalCaptionWrapped || !zoom?.modalCaptionContained || !zoom?.modalImageContained
      || !zoom?.captionAssociated || !(zoom?.sourceCaptionFontSize >= 14) || !(zoom?.modalCaptionFontSize >= 14)) {
    errors.push(`S088 figure receipt requires a contained, readable, associated 200 percent zoom observation: ${JSON.stringify(zoom)}`);
  }
  const print = receipt?.figurePrint;
  if (!print?.sourceImageVisible || !print?.sourceCaptionVisible || !print?.affordanceHidden
      || !print?.dialogHidden || !print?.legacyChromeHidden) {
    errors.push(`S088 figure receipt requires static print rendering without interactive chrome: ${JSON.stringify(print)}`);
  }
  const noScript = receipt?.figureNoScript;
  if (!noScript?.sourceImagesVisible || !noScript?.captionsVisible || !noScript?.interactiveChromeAbsent
      || !noScript?.legacyChromeHidden) {
    errors.push(`S088 figure receipt requires readable no-JavaScript rendering without interactive chrome: ${JSON.stringify(noScript)}`);
  }
  if (Array.isArray(receipt?.figureFailures) && receipt.figureFailures.length > 0) {
    errors.push(...receipt.figureFailures.map((failure) => `S088 browser: ${failure}`));
  }
  return [...new Set(errors)];
}

async function executableExists(candidate) {
  if (!candidate) return false;
  try {
    await access(candidate);
    return true;
  } catch {
    return false;
  }
}

export async function findBrowserExecutable(environment = process.env, platform = process.platform) {
  const explicit = environment.DOCS_CHROME_BIN;
  if (explicit) {
    if (await executableExists(explicit)) return explicit;
    throw new Error(`DOCS_CHROME_BIN does not exist: ${explicit}`);
  }
  const candidates = platform === "win32"
    ? [
        path.join(environment.PROGRAMFILES ?? "", "Google", "Chrome", "Application", "chrome.exe"),
        path.join(environment["PROGRAMFILES(X86)"] ?? "", "Microsoft", "Edge", "Application", "msedge.exe"),
        path.join(environment.LOCALAPPDATA ?? "", "Google", "Chrome", "Application", "chrome.exe"),
      ]
    : platform === "darwin"
      ? ["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"]
      : ["/usr/bin/google-chrome", "/usr/bin/google-chrome-stable", "/usr/bin/chromium", "/usr/bin/chromium-browser"];
  for (const candidate of candidates) if (await executableExists(candidate)) return candidate;
  throw new Error("No Chrome-compatible browser found. Set DOCS_CHROME_BIN to an absolute executable path.");
}

function contentType(filename) {
  switch (path.extname(filename).toLowerCase()) {
    case ".html": return "text/html; charset=utf-8";
    case ".css": return "text/css; charset=utf-8";
    case ".js": return "text/javascript; charset=utf-8";
    case ".svg": return "image/svg+xml";
    case ".png": return "image/png";
    case ".woff2": return "font/woff2";
    default: return "application/octet-stream";
  }
}

function browserObservationExpression(diagram, theme, viewportWidth) {
  const configuration = JSON.stringify({ diagram, theme, viewportWidth, sentinel: PASS_SENTINEL });
  return `(async () => {
const configuration = ${configuration};
const observations = [];
const failures = [];
function nextFrame() { return new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))); }
function isVisiblyPainted(element) {
  const rectangle = element.getBoundingClientRect();
  const style = getComputedStyle(element);
  return rectangle.width > 0 && rectangle.height > 0 && style.display !== "none" &&
    style.visibility !== "hidden" && style.visibility !== "collapse" && Number.parseFloat(style.opacity) > 0;
}

function paintStats(image) {
  const width = 160;
  const height = Math.max(1, Math.round(width * image.naturalHeight / image.naturalWidth));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  const context = canvas.getContext("2d", { willReadFrequently: true });
  context.drawImage(image, 0, 0, width, height);
  const pixels = context.getImageData(0, 0, width, height).data;
  let opaque = 0;
  const colors = new Map();
  for (let index = 0; index < pixels.length; index += 4) {
    if (pixels[index + 3] < 250) continue;
    opaque += 1;
    const color = (pixels[index] << 16) | (pixels[index + 1] << 8) | pixels[index + 2];
    colors.set(color, (colors.get(color) || 0) + 1);
  }
  const dominant = Math.max(0, ...colors.values());
  const total = width * height;
  return { opaqueCoverage: opaque / total, opaqueColorCount: colors.size, nonBackgroundCoverage: opaque === 0 ? 0 : (opaque - dominant) / opaque };
}
function observe(diagramId, state, image, boundary) {
  const rectangle = image.getBoundingClientRect();
  const style = getComputedStyle(image);
  const horizontalDecoration = parseFloat(style.paddingLeft) + parseFloat(style.paddingRight) + parseFloat(style.borderLeftWidth) + parseFloat(style.borderRightWidth);
  const verticalDecoration = parseFloat(style.paddingTop) + parseFloat(style.paddingBottom) + parseFloat(style.borderTopWidth) + parseFloat(style.borderBottomWidth);
  const tolerance = 1.5;
  return {
    diagramId,
    surface: "generated-loopback",
    theme: configuration.theme,
    viewportWidth: configuration.viewportWidth,
    state,
    naturalWidth: image.naturalWidth,
    naturalHeight: image.naturalHeight,
    renderedWidth: rectangle.width - horizontalDecoration,
    renderedHeight: rectangle.height - verticalDecoration,
    visible: isVisiblyPainted(image),
    contained: rectangle.left >= boundary.left - tolerance && rectangle.top >= boundary.top - tolerance && rectangle.right <= boundary.right + tolerance && rectangle.bottom <= boundary.bottom + tolerance,
    ...paintStats(image),
  };
}
try {
  document.documentElement.classList.remove("ayu", "coal", "light", "navy", "rust");
  document.documentElement.classList.add(configuration.theme);
  const figures = document.querySelectorAll("figure.docs-flow-diagram");
  const figure = figures.length === 1 ? figures[0] : null;
  const control = figure?.querySelector(":scope > p > button.docs-figure-trigger");
  const primary = control?.querySelector(":scope > img");
  const dialog = document.querySelector("dialog[data-docs-figure-dialog]");
  const expanded = dialog?.querySelector(".docs-figure-dialog__image");
  if (!figure || !control || !primary || !dialog || !expanded) throw new Error("expected shared figure dialog DOM is missing");
  if (figure.querySelectorAll(".checkbox-img, .img-wrapper").length !== 0) throw new Error("legacy diagram modal remains after enhancement");
  const expectedPath = "/eso-weave/assets/diagrams/" + configuration.diagram.asset;
  if (new URL(primary.currentSrc).pathname !== expectedPath) throw new Error("generated primary image source does not match the expected local asset");
  if (control.getAttribute("aria-label") !== "Expand image: " + configuration.diagram.alt) throw new Error("diagram trigger alternative is incorrect");
  await primary.decode();
  await nextFrame();
  if (!isVisiblyPainted(primary)) throw new Error("primary image is not visible before expansion");
  if (dialog.open || isVisiblyPainted(expanded)) throw new Error("expanded image is visible before activation");
  observations.push(observe(configuration.diagram.id, "normal", primary, figure.getBoundingClientRect()));
  control.click();
  await expanded.decode();
  await nextFrame();
  if (!dialog.matches(":modal") || !isVisiblyPainted(expanded)) throw new Error("expanded image did not become visible after activation");
  if (new URL(expanded.currentSrc).pathname !== expectedPath) throw new Error("expanded image source does not match the expected local asset");
  observations.push(observe(configuration.diagram.id, "expanded", expanded, { left: 0, top: 0, right: innerWidth, bottom: innerHeight }));
  if (expanded.alt !== "" || expanded.getAttribute("aria-hidden") !== "true") throw new Error("expanded clone is not decorative");
  dialog.close();
  await nextFrame();
} catch (error) {
  failures.push(configuration.diagram.id + ": " + error.message);
}
return { schemaVersion: 1, sentinel: failures.length === 0 ? configuration.sentinel : "FAILED", observations, failures };
})()`;
}

function syntaxObservationExpression(syntaxCase, theme, viewportWidth) {
  const configuration = JSON.stringify({ syntaxCase, theme, viewportWidth });
  return `(async () => {
const configuration = ${configuration};
const failures = [];
function nextFrame() { return new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))); }
function channels(value) {
  const match = value.match(/rgba?\\((\\d+),\\s*(\\d+),\\s*(\\d+)/u);
  return match ? match.slice(1).map(Number) : null;
}
function luminance(color) {
  const values = channels(color);
  if (!values) return NaN;
  const linear = values.map((channel) => {
    const value = channel / 255;
    return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2];
}
function contrast(foreground, background) {
  const first = luminance(foreground);
  const second = luminance(background);
  return (Math.max(first, second) + 0.05) / (Math.min(first, second) + 0.05);
}
try {
  document.documentElement.classList.remove("ayu", "coal", "light", "navy", "rust");
  document.documentElement.classList.add(configuration.theme);
  const light = document.getElementById("mdbook-highlight-css");
  const dark = document.getElementById("mdbook-tomorrow-night-css");
  const ayu = document.getElementById("mdbook-ayu-highlight-css");
  if (light) light.disabled = !["light", "rust"].includes(configuration.theme);
  if (dark) dark.disabled = !["navy", "coal"].includes(configuration.theme);
  if (ayu) ayu.disabled = configuration.theme !== "ayu";
  let code;
  if (configuration.syntaxCase.probe) {
    const pre = document.createElement("pre");
    code = document.createElement("code");
    code.className = "language-json";
    code.textContent = configuration.syntaxCase.probe;
    pre.append(code);
    document.querySelector("main").append(pre);
    globalThis.hljs.highlightBlock(code);
  } else {
    code = document.querySelectorAll(configuration.syntaxCase.selector)[configuration.syntaxCase.selectorIndex ?? 0];
  }
  if (!code) throw new Error("required code block is missing");
  await nextFrame();
  const withoutFenceTerminator = (value) => value.endsWith("\\n") ? value.slice(0, -1) : value;
  const sourceText = withoutFenceTerminator(configuration.syntaxCase.probe ?? code.esoSourceText ?? code.textContent);
  const renderedText = withoutFenceTerminator(code.textContent);
  const selection = getSelection();
  const range = document.createRange();
  range.selectNodeContents(code);
  selection.removeAllRanges();
  selection.addRange(range);
  const selectedText = withoutFenceTerminator(selection.toString());
  const copyText = withoutFenceTerminator(code.innerText);
  const selectable = selection.rangeCount === 1 && selectedText === sourceText && copyText === sourceText && getComputedStyle(code).userSelect !== "none";
  selection.removeAllRanges();
  const style = getComputedStyle(code);
  const spans = [...code.querySelectorAll("span[class*='hljs-']")];
  const tokenClasses = [...new Set(spans.flatMap((span) => [...span.classList].filter((name) => name.startsWith("hljs-"))))];
  const tokenColors = spans.map((span) => getComputedStyle(span).color);
  const tokenContrasts = tokenColors.map((color) => contrast(color, style.backgroundColor));
  return {
    observation: {
      caseId: configuration.syntaxCase.id,
      surface: "generated-loopback",
      language: configuration.syntaxCase.language,
      theme: configuration.theme,
      viewportWidth: configuration.viewportWidth,
      semantic: code.tagName === "CODE" && code.parentElement?.tagName === "PRE",
      sourceText,
      renderedText,
      selectedText,
      copyText,
      selectable,
      codeOverflow: ["auto", "scroll"].includes(style.overflowX),
      actualOverflow: code.scrollWidth > code.clientWidth + 1,
      pageContained: document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1,
      tokenClasses,
      tokenColors,
      tokenContrasts,
    },
    failures,
  };
} catch (error) {
  failures.push(configuration.syntaxCase.id + ": " + error.message);
  return { failures };
}
})()`;
}

function figureObservationExpression(figureCase, theme, viewportWidth) {
  const configuration = JSON.stringify({ figureCase, theme, viewportWidth });
  return `(async () => {
const configuration = ${configuration};
function nextFrame() { return new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))); }
function channels(value) {
  const match = value.match(/rgba?\\((\\d+),\\s*(\\d+),\\s*(\\d+)/u);
  return match ? match.slice(1).map(Number) : null;
}
function luminance(color) {
  const values = channels(color);
  if (!values) return NaN;
  const linear = values.map((channel) => {
    const value = channel / 255;
    return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  });
  return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2];
}
function contrast(foreground, background) {
  const first = luminance(foreground);
  const second = luminance(background);
  return (Math.max(first, second) + 0.05) / (Math.min(first, second) + 0.05);
}
function backgroundFor(element) {
  let current = element;
  while (current) {
    const color = getComputedStyle(current).backgroundColor;
    if (!/rgba\\([^,]+,[^,]+,[^,]+,\\s*0(?:\\.0+)?\\)/u.test(color) && color !== "transparent") return color;
    current = current.parentElement;
  }
  return getComputedStyle(document.documentElement).backgroundColor;
}
document.documentElement.classList.remove("ayu", "coal", "light", "navy", "rust");
document.documentElement.classList.add(configuration.theme);
const selector = configuration.theme === "light" && configuration.figureCase.lightSelector
  ? configuration.figureCase.lightSelector
  : configuration.figureCase.selector;
const alternative = configuration.theme === "light" && configuration.figureCase.lightAlternative
  ? configuration.figureCase.lightAlternative
  : configuration.figureCase.alternative;
const trigger = document.querySelector(selector);
if (!trigger) throw new Error("figure trigger is missing");
const figure = trigger.closest("figure");
const sourceImage = trigger.querySelector(":scope > img");
const sourceCaption = figure?.querySelector(":scope > figcaption") ?? null;
const dialog = document.querySelector("dialog[data-docs-figure-dialog]");
const close = dialog?.querySelector(".docs-figure-dialog__close");
const expanded = dialog?.querySelector(".docs-figure-dialog__image");
const modalCaption = dialog?.querySelector(".docs-figure-dialog__caption");
if (!figure || !sourceImage || !dialog || !close || !expanded || !modalCaption) throw new Error("figure system DOM is incomplete");
if (trigger.getAttribute("aria-label") !== "Expand image: " + alternative) throw new Error("figure trigger alternative is incorrect");
const affordanceStyle = getComputedStyle(trigger, "::after");
const visibleAffordance = affordanceStyle.display !== "none" && affordanceStyle.content.includes("Expand image");
trigger.click();
await expanded.decode();
await nextFrame();
const closeFocused = document.activeElement === close;
const backgroundControl = document.querySelector("main a, main button:not(.docs-figure-trigger)");
backgroundControl?.focus();
const backgroundInert = dialog.matches(":modal") && dialog.contains(document.activeElement);
close.focus({ preventScroll: true });
const rectangle = expanded.getBoundingClientRect();
const captionStyle = sourceCaption ? getComputedStyle(sourceCaption) : null;
const captionRectangle = sourceCaption?.getBoundingClientRect();
const modalCaptionStyle = !modalCaption.hidden ? getComputedStyle(modalCaption) : null;
const modalCaptionRectangle = !modalCaption.hidden ? modalCaption.getBoundingClientRect() : null;
const figureRectangle = figure.getBoundingClientRect();
const dialogRectangle = dialog.getBoundingClientRect();
const observation = {
  caseId: configuration.figureCase.id,
  surface: "generated-loopback",
  theme: configuration.theme,
  viewportWidth: configuration.viewportWidth,
  alternative,
  triggerSemantic: trigger.tagName === "BUTTON" && trigger.type === "button",
  triggerName: trigger.getAttribute("aria-label"),
  visibleAffordance,
  dialogCount: document.querySelectorAll("dialog[data-docs-figure-dialog]").length,
  legacyModalCount: document.querySelectorAll("input.checkbox-img, .img-wrapper").length,
  modalOpen: dialog.open && dialog.matches(":modal"),
  closeFocused,
  backgroundInert,
  dialogLabel: dialog.getAttribute("aria-label"),
  captionRequired: configuration.figureCase.captionRequired,
  captionAssociated: configuration.figureCase.captionRequired
    ? Boolean(sourceCaption && !modalCaption.hidden && dialog.getAttribute("aria-describedby") === modalCaption.id && modalCaption.textContent === sourceCaption.textContent)
    : !dialog.hasAttribute("aria-describedby") && modalCaption.hidden,
  naturalWidth: expanded.naturalWidth,
  naturalHeight: expanded.naturalHeight,
  renderedWidth: rectangle.width,
  renderedHeight: rectangle.height,
  contained: rectangle.left >= -1 && rectangle.top >= -1 && rectangle.right <= innerWidth + 1 && rectangle.bottom <= innerHeight + 1,
  upscaled: rectangle.width > expanded.naturalWidth + 1 || rectangle.height > expanded.naturalHeight + 1,
  captionFontSize: captionStyle ? Number.parseFloat(captionStyle.fontSize) : null,
  captionLineHeight: captionStyle ? Number.parseFloat(captionStyle.lineHeight) : null,
  captionContrast: captionStyle ? contrast(captionStyle.color, backgroundFor(sourceCaption)) : null,
  captionContained: captionRectangle ? captionRectangle.left >= figureRectangle.left - 1 && captionRectangle.right <= figureRectangle.right + 1 && captionRectangle.width > 0 && captionRectangle.height > 0 : null,
  modalCaptionFontSize: modalCaptionStyle ? Number.parseFloat(modalCaptionStyle.fontSize) : null,
  modalCaptionLineHeight: modalCaptionStyle ? Number.parseFloat(modalCaptionStyle.lineHeight) : null,
  modalCaptionContrast: modalCaptionStyle ? contrast(modalCaptionStyle.color, backgroundFor(modalCaption)) : null,
  modalCaptionContained: modalCaptionRectangle ? modalCaptionRectangle.left >= dialogRectangle.left - 1 && modalCaptionRectangle.right <= dialogRectangle.right + 1 && modalCaptionRectangle.width > 0 && modalCaptionRectangle.height > 0 : null,
  brandSurface: configuration.figureCase.id === "S088-BRAND" ? (figure.classList.contains("brand-surface--light") ? "light" : "dark") : null,
  pageContained: document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1,
};
close.click();
await nextFrame();
return observation;
})()`;
}

function figureJourneySetupExpression(figureCase) {
  const configuration = JSON.stringify({ figureCase });
  return `(() => {
const configuration = ${configuration};
const trigger = document.querySelector(configuration.figureCase.selector);
if (!trigger) return false;
trigger.focus({ preventScroll: true });
return document.activeElement === trigger;
})()`;
}

function figureJourneyStateExpression(figureCase) {
  const configuration = JSON.stringify({ figureCase });
  return `(() => {
const configuration = ${configuration};
const trigger = document.querySelector(configuration.figureCase.selector);
const dialog = document.querySelector("dialog[data-docs-figure-dialog]");
const close = dialog?.querySelector(".docs-figure-dialog__close");
return {
  opened: Boolean(dialog?.open && dialog.matches(":modal") && document.activeElement === close),
  closed: Boolean(dialog && !dialog.open),
  focusReturned: document.activeElement === trigger,
};
})()`;
}

function figurePointerPointExpression(figureCase, target) {
  const configuration = JSON.stringify({ figureCase, target });
  return `(() => {
const configuration = ${configuration};
const trigger = document.querySelector(configuration.figureCase.selector);
const dialog = document.querySelector("dialog[data-docs-figure-dialog]");
const close = dialog?.querySelector(".docs-figure-dialog__close");
const panel = dialog?.querySelector(".docs-figure-dialog__panel");
const center = (rectangle) => ({ x: rectangle.left + rectangle.width / 2, y: rectangle.top + rectangle.height / 2 });
if (configuration.target === "trigger") {
  trigger?.scrollIntoView({ block: "center", inline: "center" });
  return trigger ? center(trigger.getBoundingClientRect()) : null;
}
if (configuration.target === "close-button") return close ? center(close.getBoundingClientRect()) : null;
if (configuration.target !== "backdrop" || !dialog?.open || !panel) return null;
const rectangle = panel.getBoundingClientRect();
const candidates = [
  { x: 4, y: 4 },
  { x: innerWidth - 4, y: 4 },
  { x: 4, y: innerHeight - 4 },
  { x: innerWidth - 4, y: innerHeight - 4 },
];
return candidates.find(({ x, y }) => x < rectangle.left || x > rectangle.right || y < rectangle.top || y > rectangle.bottom) ?? null;
})()`;
}

function figureZoomExpression(figureCase) {
  const configuration = JSON.stringify({ figureCase });
  return `(async () => {
const configuration = ${configuration};
function nextFrame() { return new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))); }
const trigger = document.querySelector(configuration.figureCase.selector);
const figure = trigger?.closest("figure");
const sourceCaption = figure?.querySelector(":scope > figcaption");
const dialog = document.querySelector("dialog[data-docs-figure-dialog]");
const expanded = dialog?.querySelector(".docs-figure-dialog__image");
const modalCaption = dialog?.querySelector(".docs-figure-dialog__caption");
if (!trigger || !figure || !sourceCaption || !dialog || !expanded || !modalCaption) return null;
trigger.scrollIntoView({ block: "center", inline: "center" });
trigger.click();
await expanded.decode();
await nextFrame();
const viewport = visualViewport;
const sourceRectangle = sourceCaption.getBoundingClientRect();
const modalRectangle = modalCaption.getBoundingClientRect();
const imageRectangle = expanded.getBoundingClientRect();
const sourceStyle = getComputedStyle(sourceCaption);
const modalStyle = getComputedStyle(modalCaption);
const withinViewport = (rectangle) => rectangle.left >= viewport.offsetLeft - 1 && rectangle.top >= viewport.offsetTop - 1
  && rectangle.right <= viewport.offsetLeft + viewport.width + 1 && rectangle.bottom <= viewport.offsetTop + viewport.height + 1;
return {
  scale: viewport.scale,
  devicePixelRatio,
  sourceCaptionWrapped: sourceCaption.scrollWidth <= sourceCaption.clientWidth + 1 && sourceCaption.scrollHeight > Number.parseFloat(sourceStyle.lineHeight),
  sourceCaptionContained: sourceRectangle.width > 0 && sourceRectangle.height > 0 && document.documentElement.scrollWidth <= document.documentElement.clientWidth + 1,
  sourceCaptionFontSize: Number.parseFloat(sourceStyle.fontSize),
  modalCaptionWrapped: modalCaption.scrollWidth <= modalCaption.clientWidth + 1 && modalCaption.scrollHeight > Number.parseFloat(modalStyle.lineHeight),
  modalCaptionContained: withinViewport(modalRectangle),
  modalCaptionFontSize: Number.parseFloat(modalStyle.fontSize),
  modalImageContained: withinViewport(imageRectangle),
  captionAssociated: dialog.getAttribute("aria-describedby") === modalCaption.id && modalCaption.textContent === sourceCaption.textContent,
};
})()`;
}

function figurePrintExpression(figureCase) {
  const configuration = JSON.stringify({ figureCase });
  return `(() => {
const configuration = ${configuration};
const trigger = document.querySelector(configuration.figureCase.selector);
const figure = trigger?.closest("figure");
const sourceImage = trigger?.querySelector(":scope > img");
const sourceCaption = figure?.querySelector(":scope > figcaption");
const dialog = document.querySelector("dialog[data-docs-figure-dialog]");
const visible = (element) => {
  if (!element) return false;
  const rectangle = element.getBoundingClientRect();
  const style = getComputedStyle(element);
  return rectangle.width > 0 && rectangle.height > 0 && style.display !== "none" && style.visibility !== "hidden";
};
return {
  sourceImageVisible: visible(sourceImage),
  sourceCaptionVisible: visible(sourceCaption),
  affordanceHidden: trigger ? getComputedStyle(trigger, "::after").display === "none" : false,
  dialogHidden: dialog ? getComputedStyle(dialog).display === "none" : false,
  legacyChromeHidden: [...document.querySelectorAll(".checkbox-img, .img-wrapper")].every((element) => !visible(element)),
};
})()`;
}

function figureNoScriptExpression() {
  return `(async () => {
const meaningful = [...document.querySelectorAll("figure.docs-screenshot > img, figure.docs-flow-diagram .checkbox-img + img, .brand-surface__assets > img")]
  .filter((image) => image.getAttribute("alt")?.trim());
const captions = [...document.querySelectorAll("figure.docs-screenshot > figcaption, figure.brand-surface > figcaption")];
for (const checkbox of document.querySelectorAll(".docs-flow-diagram .checkbox-img")) checkbox.checked = true;
await Promise.race([
  Promise.all(meaningful.map((image) => image.decode().catch(() => undefined))),
  new Promise((resolve) => setTimeout(resolve, 2000)),
]);
await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
const visible = (element) => {
  const rectangle = element.getBoundingClientRect();
  const style = getComputedStyle(element);
  return rectangle.width > 0 && rectangle.height > 0 && style.display !== "none" && style.visibility !== "hidden";
};
return {
  sourceImagesVisible: meaningful.length > 0 && meaningful.every(visible),
  imageCount: meaningful.length,
  visibleImageCount: meaningful.filter(visible).length,
  captionsPresent: captions.length > 0,
  captionsVisible: captions.every(visible),
  interactiveChromeAbsent: document.querySelectorAll(".docs-figure-trigger, dialog[data-docs-figure-dialog]").length === 0,
  legacyChromeHidden: [...document.querySelectorAll(".checkbox-img, .img-wrapper")].every((element) => !visible(element)),
};
})()`;
}

async function startServer(siteRoot) {
  const diagramRequests = new Map();
  const resolvedRoot = await realpath(siteRoot);
  const server = createServer(async (request, response) => {
    try {
      if (request.method !== "GET" && request.method !== "HEAD") {
        response.writeHead(405, { Allow: "GET, HEAD", "Content-Type": "text/plain; charset=utf-8" });
        response.end("method not allowed");
        return;
      }
      const url = new URL(request.url ?? "/", "http://127.0.0.1");
      const relative = decodeURIComponent(url.pathname).replace(/^\/eso-weave\//u, "").replace(/^\/+/, "");
      if (relative.includes("\\") || relative.includes("\0")) throw new Error("ambiguous request path");
      const lexicalFilename = path.resolve(resolvedRoot, relative || "index.html");
      if (lexicalFilename !== resolvedRoot && !lexicalFilename.startsWith(`${resolvedRoot}${path.sep}`)) throw new Error("request escaped site root");
      const filename = await realpath(lexicalFilename);
      if (filename !== resolvedRoot && !filename.startsWith(`${resolvedRoot}${path.sep}`)) throw new Error("request escaped real site root");
      const metadata = await stat(filename);
      if (!metadata.isFile()) throw new Error("not a file");
      const type = contentType(filename);
      const body = await readFile(filename);
      response.writeHead(200, {
        "Content-Type": type,
        "Cache-Control": "no-store",
        "Content-Security-Policy": "default-src 'none'; style-src 'self'; script-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'none'; frame-src 'none'; object-src 'none'; base-uri 'none'; form-action 'none'",
        "X-Content-Type-Options": "nosniff",
      });
      response.end(request.method === "HEAD" ? undefined : body);
      const asset = DIAGRAMS.find((diagram) => relative.endsWith(`assets/diagrams/${diagram.asset}`))?.asset;
      if (asset) diagramRequests.set(asset, { asset, status: 200, contentType: type });
    } catch {
      response.writeHead(404, { "Content-Type": "text/plain; charset=utf-8" });
      response.end("not found");
    }
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  return { server, port: server.address().port, diagramRequests };
}

function launchBrowser(executable, profile) {
  const arguments_ = [
    "--headless=new",
    "--disable-background-networking",
    "--disable-component-update",
    "--disable-default-apps",
    "--disable-sync",
    "--no-first-run",
    "--force-device-scale-factor=1",
    "--run-all-compositor-stages-before-draw",
    `--user-data-dir=${profile}`,
    "--remote-debugging-port=0",
    "about:blank",
  ];
  return new Promise((resolve, reject) => {
    const child = spawn(executable, arguments_, { shell: false, windowsHide: true, stdio: ["ignore", "pipe", "pipe"] });
    let stderr = "";
    let settled = false;
    child.stderr.setEncoding("utf8");
    let timeout;
    const rejectAfterCleanup = (error) => {
      if (settled) return;
      settled = true;
      clearTimeout(timeout);
      void (async () => {
        if (child.exitCode === null) {
          const closed = new Promise((finish) => child.once("close", finish));
          child.kill();
          await Promise.race([closed, new Promise((finish) => setTimeout(finish, 2000))]);
        }
        reject(error);
      })();
    };
    timeout = setTimeout(() => rejectAfterCleanup(new Error(`browser debugging endpoint timed out: ${stderr.slice(-2000)}`)), 15000);
    child.stderr.on("data", (chunk) => {
      stderr += chunk;
      const webSocketUrl = stderr.match(/DevTools listening on (ws:\/\/[^\s]+)/u)?.[1];
      if (webSocketUrl && !settled) {
        settled = true;
        clearTimeout(timeout);
        resolve({ child, webSocketUrl });
      }
    });
    child.once("error", rejectAfterCleanup);
    child.once("close", (code) => {
      if (!settled) rejectAfterCleanup(new Error(`browser exited ${code}: ${stderr.slice(-2000)}`));
    });
  });
}

class DevToolsClient {
  constructor(socket) {
    this.socket = socket;
    this.nextId = 1;
    this.pending = new Map();
    this.waiters = new Map();
    socket.addEventListener("message", (event) => {
      const message = JSON.parse(event.data);
      if (message.id) {
        const pending = this.pending.get(message.id);
        this.pending.delete(message.id);
        clearTimeout(pending?.timeout);
        if (message.error) pending?.reject(new Error(message.error.message));
        else pending?.resolve(message.result);
        return;
      }
      const waiters = this.waiters.get(message.method) ?? [];
      this.waiters.delete(message.method);
      for (const waiter of waiters) {
        clearTimeout(waiter.timeout);
        waiter.resolve(message.params);
      }
    });
    const rejectOutstanding = () => {
      const error = new Error("browser DevTools connection closed");
      for (const pending of this.pending.values()) {
        clearTimeout(pending.timeout);
        pending.reject(error);
      }
      this.pending.clear();
      for (const waiters of this.waiters.values()) {
        for (const waiter of waiters) {
          clearTimeout(waiter.timeout);
          waiter.reject(error);
        }
      }
      this.waiters.clear();
    };
    socket.addEventListener("close", rejectOutstanding, { once: true });
    socket.addEventListener("error", rejectOutstanding, { once: true });
  }

  static async connect(url) {
    const socket = new WebSocket(url);
    await new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        socket.close();
        reject(new Error("browser DevTools connection timed out"));
      }, 15000);
      socket.addEventListener("open", () => {
        clearTimeout(timeout);
        resolve();
      }, { once: true });
      socket.addEventListener("error", (error) => {
        clearTimeout(timeout);
        reject(error);
      }, { once: true });
    });
    return new DevToolsClient(socket);
  }

  send(method, params = {}) {
    const id = this.nextId++;
    const result = new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pending.delete(id);
        reject(new Error(`browser DevTools command timed out: ${method}`));
      }, 15000);
      this.pending.set(id, { resolve, reject, timeout });
    });
    this.socket.send(JSON.stringify({ id, method, params }));
    return result;
  }

  waitFor(method) {
    return new Promise((resolve, reject) => {
      const waiter = { resolve, reject };
      waiter.timeout = setTimeout(() => {
        const remaining = (this.waiters.get(method) ?? []).filter((candidate) => candidate !== waiter);
        if (remaining.length > 0) this.waiters.set(method, remaining);
        else this.waiters.delete(method);
        reject(new Error(`browser DevTools event timed out: ${method}`));
      }, 15000);
      this.waiters.set(method, [...(this.waiters.get(method) ?? []), waiter]);
    });
  }

  close() {
    this.socket.close();
  }
}

async function pageDebuggerUrl(browserWebSocketUrl) {
  const endpoint = new URL(browserWebSocketUrl);
  const response = await fetch(`http://${endpoint.host}/json/list`);
  if (!response.ok) throw new Error(`browser target discovery failed with ${response.status}`);
  const targets = await response.json();
  const page = targets.find((target) => target.type === "page");
  if (!page?.webSocketDebuggerUrl) throw new Error("browser page target is missing");
  return page.webSocketDebuggerUrl;
}

export async function run(siteRoot) {
  const browser = await findBrowserExecutable();
  const profileRoot = await mkdtemp(path.join(tmpdir(), "eso-weave-docs-browser-"));
  const { server, port, diagramRequests } = await startServer(siteRoot);
  const receipt = {
    schemaVersion: 1,
    sentinel: PASS_SENTINEL,
    syntaxSentinel: SYNTAX_PASS_SENTINEL,
    browser: "pending",
    observations: [],
    requests: [],
    failures: [],
    syntaxObservations: [],
    syntaxFailures: [],
    figureSentinel: FIGURE_PASS_SENTINEL,
    figureObservations: [],
    figureJourneys: [],
    figureZoom: null,
    figurePrint: null,
    figureNoScript: null,
    figureFailures: [],
  };
  let browserProcess;
  let client;
  try {
    const launched = await launchBrowser(browser, profileRoot);
    browserProcess = launched.child;
    client = await DevToolsClient.connect(await pageDebuggerUrl(launched.webSocketUrl));
    await Promise.all([client.send("Page.enable"), client.send("Runtime.enable"), client.send("Network.enable")]);
    const version = await client.send("Browser.getVersion");
    receipt.browser = `${version.product} (${version.userAgent})`;
    for (const theme of THEMES) {
      for (const viewportWidth of VIEWPORTS) {
        await client.send("Emulation.setDeviceMetricsOverride", { width: viewportWidth, height: 920, deviceScaleFactor: 1, mobile: false });
        for (const diagram of DIAGRAMS) {
          const url = `http://127.0.0.1:${port}/eso-weave/${diagram.page}`;
          const loaded = client.waitFor("Page.loadEventFired");
          await client.send("Page.navigate", { url });
          await loaded;
          const evaluated = await client.send("Runtime.evaluate", {
            expression: browserObservationExpression(diagram, theme, viewportWidth),
            awaitPromise: true,
            returnByValue: true,
          });
          if (evaluated.exceptionDetails || !evaluated.result?.value) throw new Error(`browser evaluation failed for ${diagram.id} ${theme} ${viewportWidth}`);
          const partial = evaluated.result.value;
          receipt.observations.push(...(partial.observations ?? []));
          receipt.failures.push(...(partial.failures ?? []).map((failure) => `${theme} ${viewportWidth}: ${failure}`));
        }
      }
    }
    receipt.requests = [...diagramRequests.values()];
    for (const theme of SYNTAX_THEMES) {
      for (const viewportWidth of VIEWPORTS) {
        await client.send("Emulation.setDeviceMetricsOverride", { width: viewportWidth, height: 920, deviceScaleFactor: 1, mobile: false });
        for (const syntaxCase of SYNTAX_CASES) {
          const url = `http://127.0.0.1:${port}/eso-weave/${syntaxCase.page}`;
          const loaded = client.waitFor("Page.loadEventFired");
          await client.send("Page.navigate", { url });
          await loaded;
          const evaluated = await client.send("Runtime.evaluate", {
            expression: syntaxObservationExpression(syntaxCase, theme, viewportWidth),
            awaitPromise: true,
            returnByValue: true,
          });
          if (evaluated.exceptionDetails || !evaluated.result?.value) throw new Error(`browser syntax evaluation failed for ${syntaxCase.id} ${theme} ${viewportWidth}`);
          const partial = evaluated.result.value;
          if (partial.observation) receipt.syntaxObservations.push(partial.observation);
          receipt.syntaxFailures.push(...(partial.failures ?? []).map((failure) => `${theme} ${viewportWidth}: ${failure}`));
        }
      }
    }
    for (const theme of FIGURE_THEMES) {
      for (const viewportWidth of VIEWPORTS) {
        await client.send("Emulation.setDeviceMetricsOverride", { width: viewportWidth, height: 920, deviceScaleFactor: 1, mobile: false });
        for (const figureCase of FIGURE_CASES) {
          const url = `http://127.0.0.1:${port}/eso-weave/${figureCase.page}`;
          const loaded = client.waitFor("Page.loadEventFired");
          await client.send("Page.navigate", { url });
          await loaded;
          const evaluated = await client.send("Runtime.evaluate", {
            expression: figureObservationExpression(figureCase, theme, viewportWidth),
            awaitPromise: true,
            returnByValue: true,
          });
          if (evaluated.exceptionDetails || !evaluated.result?.value) throw new Error(`browser figure evaluation failed for ${figureCase.id} ${theme} ${viewportWidth}`);
          receipt.figureObservations.push(evaluated.result.value);
        }
      }
    }

    const navigateToFigure = async (figureCase) => {
      const loaded = client.waitFor("Page.loadEventFired");
      await client.send("Page.navigate", { url: `http://127.0.0.1:${port}/eso-weave/${figureCase.page}` });
      await loaded;
    };
    const evaluateValue = async (expression, awaitPromise = false) => {
      const evaluated = await client.send("Runtime.evaluate", { expression, awaitPromise, returnByValue: true });
      if (evaluated.exceptionDetails || evaluated.result?.value === undefined) throw new Error("browser figure journey evaluation failed");
      return evaluated.result.value;
    };
    const nextBrowserFrames = () => evaluateValue("new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve(true))))", true);
    const dispatchMouseClick = async ({ x, y }) => {
      await client.send("Input.dispatchMouseEvent", { type: "mouseMoved", x, y });
      await client.send("Input.dispatchMouseEvent", { type: "mousePressed", x, y, button: "left", buttons: 1, clickCount: 1 });
      await client.send("Input.dispatchMouseEvent", { type: "mouseReleased", x, y, button: "left", buttons: 0, clickCount: 1 });
    };
    await client.send("Emulation.setDeviceMetricsOverride", { width: 1280, height: 920, deviceScaleFactor: 1, mobile: false });

    for (const [id, figureCase, key, code, virtualKeyCode] of [
      ["keyboard-enter-escape", FIGURE_CASES[1], "Enter", "Enter", 13],
      ["keyboard-space-escape", FIGURE_CASES[0], " ", "Space", 32],
    ]) {
      await navigateToFigure(figureCase);
      const prepared = await evaluateValue(figureJourneySetupExpression(figureCase));
      if (!prepared) throw new Error(`${id} could not focus its figure trigger`);
      await client.send("Input.dispatchKeyEvent", { type: "keyDown", key, code, windowsVirtualKeyCode: virtualKeyCode });
      await client.send("Input.dispatchKeyEvent", { type: "keyUp", key, code, windowsVirtualKeyCode: virtualKeyCode });
      await nextBrowserFrames();
      const openedState = await evaluateValue(figureJourneyStateExpression(figureCase));
      await client.send("Input.dispatchKeyEvent", { type: "keyDown", key: "Escape", code: "Escape", windowsVirtualKeyCode: 27 });
      await client.send("Input.dispatchKeyEvent", { type: "keyUp", key: "Escape", code: "Escape", windowsVirtualKeyCode: 27 });
      await nextBrowserFrames();
      const closedState = await evaluateValue(figureJourneyStateExpression(figureCase));
      receipt.figureJourneys.push({ id, opened: openedState.opened, closed: closedState.closed, focusReturned: closedState.focusReturned });
    }

    await navigateToFigure(FIGURE_CASES[1]);
    if (!await evaluateValue(figureJourneySetupExpression(FIGURE_CASES[1]))) throw new Error("keyboard-focus-cycle could not focus its figure trigger");
    await client.send("Input.dispatchKeyEvent", { type: "keyDown", key: "Enter", code: "Enter", windowsVirtualKeyCode: 13 });
    await client.send("Input.dispatchKeyEvent", { type: "keyUp", key: "Enter", code: "Enter", windowsVirtualKeyCode: 13 });
    await nextBrowserFrames();
    const focusOpenedState = await evaluateValue(figureJourneyStateExpression(FIGURE_CASES[1]));
    await client.send("Input.dispatchKeyEvent", { type: "keyDown", key: "Tab", code: "Tab", windowsVirtualKeyCode: 9 });
    await client.send("Input.dispatchKeyEvent", { type: "keyUp", key: "Tab", code: "Tab", windowsVirtualKeyCode: 9 });
    const tabForwardContained = await evaluateValue('document.querySelector("dialog[data-docs-figure-dialog]").contains(document.activeElement)');
    await client.send("Input.dispatchKeyEvent", { type: "keyDown", key: "Tab", code: "Tab", windowsVirtualKeyCode: 9, modifiers: 8 });
    await client.send("Input.dispatchKeyEvent", { type: "keyUp", key: "Tab", code: "Tab", windowsVirtualKeyCode: 9, modifiers: 8 });
    const tabReverseContained = await evaluateValue('document.querySelector("dialog[data-docs-figure-dialog]").contains(document.activeElement)');
    await client.send("Input.dispatchKeyEvent", { type: "keyDown", key: "Escape", code: "Escape", windowsVirtualKeyCode: 27 });
    await client.send("Input.dispatchKeyEvent", { type: "keyUp", key: "Escape", code: "Escape", windowsVirtualKeyCode: 27 });
    await nextBrowserFrames();
    const focusClosedState = await evaluateValue(figureJourneyStateExpression(FIGURE_CASES[1]));
    receipt.figureJourneys.push({
      id: "keyboard-focus-cycle",
      opened: focusOpenedState.opened,
      closed: focusClosedState.closed,
      focusReturned: focusClosedState.focusReturned,
      tabForwardContained,
      tabReverseContained,
    });

    for (const [id, figureCase, action] of [
      ["pointer-close-button", FIGURE_CASES[2], "close-button"],
      ["pointer-backdrop", FIGURE_CASES[4], "backdrop"],
    ]) {
      await navigateToFigure(figureCase);
      const triggerPoint = await evaluateValue(figurePointerPointExpression(figureCase, "trigger"));
      if (!triggerPoint) throw new Error(`${id} could not locate its trigger hit target`);
      await dispatchMouseClick(triggerPoint);
      await nextBrowserFrames();
      const openedState = await evaluateValue(figureJourneyStateExpression(figureCase));
      const closePoint = await evaluateValue(figurePointerPointExpression(figureCase, action));
      if (!closePoint) throw new Error(`${id} could not locate its close hit target`);
      await dispatchMouseClick(closePoint);
      await nextBrowserFrames();
      const closedState = await evaluateValue(figureJourneyStateExpression(figureCase));
      receipt.figureJourneys.push({ id, opened: openedState.opened, closed: closedState.closed, focusReturned: closedState.focusReturned });
    }

    await client.send("Emulation.setDeviceMetricsOverride", { width: 1280, height: 920, deviceScaleFactor: 1, mobile: true });
    await navigateToFigure(FIGURE_CASES[1]);
    await client.send("Emulation.setPageScaleFactor", { pageScaleFactor: 2 });
    await nextBrowserFrames();
    receipt.figureZoom = await evaluateValue(figureZoomExpression(FIGURE_CASES[1]), true);
    await client.send("Emulation.setPageScaleFactor", { pageScaleFactor: 1 });
    await client.send("Emulation.setDeviceMetricsOverride", { width: 1280, height: 920, deviceScaleFactor: 1, mobile: false });

    await navigateToFigure(FIGURE_CASES[1]);
    await client.send("Emulation.setEmulatedMedia", { media: "print" });
    receipt.figurePrint = await evaluateValue(figurePrintExpression(FIGURE_CASES[1]));
    await client.send("Emulation.setEmulatedMedia", { media: "screen" });

    await client.send("Network.setBlockedURLs", { urls: ["*.js"] });
    await navigateToFigure(FIGURE_CASES[4]);
    const noScriptBrand = await evaluateValue(figureNoScriptExpression(), true);
    await navigateToFigure(FIGURE_CASES[0]);
    const noScriptDiagram = await evaluateValue(figureNoScriptExpression(), true);
    receipt.figureNoScript = {
      sourceImagesVisible: noScriptBrand.sourceImagesVisible && noScriptDiagram.sourceImagesVisible,
      captionsVisible: noScriptBrand.captionsPresent && noScriptBrand.captionsVisible,
      interactiveChromeAbsent: noScriptBrand.interactiveChromeAbsent && noScriptDiagram.interactiveChromeAbsent,
      legacyChromeHidden: noScriptDiagram.legacyChromeHidden,
      brand: noScriptBrand,
      diagram: noScriptDiagram,
    };
    await client.send("Network.setBlockedURLs", { urls: [] });

    const errors = [...validateRenderingReceipt(receipt), ...validateSyntaxReceipt(receipt), ...validateFigureReceipt(receipt)];
    if (errors.length > 0) throw new Error(errors.join("\n"));
    console.log(JSON.stringify(receipt, null, 2));
    console.log(PASS_SENTINEL);
    console.log(SYNTAX_PASS_SENTINEL);
    console.log(FIGURE_PASS_SENTINEL);
  } finally {
    if (client) {
      await client.send("Browser.close").catch(() => {});
      client.close();
    }
    if (browserProcess && browserProcess.exitCode === null) {
      await Promise.race([
        new Promise((resolve) => browserProcess.once("close", resolve)),
        new Promise((resolve) => setTimeout(resolve, 2000)),
      ]);
      if (browserProcess.exitCode === null) {
        browserProcess.kill();
        await new Promise((resolve) => browserProcess.once("close", resolve));
      }
    }
    await new Promise((resolve) => server.close(resolve));
    await rm(profileRoot, { recursive: true, force: true, maxRetries: 5, retryDelay: 100 });
  }
}

const invokedPath = process.argv[1] ? pathToFileURL(process.argv[1]).href : "";
if (import.meta.url === invokedPath) {
  const siteIndex = process.argv.indexOf("--site");
  const siteRoot = siteIndex >= 0 ? process.argv[siteIndex + 1] : "target/docs-site/html";
  await run(siteRoot);
}
