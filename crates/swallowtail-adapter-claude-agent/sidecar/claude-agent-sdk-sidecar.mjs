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
// subagents, no system prompt, no session persistence, and an explicitly
// admitted tool set. Unknown semantics fail closed.
//
// The admitted tool set and the permission mode are decided by the host and
// arrive on `open`. This process never widens either: an unadmitted tool is
// denied without asking, an unknown tool name is refused before the SDK is
// constructed, and `bypassPermissions` — along with every other
// auto-approving upstream mode — is refused the same way and can never reach
// the SDK.
//
// The SDK supplies no joined stop: its cleanup races a bounded timer and
// discards the outcome, and its own escalation is unref'd and reaches only
// the direct child. This sidecar therefore holds its own native child handle
// through `spawnClaudeCodeProcess`, joins that handle independently of SDK
// cleanup, and reports an explicit three-valued close state. `unconfirmed` is
// a cleanup failure that the host escalates through its descendant-tree
// termination authority; it is never a slow success.

import { spawn } from "node:child_process";
import { realpathSync } from "node:fs";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { pathToFileURL } from "node:url";
import process from "node:process";

const WIRE = "swallowtail-claude-agent-sdk-jsonl-v1";
const BEHAVIOR = "claude-agent.sdk-v1";
const SDK_PACKAGE = "@anthropic-ai/claude-agent-sdk";
const SDK_VERSION = "0.3.259";
const NATIVE_VERSION = "2.1.259";
const NODE_FLOOR = [22, 19, 0];

const MAXIMUM_RECORD_BYTES = 1024 * 1024;
const MAXIMUM_COMMAND_ID_BYTES = 128;
const MAXIMUM_PENDING_COMMANDS = 16;
const MAXIMUM_PENDING_CALLBACKS = 8;
const MAXIMUM_CAPABILITIES = 64;
const MAXIMUM_CAPABILITY_BYTES = 96;
// Keep callback text aligned with the runtime's existing
// MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES bound.
const MAXIMUM_CALLBACK_TEXT_BYTES = 128;
const MAXIMUM_PROMPT_BYTES = 256 * 1024;
const MINIMUM_JOIN_BOUND_MS = 100;
const MAXIMUM_JOIN_BOUND_MS = 60_000;
const SDK_CONTROL_BOUND_MS = 60_000;
const READINESS_REQUESTED = "requested-with-supported-list";
const READINESS_CONFIRMED = "confirmed";

// Every tool this route can admit, in the exact order the host sends them.
// Background shells, terminal, notebook, and network tools stay outside this
// route. Bash is admitted only through the explicit Contract 023/041-mediated
// path below.
//
// The admitted subset is passed as `tools`, which restricts availability. It
// is never passed as `allowedTools`, which auto-allows a tool without
// prompting and would bypass the host's per-use admission decision.
const ADMISSIBLE_TOOLS = ["Read", "Glob", "Grep", "Edit", "Write", "MultiEdit", "Bash"];
const DEFAULT_TOOLS = ["Read", "Glob", "Grep"];
// Never available on this route, whatever the host admits. Anything
// admissible but not admitted is added to this list at open.
const NEVER_AVAILABLE_TOOLS = [
  "BashOutput",
  "KillShell",
  "NotebookEdit",
  "WebFetch",
  "WebSearch",
  "Task",
];

// The only three modes this route represents. `bypassPermissions`, `auto`,
// and `dontAsk` auto-approve tool use, so they are refused by name rather
// than merely omitted, and the refusal happens before the SDK is loaded.
const PERMISSION_MODES = ["default", "plan", "acceptEdits"];
const REJECTED_PERMISSION_MODES = ["bypassPermissions", "auto", "dontAsk"];

const ENV_SDK_MODULE = "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE";
const ENV_NATIVE_BINARY = "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY";
const ENV_MANIFEST = "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST";

