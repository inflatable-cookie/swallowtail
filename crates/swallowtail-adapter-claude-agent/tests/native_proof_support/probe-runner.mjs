// Probe-only runner for the Claude Agent SDK native proof.
//
// This path exists for one reason: the shipped sidecar admits only the
// maintained pair map, so it refuses every proposed hop before the SDK loads.
// Running the probe through the sidecar would therefore need a production
// admission change, which the planner did not grant.
//
// Instead the runner drives the *same selected option surface* the sidecar
// passes - proven equal by the harness - against the SDK module and the native
// peer, and labels every record it writes as probe-only. It never imports the
// adapter, never edits a claim, and never widens the maintained pair map.
//
// Original-artifact mode stays disabled: it needs `authority.original_run_authorised`
// in the admission ledger (false here) and the exact token it names, and the
// committed ledger has neither.

import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import process from "node:process";
import { pathToFileURL } from "node:url";

const EXIT = {
  ok: 0,
  refused: 2,
  failed: 3,
};

function parseArguments(argv) {
  const values = new Map();
  for (let index = 0; index < argv.length; index += 2) {
    const key = argv[index];
    if (!key?.startsWith("--")) throw new Error(`unexpected argument ${key}`);
    values.set(key.slice(2), argv[index + 1]);
  }
  return values;
}

function sha256(value) {
  return createHash("sha256").update(value).digest("hex");
}

class Refusal extends Error {
  constructor(code, message) {
    super(message);
    this.code = code;
  }
}

function canonical(value) {
  if (Array.isArray(value)) return value.map(canonical);
  if (value !== null && typeof value === "object") {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonical(value[key])]),
    );
  }
  return value;
}

/// The digest the harness also computes: the option surface with per-run paths
/// and the environment value list normalised away.
function optionsDigest(options) {
  const surface = { ...options };
  surface.cwd = "<workspace>";
  surface.pathToClaudeCodeExecutable = "<native>";
  surface.env = "<allowlist>";
  surface.canUseTool = typeof surface.canUseTool === "function" ? true : surface.canUseTool;
  surface.spawnClaudeCodeProcess =
    typeof surface.spawnClaudeCodeProcess === "function" ? true : surface.spawnClaudeCodeProcess;
  return `sha256:${sha256(JSON.stringify(canonical(surface)))}`;
}

function loadJson(where, what) {
  try {
    return JSON.parse(readFileSync(where, "utf8"));
  } catch (error) {
    throw new Refusal("invalid_json", `${what} at ${where} is not readable JSON: ${error.message}`);
  }
}

function requireLoopback(endpoint) {
  if (endpoint === undefined) return;
  const [host] = endpoint.split(":");
  if (host !== "127.0.0.1" && host !== "::1" && host !== "localhost") {
    throw new Refusal(
      "non_loopback_endpoint",
      `the probe endpoint ${endpoint} is not loopback; the probe only reaches a local fake`,
    );
  }
}

function checkAuthority(admission, mode, token) {
  if (admission.mode !== "probe-only") {
    throw new Refusal("production_admission_refused", "the ledger mode must be probe-only");
  }
  if (admission.production_admission !== false) {
    throw new Refusal(
      "production_admission_refused",
      "a probe ledger may not carry production admission",
    );
  }
  if (admission.authority?.kind !== "planner-temporary-probe-admission") {
    throw new Refusal(
      "authority_missing",
      "only a planner temporary probe admission may drive this path",
    );
  }
  if (mode !== "original") return;
  const original = admission.authority;
  if (original.original_run_authorised !== true) {
    throw new Refusal(
      "original_mode_disabled",
      "original-artifact execution is not authorised: the ledger still has original_run_authorised false",
    );
  }
  if (typeof original.original_run_token !== "string" || original.original_run_token.length === 0) {
    throw new Refusal("authority_missing", "the ledger names no original-run token");
  }
  if (token !== original.original_run_token) {
    throw new Refusal("authority_missing", "the presented authority token does not match the ledger");
  }
}

