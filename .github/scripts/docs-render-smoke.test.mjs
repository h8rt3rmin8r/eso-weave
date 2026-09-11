import assert from "node:assert/strict";
import test from "node:test";

import {
  FIGURE_PASS_SENTINEL,
  PASS_SENTINEL,
  SYNTAX_PASS_SENTINEL,
  TABLE_PASS_SENTINEL,
  validateFigureObservation,
  validateFigureReceipt,
  validateObservation,
  validateRenderingReceipt,
  validateSyntaxObservation,
  validateSyntaxReceipt,
  validateTableObservation,
  validateTableReceipt,
} from "./docs-render-smoke.mjs";

const validObservation = {
  diagramId: "S082-D01",
  surface: "generated-loopback",
  theme: "navy",
  viewportWidth: 320,
  state: "normal",
  naturalWidth: 400,
  naturalHeight: 650,
  renderedWidth: 280,
  renderedHeight: 455,
  visible: true,
  contained: true,
  opaqueCoverage: 1,
  opaqueColorCount: 12,
  nonBackgroundCoverage: 0.15,
};

test("S086 accepts a decoded, proportionate, contained, nonblank observation", () => {
  assert.deepEqual(validateObservation(validObservation), []);
});

test("S086 rejects blank, collapsed, distorted, and clipped observations", () => {
  assert.match(validateObservation({ ...validObservation, naturalWidth: 0 }).join("\n"), /decode/i);
  assert.match(validateObservation({ ...validObservation, renderedHeight: 300 }).join("\n"), /aspect ratio/i);
  assert.match(validateObservation({ ...validObservation, visible: false }).join("\n"), /visibly painted/i);
  assert.match(validateObservation({ ...validObservation, contained: false }).join("\n"), /containment/i);
  assert.match(validateObservation({ ...validObservation, opaqueCoverage: 0.5 }).join("\n"), /opaque/i);
  assert.match(validateObservation({ ...validObservation, opaqueColorCount: 1 }).join("\n"), /colors/i);
  assert.match(validateObservation({ ...validObservation, nonBackgroundCoverage: 0.001 }).join("\n"), /background/i);
  assert.match(validateObservation({ ...validObservation, surface: "other" }).join("\n"), /surface/i);
});

test("S086 requires the complete receipt, SVG media types, and pass sentinel", () => {
  const observations = [];
  for (const diagramId of ["S082-D01", "S082-D02", "S082-D03", "S082-D04"]) {
    for (const theme of ["navy", "light"]) {
      for (const viewportWidth of [320, 1280]) {
        for (const state of ["normal", "expanded"]) {
          observations.push({ ...validObservation, diagramId, theme, viewportWidth, state });
        }
      }
    }
  }
  const receipt = {
    schemaVersion: 1,
    sentinel: PASS_SENTINEL,
    observations,
    requests: ["architecture-ownership.svg", "action-authorization.svg", "safety-recovery.svg", "pixel-bus-validation.svg"].map((asset) => ({ asset, status: 200, contentType: "image/svg+xml" })),
  };
  assert.deepEqual(validateRenderingReceipt(receipt), []);
  assert.match(validateRenderingReceipt({ ...receipt, sentinel: "wrong" }).join("\n"), /sentinel/i);
  assert.match(validateRenderingReceipt({ ...receipt, observations: observations.slice(1) }).join("\n"), /matrix/i);
  assert.match(validateRenderingReceipt({ ...receipt, observations: [...observations, observations[0]] }).join("\n"), /matrix/i);
  assert.match(validateRenderingReceipt({ ...receipt, requests: [{ asset: "architecture-ownership.svg", status: 200, contentType: "text/plain" }] }).join("\n"), /media type|request/i);
});

const validSyntaxObservation = {
  caseId: "S087-COMMAND",
  surface: "generated-loopback",
  language: "bash",
  theme: "navy",
  viewportWidth: 320,
  semantic: true,
  sourceText: "eso-catalog compile --input PATH",
  renderedText: "eso-catalog compile --input PATH",
  selectedText: "eso-catalog compile --input PATH",
  copyText: "eso-catalog compile --input PATH",
  selectable: true,
  codeOverflow: true,
  actualOverflow: true,
  pageContained: true,
  tokenClasses: ["hljs-title", "hljs-attr", "hljs-string", "hljs-punctuation"],
  tokenColors: ["rgb(141, 199, 255)", "rgb(215, 184, 255)", "rgb(127, 224, 207)", "rgb(230, 237, 243)"],
  tokenContrasts: [7.1, 6.8, 8.2, 11.4],
};

