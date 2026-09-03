// swallowtail-claude-agent-sdk-sidecar — source-tagged Swallowtail adapter asset.
//
// This file is owned by the `swallowtail-adapter-claude-agent` crate source
// tag. The consuming application provisions the exact approved Node runtime,
// this entry point, the exact `@anthropic-ai/claude-agent-sdk` package, its
// peer dependencies, and the exact platform package that carries the native
// `claude` binary. Swallowtail never installs, vendors, updates, repairs, or
// redistributes any of them.
//
// This process speaks the private strict LF-JSON wire
// `swallowtail-claude-agent-sdk-jsonl-v1` on stdin/stdout and nothing else.
// The provisioned SDK module, native binary, and shipped manifest arrive
// through the application-approved environment
// (`CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE`,
// `CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY`,
// `CLAUDE_AGENT_SDK_SIDECAR_MANIFEST`), never through ambient discovery;
// `process.argv` is intentionally unused.
//
// Credential non-custody is structural, not incidental. Only the `.` entry
// point is imported. The `/bridge` and `/browser` subpaths, which declare raw
// access tokens, minted worker credentials, and OAuth credential messages,
// are never imported and never named here as reachable module specifiers.
// No API-key helper, cloud auth refresh, or token field is ever set, read, or
// forwarded, and `env` is always passed explicitly so the native binary can
// never silently inherit an API key and switch away from the user's
// subscription.
//
// Ambient behavior is suppressed by construction: empty setting sources, an
// explicit empty skill list, no MCP servers, no plugins, no hooks, no
// subagents, no system prompt, no session persistence, and a read-only tool
// set. Unknown semantics fail closed.
//
// The SDK supplies no joined stop: its cleanup races a bounded timer and
// discards the outcome, and its own escalation is unref'd and reaches only
// the direct child. This sidecar therefore holds its own native child handle
// through `spawnClaudeCodeProcess`, joins that handle independently of SDK
// cleanup, and reports an explicit three-valued close state. `unconfirmed` is
// a cleanup failure that the host escalates through its descendant-tree
// termination authority; it is never a slow success.