function checkPeer(peer, tuple, platform) {
  const manifest = loadJson(path.join(peer, "package.json"), "peer package identity");
  if (manifest.name !== "@anthropic-ai/claude-agent-sdk") {
    throw new Refusal("identity_substitution", `peer package is ${manifest.name}`);
  }
  if (manifest.version !== tuple.sdk_version) {
    throw new Refusal(
      "identity_substitution",
      `peer package version ${manifest.version} is not the admitted ${tuple.sdk_version}`,
    );
  }
  const nativeManifest = loadJson(path.join(peer, "native-manifest.json"), "peer native manifest");
  if (nativeManifest.version !== tuple.native_version) {
    throw new Refusal(
      "identity_substitution",
      `peer native version ${nativeManifest.version} is not the admitted ${tuple.native_version}`,
    );
  }
  const nativeBytes = readFileSync(path.join(peer, nativeManifest.binary));
  const digest = sha256(nativeBytes);
  const admitted = tuple.platforms[platform];
  if (admitted === undefined) {
    throw new Refusal("unknown_platform", `the ledger does not admit platform ${platform}`);
  }
  if (tuple.fake_native_sha256 !== digest) {
    throw new Refusal(
      "identity_substitution",
      `peer native bytes ${digest} are not the admitted fake ${tuple.fake_native_sha256}`,
    );
  }
  return { nativeBinary: path.join(peer, nativeManifest.binary), nativeDigest: digest };
}