test("S087 accepts meaningful highlighted and intentional plain observations", () => {
  assert.deepEqual(validateSyntaxObservation(validSyntaxObservation), []);
  assert.deepEqual(validateSyntaxObservation({
    ...validSyntaxObservation,
    caseId: "S087-PLAIN",
    language: "text",
    tokenClasses: [],
    tokenColors: [],
    tokenContrasts: [],
  }), []);
});

test("S087 rejects lost semantics, text, selection, containment, tokens, and contrast", () => {
  for (const [change, expected] of [
    [{ semantic: false }, /semantic/i],
    [{ renderedText: "changed" }, /rendered text/i],
    [{ selectedText: "changed" }, /selected and copied text/i],
    [{ copyText: "changed" }, /selected and copied text/i],
    [{ selectable: false }, /select/i],
    [{ codeOverflow: false }, /horizontal/i],
    [{ actualOverflow: false }, /code-local overflow/i],
    [{ pageContained: false }, /page-level overflow/i],
    [{ tokenClasses: [] }, /token/i],
    [{ tokenColors: ["rgb(230, 237, 243)"] }, /distinct token colors/i],
    [{ tokenContrasts: [4.49] }, /contrast/i],
    [{ surface: "other" }, /surface/i],
  ]) {
    assert.match(validateSyntaxObservation({ ...validSyntaxObservation, ...change }).join("\n"), expected);
  }
  assert.match(validateSyntaxObservation({
    ...validSyntaxObservation,
    caseId: "S087-PLAIN",
    language: "text",
  }).join("\n"), /plain.*token/i);
});

test("S087 requires the complete 40-cell syntax receipt", () => {
  const observations = [];
  for (const [caseId, language, tokenClasses] of [
    ["S087-COMMAND", "bash", validSyntaxObservation.tokenClasses],
    ["S087-POWERSHELL", "powershell", ["hljs-title", "hljs-attr", "hljs-string", "hljs-variable", "hljs-punctuation"]],
    ["S087-JSON", "json", ["hljs-attr", "hljs-number", "hljs-literal"]],
    ["S087-PLAIN", "text", []],
  ]) {
    for (const theme of ["navy", "light", "coal", "ayu", "rust"]) {
      for (const viewportWidth of [320, 1280]) {
        observations.push({
          ...validSyntaxObservation,
          caseId,
          language,
          theme,
          viewportWidth,
          tokenClasses,
          tokenColors: tokenClasses.map((_, index) => `rgb(${index + 80}, ${index + 100}, ${index + 120})`),
          tokenContrasts: tokenClasses.map(() => 7),
        });
      }
    }
  }
  const receipt = {
    syntaxSentinel: SYNTAX_PASS_SENTINEL,
    syntaxObservations: observations,
    syntaxFailures: [],
  };
  assert.deepEqual(validateSyntaxReceipt(receipt), []);
  assert.match(validateSyntaxReceipt({ ...receipt, syntaxSentinel: "wrong" }).join("\n"), /sentinel/i);
  assert.match(validateSyntaxReceipt({ ...receipt, syntaxObservations: observations.slice(1) }).join("\n"), /40 unique/i);
  assert.match(validateSyntaxReceipt({ ...receipt, syntaxObservations: [...observations, observations[0]] }).join("\n"), /40 unique/i);
  assert.match(validateSyntaxReceipt({ ...receipt, syntaxFailures: ["runtime failed"] }).join("\n"), /runtime failed/i);
});

const validFigureObservation = {
  caseId: "S088-LANDSCAPE",
  surface: "generated-loopback",
  theme: "navy",
  viewportWidth: 320,
  alternative: "ESO Weave showing a healthy running game",
  triggerSemantic: true,
  triggerName: "Expand image: ESO Weave showing a healthy running game",
  visibleAffordance: true,
  dialogCount: 1,
  legacyModalCount: 0,
  modalOpen: true,
  closeFocused: true,
  backgroundInert: true,
  dialogLabel: "ESO Weave showing a healthy running game",
  captionRequired: true,
  captionAssociated: true,
  naturalWidth: 1280,
  naturalHeight: 640,
  renderedWidth: 288,
  renderedHeight: 144,
  contained: true,
  upscaled: false,
  captionFontSize: 14.4,
  captionLineHeight: 22.32,
  captionContrast: 12,
  captionContained: true,
  modalCaptionFontSize: 14.4,
  modalCaptionLineHeight: 22.32,
  modalCaptionContrast: 12,
  modalCaptionContained: true,
  brandSurface: null,
  pageContained: true,
};

test("S088 accepts an accessible, contained, non-upscaled figure observation", () => {
  assert.deepEqual(validateFigureObservation(validFigureObservation), []);
  assert.deepEqual(validateFigureObservation({
    ...validFigureObservation,
    caseId: "S088-DIAGRAM",
    captionRequired: false,
    captionFontSize: null,
    captionLineHeight: null,
    captionContrast: null,
    captionContained: null,
    modalCaptionFontSize: null,
    modalCaptionLineHeight: null,
    modalCaptionContrast: null,
    modalCaptionContained: null,
  }), []);
});

