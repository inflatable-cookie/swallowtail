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
// Original-artifact mode is refused unconditionally. This runner has no
// OS-level confinement; a ledger flag or authority token cannot supply it.

import { createHash } from "node:crypto";
import {
  chmodSync as fsChmodSync,
  lstatSync,
  mkdtempSync as fsMkdtempSync,
  readFileSync,
  readdirSync,
  realpathSync as fsRealpathSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { spawn } from "node:child_process";
import path from "node:path";
import process from "node:process";
import { pathToFileURL } from "node:url";

const EXIT = {
  ok: 0,
  refused: 2,
  failed: 3,
};

/// SHA-256 of the committed probe admission. Changing it is a source change the
/// fixture verifier cross-checks, not a runtime option: a caller cannot supply
/// it, so a ledger and a peer rewritten to agree with each other are refused.
const COMMITTED_ADMISSION_SHA256 = "2afdd4dfd8a35b9eb7765fc5e55bd3e63d84acd32ef0b190ff212ec1f9735b2a";

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
  if (mode === "original") {
    // Original mode is refused unconditionally, whatever the ledger or the
    // caller presents. This process cannot confine a vendor artifact: it has no
    // network namespace or egress filter (the loopback endpoint is an
    // observation point, not a firewall), no filesystem namespace or mount
    // ownership, no uid/gid separation from the host user and no process
    // cgroup or supervisor. A token or a ledger flag cannot create those
    // capabilities, so a probe run must never claim them.
    throw new Refusal(
      "original_mode_unavailable",
      "original-artifact execution needs kernel-level confinement this harness " +
        "does not have: a network namespace or egress filter, a filesystem " +
        "namespace with owned mounts, a separate uid/gid, and a process " +
        "supervisor or cgroup. No ledger flag or token can substitute for them.",
    );
  }
  if (token !== undefined) {
    throw new Refusal(
      "invalid_invocation",
      "an authority token is meaningless while original mode is unavailable",
    );
  }
}

function realpath(directory) {
  try {
    return fsRealpathSync(directory);
  } catch {
    throw new Refusal("isolation_refused", `${directory} does not resolve`);
  }
}

function isInside(rootReal, candidateReal) {
  return candidateReal === rootReal || candidateReal.startsWith(`${rootReal}${path.sep}`);
}

/// A confined root is one this probe created: real directory, mode 0700, owned
/// by this process's uid, and reachable without a symlinked component.
function assertOwnedDirectory(directory, role) {
  const stats = lstatSync(directory);
  if (!stats.isDirectory() || stats.isSymbolicLink()) {
    throw new Refusal("isolation_refused", `${role} ${directory} is not a real directory`);
  }
  if (typeof process.getuid === "function" && stats.uid !== process.getuid()) {
    throw new Refusal("isolation_refused", `${role} ${directory} is not owned by this process`);
  }
  if ((stats.mode & 0o077) !== 0) {
    throw new Refusal("isolation_refused", `${role} ${directory} is not mode 0700`);
  }
  return realpath(directory);
}

function createConfinedRoot(base, name) {
  const root = fsMkdtempSync(path.join(base, `${name}-`));
  fsChmodSync(root, 0o700);
  return { path: root, real: assertOwnedDirectory(root, name) };
}

/// Creates the workspace, HOME and TMPDIR the probe runs in. The caller supplies
/// only an owned base directory; every path the probe uses is created here, so
/// no caller marker or lexical prefix can stand in for ownership.
function createProbeRoots(base) {
  const baseReal = assertOwnedDirectory(base, "probe base");
  const workspace = createConfinedRoot(baseReal, "workspace");
  const home = createConfinedRoot(baseReal, "home");
  const tmpdir = createConfinedRoot(baseReal, "tmpdir");
  return { workspace, home, tmpdir, baseReal };
}

