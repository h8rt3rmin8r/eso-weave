import assert from "node:assert/strict";
import test from "node:test";

import {
  PASS_SENTINEL,
  SYNTAX_PASS_SENTINEL,
  validateObservation,
  validateRenderingReceipt,
  validateSyntaxObservation,
  validateSyntaxReceipt,
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