import { spawn } from "node:child_process";
import { readFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import process from "node:process";

const WIRE = "swallowtail-claude-agent-sdk-jsonl-v1";
const BEHAVIOR = "claude-agent.sdk-v1";
const SDK_PACKAGE = "@anthropic-ai/claude-agent-sdk";
const SDK_VERSION = "0.3.258";
const NATIVE_VERSION = "2.1.258";
const NODE_FLOOR = [22, 19, 0];

const MAXIMUM_RECORD_BYTES = 1024 * 1024;
const MAXIMUM_COMMAND_ID_BYTES = 128;
const MAXIMUM_PENDING_COMMANDS = 16;
const MAXIMUM_PENDING_CALLBACKS = 8;
const MAXIMUM_CAPABILITIES = 64;
const MAXIMUM_CAPABILITY_BYTES = 96;
const MAXIMUM_PROMPT_BYTES = 256 * 1024;
const MINIMUM_JOIN_BOUND_MS = 100;
const MAXIMUM_JOIN_BOUND_MS = 60_000;

// Read-only tools only. Bash, terminal, and every write tool stay outside
// this route until Contract 023 process authority and Contract 041 mediation
// evidence admit them.
//
// This list is passed as `tools`, which restricts availability. It is never
// passed as `allowedTools`, which auto-allows a tool without prompting and
// would bypass the host's per-use admission decision.
const ALLOWED_TOOLS = ["Read", "Glob", "Grep"];
const DISALLOWED_TOOLS = [
  "Bash",
  "BashOutput",
  "KillShell",
  "Edit",
  "Write",
  "NotebookEdit",
  "WebFetch",
  "WebSearch",
  "Task",
];

const ENV_SDK_MODULE = "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE";
const ENV_NATIVE_BINARY = "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY";
const ENV_MANIFEST = "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST";

const COMMANDS = new Set(["open", "query", "interrupt", "close"]);

// Keep the stdout wire exclusive: SDK or dependency console output must never
// corrupt framing. Diagnostics belong on stderr, which the host bounds.
console.log = (...args) => process.stderr.write(`${args.join(" ")}\n`);
console.info = console.log;
console.warn = console.log;
console.error = console.log;

class SidecarFailure extends Error {
  constructor(code) {
    super(code);
    this.code = code;
  }
}

const state = {
  sdk: null,
  query: null,
  cwd: null,
  capabilities: [],
  native: null,
  reader: null,
  turnActive: false,
  pending: new Map(),
  usedIds: new Set(),
  callbacks: new Map(),
  nextCallbackId: 0,
  closed: false,
};

let writes = Promise.resolve();

function writeRecord(record) {
  const line = `${JSON.stringify(record)}\n`;
  writes = writes.then(
    () =>
      new Promise((resolve, reject) => {
        process.stdout.write(line, (error) => (error ? reject(error) : resolve()));
      }),
  );
  return writes;
}

async function respond(id, command, success, body) {
  const record = { type: "response", id, command, success };
  if (success) {
    record.data = body ?? {};
  } else {
    record.failure = body;
  }
  await writeRecord(record);
}

async function respondFailure(id, command, code) {
  await respond(id, command, false, { code, message: `sidecar command failed: ${code}` });
}

async function emitDiagnostic(level, code) {
  await writeRecord({
    type: "diagnostic",
    level,
    code,
    message: `sidecar diagnostic: ${code}`,
  });
}

async function emitEvent(event) {
  await writeRecord({ type: "event", ...event });
}

async function terminal(code) {
  try {
    await writeRecord({
      type: "terminal",
      failure: { code, message: `sidecar terminated: ${code}` },
    });
  } finally {
    process.exit(1);
  }
}

function checkNodeFloor() {
  const parts = process.versions.node.split(".").map((part) => Number.parseInt(part, 10));
  for (let index = 0; index < NODE_FLOOR.length; index += 1) {
    const actual = parts[index] || 0;
    if (actual !== NODE_FLOOR[index]) {
      return actual > NODE_FLOOR[index];
    }
  }
  return true;
}

function requireEnvironment(name) {
  const value = process.env[name];
  if (typeof value !== "string" || value.length === 0) {
    throw new SidecarFailure("missing_environment");
  }
  return value;
}

function requireExactParams(params, allowed) {
  if (!params || typeof params !== "object" || Array.isArray(params)) {
    throw new SidecarFailure("invalid_command");
  }
  for (const key of Object.keys(params)) {
    if (!allowed.includes(key)) {
      throw new SidecarFailure("invalid_command");
    }
  }
  return params;
}

function requireString(params, field) {
  const value = params[field];
  if (typeof value !== "string" || value.length === 0) {
    throw new SidecarFailure("invalid_command");
  }
  return value;
}

async function importSdk() {
  let sdk;
  try {
    // The default entry point only. `/bridge` and `/browser` declare raw
    // credential parameters and are never reachable from this process.
    sdk = await import(pathToFileURL(requireEnvironment(ENV_SDK_MODULE)).href);
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure("sdk_unavailable");
  }
  if (typeof sdk.query !== "function") {
    throw new SidecarFailure("sdk_export_missing");
  }
  return sdk;
}

async function readNativeVersion() {
  let manifest;
  try {
    manifest = JSON.parse(await readFile(requireEnvironment(ENV_MANIFEST), "utf8"));
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure("native_manifest_unavailable");
  }
  const version = manifest?.version;
  if (typeof version !== "string" || version !== NATIVE_VERSION) {
    throw new SidecarFailure("native_version_mismatch");
  }
  return version;
}

/// One shared bound for every await inside close. The timer is unreferenced,
/// so a resolved close never keeps the event loop alive.
function boundedWait(boundMs) {
  let timer;
  const expiry = new Promise((resolve) => {
    timer = setTimeout(resolve, boundMs);
    if (typeof timer.unref === "function") {
      timer.unref();
    }
  });
  return { expiry, cancel: () => clearTimeout(timer) };
}

/// Retains the native child independently of SDK cleanup. The SDK's own
/// bounded wait discards its outcome, so only this handle may be joined.
class NativeChild {
  constructor(child) {
    this.child = child;
    this.exited = false;
    this.exitObserved = new Promise((resolve) => {
      child.once("exit", () => {
        this.exited = true;
        resolve();
      });
    });
  }

  /// Joins the retained handle to `boundMs`. Returns true only on an
  /// observed exit; an elapsed bound is never evidence of exit.
  async join(boundMs) {
    if (this.exited) {
      return true;
    }
    let timer;
    const expiry = new Promise((resolve) => {
      timer = setTimeout(() => resolve(false), boundMs);
    });
    try {
      return await Promise.race([this.exitObserved.then(() => true), expiry]);
    } finally {
      clearTimeout(timer);
    }
  }
}

function spawnNative(command, args, options) {
  // The host launched this sidecar inside its own descendant-tree authority,
  // so the native child and everything it spawns stay enrolled in that tree.
  // No detachment, no new session, no new process group.
  const child = spawn(command, args, {
    ...options,
    detached: false,
    stdio: options?.stdio ?? ["pipe", "pipe", "pipe"],
  });
  state.native = new NativeChild(child);
  return child;
}

function boundedCapabilities(values) {
  if (values === undefined) {
    return [];
  }
  if (!Array.isArray(values) || values.length > MAXIMUM_CAPABILITIES) {
    throw new SidecarFailure("capabilities_overflow");
  }
  const capabilities = [];
  for (const value of values) {
    if (
      typeof value !== "string" ||
      value.length === 0 ||
      value.length > MAXIMUM_CAPABILITY_BYTES ||
      [...value].some((character) => character.charCodeAt(0) < 0x20)
    ) {
      throw new SidecarFailure("capabilities_invalid");
    }
    capabilities.push(value);
  }
  return capabilities;
}

/// Projects readiness provenance labels only. No email, organization, token,
/// or raw account value ever crosses this wire.
function accountProjection(account, apiKeySource) {
  const apiProvider = account?.apiProvider;
  if (apiProvider !== "firstParty") {
    throw new SidecarFailure("account_not_first_party");
  }
  if (apiKeySource !== "oauth") {
    throw new SidecarFailure("account_not_subscription");
  }
  return {
    apiProvider,
    apiKeySource,
    subscriptionPresent: typeof account?.subscriptionType === "string",
  };
}

function callbackRecord(toolName, callbackId) {
  return {
    type: "callback",
    id: callbackId,
    callback: "can_use_tool",
    toolName: String(toolName ?? ""),
  };
}

/// Bridges one SDK `canUseTool` request onto the private wire and blocks on
/// the correlated host decision.
///
/// Three rules hold together. The exact read-only allow-list is enforced here
/// first, so an unknown tool is denied without ever reaching the consumer.
/// The tool's input never crosses the wire: it is retained privately and
/// returned unchanged on allow, because `updatedInput` replaces the input the
/// provider would otherwise use. And admission is never inferred locally: an
/// allowed tool always waits for the host's decision.
function canUseTool(toolName, input) {
  const name = String(toolName ?? "");
  if (!ALLOWED_TOOLS.includes(name)) {
    return Promise.resolve({ behavior: "deny", message: "tool is outside the read-only set" });
  }
  if (state.callbacks.size >= MAXIMUM_PENDING_CALLBACKS) {
    return Promise.resolve({ behavior: "deny", message: "callback capacity exceeded" });
  }
  state.nextCallbackId += 1;
  const callbackId = `cb-${state.nextCallbackId}`;
  return new Promise((resolve) => {
    state.callbacks.set(callbackId, { resolve, input });
    void writeRecord(callbackRecord(name, callbackId));
  });
}

function resolveCallback(record) {
  const pending = state.callbacks.get(record.id);
  if (!pending) {
    return terminal("callback_unknown");
  }
  if (record.decision !== "allow" && record.decision !== "deny") {
    return terminal("callback_invalid");
  }
  state.callbacks.delete(record.id);
  // An allow returns the provider's own input unchanged. Returning an empty
  // object here would silently destroy the path or pattern the tool needs.
  pending.resolve(
    record.decision === "allow"
      ? { behavior: "allow", updatedInput: pending.input }
      : { behavior: "deny", message: "host denied tool use" },
  );
  return Promise.resolve();
}

function projectMessage(message) {
  switch (message.type) {
    case "assistant": {
      const parts = Array.isArray(message.message?.content) ? message.message.content : [];
      const events = [];
      for (const part of parts) {
        if (part?.type === "text") {
          events.push({ event: "output_delta", delta: String(part.text ?? "") });
        } else if (part?.type === "tool_use") {
          events.push({
            event: "tool_started",
            toolCallId: String(part.id ?? ""),
            toolName: String(part.name ?? ""),
          });
        } else if (part?.type !== "thinking") {
          return "unknown";
        }
      }
      return events;
    }
    case "user": {
      const parts = Array.isArray(message.message?.content) ? message.message.content : [];
      const events = [];
      for (const part of parts) {
        if (part?.type === "tool_result") {
          events.push({
            event: "tool_ended",
            toolCallId: String(part.tool_use_id ?? ""),
            isError: part.is_error === true,
          });
        }
      }
      return events;
    }
    case "result":
      return [
        {
          event: "turn_ended",
          stopReason: String(message.subtype ?? ""),
          isError: message.is_error === true,
        },
      ];
    case "stream_event":
    case "system":
      return [{ event: "progress" }];
    default:
      return "unknown";
  }
}

async function drainQuery() {
  try {
    for await (const message of state.query) {
      const projected = projectMessage(message);
      if (projected === "unknown") {
        await terminal("unknown_message");
        return;
      }
      for (const event of projected) {
        await emitEvent(event);
      }
      if (projected.some((event) => event.event === "turn_ended")) {
        state.turnActive = false;
      }
    }
  } catch {
    state.turnActive = false;
    await emitEvent({ event: "turn_failed" });
  }
}

async function handleOpen(params) {
  if (state.query) {
    throw new SidecarFailure("already_open");
  }
  requireExactParams(params, ["cwd", "model"]);
  const cwd = requireString(params, "cwd");
  const model = requireString(params, "model");
  if (!checkNodeFloor()) {
    throw new SidecarFailure("node_runtime_unsupported");
  }
  const nativeBinary = requireEnvironment(ENV_NATIVE_BINARY);
  const nativeVersion = await readNativeVersion();
  const sdk = await importSdk();
  state.sdk = sdk;

  let query;
  try {
    query = sdk.query({
      prompt: inputStream(),
      options: {
        cwd,
        model,
        executable: "node",
        pathToClaudeCodeExecutable: nativeBinary,
        // Always explicit: omission would inherit `process.env` and could
        // silently select API-key authentication over the subscription.
        env: {},
        settingSources: [],
        skills: [],
        plugins: [],
        agents: {},
        mcpServers: {},
        strictMcpConfig: true,
        hooks: {},
        persistSession: false,
        includePartialMessages: false,
        // `tools` restricts availability; `allowedTools` is deliberately
        // never set, because it auto-allows without prompting.
        tools: ALLOWED_TOOLS,
        disallowedTools: DISALLOWED_TOOLS,
        permissionMode: "default",
        canUseTool: (toolName, input) => canUseTool(toolName, input),
        spawnClaudeCodeProcess: (command, args, options) => spawnNative(command, args, options),
      },
    });
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure("construction_failed");
  }
  state.query = query;

  let initialized;
  try {
    initialized = await query.next();
  } catch {
    throw new SidecarFailure("initialization_failed");
  }
  const system = initialized?.value;
  if (initialized?.done === true || system?.type !== "system" || system.subtype !== "init") {
    throw new SidecarFailure("initialization_failed");
  }
  if (typeof system.cwd !== "string" || system.cwd !== cwd) {
    throw new SidecarFailure("cwd_mismatch");
  }
  // The effective model is proved from the runtime's own init evidence, never
  // assumed from the request.
  if (typeof system.model !== "string" || system.model !== model) {
    throw new SidecarFailure("model_mismatch");
  }
  const capabilities = boundedCapabilities(system.capabilities);
  let account;
  try {
    account = await query.accountInfo();
  } catch {
    throw new SidecarFailure("account_unavailable");
  }
  const readiness = accountProjection(account, system.apiKeySource);
  if (!state.native) {
    throw new SidecarFailure("native_child_unavailable");
  }
  state.cwd = cwd;
  state.capabilities = capabilities;
  state.reader = drainQuery();
  return {
    wire: WIRE,
    behavior: BEHAVIOR,
    sdkPackage: SDK_PACKAGE,
    sdkVersion: SDK_VERSION,
    nativeVersion,
    nodeVersion: process.versions.node,
    cwd,
    model: system.model,
    capabilities,
    account: readiness,
    tools: ALLOWED_TOOLS,
  };
}

let inputResolve = null;
const inputQueue = [];
let inputClosed = false;

async function* inputStream() {
  while (!inputClosed) {
    if (inputQueue.length === 0) {
      await new Promise((resolve) => {
        inputResolve = resolve;
      });
      continue;
    }
    yield inputQueue.shift();
  }
}

function pushInput(message) {
  inputQueue.push(message);
  if (inputResolve) {
    const resolve = inputResolve;
    inputResolve = null;
    resolve();
  }
}

function endInput() {
  inputClosed = true;
  if (inputResolve) {
    const resolve = inputResolve;
    inputResolve = null;
    resolve();
  }
}

async function handleQuery(params) {
  if (!state.query) {
    throw new SidecarFailure("not_open");
  }
  if (state.turnActive) {
    throw new SidecarFailure("turn_active");
  }
  requireExactParams(params, ["text"]);
  const text = requireString(params, "text");
  if (text.length > MAXIMUM_PROMPT_BYTES) {
    throw new SidecarFailure("prompt_too_large");
  }
  state.turnActive = true;
  pushInput({
    type: "user",
    message: { role: "user", content: [{ type: "text", text }] },
    parent_tool_use_id: null,
    session_id: "",
  });
  await emitEvent({ event: "turn_started" });
  return { accepted: true };
}

async function handleInterrupt(params) {
  if (!state.query) {
    throw new SidecarFailure("not_open");
  }
  requireExactParams(params, []);
  let receipt;
  try {
    receipt = await state.query.interrupt();
  } catch {
    throw new SidecarFailure("interrupt_failed");
  }
  // A receipt only exists on CLIs advertising `interrupt_receipt_v1`. Absence
  // is reported, never assumed away.
  return {
    interrupted: true,
    receipt: state.capabilities.includes("interrupt_receipt_v1") && receipt !== undefined,
  };
}

/// Closes in contract order: interrupt a live turn, end input, dispose SDK
/// state, then join the independently retained native handle to the declared
/// bound. The SDK's own cleanup outcome is never treated as evidence.
async function handleClose(id, command, params) {
  state.closed = true;
  let boundMs;
  try {
    requireExactParams(params, ["joinBoundMs"]);
    boundMs = params.joinBoundMs;
    if (
      !Number.isSafeInteger(boundMs) ||
      boundMs < MINIMUM_JOIN_BOUND_MS ||
      boundMs > MAXIMUM_JOIN_BOUND_MS
    ) {
      throw new SidecarFailure("invalid_command");
    }
  } catch (error) {
    await respondFailure(id, command, error instanceof SidecarFailure ? error.code : "invalid_command");
    await writes;
    process.exit(1);
    return;
  }
  // The declared bound governs everything after this point. The native join
  // starts first, so no SDK-side drain can consume the bound and leave the
  // join unreachable, and every remaining await is raced against the same
  // bound. An expired bound is not evidence of exit; it hands the tree to the
  // host's termination authority.
  const joinPromise = state.native ? state.native.join(boundMs) : Promise.resolve(false);
  const bound = boundedWait(boundMs);
  if (state.query) {
    if (state.turnActive) {
      try {
        await Promise.race([state.query.interrupt(), bound.expiry]);
      } catch {
        await emitDiagnostic("warning", "interrupt_before_close_failed");
      }
      state.turnActive = false;
    }
    endInput();
    try {
      state.query.close();
    } catch {
      await emitDiagnostic("warning", "sdk_close_failed");
    }
    if (state.reader) {
      try {
        await Promise.race([state.reader, bound.expiry]);
      } catch {
        await emitDiagnostic("warning", "reader_drain_failed");
      }
    }
  }
  for (const pending of state.callbacks.values()) {
    pending.resolve({ behavior: "deny", message: "session closing" });
  }
  state.callbacks.clear();
  const joined = await joinPromise;
  bound.cancel();
  // Report what was observed, not what is hoped. The retained handle still
  // showing an unexited child is a positive survivor observation; the host
  // treats that as cleanup failure rather than an absence of news.
  await respond(id, command, true, {
    nativeJoin: joined ? "exited" : "survivor",
    joinBoundMs: boundMs,
    nativeExitObserved: joined,
  });
  await writes;
  process.exit(joined ? 0 : 1);
}

async function dispatch(record) {
  const { id, command } = record;
  const params = record.params ?? {};
  if (command === "close") {
    await handleClose(id, command, params);
    return;
  }
  try {
    let data;
    switch (command) {
      case "open":
        data = await handleOpen(params);
        break;
      case "query":
        data = await handleQuery(params);
        break;
      case "interrupt":
        data = await handleInterrupt(params);
        break;
      default:
        throw new SidecarFailure("unknown_command");
    }
    await respond(id, command, true, data);
  } catch (error) {
    if (error instanceof SidecarFailure) {
      await respondFailure(id, command, error.code);
    } else {
      await respondFailure(id, command, "command_failed");
    }
  } finally {
    state.pending.delete(id);
  }
}

function validCommandId(id) {
  return (
    typeof id === "string" &&
    id.length > 0 &&
    id.length <= MAXIMUM_COMMAND_ID_BYTES &&
    ![...id].some((character) => character.charCodeAt(0) < 0x20 || character.charCodeAt(0) === 0x7f)
  );
}

function handleLine(line) {
  if (line.length > MAXIMUM_RECORD_BYTES) {
    return terminal("record_too_large");
  }
  if (line.length === 0) {
    return terminal("empty_record");
  }
  let record;
  try {
    record = JSON.parse(line);
  } catch {
    return terminal("malformed_json");
  }
  if (!record || (record.type !== "command" && record.type !== "callback_response")) {
    return terminal(record?.type === undefined ? "missing_type" : "unknown_record");
  }
  if (record.type === "callback_response") {
    if (!validCommandId(record.id)) {
      return terminal("invalid_command");
    }
    return resolveCallback(record);
  }
  if (!validCommandId(record.id) || !COMMANDS.has(record.command)) {
    return terminal("invalid_command");
  }
  if (state.usedIds.has(record.id)) {
    return terminal("command_id_reused");
  }
  state.usedIds.add(record.id);
  state.pending.set(record.id, record.command);
  if (state.pending.size > MAXIMUM_PENDING_COMMANDS) {
    return terminal("too_many_pending");
  }
  return dispatch({ id: record.id, command: record.command, params: record.params });
}

let buffer = "";
process.stdin.setEncoding("utf8");
process.stdin.on("data", (chunk) => {
  buffer += chunk;
  let end = buffer.indexOf("\n");
  while (end !== -1) {
    const line = buffer.slice(0, end);
    buffer = buffer.slice(end + 1);
    void handleLine(line);
    end = buffer.indexOf("\n");
  }
  if (buffer.length > MAXIMUM_RECORD_BYTES) {
    void terminal("record_too_large");
  }
});
process.stdin.on("end", () => {
  if (state.closed) {
    return;
  }
  endInput();
  if (state.query) {
    try {
      state.query.close();
    } catch {
      process.exitCode = 1;
    }
  }
  // Stdin EOF is not a joined close: the host escalates through its
  // descendant-tree authority when it observes no explicit close state.
  process.exit(process.exitCode ?? 1);
});
process.on("unhandledRejection", () => {
  void terminal("internal_error");
});