function verifyPeerOwnership(peer, ownershipNonce) {
  const peerReal = assertOwnedDirectory(peer, "peer");
  if (typeof ownershipNonce !== "string" || ownershipNonce.length < 16) {
    throw new Refusal("isolation_refused", "the probe needs the run's peer ownership nonce");
  }
  let record;
  try {
    record = readFileSync(path.join(peerReal, "ownership.json"), "utf8");
  } catch {
    throw new Refusal("isolation_refused", "the peer carries no ownership record");
  }
  let parsed;
  try {
    parsed = JSON.parse(record);
  } catch {
    throw new Refusal("isolation_refused", "the peer ownership record is not JSON");
  }
  if (parsed.nonce !== ownershipNonce) {
    throw new Refusal("isolation_refused", "the peer ownership nonce does not match this run");
  }
  // The directory's own ownership was checked by `assertOwnedDirectory`; the
  // nonce binds this peer tree to this invocation.
  return peerReal;
}

function checkPeer(peer, tuple, platform, admission, ownershipNonce) {
  const peerReal = verifyPeerOwnership(peer, ownershipNonce);
  const manifestPath = path.join(peerReal, "package.json");
  const manifest = loadJson(manifestPath, "peer package identity");
  if (manifest.name !== "@anthropic-ai/claude-agent-sdk") {
    throw new Refusal("identity_substitution", `peer package is ${manifest.name}`);
  }
  if (manifest.version !== tuple.sdk_version) {
    throw new Refusal(
      "identity_substitution",
      `peer package version ${manifest.version} is not the admitted ${tuple.sdk_version}`,
    );
  }
  const nativeManifest = loadJson(
    path.join(peerReal, "native-manifest.json"),
    "peer native manifest",
  );
  if (nativeManifest.version !== tuple.native_version) {
    throw new Refusal(
      "identity_substitution",
      `peer native version ${nativeManifest.version} is not the admitted ${tuple.native_version}`,
    );
  }
  const admitted = tuple.platforms[platform];
  if (admitted === undefined) {
    throw new Refusal("unknown_platform", `the ledger does not admit platform ${platform}`);
  }

  // Actual bytes, resolved through realpath so a symlink cannot point somewhere
  // else than the path that was hashed.
  const nativeCandidate = path.join(peerReal, String(nativeManifest.binary));
  const nativeReal = realpath(nativeCandidate);
  if (!isInside(peerReal, nativeReal)) {
    throw new Refusal("identity_substitution", "the peer native resolves outside the peer");
  }
  const nativeBytes = readFileSync(nativeReal);
  const nativeDigest = sha256(nativeBytes);
  if (tuple.fake_native_sha256 !== nativeDigest) {
    throw new Refusal(
      "identity_substitution",
      `peer native bytes ${nativeDigest} are not the admitted fake ${tuple.fake_native_sha256}`,
    );
  }
  const sdkModuleReal = realpath(path.join(peerReal, "probe-sdk.mjs"));
  if (!isInside(peerReal, sdkModuleReal)) {
    throw new Refusal("identity_substitution", "the peer SDK module resolves outside the peer");
  }
  const sdkDigest = sha256(readFileSync(sdkModuleReal));
  if (admission.fake_peer?.sdk_module_sha256 !== sdkDigest) {
    throw new Refusal(
      "identity_substitution",
      `peer SDK bytes ${sdkDigest} are not the admitted ${admission.fake_peer?.sdk_module_sha256}`,
    );
  }
  return {
    nativeBinary: nativeReal,
    nativeDigest,
    sdkModulePath: sdkModuleReal,
    sdkDigest,
    sdkVersion: manifest.version,
    peerReal,
  };
}

function directoryBytes(directory) {
  let total = 0;
  const visit = (entry) => {
    let stats;
    try {
      stats = statSync(entry);
    } catch {
      return;
    }
    if (stats.isDirectory()) {
      for (const name of readdirSync(entry)) visit(path.join(entry, name));
      return;
    }
    total += stats.size;
  };
  visit(directory);
  return total;
}

/// Records are bounded while the probe runs, not only when the evidence is
/// written, so a talkative peer cannot fill memory or the volume first.
function accountRecord(records, record, bounds, roots) {
  records.push(record);
  const evidenceBytes = Buffer.byteLength(JSON.stringify(records), "utf8");
  if (evidenceBytes > bounds.captured_stdout_bytes) {
    throw new Refusal(
      "evidence_limit_exceeded",
      `the probe's records passed ${bounds.captured_stdout_bytes} bytes while running`,
    );
  }
  if (records.length % 32 === 0) {
    const disk = roots.reduce((total, root) => total + directoryBytes(root), 0);
    if (disk > bounds.disk_bytes) {
      throw new Refusal(
        "disk_limit_exceeded",
        `the probe wrote ${disk} bytes while running, over the admitted ${bounds.disk_bytes}`,
      );
    }
  }
  return record;
}

