import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";

export const EXPECTED_PACKAGE = Object.freeze({
  id: "eso-weave-brand-1.0.0-bb2.0.1",
  filename: "eso-weave-brand-1.0.0-bb2.0.1.zip",
  brand_slug: "eso-weave",
  brand_version: "1.0.0",
  brandbuilder_version: "2.0.1",
  url: "https://brand.shruggie.tech/eso-weave/downloads/eso-weave-brand-1.0.0-bb2.0.1.zip",
  archive_sha256: "1b1ba26e57472573d31e2dc7dd20ac2f30a6eb89e7fb8d1e1c9b0ea0fe46c595",
  source_revision: "801ed912aaa8bbc66c12d8a97b5487fcddb0d9da",
  release_tag: "v2.0.1",
});

const EXPECTED_VERSIONS = Object.freeze({
  brand_canon: "1.2.1",
  interface_canon: "1.0.0",
  component_recipes: "1.1.0",
  web_react_adapter: "1.1.0",
  egui_adapter: "1.0.1",
  compiler: "2.0.1",
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
    secondary: "#1D2430",
    hover: "#252E3B",
    foreground: "#FFFFFF",
    muted_foreground: "#9A9A9A",
    primary: "#2DD4BF",
    on_primary: "#000000",
    emphasis: "#2DD4BF",
    destructive: "#E9505F",
    on_destructive: "#000000",
    border: "#262626",
    focus: "#2DD4BF",
  },
  light: {
    background: "#F8F8F6",
    card: "#FFFFFF",
    overlay: "#FFFFFF",
    secondary: "#F0EFED",
    hover: "#F0EFED",
    foreground: "#0A0A0A",
    muted_foreground: "#6B6B6B",
    primary: "#986000",
    on_primary: "#FFFFFF",
    emphasis: "#986000",
    destructive: "#C0293A",
    on_destructive: "#FFFFFF",
    border: "#E5E5E5",
    focus: "#986000",
  },
});

const EXPECTED_NATIVE_DENSITY = Object.freeze({
  profile: "fine-pointer-comfortable",
  control_height: 28,
  item_spacing: Object.freeze({ horizontal: 8, vertical: 2 }),
  button_padding: Object.freeze({ horizontal: 8, vertical: 4 }),
  conservative_target: 44,
});

const EXPECTED_ARTIFACTS = new Map([
  ["assets/brand/LICENSE-BRAND.md", "bd1107a804108bbe02955ca64000945322a6fc2456b45b795dac11a253c0023a"],
  ["assets/brand/fonts/Inter-Regular.ttf", "529be850e06f62f8904f22bda77e45bde4834498fdbec4ff4201fa3177447a3a"],
  ["assets/brand/fonts/Inter-Medium.ttf", "6df88fcb83ac96582350f801355c6eff55f15710093e9627fb431caa40521151"],
  ["assets/brand/fonts/Inter-SemiBold.ttf", "2de533bda937a063c595b07c6bd9b70c8c5087d0649a1c8330f7ac11fcc05602"],
  ["assets/brand/fonts/GeistMono-Regular.ttf", "990f0e094fe02b8872429209c09abf4c03d22183c33c2a2ddb891dc7f086271c"],
  ["assets/brand/fonts/GeistMono-OFL.txt", "f2001a42a9a4f3f569c8a78cf7362e0da86966537b16e6fc48ee5532f3e32e6c"],
  ["assets/brand/eso-weave-glyph.svg", "552f3203f0001b15e3adea9b720cb2f78be1427a12410f3e304170d973fef5ea"],
  ["assets/brand/eso-weave-mark.svg", "696d256c4ec0eae9aed315a1b489bbf5115ec33827e966a6e993708bf3f3109f"],
  ["assets/icon.ico", "3b3830fb98662d7e1fb0277e38d94072bfc67f9eac81a2f03433e6f11ee3c20a"],
  ["assets/brand/window-icon-256.png", "c029b18d5541c5e5f2c363836152f0cc69325d187657cc385d0095cd4edabeee"],
  ["packaging/linux/eso-weave.png", "c029b18d5541c5e5f2c363836152f0cc69325d187657cc385d0095cd4edabeee"],
  ["packaging/appimage/AppDir/eso-weave.png", "c029b18d5541c5e5f2c363836152f0cc69325d187657cc385d0095cd4edabeee"],
  ["packaging/windows/dialog.bmp", "2ee9e8edebcb02b3d4db491d255220738b59e350add51e47297554c6933191e6"],
  ["packaging/windows/banner.bmp", "d5354c4c8ce56d0e669020c3279ab89450dc96bc383bc2fbb77042e27eaca978"],
  ["assets/eso-weave-banner.png", "fe6f1bb45c0aafce463ecb182ba15b74017c6834496409bd809ef9aad15829e4"],
  ["assets/eso-weave-logo-clear.png", "8fbb63301d99fd5fe2c18bd404967e6d7c5c97b1c6851e7b165b79d9f55c205d"],
  ["assets/eso-weave-logo-white.png", "5bc07099259bf7fde39c2a7b2fa7cd2d6dac517b7c5fd8a14a32a213b82579d2"],
  ["assets/eso-weave-social.png", "e45a1ede2cb83ffcee1392e9aff188a57a51013b23e6e0ead65aae35551a9ef0"],
  ["docs/src/assets/brand/eso-weave-glyph.svg", "552f3203f0001b15e3adea9b720cb2f78be1427a12410f3e304170d973fef5ea"],
  ["docs/src/assets/brand/eso-weave-mark.svg", "696d256c4ec0eae9aed315a1b489bbf5115ec33827e966a6e993708bf3f3109f"],
  ["docs/src/assets/brand/eso-weave-banner.png", "fe6f1bb45c0aafce463ecb182ba15b74017c6834496409bd809ef9aad15829e4"],
  ["docs/src/assets/brand/fonts/Inter-Medium.ttf", "6df88fcb83ac96582350f801355c6eff55f15710093e9627fb431caa40521151"],
  ["docs/src/assets/brand/fonts/GeistMono-Regular.ttf", "990f0e094fe02b8872429209c09abf4c03d22183c33c2a2ddb891dc7f086271c"],
  ["docs/src/assets/brand/fonts/GeistMono-OFL.txt", "f2001a42a9a4f3f569c8a78cf7362e0da86966537b16e6fc48ee5532f3e32e6c"],
]);