// Keep the native child useful for its local runtime and locale without
// inheriting credentials, provider switches, Claude configuration, or other
// ambient application state. LC_* is the only prefix allowlist; every other
// accepted name is explicit, including the macOS process essentials.
const CHILD_ENV_EXACT_KEYS = new Set([
  "HOME",
  "PATH",
  "TMPDIR",
  "LANG",
  "USER",
  "SHELL",
  "TERM",
  "COLORTERM",
  "__CF_USER_TEXT_ENCODING",
  "XPC_FLAGS",
  "XPC_SERVICE_NAME",
  "MallocNanoZone",
  "COMMAND_MODE",
]);

// Names from the frozen 0.3.259 SDKResultSuccess/SDKResultError union. The
// sidecar records only presence booleans; values such as result text, usage,
// UUIDs, paths, and provider errors never cross the wire.
const SDK_RESULT_FIELD_NAMES = [
  "type",
  "subtype",
  "duration_ms",
  "duration_api_ms",
  "ttft_ms",
  "ttft_stream_ms",
  "time_to_request_ms",
  "user_message_uuid",
  "user_message_uuids",
  "request_sent_wall_ms",
  "time_to_request_from_spawn_ms",
  "warm_spare_claimed",
  "time_origin_ms",
  "is_error",
  "api_error_status",
  "num_turns",
  "result",
  "stop_reason",
  "total_cost_usd",
  "usage",
  "modelUsage",
  "permission_denials",
  "queued_turn_count",
  "structured_output",
  "deferred_tool_use",
  "terminal_reason",
  "fast_mode_state",
  "fast_mode_disabled_reason",
  "origin",
  "uuid",
  "session_id",
  "errors",
];

function childEnvironment() {
  return Object.fromEntries(
    Object.entries(process.env).filter(
      ([key, value]) =>
        typeof value === "string" && (CHILD_ENV_EXACT_KEYS.has(key) || key.startsWith("LC_")),
    ),
  );
}

const COMMANDS = new Set(["open", "query", "interrupt", "set_permission_mode", "close"]);
const COMMAND_FAILURE_CODES = new Set([
  "missing_environment",
  "invalid_command",
  "tools_invalid",
  "permission_mode_invalid",
  "permission_mode_rejected",
  "sdk_unavailable",
  "sdk_export_missing",
  "native_manifest_unavailable",
  "native_version_mismatch",
  "capabilities_overflow",
  "capabilities_invalid",
  "account_not_first_party",
  // Retired compatibility label: subscription evidence is observation-only,
  // so the sidecar no longer rejects when subscriptionType is absent.
  "account_not_subscription",
  "already_open",
  "node_runtime_unsupported",
  "construction_failed",
  "initialization_failed",
  "init_missing",
  "cwd_mismatch",
  "model_mismatch",
  "model_missing",
  "supported_model_rejected",
  "account_unavailable",
  "native_child_unavailable",
  "not_open",
  "turn_active",
  "prompt_too_large",
  "interrupt_failed",
  "permission_mode_unsupported",
  "permission_mode_failed",
  "permission_mode_unconfirmed",
  "unknown_command",
  "command_failed",
]);

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
  requestedModel: null,
  tools: DEFAULT_TOOLS,
  permissionMode: "default",
  capabilities: [],
  native: null,
  reader: null,
  closeTimeline: [],
  initialized: false,
  effectiveModel: null,
  supportedModels: [],
  supportedModelsAvailable: false,
  turnActive: false,
  pending: new Map(),
  usedIds: new Set(),
  callbacks: new Map(),
  nextCallbackId: 0,
  closed: false,
  sdkTransportCloseRan: false,
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
  const safeCode = COMMAND_FAILURE_CODES.has(code) ? code : "command_failed";
  await respond(id, command, false, {
    code: safeCode,
    message: `sidecar command failed: ${safeCode}`,
  });
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