async function main() {
  const args = parseArguments(process.argv.slice(2));
  const admissionPath = args.get("admission");
  const optionsPath = args.get("options");
  const peer = args.get("peer");
  const platform = args.get("platform");
  const sdkVersion = args.get("sdk-version");
  const base = args.get("base");
  const ownershipNonce = args.get("ownership-nonce");
  const evidencePath = args.get("evidence");
  const requestedBoundMs = Number(args.get("bound-ms") ?? "0");
  const requestedEvidenceBytes = Number(args.get("max-evidence-bytes") ?? "0");
  const requestedDiskBytes = Number(args.get("max-disk-bytes") ?? "0");
  const requestedJoinWaitMs = Number(args.get("join-wait-ms") ?? "0");
  const scenario = args.get("scenario") ?? "ok";
  const endpoint = args.get("endpoint");
  const mode = args.get("mode") ?? "fake";
  const token = args.get("authority-token");

  if (!admissionPath || !optionsPath || !peer || !platform || !sdkVersion || !base || !evidencePath) {
    throw new Refusal(
      "invalid_invocation",
      "the probe needs admission, options, peer, platform, sdk-version, base, ownership-nonce and evidence",
    );
  }

  const admissionBytes = readFileSync(admissionPath);
  if (sha256(admissionBytes) !== COMMITTED_ADMISSION_SHA256) {
    throw new Refusal(
      "admission_substitution",
      "the admission document does not match the digest compiled into this runner; a substituted ledger and peer are refused",
    );
  }
  const admission = JSON.parse(admissionBytes.toString("utf8"));
  if (admission.schema !== "claude-agent-sdk-probe-admission-v1") {
    throw new Refusal("invalid_admission", "the admission schema tag is unknown");
  }
  checkAuthority(admission, mode, token);
  requireLoopback(endpoint);

  // The wall clock comes from the ledger, not from the caller. A caller may ask
  // for less; it may never ask for more.
  const ledgerBoundMs = admission.bounds.wall_clock_seconds * 1000;
  const boundMs =
    requestedBoundMs > 0 ? Math.min(requestedBoundMs, ledgerBoundMs) : ledgerBoundMs;
  if (Number.isFinite(requestedBoundMs) && requestedBoundMs > ledgerBoundMs) {
    throw new Refusal(
      "bound_refused",
      `the requested ${requestedBoundMs} ms exceeds the admitted ${ledgerBoundMs} ms`,
    );
  }
  const bounds = { ...admission.bounds };
  if (requestedEvidenceBytes > 0) {
    if (requestedEvidenceBytes > admission.bounds.captured_stdout_bytes) {
      throw new Refusal(
        "bound_refused",
        "an evidence budget may only be tightened, never raised past the ledger",
      );
    }
    bounds.captured_stdout_bytes = requestedEvidenceBytes;
  }
  if (requestedDiskBytes > 0) {
    if (requestedDiskBytes > admission.bounds.disk_bytes) {
      throw new Refusal(
        "bound_refused",
        "a disk budget may only be tightened, never raised past the ledger",
      );
    }
    bounds.disk_bytes = requestedDiskBytes;
  }
  let joinWaitMs = Math.max(1_000, Math.min(boundMs, 10_000));
  if (requestedJoinWaitMs > 0) {
    if (requestedJoinWaitMs > boundMs) {
      throw new Refusal(
        "bound_refused",
        "a join wait may only be tightened, never raised past the wall clock",
      );
    }
    joinWaitMs = requestedJoinWaitMs;
  }
  const roots = createProbeRoots(base);

  const tuple = admission.tuples.find((row) => row.sdk_version === sdkVersion);
  if (tuple === undefined) {
    throw new Refusal("unknown_tuple", `the ledger does not admit sdk ${sdkVersion}`);
  }
  if (tuple.platforms[platform] === undefined) {
    throw new Refusal("unknown_platform", `tuple ${sdkVersion} does not admit ${platform}`);
  }
  if (typeof tuple.behavior_revision !== "string" || tuple.behavior_revision.length === 0) {
    throw new Refusal(
      "invalid_admission",
      `tuple ${sdkVersion} names no behavior revision, so the run cannot identify its contract`,
    );
  }
  const environment = Object.fromEntries(
    Object.entries(process.env).filter(([key]) =>
      admission.environment_allowlist.includes(key) || key.startsWith("LC_"),
    ),
  );
  environment.HOME = roots.home.path;
  environment.TMPDIR = roots.tmpdir.path;
  // TMP/TEMP follow TMPDIR in the child, so they are set explicitly.
  environment.TMP = roots.tmpdir.path;
  environment.TEMP = roots.tmpdir.path;
  const { nativeBinary, nativeDigest, sdkModulePath, peerReal } = checkPeer(
    peer,
    tuple,
    platform,
    admission,
    ownershipNonce,
  );
  const workspace = roots.workspace.path;

  const template = loadJson(optionsPath, "probe options");
  let nativeSpawns = 0;
  let spawnRefusal = null;
  const nativePids = [];
  const options = {
    ...template.options,
    cwd: workspace,
    pathToClaudeCodeExecutable: nativeBinary,
    env: environment,
    spawnClaudeCodeProcess: (spawnOptions) => {
      nativeSpawns += 1;
      if (nativeSpawns > bounds.native_processes) {
        // A peer that catches the hook's error must still fail the probe, so the
        // refusal is latched and reported after the run settles.
        spawnRefusal ??= new Refusal(
          "process_limit_exceeded",
          `the probe may start ${bounds.native_processes} native process`,
        );
        throw spawnRefusal;
      }
      const child = spawn(spawnOptions.command, spawnOptions.args, {
        cwd: spawnOptions.cwd,
        env: spawnOptions.env,
        stdio: ["pipe", "pipe", "pipe"],
      });
      nativePids.push(child.pid ?? null);
      return child;
    },
  };
  const envKeys = Object.keys(environment).sort();
  if (optionsDigest({ ...options, env: envKeys }) !== admission.selected_options_digest) {
    throw new Refusal(
      "options_surface_mismatch",
      "the probe option surface differs from the admitted one",
    );
  }

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
    accountRecord(records, { event: "initialized", models: initialization.models?.length ?? 0 }, bounds, [peerReal, workspace]);
    const supported = await query.supportedModels();
    accountRecord(records, { event: "supported_models", count: supported.length }, bounds, [peerReal, workspace]);
    const account = await query.accountInfo();
    accountRecord(records, { event: "account", apiProvider: account?.apiProvider ?? null }, bounds, [peerReal, workspace]);
    for await (const message of query) {
      accountRecord(
        records,
        { event: "message", type: message?.type ?? null, subtype: message?.subtype ?? null },
        bounds,
        [peerReal, workspace],
      );
    }
  })().then(
    () => "done",
    (error) => {
      failure =
        error instanceof Refusal ? error : new Refusal("probe_failed", String(error?.message ?? error));
      return "failed";
    },
  );
  if (spawnRefusal !== null && failure === null) failure = spawnRefusal;
  const expiry = new Promise((resolve) => setTimeout(() => resolve("expired"), boundMs));
  const outcome = await Promise.race([document, expiry]);
  if (outcome === "expired") {
    killedByBound = true;
    failure = new Refusal(
      "cleanup_failed",
      "the probe exceeded its wall-clock bound before the peer joined; the peer was killed",
    );
    nativeExit = await stopPeer(query, joinWaitMs);
  } else {
    nativeExit = await settle(query, joinWaitMs);
  }

  const joinedCleanly =
    nativeExit?.timedOut !== true &&
    (nativeExit?.join === "exited" ||
      (nativeExit?.join === "killed" && nativeExit?.observed === true));
  if (!joinedCleanly) {
    failure ??= new Refusal(
      "cleanup_failed",
      `the native join was ${nativeExit?.join ?? "absent"} (observed: ${
        nativeExit?.observed === true
      }, timedOut: ${nativeExit?.timedOut === true}), so cleanup is unknown`,
    );
  }

  const evidence = {
    schema: "claude-agent-sdk-probe-evidence-v1",
    probeOnly: true,
    productionAdmission: false,
    mode,
    tuple: { sdk_version: tuple.sdk_version, native_version: tuple.native_version },
    behaviorRevision: tuple.behavior_revision,
    platform,
    scenario,
    endpoint: endpoint ?? null,
    workspace,
    workspaceCreatedByProbe: true,
    home: roots.home.path,
    tmpdir: roots.tmpdir.path,
    peerReal,
    nativeDigest,
    optionsDigest: admission.selected_options_digest,
    boundMs,
    boundSource: "ledger wall_clock_seconds",
    records,
    nativeExit,
    nativePids,
    cleanup: nativeExit?.join ?? "unknown",
    elapsedMs: Date.now() - started,
    failure: failure === null ? null : { code: failure.code, message: failure.message },
  };
  const serialized = JSON.stringify(evidence, null, 2);
  const evidenceBytes = Buffer.byteLength(serialized, "utf8");
  if (evidenceBytes > bounds.captured_stdout_bytes) {
    throw new Refusal(
      "evidence_limit_exceeded",
      `the evidence is ${evidenceBytes} bytes, over the effective ${bounds.captured_stdout_bytes}`,
    );
  }
  const disk = [peerReal, workspace].reduce(
    (total, root) => total + directoryBytes(root),
    0,
  );
  if (disk > bounds.disk_bytes) {
    throw new Refusal(
      "disk_limit_exceeded",
      `the probe wrote ${disk} bytes, over the effective ${bounds.disk_bytes}`,
    );
  }
  writeFileSync(evidencePath, serialized);

  if (spawnRefusal !== null && failure === null) failure = spawnRefusal;
  if (failure !== null) throw failure;
  console.log(JSON.stringify({ probeOnly: true, status: "ok", tuple: evidence.tuple, platform }));
  return EXIT.ok;
}