const EXPECTED_RECOVERY = Object.freeze({
  path: "assets/brand/recovery/shruggie-brandbuilder-2.0.1.skill",
  source: "enforcement/distributions/shruggie-brandbuilder-2.0.1.skill",
  extract_to: "enforcement/brandbuilder",
  sha256: "5d712a07bab9f535207f1a4b81704ae8790274907919d1e38a83f8e4aa395742",
});

const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");

function compareObject(actual, expected, label, failures) {
  for (const [key, value] of Object.entries(expected)) {
    if (actual?.[key] !== value) {
      failures.push(`${label} ${key.replaceAll("_", " ")} must be ${value}`);
    }
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
  { expectedRecovery = EXPECTED_RECOVERY, expectedArtifacts = EXPECTED_ARTIFACTS } = {},
) {
  const failures = [];
  if (record?.schema_version !== 1) failures.push("schema_version must be 1");

  compareObject(record?.package, EXPECTED_PACKAGE, "package", failures);
  if (!/^[0-9a-f]{64}$/u.test(record?.package?.archive_sha256 ?? "")) {
    failures.push("archive SHA-256 must be 64 lowercase hexadecimal characters");
  }
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

  compareObject(
    record?.native_density,
    {
      profile: EXPECTED_NATIVE_DENSITY.profile,
      control_height: EXPECTED_NATIVE_DENSITY.control_height,
      conservative_target: EXPECTED_NATIVE_DENSITY.conservative_target,
    },
    "native density",
    failures,
  );
  compareObject(
    record?.native_density?.item_spacing,
    EXPECTED_NATIVE_DENSITY.item_spacing,
    "native density item spacing",
    failures,
  );
  compareObject(
    record?.native_density?.button_padding,
    EXPECTED_NATIVE_DENSITY.button_padding,
    "native density button padding",
    failures,
  );

  const recovery = record?.recovery;
  compareObject(recovery, expectedRecovery, "recovery", failures);
  if (recovery?.path) await validateFile(recovery, readBytes, "recovery", failures);
  else failures.push("recovery path is missing");

  if (!Array.isArray(record?.artifacts) || record.artifacts.length === 0) {
    failures.push("artifacts must contain at least one binding");
  } else {
    const paths = new Set();
    for (const artifact of record.artifacts) {
      if (paths.has(artifact.path)) failures.push(`artifact ${artifact.path} is duplicated`);
      paths.add(artifact.path);
      const expectedSha256 = expectedArtifacts.get(artifact.path);
      if (!expectedSha256) {
        failures.push(`artifact ${artifact.path} is not in the pinned inventory`);
        continue;
      }
      if (artifact.sha256 !== expectedSha256) {
        failures.push(`artifact ${artifact.path} SHA-256 must be ${expectedSha256}`);
      }
      await validateFile({ ...artifact, sha256: expectedSha256 }, readBytes, "artifact", failures);
    }
    for (const expectedPath of expectedArtifacts.keys()) {
      if (!paths.has(expectedPath)) failures.push(`artifact ${expectedPath} is missing from the pinned inventory`);
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
    console.log("brand-kit-policy: exact S119 adoption record, density, and artifact hashes verified");
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) await main();