test("S088 rejects incomplete controls, modal lifecycle, geometry, and caption hierarchy", () => {
  for (const [change, expected] of [
    [{ triggerSemantic: false }, /button/i],
    [{ triggerName: "" }, /accessible name/i],
    [{ visibleAffordance: false }, /affordance/i],
    [{ dialogCount: 2 }, /one shared dialog/i],
    [{ legacyModalCount: 1 }, /legacy|duplicate/i],
    [{ modalOpen: false }, /native modal/i],
    [{ closeFocused: false }, /close control/i],
    [{ backgroundInert: false }, /inert/i],
    [{ dialogLabel: "wrong" }, /dialog label/i],
    [{ renderedHeight: 120 }, /aspect ratio/i],
    [{ upscaled: true }, /upscal/i],
    [{ contained: false }, /containment/i],
    [{ captionFontSize: 13.9 }, /caption.*size/i],
    [{ captionLineHeight: 15 }, /caption.*line/i],
    [{ captionContrast: 4.49 }, /caption.*contrast/i],
    [{ captionContained: false }, /caption.*contain/i],
    [{ modalCaptionFontSize: 13.9 }, /modal caption.*size/i],
    [{ modalCaptionLineHeight: 15 }, /modal caption.*line/i],
    [{ modalCaptionContrast: 4.49 }, /modal caption.*contrast/i],
    [{ modalCaptionContained: false }, /modal caption.*contain/i],
    [{ pageContained: false }, /page-level overflow/i],
  ]) {
    assert.match(validateFigureObservation({ ...validFigureObservation, ...change }).join("\n"), expected);
  }
  assert.match(validateFigureObservation({ ...validFigureObservation, caseId: "S088-BRAND", brandSurface: "light" }).join("\n"), /matching light or dark surface/i);
});

test("S088 requires the complete matrix and all pointer plus keyboard journeys", () => {
  const observations = [];
  for (const caseId of ["S088-DIAGRAM", "S088-LANDSCAPE", "S088-PORTRAIT", "S088-ILLUSTRATION", "S088-BRAND"]) {
    for (const theme of ["navy", "light"]) {
      for (const viewportWidth of [320, 1280]) {
        observations.push({
          ...validFigureObservation,
          caseId,
          theme,
          viewportWidth,
          captionRequired: caseId !== "S088-DIAGRAM",
          brandSurface: caseId === "S088-BRAND" ? (theme === "light" ? "light" : "dark") : null,
        });
      }
    }
  }
  const journeys = [
    { id: "keyboard-enter-escape", opened: true, closed: true, focusReturned: true },
    { id: "keyboard-space-escape", opened: true, closed: true, focusReturned: true },
    { id: "keyboard-focus-cycle", opened: true, closed: true, focusReturned: true, tabForwardContained: true, tabReverseContained: true },
    { id: "pointer-close-button", opened: true, closed: true, focusReturned: true },
    { id: "pointer-backdrop", opened: true, closed: true, focusReturned: true },
  ];
  const receipt = {
    figureSentinel: FIGURE_PASS_SENTINEL,
    figureObservations: observations,
    figureJourneys: journeys,
    figureZoom: {
      scale: 2,
      sourceCaptionWrapped: true,
      sourceCaptionContained: true,
      sourceCaptionFontSize: 14.4,
      modalCaptionWrapped: true,
      modalCaptionContained: true,
      modalCaptionFontSize: 14.4,
      modalImageContained: true,
      captionAssociated: true,
    },
    figurePrint: {
      sourceImageVisible: true,
      sourceCaptionVisible: true,
      affordanceHidden: true,
      dialogHidden: true,
      legacyChromeHidden: true,
    },
    figureNoScript: {
      sourceImagesVisible: true,
      captionsVisible: true,
      interactiveChromeAbsent: true,
      legacyChromeHidden: true,
    },
    figureFailures: [],
  };
  assert.deepEqual(validateFigureReceipt(receipt), []);
  assert.match(validateFigureReceipt({ ...receipt, figureSentinel: "wrong" }).join("\n"), /sentinel/i);
  assert.match(validateFigureReceipt({ ...receipt, figureObservations: observations.slice(1) }).join("\n"), /20 unique/i);
  assert.match(validateFigureReceipt({ ...receipt, figureJourneys: journeys.slice(1) }).join("\n"), /journey/i);
  assert.match(validateFigureReceipt({ ...receipt, figureJourneys: journeys.map((item, index) => index === 0 ? { ...item, focusReturned: false } : item) }).join("\n"), /focus return/i);
  assert.match(validateFigureReceipt({ ...receipt, figureZoom: { ...receipt.figureZoom, scale: 1 } }).join("\n"), /200 percent zoom/i);
  assert.match(validateFigureReceipt({ ...receipt, figurePrint: { ...receipt.figurePrint, dialogHidden: false } }).join("\n"), /print rendering/i);
  assert.match(validateFigureReceipt({ ...receipt, figureNoScript: { ...receipt.figureNoScript, interactiveChromeAbsent: false } }).join("\n"), /no-JavaScript/i);
  assert.match(validateFigureReceipt({ ...receipt, figureFailures: ["runtime failed"] }).join("\n"), /runtime failed/i);
});

