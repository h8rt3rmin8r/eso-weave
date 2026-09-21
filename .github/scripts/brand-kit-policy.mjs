import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";

export const EXPECTED_PACKAGE = Object.freeze({
  id: "eso-weave-brand-1.0.0-bb2.0.0",
  filename: "eso-weave-brand-1.0.0-bb2.0.0.zip",
  brand_slug: "eso-weave",
  brand_version: "1.0.0",
  brandbuilder_version: "2.0.0",
  url: "https://brand.shruggie.tech/eso-weave/downloads/eso-weave-brand-1.0.0-bb2.0.0.zip",
  archive_sha256: "b37ac1459666ae33d772229bd5247699c2c845971eabed267ff85465b68a1ba1",
  source_revision: "f974fbb5c532a3394980be4dd985aee86e7e2c9e",
  release_tag: "v2.0.0",
});

const EXPECTED_VERSIONS = Object.freeze({
  brand_canon: "1.2.1",
  interface_canon: "1.0.0",
  component_recipes: "1.1.0",
  web_react_adapter: "1.1.0",
  egui_adapter: "1.0.0",
  compiler: "2.0.0",
  brand: "1.0.0",
});

const EXPECTED_AUTHORITY = Object.freeze([
  "brand.json",
  "enforcement/bundle.json",
  "enforcement/release-impact.json",
  "enforcement/interface-canon.json",
  "enforcement/component-recipes.json",
  "enforcement/version-policy.json",
  "enforcement/documentation-contract.json",
  "enforcement/consumer-contract.json",
]);

const EXPECTED_TOKENS = Object.freeze({
  dark: {
    background: "#0E1116",
    card: "#171C24",
    overlay: "#0A0D12",
    primary: "#2DD4BF",
    on_primary: "#000000",
    destructive: "#E9505F",
  },
  light: {
    background: "#F8F8F6",
    card: "#FFFFFF",
    overlay: "#FFFFFF",
    primary: "#986000",
    on_primary: "#FFFFFF",
    destructive: "#C0293A",
  },
});

const RECOVERY_SHA256 = "26578eb150a9c24d9e625fb77b192e0415a6ac8faf67c83834ac914f2da15e90";

const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");

function compareObject(actual, expected, label, failures) {
  for (const [key, value] of Object.entries(expected)) {
    if (actual?.[key] !== value) failures.push(`${label} ${key} must be ${value}`);
  }
}

async function validateFile(binding, readBytes, label, failures) {
  let bytes;
  try {
    bytes = await readBytes(binding.path);
  } catch {
    bytes = undefined;
  }
  if (bytes === undefined) {
    failures.push(`${label} ${binding.path} is missing`);
    return;
  }
  const observed = digest(bytes);
  if (observed !== binding.sha256) {
    failures.push(`${label} ${binding.path} SHA-256 is ${observed}, expected ${binding.sha256}`);
  }
}

export async function validateAdoptionRecord(
  record,
  readBytes,
  { recoverySha256 = RECOVERY_SHA256 } = {},
) {
  const failures = [];
  if (record?.schema_version !== 1) failures.push("schema_version must be 1");

  compareObject(record?.package, EXPECTED_PACKAGE, "package", failures);
  if (record?.package?.archive_sha256 !== EXPECTED_PACKAGE.archive_sha256) {
    failures.push(`archive SHA-256 must be ${EXPECTED_PACKAGE.archive_sha256}`);
  }
  compareObject(record?.versions, EXPECTED_VERSIONS, "version", failures);

  if (JSON.stringify(record?.authority) !== JSON.stringify(EXPECTED_AUTHORITY)) {
    failures.push("authority order must match the delivered consumer contract");
  }

  for (const [theme, expected] of Object.entries(EXPECTED_TOKENS)) {
    for (const [role, value] of Object.entries(expected)) {
      if (record?.runtime_tokens?.[theme]?.[role] !== value) {
        failures.push(`${theme} ${role.replaceAll("_", " ")} must be ${value}`);
      }
    }
  }

  const recovery = record?.recovery;
  if (recovery?.sha256 !== recoverySha256) {
    failures.push(`recovery SHA-256 must be ${recoverySha256}`);
  }
  if (recovery?.path) await validateFile(recovery, readBytes, "recovery", failures);
  else failures.push("recovery path is missing");

  if (!Array.isArray(record?.artifacts) || record.artifacts.length === 0) {
    failures.push("artifacts must contain at least one binding");
  } else {
    const paths = new Set();
    for (const artifact of record.artifacts) {
      if (paths.has(artifact.path)) failures.push(`artifact ${artifact.path} is duplicated`);
      paths.add(artifact.path);
      if (!/^[0-9a-f]{64}$/u.test(artifact.sha256 ?? "")) {
        failures.push(`artifact ${artifact.path} has an invalid SHA-256`);
        continue;
      }
      await validateFile(artifact, readBytes, "artifact", failures);
    }
  }

  const deviations = record?.adapter_deviations;
  const requiredRoles = ["surface.background", "surface.card", "text.muted", "action.destructive"];
  if (!Array.isArray(deviations) || !deviations.some((item) =>
    item.source === "native/egui/src/tokens.rs"
    && requiredRoles.every((role) => item.roles?.includes(role))
    && /brand\.json/u.test(item.authority ?? "")
    && /interface-canon\.json/u.test(item.authority ?? ""))) {
    failures.push("generated egui light-theme deviations must name all four roles and higher authorities");
  }

  return failures;
}

export async function validateRepository(repositoryRoot = process.cwd()) {
  const recordPath = path.join(repositoryRoot, "assets", "brand", "brand-kit-adoption.json");
  let record;
  try {
    record = JSON.parse(await readFile(recordPath, "utf8"));
  } catch (error) {
    return [`cannot read brand-kit adoption record: ${error.message}`];
  }
  return validateAdoptionRecord(record, async (relativePath) =>
    readFile(path.join(repositoryRoot, ...relativePath.split("/"))));
}

async function main() {
  const failures = await validateRepository();
  if (failures.length > 0) {
    for (const failure of failures) console.error(`brand-kit-policy: ${failure}`);
    process.exitCode = 1;
  } else {
    console.log("brand-kit-policy: exact S117 adoption record and artifact hashes verified");
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) await main();
