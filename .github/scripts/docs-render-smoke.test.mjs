import assert from "node:assert/strict";
import test from "node:test";

import {
  PASS_SENTINEL,
  validateObservation,
  validateRenderingReceipt,
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
