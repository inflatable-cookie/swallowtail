// swallowtail-pi-sdk-sidecar — source-tagged Swallowtail adapter asset.
//
// This file is owned by the `swallowtail-adapter-pi` crate source tag. The
// consuming application provisions the exact approved Node runtime, this entry
// point, and the exact `@earendil-works/pi-coding-agent` SDK package through a
// host-approved launch recipe. This process speaks the private strict LF-JSON
// wire `swallowtail-pi-sdk-jsonl-v1` on stdin/stdout and nothing else. The
// exact SDK module path, agent directory, and session directory arrive through
// the application-approved environment (`PI_SDK_SIDECAR_SDK_MODULE`,
// `PI_SDK_SIDECAR_AGENT_DIR`, `PI_SDK_SIDECAR_SESSION_DIR`), never through
// ambient discovery; `process.argv` is intentionally unused.
//
// Ambient behavior is suppressed by construction: in-memory settings with
// retry and compaction disabled, resource loading restricted to explicit
// no-* flags, no model network, no extension/skill/prompt/theme/context
// loading, and no update checks. Unknown semantics fail closed.

import { realpath } from "node:fs/promises";
import { isAbsolute, join, relative, sep } from "node:path";
import { pathToFileURL } from "node:url";
import process from "node:process";

const WIRE = "swallowtail-pi-sdk-jsonl-v1";
const BEHAVIOR = "pi.sdk-sidecar-v1";
const SDK_PACKAGE = "@earendil-works/pi-coding-agent";
const SDK_VERSION = "0.84.2";
const NODE_FLOOR = [22, 19, 0];

const MAXIMUM_RECORD_BYTES = 1024 * 1024;
const MAXIMUM_COMMAND_ID_BYTES = 128;
const MAXIMUM_PENDING_COMMANDS = 16;
const MAXIMUM_REPLAY_ITEMS = 1024;
const MAXIMUM_IMAGE_BYTES = 1024 * 1024;
const MAXIMUM_CATALOGUE_MODELS = 256;
const MAXIMUM_CATALOGUE_TEXT_BYTES = 256;
const TOOLS = ["read", "grep", "find", "ls"];

// Application-provisioned inputs arrive through the host-approved environment,
// never through ambient discovery: the host launches this process with a
// fully cleared environment containing only approved entries.
const ENV_SDK_MODULE = "PI_SDK_SIDECAR_SDK_MODULE";
const ENV_AGENT_DIR = "PI_SDK_SIDECAR_AGENT_DIR";
const ENV_SESSION_DIR = "PI_SDK_SIDECAR_SESSION_DIR";

const COMMANDS = new Set([
  "bootstrap",
  "session_new",
  "session_switch",
  "session_replay",
  "prompt",
  "steer",
  "follow_up",
  "abort",
  "state",
  "close",
]);

const QUALIFIED_UPDATE_KINDS = new Set([
  "start",
  "text_start",
  "text_delta",
  "text_end",
  "thinking_start",
  "thinking_delta",
  "thinking_end",
  "toolcall_start",
  "toolcall_delta",
  "toolcall_end",
  "done",
  "error",
]);

const PROGRESS_EVENTS = new Set([
  "queue_update",
  "session_info_changed",
  "thinking_level_changed",
  "entry_appended",
]);