const validTableObservation = {
  caseId: "S089-COVERAGE",
  surface: "generated-loopback",
  theme: "navy",
  viewportWidth: 320,
  semantic: true,
  locallyContained: true,
  pageContained: true,
  columnsVisible: true,
  readableTokens: true,
  fontSize: 16,
  overflowing: true,
  focusable: true,
  namedRegion: true,
  described: true,
  hintVisible: true,
};

test("S089 accepts readable tables with conditional overflow semantics", () => {
  assert.deepEqual(validateTableObservation(validTableObservation), []);
  assert.deepEqual(validateTableObservation({
    ...validTableObservation,
    viewportWidth: 1280,
    overflowing: false,
    focusable: false,
    namedRegion: false,
    described: false,
    hintVisible: false,
  }), []);
});

test("S089 rejects lost semantics, containment, readability, and conditional focus", () => {
  for (const [change, expected] of [
    [{ semantic: false }, /semantics/i],
    [{ locallyContained: false }, /local/i],
    [{ pageContained: false }, /page-level/i],
    [{ columnsVisible: false }, /columns/i],
    [{ readableTokens: false }, /tokens/i],
    [{ fontSize: 13.9 }, /readable/i],
    [{ focusable: false }, /focusable/i],
    [{ hintVisible: false }, /visible/i],
  ]) assert.match(validateTableObservation({ ...validTableObservation, ...change }).join("\n"), expected);
  assert.match(validateTableObservation({ ...validTableObservation, overflowing: false }).join("\n"), /redundant/i);
});

test("S089 requires the 20-cell matrix and keyboard, resize, zoom, print, and fallback evidence", () => {
  const observations = [];
  for (const caseId of ["S089-COVERAGE", "S089-TEST", "S089-STATE", "S089-STATUS", "S089-PROTOCOL"]) {
    for (const theme of ["navy", "light"]) {
      for (const viewportWidth of [320, 1280]) observations.push({ ...validTableObservation, caseId, theme, viewportWidth });
    }
  }
  const receipt = {
    tableSentinel: TABLE_PASS_SENTINEL,
    tableObservations: observations,
    tableKeyboard: { focused: true, scrolled: true, pageStayedPut: true },
    tableResize: { narrowOverflow: true, narrowFocusable: true, wideOverflow: false, wideFocusable: false },
    tableZoom: { scale: 2, locallyContained: true, pageContained: true, hintVisible: true, conditionalSemantics: true },
    tablePrint: { semantic: true, hintHidden: true, overflowVisible: true, screenMinWidthRemoved: true },
    tableNoScript: { semantic: true, localOverflow: true, pageContained: true, enhancementAbsent: true, readableTokens: true, fontSize: 16 },
    tableFailures: [],
  };
  assert.deepEqual(validateTableReceipt(receipt), []);
  assert.match(validateTableReceipt({ ...receipt, tableSentinel: "wrong" }).join("\n"), /sentinel/i);
  assert.match(validateTableReceipt({ ...receipt, tableObservations: observations.slice(1) }).join("\n"), /20 unique/i);
  assert.match(validateTableReceipt({ ...receipt, tableKeyboard: { ...receipt.tableKeyboard, scrolled: false } }).join("\n"), /keyboard/i);
  assert.match(validateTableReceipt({ ...receipt, tableResize: { ...receipt.tableResize, wideFocusable: true } }).join("\n"), /resize/i);
  assert.match(validateTableReceipt({ ...receipt, tableZoom: { ...receipt.tableZoom, scale: 1 } }).join("\n"), /200 percent/i);
  assert.match(validateTableReceipt({ ...receipt, tablePrint: { ...receipt.tablePrint, hintHidden: false } }).join("\n"), /print/i);
  assert.match(validateTableReceipt({ ...receipt, tableNoScript: { ...receipt.tableNoScript, enhancementAbsent: false } }).join("\n"), /no-JavaScript/i);
  assert.match(validateTableReceipt({ ...receipt, tableFailures: ["runtime failed"] }).join("\n"), /runtime failed/i);
});