async function main() {
  const args = parseArguments(process.argv.slice(2));
  const admissionPath = args.get("admission");
  const optionsPath = args.get("options");
  const peer = args.get("peer");
  const platform = args.get("platform");
  const sdkVersion = args.get("sdk-version");
  const workspace = args.get("workspace");
  const evidencePath = args.get("evidence");
  const boundMs = Number(args.get("bound-ms") ?? "120000");
  const scenario = args.get("scenario") ?? "ok";
  const endpoint = args.get("endpoint");
  const mode = args.get("mode") ?? "fake";
  const token = args.get("authority-token");

  if (!admissionPath || !optionsPath || !peer || !platform || !sdkVersion || !workspace || !evidencePath) {
    throw new Refusal("invalid_invocation", "the probe needs admission, options, peer, platform, sdk-version, workspace and evidence");
  }
  if (!Number.isSafeInteger(boundMs) || boundMs <= 0) {
    throw new Refusal("invalid_invocation", "the probe bound must be a positive integer");
  }

  const admission = loadJson(admissionPath, "probe admission");
  if (admission.schema !== "claude-agent-sdk-probe-admission-v1") {
    throw new Refusal("invalid_admission", "the admission schema tag is unknown");
  }
  checkAuthority(admission, mode, token);
  requireLoopback(endpoint);

  const tuple = admission.tuples.find((row) => row.sdk_version === sdkVersion);
  if (tuple === undefined) {
    throw new Refusal("unknown_tuple", `the ledger does not admit sdk ${sdkVersion}`);
  }
  if (tuple.platforms[platform] === undefined) {
    throw new Refusal("unknown_platform", `tuple ${sdkVersion} does not admit ${platform}`);
  }
  const { nativeBinary, nativeDigest } = checkPeer(peer, tuple, platform);

  const template = loadJson(optionsPath, "probe options");
  const environment = Object.fromEntries(
    Object.entries(process.env).filter(([key]) =>
      admission.environment_allowlist.includes(key) || key.startsWith("LC_"),
    ),
  );
  const options = {
    ...template.options,
    cwd: workspace,
    pathToClaudeCodeExecutable: nativeBinary,
    env: environment,
  };
  const envKeys = Object.keys(environment).sort();
  if (optionsDigest({ ...options, env: envKeys }) !== admission.selected_options_digest) {
    throw new Refusal(
      "options_surface_mismatch",
      "the probe option surface differs from the admitted one",
    );
  }

  const sdkModulePath = path.join(peer, "probe-sdk.mjs");
  const sdk = await import(pathToFileURL(sdkModulePath).href);
  if (typeof sdk.query !== "function") {
    throw new Refusal("sdk_unavailable", "the peer does not export query()");
  }

  const started = Date.now();
  let query;
  const records = [];
  let failure = null;
  let nativeExit = null;
  let killedByBound = false;
  const document = (async () => {
    query = sdk.query({ prompt: prompt(), options });
    const initialization = await query.initializationResult();
    records.push({ event: "initialized", models: initialization.models?.length ?? 0 });
    const supported = await query.supportedModels();
    records.push({ event: "supported_models", count: supported.length });
    const account = await query.accountInfo();
    records.push({ event: "account", apiProvider: account?.apiProvider ?? null });
    for await (const message of query) {
      records.push({ event: "message", type: message?.type ?? null, subtype: message?.subtype ?? null });
    }
  })().then(
    () => "done",
    (error) => {
      failure =
        error instanceof Refusal ? error : new Refusal("probe_failed", String(error?.message ?? error));
      return "failed";
    },
  );
  const expiry = new Promise((resolve) => setTimeout(() => resolve("expired"), boundMs));
  const outcome = await Promise.race([document, expiry]);
  if (outcome === "expired") {
    killedByBound = true;
    failure = new Refusal(
      "cleanup_failed",
      "the probe exceeded its wall-clock bound before the peer joined; the peer was killed",
    );
    query?.killNative?.();
  }
  nativeExit = killedByBound
    ? { join: "killed", code: null, signal: "SIGKILL" }
    : await settle(query, Math.max(1_000, Math.min(boundMs, 10_000)));

  const evidence = {
    schema: "claude-agent-sdk-probe-evidence-v1",
    probeOnly: true,
    productionAdmission: false,
    mode,
    tuple: { sdk_version: tuple.sdk_version, native_version: tuple.native_version },
    platform,
    scenario,
    endpoint: endpoint ?? null,
    workspace,
    nativeDigest,
    optionsDigest: admission.selected_options_digest,
    records,
    nativeExit,
    cleanup: nativeExit?.join ?? "unknown",
    elapsedMs: Date.now() - started,
    boundMs,
    failure: failure === null ? null : { code: failure.code, message: failure.message },
  };
  writeFileSync(evidencePath, JSON.stringify(evidence, null, 2));
  if (!killedByBound && nativeExit?.join !== "exited") {
    throw new Refusal("cleanup_failed", "the native peer was not joined within the probe bound");
  }
  if (failure !== null) throw failure;
  console.log(JSON.stringify({ probeOnly: true, status: "ok", tuple: evidence.tuple, platform }));
  return EXIT.ok;
}

async function settle(query, boundMs) {
  if (query === undefined) return { join: "absent", code: null, signal: null };
  let timer;
  const bound = new Promise((resolve) => {
    timer = setTimeout(() => resolve("killed"), boundMs);
  });
  try {
    const joined = query.joinNative === undefined ? "exited" : await Promise.race([query.joinNative(), bound]);
    if (joined === "killed") {
      query.killNative?.();
      return { join: "killed", code: null, signal: "SIGKILL" };
    }
    return { join: "exited", ...(joined ?? { code: null, signal: null }) };
  } finally {
    clearTimeout(timer);
  }
}

async function* prompt() {
  yield "probe turn";
}

main()
  .then((code) => process.exit(code))
  .catch((error) => {
    const refusal = error instanceof Refusal ? error : new Refusal("probe_failed", String(error?.message ?? error));
    console.error(JSON.stringify({ probeOnly: true, status: "refused", code: refusal.code, message: refusal.message }));
    process.exit(EXIT.refused);
  });