/// Parses the host's admitted tool set. An unknown name, a repeat, an empty
/// set, or a non-array fails closed before the SDK exists.
function admittedTools(values) {
  if (values === undefined) {
    return DEFAULT_TOOLS;
  }
  if (!Array.isArray(values) || values.length === 0) {
    throw new SidecarFailure("tools_invalid");
  }
  const admitted = [];
  for (const value of values) {
    if (typeof value !== "string" || !ADMISSIBLE_TOOLS.includes(value) || admitted.includes(value)) {
      throw new SidecarFailure("tools_invalid");
    }
    admitted.push(value);
  }
  return admitted;
}

/// Parses the host's permission mode. An auto-approving upstream mode is
/// refused by name, so it can never reach the SDK.
function admittedPermissionMode(value) {
  if (value === undefined) {
    return "default";
  }
  if (typeof value !== "string") {
    throw new SidecarFailure("permission_mode_invalid");
  }
  if (REJECTED_PERMISSION_MODES.includes(value)) {
    throw new SidecarFailure("permission_mode_rejected");
  }
  if (!PERMISSION_MODES.includes(value)) {
    throw new SidecarFailure("permission_mode_invalid");
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

/// Bounds one SDK control exchange. The SDK's control methods can wait on the
/// native process, so open never inherits an unbounded await from the wrapper.
async function boundedControl(operation, failureCode) {
  let timer;
  const timeout = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new SidecarFailure(failureCode)), SDK_CONTROL_BOUND_MS);
  });
  try {
    return await Promise.race([Promise.resolve().then(operation), timeout]);
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure(failureCode);
  } finally {
    clearTimeout(timer);
  }
}

/// Retains the native child independently of SDK cleanup. The SDK's own
/// bounded wait discards its outcome, so only this handle may be joined.
class NativeChild {
  constructor(child) {
    this.child = child;
    this.exited = false;
    this.exitEvent = null;
    this.exitCode = null;
    this.exitSignal = null;
    this.exitObserved = new Promise((resolve) => {
      child.once("error", () => {
        if (this.exitEvent === null) {
          this.exitEvent = "error";
        }
      });
      child.once("exit", (code, signal) => {
        this.exited = true;
        this.exitEvent = "exit";
        this.exitCode = code;
        this.exitSignal = signal;
        resolve();
      });
    });
  }