const DISABLED_FEATURE_EVENTS = new Set([
  "compaction_start",
  "compaction_end",
  "auto_retry_start",
  "auto_retry_end",
  "summarization_retry_scheduled",
  "summarization_retry_attempt_start",
  "summarization_retry_finished",
  "bash_execution_update",
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
  runtime: null,
  sessionManager: null,
  sessionDir: null,
  catalogued: false,
  unsubscribe: null,
  pending: new Map(),
  usedIds: new Set(),
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

function usageProjection(usage) {
  if (!usage || typeof usage !== "object") {
    return undefined;
  }
  const fields = ["input", "output", "cacheRead", "cacheWrite"];
  if (!fields.every((field) => Number.isSafeInteger(usage[field]) && usage[field] >= 0)) {
    return undefined;
  }
  return {
    input: usage.input,
    output: usage.output,
    cacheRead: usage.cacheRead,
    cacheWrite: usage.cacheWrite,
  };
}

function projectMessageUpdate(event) {
  const update = event.assistantMessageEvent;
  if (!update || !QUALIFIED_UPDATE_KINDS.has(update.type)) {
    return "unknown";
  }
  switch (update.type) {
    case "text_delta":
      return { event: "output_delta", delta: String(update.delta ?? "") };
    case "thinking_delta":
      return { event: "reasoning_delta", delta: String(update.delta ?? "") };
    case "thinking_start":
      return { event: "reasoning_start" };
    case "thinking_end":
      return { event: "reasoning_end" };
    default:
      return { event: "progress" };
  }
}

function projectEvent(event) {
  switch (event.type) {
    case "agent_start":
      return { event: "agent_start" };
    case "turn_start":
      return { event: "turn_start" };
    case "turn_end":
      return { event: "turn_end" };
    case "agent_end":
      if (event.willRetry === true) {
        return "retry";
      }
      return { event: "agent_end" };
    case "agent_settled":
      return { event: "agent_settled" };
    case "message_start":
      return { event: "message_start", role: String(event.message?.role ?? "") };
    case "message_update":
      return projectMessageUpdate(event);
    case "message_end": {
      const message = event.message;
      if (message?.role !== "assistant") {
        return { event: "progress" };
      }
      const record = {
        event: "message_end",
        role: "assistant",
        stopReason: String(message.stopReason ?? ""),
      };
      const usage = usageProjection(message.usage);
      if (usage) {
        record.usage = usage;
      }
      return record;
    }
    case "tool_execution_start":
    case "tool_execution_update":
      return {
        event: event.type,
        toolCallId: String(event.toolCallId ?? ""),
        toolName: String(event.toolName ?? ""),
      };
    case "tool_execution_end":
      return {
        event: "tool_execution_end",
        toolCallId: String(event.toolCallId ?? ""),
        toolName: String(event.toolName ?? ""),
        isError: event.isError === true,
      };
    default:
      if (PROGRESS_EVENTS.has(event.type)) {
        return { event: "progress" };
      }
      if (DISABLED_FEATURE_EVENTS.has(event.type)) {
        return "disabled";
      }
      return "unknown";
  }
}

function subscribeSession(session) {
  if (state.unsubscribe) {
    state.unsubscribe();
    state.unsubscribe = null;
  }
  state.unsubscribe = session.subscribe((event) => {
    const projected = projectEvent(event);
    if (projected === "unknown") {
      void terminal("unknown_event");
    } else if (projected === "disabled") {
      void terminal("unexpected_event");
    } else if (projected === "retry") {
      void terminal("retry_observed");
    } else {
      void writeRecord({ type: "event", ...projected });
    }
  });
}

function sessionSnapshot() {
  const session = state.runtime.session;
  return {
    provider: String(session.model?.provider ?? ""),
    model: String(session.model?.id ?? ""),
    thinkingLevel: String(session.thinkingLevel ?? ""),
    cwd: state.runtime.cwd,
    sessionRef: session.sessionId ?? null,
    sessionId: session.sessionId,
    idle: session.isIdle === true,
    streaming: session.isStreaming === true,
    messages: session.messages.length,
    tools: session.getActiveToolNames(),
  };
}

function requireBootstrapped() {
  if (!state.runtime) {
    throw new SidecarFailure("not_bootstrapped");
  }
  return state.runtime;
}

function requireString(params, field) {
  const value = params[field];
  if (typeof value !== "string" || value.length === 0) {
    throw new SidecarFailure("invalid_command");
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

function parseImages(params) {
  if (params.images === undefined) {
    return undefined;
  }
  if (!Array.isArray(params.images) || params.images.length > 1) {
    throw new SidecarFailure("invalid_command");
  }
  return params.images.map((image) => {
    if (
      !image ||
      typeof image !== "object" ||
      image.mimeType !== "image/png" ||
      typeof image.data !== "string" ||
      image.data.length === 0 ||
      image.data.length > Math.ceil(MAXIMUM_IMAGE_BYTES / 3) * 4
    ) {
      throw new SidecarFailure("invalid_command");
    }
    return { type: "image", data: image.data, mimeType: image.mimeType };
  });
}

function requireEnvironment(name) {
  const value = process.env[name];
  if (typeof value !== "string" || value.length === 0) {
    throw new SidecarFailure("missing_environment");
  }
  return value;
}

async function importSdk() {
  let sdk;
  try {
    sdk = await import(pathToFileURL(requireEnvironment(ENV_SDK_MODULE)).href);
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure("sdk_unavailable");
  }
  const required = [
    "VERSION",
    "createAgentSessionRuntime",
    "createAgentSessionServices",
    "createAgentSessionFromServices",
    "SessionManager",
    "SettingsManager",
    "ModelRuntime",
  ];
  if (!required.every((name) => sdk[name] !== undefined && sdk[name] !== null)) {
    throw new SidecarFailure("sdk_export_missing");
  }
  if (sdk.VERSION !== SDK_VERSION) {
    throw new SidecarFailure("sdk_version_mismatch");
  }
  return sdk;
}

async function handleCatalogue(sdk) {
  const agentDir = requireEnvironment(ENV_AGENT_DIR);
  const modelRuntime = await sdk.ModelRuntime.create({
    authPath: join(agentDir, "auth.json"),
    modelsPath: null,
    allowModelNetwork: false,
  });
  const models = [];
  for (const provider of modelRuntime.getProviders()) {
    let available;
    try {
      available = await modelRuntime.getAvailable(provider.id);
    } catch {
      throw new SidecarFailure("catalogue_unavailable");
    }
    for (const model of available) {
      const providerId = String(model?.provider ?? "");
      const id = String(model?.id ?? "");
      if (
        providerId.length === 0 ||
        id.length === 0 ||
        providerId.length > MAXIMUM_CATALOGUE_TEXT_BYTES ||
        id.length > MAXIMUM_CATALOGUE_TEXT_BYTES ||
        models.length >= MAXIMUM_CATALOGUE_MODELS
      ) {
        throw new SidecarFailure("catalogue_overflow");
      }
      models.push({ provider: providerId, id });
    }
  }
  state.catalogued = true;
  return {
    wire: WIRE,
    behavior: BEHAVIOR,
    sdkPackage: SDK_PACKAGE,
    sdkVersion: sdk.VERSION,
    nodeVersion: process.versions.node,
    models,
  };
}

async function handleBootstrap(params) {
  if (state.runtime || state.catalogued) {
    throw new SidecarFailure("already_bootstrapped");
  }
  requireExactParams(params, ["cwd", "provider", "model", "thinkingLevel", "catalogueOnly"]);
  if (params.catalogueOnly !== undefined && params.catalogueOnly !== true) {
    throw new SidecarFailure("invalid_command");
  }
  if (params.catalogueOnly === true) {
    // Catalogue mode needs no leased working directory or model binding; its
    // only parameter is the mode flag itself.
    if (Object.keys(params).length !== 1) {
      throw new SidecarFailure("invalid_command");
    }
    if (!checkNodeFloor()) {
      throw new SidecarFailure("node_runtime_unsupported");
    }
    process.env.PI_OFFLINE = process.env.PI_OFFLINE ?? "1";
    return handleCatalogue(await importSdk());
  }
  const cwd = requireString(params, "cwd");
  const thinkingLevel = params.thinkingLevel;
  if (thinkingLevel !== undefined && typeof thinkingLevel !== "string") {
    throw new SidecarFailure("invalid_command");
  }
  if (!checkNodeFloor()) {
    throw new SidecarFailure("node_runtime_unsupported");
  }
  process.env.PI_OFFLINE = process.env.PI_OFFLINE ?? "1";
  const sdk = await importSdk();
  const provider = requireString(params, "provider");
  const modelId = requireString(params, "model");
  const agentDir = requireEnvironment(ENV_AGENT_DIR);
  const sessionDir = requireEnvironment(ENV_SESSION_DIR);

  const settingsManager = sdk.SettingsManager.inMemory({
    retry: { enabled: false },
    compaction: { enabled: false },
  });
  const modelRuntime = await sdk.ModelRuntime.create({
    authPath: join(agentDir, "auth.json"),
    modelsPath: null,
    allowModelNetwork: false,
  });
  const resourceLoaderOptions = {
    noExtensions: true,
    noSkills: true,
    noPromptTemplates: true,
    noThemes: true,
    noContextFiles: true,
  };
  let resolvedModel;
  const createRuntime = async ({ cwd: sessionCwd, sessionManager, sessionStartEvent }) => {
    const services = await sdk.createAgentSessionServices({
      cwd: sessionCwd,
      agentDir,
      settingsManager,
      modelRuntime,
      resourceLoaderOptions,
    });
    resolvedModel ??= await modelRuntime.getModel(provider, modelId);
    if (!resolvedModel) {
      throw new SidecarFailure("model_unknown");
    }
    const created = await sdk.createAgentSessionFromServices({
      services,
      sessionManager,
      sessionStartEvent,
      model: resolvedModel,
      tools: TOOLS,
      ...(thinkingLevel === undefined ? {} : { thinkingLevel }),
    });
    return { ...created, services, diagnostics: services.diagnostics };
  };

  let runtime;
  try {
    runtime = await sdk.createAgentSessionRuntime(createRuntime, {
      cwd,
      agentDir,
      sessionManager: sdk.SessionManager.create(cwd, sessionDir),
    });
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure("construction_failed");
  }
  if (runtime.cwd !== cwd) {
    throw new SidecarFailure("cwd_mismatch");
  }
  state.runtime = runtime;
  state.sessionManager = sdk.SessionManager;
  state.sessionDir = sessionDir;
  runtime.setRebindSession((session) => {
    subscribeSession(session);
    return Promise.resolve();
  });
  subscribeSession(runtime.session);
  for (const diagnostic of runtime.diagnostics ?? []) {
    await emitDiagnostic(diagnostic.type === "error" ? "error" : "warning", "sdk_diagnostic");
  }
  if (runtime.modelFallbackMessage !== undefined) {
    await emitDiagnostic("warning", "model_fallback");
  }
  const snapshot = sessionSnapshot();
  if (typeof snapshot.sessionRef !== "string") {
    throw new SidecarFailure("session_reference_missing");
  }
  return {
    wire: WIRE,
    behavior: BEHAVIOR,
    sdkPackage: SDK_PACKAGE,
    sdkVersion: sdk.VERSION,
    nodeVersion: process.versions.node,
    ...snapshot,
  };
}

async function handleSessionNew() {
  const runtime = requireBootstrapped();
  await runtime.newSession();
  const snapshot = sessionSnapshot();
  if (typeof snapshot.sessionRef !== "string") {
    throw new SidecarFailure("session_reference_missing");
  }
  return { sessionRef: snapshot.sessionRef, sessionId: snapshot.sessionId, cwd: snapshot.cwd };
}

async function handleSessionSwitch(params) {
  const runtime = requireBootstrapped();
  requireExactParams(params, ["sessionRef", "expectedCwd"]);
  const sessionRef = requireString(params, "sessionRef");
  const expectedCwd = requireString(params, "expectedCwd");
  let sessions;
  try {
    sessions = await state.sessionManager.listAll(state.sessionDir);
  } catch {
    throw new SidecarFailure("session_invalid");
  }
  const matches = sessions.filter((session) => session?.id === sessionRef);
  if (matches.length === 0) {
    throw new SidecarFailure("session_not_found");
  }
  if (matches.length !== 1) {
    throw new SidecarFailure("session_ambiguous");
  }
  let sessionPath;
  try {
    const [sessionRoot, candidate] = await Promise.all([
      realpath(state.sessionDir),
      realpath(matches[0].path),
    ]);
    const contained = relative(sessionRoot, candidate);
    if (contained === "" || contained === ".." || contained.startsWith(`..${sep}`) || isAbsolute(contained)) {
      throw new SidecarFailure("session_outside_root");
    }
    sessionPath = candidate;
  } catch (error) {
    if (error instanceof SidecarFailure) {
      throw error;
    }
    throw new SidecarFailure("session_invalid");
  }
  try {
    await runtime.switchSession(sessionPath, { cwdOverride: expectedCwd });
  } catch {
    throw new SidecarFailure("session_invalid");
  }
  if (runtime.cwd !== expectedCwd) {
    throw new SidecarFailure("cwd_mismatch");
  }
  const snapshot = sessionSnapshot();
  if (snapshot.sessionRef !== sessionRef) {
    throw new SidecarFailure("session_substituted");
  }
  return {
    effectiveCwd: snapshot.cwd,
    sessionRef: snapshot.sessionRef,
    sessionId: snapshot.sessionId,
    messages: snapshot.messages,
  };
}

function projectReplayItem(message, sequence) {
  switch (message.role) {
    case "user": {
      const parts = Array.isArray(message.content) ? message.content : [];
      const text =
        typeof message.content === "string"
          ? message.content
          : parts
              .filter((part) => part?.type === "text")
              .map((part) => String(part.text ?? ""))
              .join("");
      const images = parts.filter((part) => part?.type === "image").length;
      return { sequence, item: { kind: "user", text, images } };
    }
    case "assistant": {
      const parts = (Array.isArray(message.content) ? message.content : []).map((part) => {
        switch (part?.type) {
          case "text":
            return { type: "text", text: String(part.text ?? "") };
          case "thinking":
            return { type: "thinking", thinking: String(part.thinking ?? "") };
          case "toolCall":
            return {
              type: "tool_call",
              name: String(part.name ?? ""),
              arguments: part.arguments ?? {},
            };
          default:
            throw new SidecarFailure("replay_unknown_message");
        }
      });
      const item = {
        kind: "assistant",
        parts,
        stopReason: String(message.stopReason ?? ""),
      };
      const usage = usageProjection(message.usage);
      if (usage) {
        item.usage = usage;
      }
      return { sequence, item };
    }
    case "toolResult": {
      const parts = Array.isArray(message.content) ? message.content : [];
      const text = parts
        .filter((part) => part?.type === "text")
        .map((part) => String(part.text ?? ""))
        .join("");
      return {
        sequence,
        item: {
          kind: "tool_result",
          toolName: String(message.toolName ?? ""),
          isError: message.isError === true,
          text,
        },
      };
    }
    default:
      throw new SidecarFailure("replay_unknown_message");
  }
}

async function handleSessionReplay(params) {
  const runtime = requireBootstrapped();
  requireExactParams(params, ["maxItems"]);
  const maxItems = params.maxItems ?? MAXIMUM_REPLAY_ITEMS;
  if (!Number.isSafeInteger(maxItems) || maxItems < 1 || maxItems > MAXIMUM_REPLAY_ITEMS) {
    throw new SidecarFailure("invalid_command");
  }
  const messages = runtime.session.messages;
  if (messages.length > maxItems) {
    throw new SidecarFailure("replay_overflow");
  }
  let sequence = 0;
  for (const message of messages) {
    const record = { type: "event", event: "replay_item", ...projectReplayItem(message, sequence) };
    const line = JSON.stringify(record);
    if (line.length + 1 > MAXIMUM_RECORD_BYTES) {
      throw new SidecarFailure("replay_item_too_large");
    }
    await writeRecord(record);
    sequence += 1;
  }
  return { items: sequence, complete: true };
}

async function handlePrompt(params) {
  const runtime = requireBootstrapped();
  requireExactParams(params, ["text", "images"]);
  const text = requireString(params, "text");
  const images = parseImages(params);
  const session = runtime.session;
  if (session.isStreaming || !session.isIdle) {
    throw new SidecarFailure("turn_active");
  }
  // Respond at acceptance so steering, follow-up, abort, and close stay
  // reachable while the run streams; the run's outcome arrives through the
  // subscribed event stream, never through this response.
  let acceptance;
  const accepted = new Promise((resolve) => {
    acceptance = resolve;
  });
  const run = session.prompt(text, {
    images,
    expandPromptTemplates: false,
    preflightResult: (success) => acceptance(success === true),
  });
  void run.catch(() => {});
  if (!(await accepted)) {
    throw new SidecarFailure("prompt_rejected");
  }
  return { accepted: true };
}

async function handleSteer(params) {
  const runtime = requireBootstrapped();
  requireExactParams(params, ["text"]);
  await runtime.session.steer(requireString(params, "text"));
  return {};
}

async function handleFollowUp(params) {
  const runtime = requireBootstrapped();
  requireExactParams(params, ["text"]);
  await runtime.session.followUp(requireString(params, "text"));
  return {};
}

async function handleAbort() {
  const runtime = requireBootstrapped();
  await runtime.session.abort();
  return {};
}

async function handleState() {
  requireBootstrapped();
  return sessionSnapshot();
}

async function handleClose(id, command) {
  state.closed = true;
  if (state.runtime) {
    try {
      if (state.unsubscribe) {
        state.unsubscribe();
        state.unsubscribe = null;
      }
      await state.runtime.dispose();
    } catch {
      state.runtime = null;
      state.sessionManager = null;
      state.sessionDir = null;
      await respondFailure(id, command, "dispose_failed");
      await writes;
      process.exit(1);
      return;
    }
    state.runtime = null;
    state.sessionManager = null;
    state.sessionDir = null;
  }
  await respond(id, command, true, {});
  await writes;
  process.exit(0);
}

async function dispatch(record) {
  const { id, command } = record;
  const params = record.params ?? {};
  if (command === "close") {
    await handleClose(id, command);
    return;
  }
  try {
    let data;
    switch (command) {
      case "bootstrap":
        data = await handleBootstrap(params);
        break;
      case "session_new":
        data = await handleSessionNew();
        break;
      case "session_switch":
        data = await handleSessionSwitch(params);
        break;
      case "session_replay":
        data = await handleSessionReplay(params);
        break;
      case "prompt":
        data = await handlePrompt(params);
        break;
      case "steer":
        data = await handleSteer(params);
        break;
      case "follow_up":
        data = await handleFollowUp(params);
        break;
      case "abort":
        data = await handleAbort();
        break;
      case "state":
        data = await handleState();
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
  if (!record || record.type !== "command") {
    return terminal(record?.type === undefined ? "missing_type" : "unknown_record");
  }
  if (!validCommandId(record.id) || typeof record.command !== "string") {
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
  if (state.runtime) {
    Promise.resolve(state.runtime.dispose())
      .catch(() => {
        process.exitCode = 1;
      })
      .finally(() => process.exit(process.exitCode ?? 0));
  } else {
    process.exit(0);
  }
});
process.on("unhandledRejection", () => {
  void terminal("internal_error");
});
