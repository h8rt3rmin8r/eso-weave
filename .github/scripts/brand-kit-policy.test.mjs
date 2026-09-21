import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import test from "node:test";

import {
  EXPECTED_PACKAGE,
  validateAdoptionRecord,
  validateRepository,
} from "./brand-kit-policy.mjs";

const sha256 = (value) => createHash("sha256").update(value).digest("hex");

function fixture() {
  const font = Buffer.from("font");
  const recovery = Buffer.from("recovery");
  const icon = Buffer.from("icon");
  const files = new Map([
    ["assets/brand/fonts/GeistMono-Regular.ttf", font],
    ["assets/brand/recovery/shruggie-brandbuilder-2.0.0.skill", recovery],
    ["assets/icon.ico", icon],
  ]);
  const record = {
    schema_version: 1,
    package: { ...EXPECTED_PACKAGE },
    versions: {
      brand_canon: "1.2.1",
      interface_canon: "1.0.0",
      component_recipes: "1.1.0",
      web_react_adapter: "1.1.0",
      egui_adapter: "1.0.0",
      compiler: "2.0.0",
      brand: "1.0.0",
    },
    authority: [
      "brand.json",
      "enforcement/bundle.json",
      "enforcement/release-impact.json",
      "enforcement/interface-canon.json",
      "enforcement/component-recipes.json",
      "enforcement/version-policy.json",
      "enforcement/documentation-contract.json",
      "enforcement/consumer-contract.json",
    ],
    recovery: {
      path: "assets/brand/recovery/shruggie-brandbuilder-2.0.0.skill",
      sha256: sha256(recovery),
    },
    artifacts: [
      { path: "assets/brand/fonts/GeistMono-Regular.ttf", sha256: sha256(font) },
      { path: "assets/icon.ico", sha256: sha256(icon) },
    ],
    runtime_tokens: {
      dark: {
        background: "#0E1116", card: "#171C24", overlay: "#0A0D12",
        primary: "#2DD4BF", on_primary: "#000000", destructive: "#E9505F",
      },
      light: {
        background: "#F8F8F6", card: "#FFFFFF", overlay: "#FFFFFF",
        primary: "#986000", on_primary: "#FFFFFF", destructive: "#C0293A",
      },
    },
    adapter_deviations: [{
      source: "native/egui/src/tokens.rs",
      roles: ["surface.background", "surface.card", "text.muted", "action.destructive"],
      authority: "brand.json and enforcement/interface-canon.json",
    }],
  };
  return { files, record };
}

test("S117 accepts the exact package, authority, tokens, recovery, and artifacts", async () => {
  const { files, record } = fixture();
  assert.deepEqual(await validateAdoptionRecord(
    record,
    async (path) => files.get(path),
    { recoverySha256: record.recovery.sha256 },
  ), []);
});

test("S117 rejects moving package identity and generated-adapter light values", async () => {
  const { files, record } = fixture();
  record.package.archive_sha256 = "0".repeat(64);
  record.runtime_tokens.light.background = "#FFFFFF";
  record.runtime_tokens.light.card = "#F8F8F6";
  record.runtime_tokens.light.destructive = "#E9505F";
  const failures = (await validateAdoptionRecord(
    record,
    async (path) => files.get(path),
    { recoverySha256: record.recovery.sha256 },
  )).join("\n");
  assert.match(failures, /archive SHA-256/i);
  assert.match(failures, /light background/i);
  assert.match(failures, /light card/i);
  assert.match(failures, /light destructive/i);
});

test("S117 rejects missing recovery bytes and artifact drift", async () => {
  const { files, record } = fixture();
  files.delete(record.recovery.path);
  files.set("assets/icon.ico", Buffer.from("drift"));
  const failures = (await validateAdoptionRecord(
    record,
    async (path) => files.get(path),
    { recoverySha256: record.recovery.sha256 },
  )).join("\n");
  assert.match(failures, /recovery.*missing/i);
  assert.match(failures, /assets\/icon\.ico.*SHA-256/i);
});

test("S117 committed repository satisfies the brand-kit contract", async () => {
  assert.deepEqual(await validateRepository(), []);
});
