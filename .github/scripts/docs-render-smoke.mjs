import { spawn } from "node:child_process";
import { access, mkdtemp, readFile, realpath, rm, stat } from "node:fs/promises";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";

export const PASS_SENTINEL = "ESO_WEAVE_DIAGRAM_SMOKE_PASS_V1";

const DIAGRAMS = [
  { id: "S082-D01", page: "development/architecture.html", asset: "architecture-ownership.svg", alt: "Architecture ownership flow keeps physical input and observed game evidence separate until named consumers" },
  { id: "S082-D02", page: "concepts/action-authorization.html", asset: "action-authorization.svg", alt: "Action authorization flow requires every positive gate or fails closed without generated input" },
  { id: "S082-D03", page: "development/state-machines.html", asset: "safety-recovery.svg", alt: "Safety recovery flow closes gates before synchronization and reopens only after a coherent baseline" },
  { id: "S082-D04", page: "reference/pixel-bus-protocol.html", asset: "pixel-bus-validation.svg", alt: "Pixel Bus validation flow rejects invalid headers and layouts before independently decoding and publishing payload signals" },
];
const THEMES = ["navy", "light"];
const VIEWPORTS = [320, 1280];

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

export function buildSafeFigure(diagram, generatedHtml) {
  const figure = generatedHtml.match(/<figure\s+class=["']docs-flow-diagram["']>[\s\S]*?<\/figure>/iu)?.[0] ?? "";
  const localSource = `../assets/diagrams/${diagram.asset}`;
  const references = figure.split(`src="${localSource}"`).length - 1;
  if (!figure.includes('class="checkbox-label"') || !figure.includes('class="checkbox-img"') ||
      !figure.includes('class="img-wrapper"') || !figure.includes(`alt="${diagram.alt}"`) || references !== 2) {
    throw new Error(`generated zoom DOM is invalid for ${diagram.page}`);
  }
  const source = `/eso-weave/assets/diagrams/${diagram.asset}`;
  return `<figure class="docs-flow-diagram" data-diagram-id="${diagram.id}"><p><label class="checkbox-label"><input class="checkbox-img" type="checkbox"><img src="${source}" alt="${diagram.alt}"><span class="img-wrapper"><img src="${source}" alt="${diagram.alt}"></span></label></p></figure>`;
}

async function prepareHarness(siteRoot) {
  const figures = [];
  let representativePage = "";
  for (const diagram of DIAGRAMS) {
    const html = await readFile(path.join(siteRoot, ...diagram.page.split("/")), "utf8");
    representativePage ||= html;
    figures.push(buildSafeFigure(diagram, html));
  }
  const styles = [...representativePage.matchAll(/<link\s+rel=["']stylesheet["'][^>]*\bhref=["']\.\.\/([^"']+)["'][^>]*>/giu)]
    .map((match) => {
      if (!/^[a-z0-9/_-]+\.css$/iu.test(match[1]) || match[1].includes("..")) throw new Error(`unsafe generated stylesheet path: ${match[1]}`);
      const media = /\bmedia=["']print["']/iu.test(match[0]) ? ' media="print"' : "";
      return `<link rel="stylesheet" href="/eso-weave/${match[1]}"${media}>`;
    })
    .join("\n");
  const themeScript = representativePage.match(/<script\s+src=["']\.\.\/(theme\/eso-weave-[^"']+\.js)["'][^>]*><\/script>/iu)?.[1];
  if (!themeScript) throw new Error("generated custom theme script is missing");
  return { figures: figures.join("\n"), styles, themeScript };
}

function harnessHtml(prepared, theme, viewportWidth) {
  const configuration = JSON.stringify({ diagrams: DIAGRAMS, theme, viewportWidth, sentinel: PASS_SENTINEL });
  return `<!doctype html><html lang="en" class="${theme}"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>pending</title>${prepared.styles}</head><body><main class="content">${prepared.figures}</main><script src="/eso-weave/${prepared.themeScript}"></script><script nonce="s086-diagram-smoke">
globalThis.__diagramSmokePromise = (async () => {
const configuration = ${configuration};
const observations = [];
const failures = [];
function nextFrame() { return new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))); }
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
    contained: rectangle.left >= boundary.left - tolerance && rectangle.top >= boundary.top - tolerance && rectangle.right <= boundary.right + tolerance && rectangle.bottom <= boundary.bottom + tolerance,
    ...paintStats(image),
  };
}
for (const diagram of configuration.diagrams) {
  try {
    const figure = document.querySelector('[data-diagram-id="' + diagram.id + '"]');
    const control = figure?.querySelector(".checkbox-img");
    const primary = figure?.querySelector(".checkbox-img + img");
    const expanded = figure?.querySelector(".img-wrapper > img");
    if (!figure || !control || !primary || !expanded) throw new Error("expected generated zoom DOM is missing");
    await Promise.all([primary.decode(), expanded.decode()]);
    await nextFrame();
    observations.push(observe(diagram.id, "normal", primary, figure.getBoundingClientRect()));
    control.checked = true;
    control.dispatchEvent(new Event("change", { bubbles: true }));
    await nextFrame();
    observations.push(observe(diagram.id, "expanded", expanded, { left: 0, top: 0, right: innerWidth, bottom: innerHeight }));
    if (expanded.alt !== "" || expanded.getAttribute("aria-hidden") !== "true") throw new Error("expanded clone is not decorative");
    control.checked = false;
    await nextFrame();
  } catch (error) {
    failures.push(diagram.id + ": " + error.message);
  }
}
const receipt = { schemaVersion: 1, sentinel: failures.length === 0 ? configuration.sentinel : "FAILED", observations, failures };
document.title = "ESO_WEAVE_RECEIPT_" + btoa(unescape(encodeURIComponent(JSON.stringify(receipt))));
const output = document.createElement("pre");
output.id = "diagram-smoke-receipt";
output.textContent = JSON.stringify(receipt);
document.body.append(output);
return receipt;
})();
</script></body></html>`;
}

async function startServer(siteRoot, prepared) {
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
      if (url.pathname === "/__diagram-smoke") {
        const theme = THEMES.includes(url.searchParams.get("theme")) ? url.searchParams.get("theme") : "navy";
        const viewportWidth = Number(url.searchParams.get("viewport"));
        response.writeHead(200, {
          "Content-Type": "text/html; charset=utf-8",
          "Cache-Control": "no-store",
          "Content-Security-Policy": "default-src 'none'; style-src 'self'; script-src 'self' 'nonce-s086-diagram-smoke'; img-src 'self' data:; font-src 'self'; connect-src 'none'; frame-src 'none'; object-src 'none'; base-uri 'none'; form-action 'none'",
        });
        response.end(request.method === "HEAD" ? undefined : harnessHtml(prepared, theme, viewportWidth));
        return;
      }
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
      response.writeHead(200, { "Content-Type": type, "Cache-Control": "no-store" });
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
  const prepared = await prepareHarness(path.resolve(siteRoot));
  const profileRoot = await mkdtemp(path.join(tmpdir(), "eso-weave-docs-browser-"));
  const { server, port, diagramRequests } = await startServer(siteRoot, prepared);
  const receipt = { schemaVersion: 1, sentinel: PASS_SENTINEL, browser: "pending", observations: [], requests: [], failures: [] };
  let browserProcess;
  let client;
  try {
    const launched = await launchBrowser(browser, profileRoot);
    browserProcess = launched.child;
    client = await DevToolsClient.connect(await pageDebuggerUrl(launched.webSocketUrl));
    await Promise.all([client.send("Page.enable"), client.send("Runtime.enable")]);
    const version = await client.send("Browser.getVersion");
    receipt.browser = `${version.product} (${version.userAgent})`;
    for (const theme of THEMES) {
      for (const viewportWidth of VIEWPORTS) {
        await client.send("Emulation.setDeviceMetricsOverride", { width: viewportWidth, height: 920, deviceScaleFactor: 1, mobile: false });
        const url = `http://127.0.0.1:${port}/__diagram-smoke?theme=${theme}&viewport=${viewportWidth}`;
        const loaded = client.waitFor("Page.loadEventFired");
        await client.send("Page.navigate", { url });
        await loaded;
        const evaluated = await client.send("Runtime.evaluate", {
          expression: "globalThis.__diagramSmokePromise",
          awaitPromise: true,
          returnByValue: true,
        });
        if (evaluated.exceptionDetails || !evaluated.result?.value) throw new Error(`browser evaluation failed for ${theme} ${viewportWidth}`);
        const partial = evaluated.result.value;
        receipt.observations.push(...(partial.observations ?? []));
        receipt.failures.push(...(partial.failures ?? []).map((failure) => `${theme} ${viewportWidth}: ${failure}`));
      }
    }
    receipt.requests = [...diagramRequests.values()];
    const errors = validateRenderingReceipt(receipt);
    if (errors.length > 0) throw new Error(errors.join("\n"));
    console.log(JSON.stringify(receipt, null, 2));
    console.log(PASS_SENTINEL);
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