  evidence() {
    return {
      event: this.exitEvent,
      code: this.exitCode,
      signal: this.exitSignal,
    };
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

function spawnNative({ command, args, cwd, env, signal }) {
  // The host launched this sidecar inside its own descendant-tree authority,
  // so the native child and everything it spawns stay enrolled in that tree.
  // No detachment, no new session, no new process group.
  const child = spawn(command, args, {
    cwd,
    env,
    signal,
    detached: false,
    stdio: ["pipe", "pipe", "pipe"],
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
/// or raw account value ever crosses this wire. Subscription fields are
/// observations, not gates.
function accountProjection(account) {
  const apiProvider = account?.apiProvider;
  if (apiProvider !== "firstParty") {
    throw new SidecarFailure("account_not_first_party");
  }
  return {
    apiProvider,
    subscriptionTypePresent:
      typeof account?.subscriptionType === "string" && account.subscriptionType.length > 0,
    tokenSourcePresent:
      typeof account?.tokenSource === "string" && account.tokenSource.length > 0,
    apiKeySourcePresent:
      typeof account?.apiKeySource === "string" && account.apiKeySource.length > 0,
  };
}

function supportedModelValues(values) {
  if (!Array.isArray(values)) {
    throw new SidecarFailure("initialization_failed");
  }
  const models = [];
  for (const entry of values) {
    const candidates = [entry?.value, entry?.resolvedModel];
    for (const candidate of candidates) {
      if (typeof candidate !== "string" || candidate.length === 0) {
        continue;
      }
      if (!models.includes(candidate)) {
        models.push(candidate);
      }
    }
  }
  return models;
}

function boundedCallbackText(value) {
  const text = typeof value === "string" ? value : "";
  const bytes = Buffer.from(text, "utf8");
  if (bytes.length <= MAXIMUM_CALLBACK_TEXT_BYTES) {
    return { text, byteLength: bytes.length, truncated: false };
  }
  let safeLength = MAXIMUM_CALLBACK_TEXT_BYTES;
  while (safeLength > 0 && (bytes[safeLength] & 0xc0) === 0x80) {
    safeLength -= 1;
  }
  return {
    text: bytes.subarray(0, safeLength).toString("utf8"),
    byteLength: bytes.length,
    truncated: true,
  };
}

function callbackRecord(toolName, callbackId, input) {
  const record = {
    type: "callback",
    id: callbackId,
    callback: "can_use_tool",
    toolName: String(toolName ?? ""),
  };
  if (record.toolName === "Bash") {
    const command = boundedCallbackText(input?.command);
    const description = boundedCallbackText(input?.description);
    record.command = command.text;
    record.commandByteLength = command.byteLength;
    record.description = description.text;
    record.truncated = command.truncated || description.truncated;
  }
  return record;
}

/// Bridges one SDK `canUseTool` request onto the private wire and blocks on
/// the correlated host decision.
///
/// Three rules hold together. The host's admitted set is enforced here
/// first, so an unadmitted tool is denied without ever reaching the consumer.
/// Bash exposes only a bounded command view; every tool's full input remains
/// private and is returned unchanged on allow, because `updatedInput` replaces
/// the input the provider would otherwise use. Admission is never inferred
/// locally: an allowed tool always waits for the host's decision.
function canUseTool(toolName, input) {
  const name = String(toolName ?? "");
  if (!state.tools.includes(name)) {
    return Promise.resolve({ behavior: "deny", message: "tool is outside the admitted set" });
  }
  if (state.callbacks.size >= MAXIMUM_PENDING_CALLBACKS) {
    return Promise.resolve({ behavior: "deny", message: "callback capacity exceeded" });
  }
  state.nextCallbackId += 1;
  const callbackId = `cb-${state.nextCallbackId}`;
  return new Promise((resolve) => {
    state.callbacks.set(callbackId, { resolve, input });
    void writeRecord(callbackRecord(name, callbackId, input));
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
    case "result": {
      const resultFieldPresence = Object.fromEntries(
        [...new Set([...SDK_RESULT_FIELD_NAMES, ...Object.keys(message)])]
          .sort()
          .map((key) => [key, Object.prototype.hasOwnProperty.call(message, key)]),
      );
      const subtype = typeof message.subtype === "string" ? message.subtype : null;
      const errorTextPresent = Object.prototype.hasOwnProperty.call(message, "error");
      let errorTextType = "absent";
      if (errorTextPresent) {
        if (message.error === null) {
          errorTextType = "null";
        } else if (Array.isArray(message.error)) {
          errorTextType = "array";
        } else {
          errorTextType = typeof message.error;
        }
      }
      const numTurns =
        Number.isSafeInteger(message.num_turns) && message.num_turns >= 0
          ? message.num_turns
          : null;
      const durationMs =
        Number.isSafeInteger(message.duration_ms) && message.duration_ms >= 0
          ? message.duration_ms
          : null;
      return [
        {
          event: "turn_ended",
          subtype,
          isError: message.is_error === true,
          numTurns,
          durationMs,
          errorTextPresent,
          errorTextType,
          resultFieldPresence,
          // Keep the existing route field while making the SDK result fields
          // explicit and separately inspectable. Never forward error text.
          stopReason: subtype ?? "",
        },
      ];
    }
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
  requireExactParams(params, ["cwd", "model", "tools", "permissionMode"]);
  const cwd = requireString(params, "cwd");
  const model = requireString(params, "model");
  const tools = admittedTools(params.tools);
  const permissionMode = admittedPermissionMode(params.permissionMode);
  const disallowed = [
    ...NEVER_AVAILABLE_TOOLS,
    ...ADMISSIBLE_TOOLS.filter((tool) => !tools.includes(tool)),
  ];
  state.tools = tools;
  state.permissionMode = permissionMode;
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
        env: childEnvironment(),
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
        tools,
        disallowedTools: disallowed,
        permissionMode,
        canUseTool: (toolName, input) => canUseTool(toolName, input),
        spawnClaudeCodeProcess: (options) => spawnNative(options),
      },
    });
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure("construction_failed");
  }
  state.query = query;

  // The SDK initialize exchange is the open-time handshake. It is deliberately
  // separate from the async-generator's first system/init message, which is
  // evidence for the first user turn rather than an open gate.
  const initialization = await boundedControl(
    () => query.initializationResult(),
    "initialization_failed",
  );
  if (!initialization || typeof initialization !== "object") {
    throw new SidecarFailure("initialization_failed");
  }
  const modelRows = await boundedControl(() => query.supportedModels(), "initialization_failed");
  const supportedModels = supportedModelValues(modelRows);
  const account = await boundedControl(
    () => query.accountInfo(),
    "account_unavailable",
  );
  const readiness = accountProjection(account);
  if (!state.native) {
    throw new SidecarFailure("native_child_unavailable");
  }
  state.cwd = cwd;
  state.requestedModel = model;
  state.capabilities = [];
  state.initialized = false;
  state.effectiveModel = null;
  state.supportedModels = supportedModels;
  state.supportedModelsAvailable = supportedModels.length > 0;
  state.sdkTransportCloseRan = false;
  state.closeTimeline = [];
  return {
    wire: WIRE,
    behavior: BEHAVIOR,
    sdkPackage: SDK_PACKAGE,
    sdkVersion: SDK_VERSION,
    nativeVersion,
    nodeVersion: process.versions.node,
    cwd,
    requestedModel: model,
    readiness: READINESS_REQUESTED,
    supportedModels,
    capabilities: [],
    account: readiness,
    tools,
    permissionMode,
  };
}

// The SDK's streaming-input query stays alive for the session. A generator
// that yields one queued message and then returns is a one-shot prompt, which
// lets the SDK close stdin and report an abort/code-1 result after turn one.
// Keep the source open across turns; only the sidecar close command ends it.
class SessionInput {
  constructor() {
    this.queue = [];
    this.waiters = [];
    this.closed = false;
  }

  next() {
    if (this.queue.length > 0) {
      return Promise.resolve({ value: this.queue.shift(), done: false });
    }
    if (this.closed) {
      return Promise.resolve({ value: undefined, done: true });
    }
    return new Promise((resolve) => this.waiters.push(resolve));
  }

  // Do not let an SDK consumer's per-turn iterator cleanup close the session.
  // The sidecar owns session lifetime and calls close() only from handleClose.
  return() {
    return Promise.resolve({ value: undefined, done: true });
  }

  [Symbol.asyncIterator]() {
    return this;
  }

  push(message) {
    if (this.closed) {
      return;
    }
    const resolve = this.waiters.shift();
    if (resolve) {
      resolve({ value: message, done: false });
    } else {
      this.queue.push(message);
    }
  }

  close() {
    if (this.closed) {
      return;
    }
    this.closed = true;
    while (this.waiters.length > 0) {
      this.waiters.shift()({ value: undefined, done: true });
    }
  }
}

const sessionInput = new SessionInput();

function inputStream() {
  return sessionInput;
}

function pushInput(message) {
  sessionInput.push(message);
}

function canonicalPath(value) {
  try {
    return realpathSync.native(path.resolve(value));
  } catch {
    return null;
  }
}

function cwdMatches(expected, observed) {
  const expectedCanonical = canonicalPath(expected);
  const observedCanonical = canonicalPath(observed);
  return expectedCanonical !== null && expectedCanonical === observedCanonical;
}

function endInput() {
  sessionInput.close();
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
  if (!state.initialized) {
    let first;
    try {
      first = await state.query.next();
    } catch {
      state.turnActive = false;
      throw new SidecarFailure("initialization_failed");
    }
    const system = first?.value;
    if (first?.done === true || system?.type !== "system" || system.subtype !== "init") {
      state.turnActive = false;
      throw new SidecarFailure("init_missing");
    }
    if (typeof system.cwd !== "string" || !cwdMatches(state.cwd, system.cwd)) {
      state.turnActive = false;
      throw new SidecarFailure("cwd_mismatch");
    }
    if (typeof system.model !== "string" || system.model.length === 0) {
      state.turnActive = false;
      throw new SidecarFailure("model_missing");
    }
    if (state.supportedModelsAvailable && !state.supportedModels.includes(system.model)) {
      state.turnActive = false;
      throw new SidecarFailure("supported_model_rejected");
    }
    const capabilities = boundedCapabilities(system.capabilities);
    state.initialized = true;
    state.effectiveModel = system.model;
    state.capabilities = capabilities;
    state.reader = drainQuery();
  }
  await emitEvent({ event: "turn_started" });
  return {
    accepted: true,
    readiness: READINESS_CONFIRMED,
    cwd: state.cwd,
    requestedModel: state.requestedModel,
    model: state.effectiveModel,
    capabilities: state.capabilities,
  };
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

/// Changes the permission mode of the live session.
///
/// The SDK's own mode change resolves without returning a value, so the
/// confirmation this reports is that the SDK accepted the change, not an
/// independent observation of provider-effective policy. A missing method, a
/// throw, or a differently-valued resolution all fail rather than report a
/// silent success, and the admitted tool set never changes here.
async function handleSetPermissionMode(params) {
  if (!state.query) {
    throw new SidecarFailure("not_open");
  }
  requireExactParams(params, ["mode"]);
  const mode = admittedPermissionMode(requireString(params, "mode"));
  if (typeof state.query.setPermissionMode !== "function") {
    throw new SidecarFailure("permission_mode_unsupported");
  }
  let confirmed;
  try {
    confirmed = await state.query.setPermissionMode(mode);
  } catch {
    throw new SidecarFailure("permission_mode_failed");
  }
  if (confirmed !== undefined && confirmed !== mode) {
    throw new SidecarFailure("permission_mode_unconfirmed");
  }
  state.permissionMode = mode;
  return { permissionMode: mode };
}

/// Closes in contract order: interrupt a live turn, end input, dispose SDK
/// state, then join the independently retained native handle to the declared
/// bound. The SDK's own cleanup outcome is never treated as evidence.
async function handleClose(id, command, params) {
  state.closed = true;
  state.closeTimeline = ["close_requested"];
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
      state.closeTimeline.push("interrupt_requested");
      try {
        await Promise.race([state.query.interrupt(), bound.expiry]);
        state.closeTimeline.push("interrupt_completed");
      } catch {
        state.closeTimeline.push("interrupt_failed");
        await emitDiagnostic("warning", "interrupt_before_close_failed");
      }
      state.turnActive = false;
    }
    endInput();
    state.closeTimeline.push("session_input_closed");
    try {
      state.query.close();
      state.sdkTransportCloseRan = true;
      state.closeTimeline.push("sdk_transport_close_ran");
    } catch {
      state.closeTimeline.push("sdk_transport_close_failed");
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
  state.closeTimeline.push(joined ? "native_join_exited" : "native_join_survivor");
  bound.cancel();
  const nativeEvidence = state.native?.evidence() ?? {
    event: null,
    code: null,
    signal: null,
  };
  // Report what was observed, not what is hoped. The retained handle still
  // showing an unexited child is a positive survivor observation; the host
  // treats that as cleanup failure rather than an absence of news.
  await respond(id, command, true, {
    nativeJoin: joined ? "exited" : "survivor",
    joinBoundMs: boundMs,
    nativeExitObserved: joined,
    nativeExitEvent: nativeEvidence.event,
    nativeExitCode: nativeEvidence.code,
    nativeExitSignal: nativeEvidence.signal,
    sdkTransportCloseRan: state.sdkTransportCloseRan,
    closeTimeline: state.closeTimeline,
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
      case "set_permission_mode":
        data = await handleSetPermissionMode(params);
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