/// Kills the peer, waits for the observed exit, and never leaves it running.
/// Every non-exited outcome is reported as unknown cleanup, and the kill is
/// retried with SIGKILL before the probe gives up.
async function stopPeer(query, boundMs) {
  const attempts = [
    () => query?.killNative?.(),
    () => query?.killNative?.("SIGKILL"),
  ];
  for (const attempt of attempts) {
    try {
      attempt();
    } catch {
      // A kill that throws is still followed by an observation attempt.
    }
    let timer;
    const bound = new Promise((resolve) => {
      timer = setTimeout(() => resolve("expired"), boundMs);
    });
    try {
      if (typeof query?.joinNative !== "function") {
        return { join: "unknown", killed: true, observed: false };
      }
      const outcome = await Promise.race([
        query.joinNative().then((exit) => ({ exit })),
        bound,
      ]);
      if (outcome !== "expired") {
        return { join: "killed", killed: true, observed: true, ...(outcome.exit ?? {}) };
      }
    } finally {
      clearTimeout(timer);
    }
  }
  return { join: "unknown", killed: true, observed: false };
}

async function settle(query, boundMs) {
  if (query === undefined) return { join: "absent", code: null, signal: null };
  if (typeof query.joinNative !== "function") {
    // A peer that cannot report its own exit is unknown cleanup, not a clean
    // exit. Kill it so the probe cannot leave a child behind, and say so.
    return await stopPeer(query, boundMs);
  }
  let timer;
  const bound = new Promise((resolve) => {
    timer = setTimeout(() => resolve("expired"), boundMs);
  });
  try {
    const outcome = await Promise.race([query.joinNative().then((exit) => ({ exit })), bound]);
    if (outcome === "expired") {
      // The join timed out. That is unknown cleanup, never a success: stop the
      // peer and report what was actually observed.
      const stopped = await stopPeer(query, boundMs);
      return { ...stopped, join: stopped.join === "killed" ? "killed" : "unknown", timedOut: true };
    }
    return { join: "exited", ...(outcome.exit ?? { code: null, signal: null }) };
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
