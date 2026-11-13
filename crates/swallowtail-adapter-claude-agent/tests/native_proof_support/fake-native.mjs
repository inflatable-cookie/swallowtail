// Provider-free stand-in for the vendored `claude` native that the SDK spawns.
//
// The real SDK launches this binary and speaks newline-delimited JSON over its
// stdio: a `control_request`/`control_response` channel for the initialize
// handshake, `can_use_tool` and `interrupt`, plus `user`/`assistant`/`result`
// messages for turns. This file implements exactly that surface, records what
// it received, and never opens a provider session, reads a credential, or
// contacts a network peer.
//
// It exists so the native-proof harness can observe the *native boundary* with
// a controllable peer: the argv the SDK built, the initialize request, every
// streamed user message, each permission request and its answer, the interrupt,
// and the process exit code. The same observation points are what an authorised
// original-artifact run would read; nothing here is evidence about the real
// native.

import { fsyncSync, openSync, closeSync, readFileSync, renameSync, writeSync } from "node:fs";
import process from "node:process";
import path from "node:path";
import readline from "node:readline";
import { fileURLToPath } from "node:url";

// The production sidecar projects a fixed child-environment allowlist, so this
// peer is configured by a sibling file instead of environment variables. That
// keeps the harness honest about the environment boundary: nothing reaches the
// native except the allowlist the sidecar builds.
const PEER_DIRECTORY = path.dirname(fileURLToPath(import.meta.url));
const CONFIG_PATH = path.join(PEER_DIRECTORY, "native-config.json");

function loadConfig() {
  try {
    return JSON.parse(readFileSync(CONFIG_PATH, "utf8"));
  } catch {
    return {};
  }
}

const CONFIG = loadConfig();
const OBSERVATIONS = CONFIG.observations;
const SCENARIO = CONFIG.scenario ?? "ok";
const ENDPOINT = CONFIG.endpoint;
const MODEL = argValue("--model") ?? "fixture-model";

const observed = {
  argv: process.argv.slice(2),
  binary: process.argv[1],
  envKeys: Object.keys(process.env).sort(),
  cwd: process.cwd(),
  initializeRequests: [],
  userMessages: [],
  permissionRequestsSent: 0,
  permissionAnswers: [],
  interruptRequests: 0,
  endpointAttempts: [],
  exitCode: null,
  events: [],
};

function argValue(name) {
  const argv = process.argv.slice(2);
  const index = argv.indexOf(name);
  if (index >= 0 && index + 1 < argv.length) return argv[index + 1];
  const inline = argv.find((value) => value.startsWith(`${name}=`));
  return inline === undefined ? undefined : inline.slice(name.length + 1);
}

function record() {
  if (OBSERVATIONS === undefined) return;
  const temporary = `${OBSERVATIONS}.${process.pid}.tmp`;
  const descriptor = openSync(temporary, "w");
  try {
    writeSync(descriptor, JSON.stringify(observed));
    fsyncSync(descriptor);
  } finally {
    closeSync(descriptor);
  }
  renameSync(temporary, OBSERVATIONS);
}

function note(event, detail) {
  observed.events.push(detail === undefined ? { event } : { event, ...detail });
  record();
}

function send(message) {
  process.stdout.write(`${JSON.stringify(message)}\n`);
}

function initializeResponse(requestId) {
  return {
    type: "control_response",
    response: {
      subtype: "success",
      request_id: requestId,
      response: {
        commands: [],
        agents: [],
        output_style: "",
        available_output_styles: [],
        models: [{ value: MODEL, displayName: "Fixture model" }],
        account: {
          apiProvider: "firstParty",
          subscriptionType: "max",
          tokenSource: "oauth",
        },
        hook_on_failure_supported: true,
        capabilities: [],
      },
    },
  };
}

function initMessage() {
  return {
    type: "system",
    subtype: "init",
    cwd: observed.cwd,
    model: MODEL,
    apiKeySource: "oauth",
    session_id: "fixture-session",
  };
}

function assistantText(text) {
  return {
    type: "assistant",
    message: { role: "assistant", content: [{ type: "text", text }] },
  };
}

function resultMessage(overrides = {}) {
  return {
    type: "result",
    subtype: "success",
    is_error: false,
    num_turns: 1,
    duration_ms: 7,
    usage: {
      input_tokens: 21,
      output_tokens: 5,
      cache_read_input_tokens: 8,
      cache_creation_input_tokens: 9,
    },
    ...overrides,
  };
}

function writeStderr(line) {
  process.stderr.write(`${line}\n`);
}

// The route blocks external egress. This records a single loopback attempt when
// the harness supplies an endpoint, so the harness can prove that the fixture
// environment reaches only the local fake and never a provider host.
async function attemptEndpoint() {
  if (ENDPOINT === undefined) return;
  const url = `http://${ENDPOINT}/v1/messages`;
  try {
    const response = await fetch(url, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ model: MODEL, note: "fixture-egress-probe" }),
    });
    observed.endpointAttempts.push({ url, status: response.status });
    note("endpoint_reached", { status: response.status });
  } catch (error) {
    observed.endpointAttempts.push({ url, error: String(error?.code ?? error?.name ?? "error") });
    note("endpoint_refused");
  }
}

let pendingControl = new Map();
let nextRequestId = 1;

function requestControl(subtype, request) {
  const requestId = `fixture-native-${nextRequestId++}`;
  send({ type: "control_request", request_id: requestId, request: { subtype, ...request } });
  return new Promise((resolve) => pendingControl.set(requestId, resolve));
}

const queue = [];
let waiting = null;
// Work that started but has not settled yet. The peer may not exit while a
// handler that will emit frames is still running, and stdin closing must not
// drop a turn that is already in flight.
let inflight = 0;
let stdinClosed = false;
let finish;
const finished = new Promise((resolve) => {
  finish = resolve;
});

function wake() {
  if (waiting !== null) {
    const resolve = waiting;
    waiting = null;
    resolve();
  }
}

function enqueue(message) {
  queue.push(message);
  wake();
}

function maybeFinish() {
  if (stdinClosed && inflight === 0) finish();
}

async function track(work) {
  inflight += 1;
  try {
    await work();
  } catch (error) {
    observed.errors ??= [];
    observed.errors.push(String(error?.message ?? error));
    note("handler_failed");
  } finally {
    inflight -= 1;
    maybeFinish();
    wake();
  }
}

async function nextMessage() {
  while (queue.length === 0) {
    if (stdinClosed && inflight === 0) return undefined;
    await new Promise((resolve) => {
      waiting = resolve;
    });
  }
  return queue.shift();
}

async function handleControlRequest(record) {
  const subtype = record.request?.subtype;
  if (subtype === "initialize") {
    observed.initializeRequests.push({
      request_id: record.request_id,
      systemPromptType: Array.isArray(record.request.systemPrompt)
        ? "array"
        : typeof record.request.systemPrompt,
      keys: Object.keys(record.request).sort(),
    });
    note("initialize_received", { request_id: record.request_id });
    send(initializeResponse(record.request_id));
    enqueue(initMessage());
    return;
  }
  if (subtype === "interrupt") {
    observed.interruptRequests += 1;
    note("interrupt_received");
    send({
      type: "control_response",
      response: {
        subtype: "success",
        request_id: record.request_id,
        response: { still_queued: [], cancelled: [] },
      },
    });
    if (SCENARIO === "abortable-turn") {
      // The turn is already in flight; the interrupt settles it with its own
      // terminal reason instead of inventing a second assistant frame.
      enqueue(resultMessage({ terminal_reason: "aborted_streaming" }));
      return;
    }
    await settleTurn({ terminal_reason: "aborted_streaming" });
    return;
  }
  send({
    type: "control_response",
    response: {
      subtype: "error",
      request_id: record.request_id,
      error: `fixture native does not implement ${String(subtype)}`,
    },
  });
}

async function settleTurn(overrides = {}) {
  if (SCENARIO === "error-result") {
    enqueue(
      resultMessage({
        subtype: "error_during_execution",
        is_error: true,
        api_error_status: 402,
        terminal_reason: "api_error",
        errors: ["fixture provider detail"],
        ...overrides,
      }),
    );
  } else if (SCENARIO === "usage-only") {
    enqueue(resultMessage(overrides));
  } else {
    enqueue(assistantText("fixture reply"));
    enqueue(resultMessage(overrides));
  }
}

async function handleUserMessage(message) {
  observed.userMessages.push({
    type: message.type,
    contentTypes: Array.isArray(message.message?.content)
      ? message.message.content.map((part) => part.type)
      : [],
  });
  note("user_received");
  if (SCENARIO === "permission") {
    // An admitted tool, so the sidecar offers its callback. A tool outside the
    // admitted set is refused by the adapter before any callback exists.
    const input = { file_path: "fixture.txt" };
    enqueue({
      type: "assistant",
      message: {
        role: "assistant",
        content: [{ type: "tool_use", id: "tool-1", name: "Read", input }],
      },
    });
    observed.permissionRequestsSent += 1;
    record();
    const answer = await requestControl("can_use_tool", {
      tool_name: "Read",
      input,
      request_id: "fixture-permission-1",
    });
    observed.permissionAnswers.push(answer);
    note("permission_answered", { answer: answer?.response?.subtype ?? null });
    await settleTurn();
    return;
  }
  if (SCENARIO === "abortable-turn") {
    enqueue(assistantText("fixture reply in progress"));
    // Settled by the interrupt handler.
    return;
  }
  if (SCENARIO === "crash-after-turn") {
    // The native reports the turn and then dies: the harness observes the
    // exit at the native boundary and the harness fails closed instead of
    // assuming the session continues.
    await settleTurn();
    observed.exitCode = 3;
    note("crash", { code: 3 });
    process.exit(3);
  }
  if (SCENARIO === "managed-deny") {
    // The native denies the call itself and never asks the client: this is the
    // shape Q-009 has to rule on, reproduced so the harness can prove it
    // distinguishes the two outcomes without inventing a permission request.
    enqueue(assistantText("denied by managed policy"));
    await settleTurn();
    return;
  }
  await settleTurn();
}

async function main() {
  record();
  await attemptEndpoint();
  const reader = readline.createInterface({ input: process.stdin });
  reader.on("line", (line) => {
    let record;
    try {
      record = JSON.parse(line);
    } catch {
      writeStderr("fixture native received a non-JSON line");
      return;
    }
    if (record.type === "control_response") {
      const resolve = pendingControl.get(record.response?.request_id);
      pendingControl.delete(record.response?.request_id);
      resolve?.(record);
      return;
    }
    if (record.type === "control_request") {
      void track(() => handleControlRequest(record));
      return;
    }
    if (record.type === "user") {
      void track(() => handleUserMessage(record));
    }
  });
  reader.on("close", () => {
    stdinClosed = true;
    maybeFinish();
    wake();
  });

  // Serve queued native messages until stdin is closed and every handler that
  // could still emit a frame has settled.
  for (;;) {
    const message = await nextMessage();
    if (message === undefined) {
      if (SCENARIO === "never-exit") {
        // Keep serving frames that a handler enqueues after stdin closed, so a
        // peer that stays alive still reports its whole turn.
        await new Promise((resolve) => setTimeout(resolve, 5));
        continue;
      }
      break;
    }
    send(message);
  }
  for (const message of queue) send(message);
  if (SCENARIO === "never-exit") {
    // Stays alive after its turn so the probe must kill and report it.
    return;
  }
  observed.exitCode = SCENARIO === "exit-nonzero" ? 3 : 0;
  note("exit", { code: observed.exitCode });
  process.exit(observed.exitCode);
}

process.on("SIGTERM", () => {
  note("sigterm");
  process.exit(143);
});

if (SCENARIO === "never-exit") {
  // Answers the handshake and the turn, then outlives the probe bound. Used to
  // prove that a peer which will not be joined fails the probe closed.
  void main();
  setInterval(() => {}, 1000);
} else if (SCENARIO === "never-initialize") {
  // Records the argv and then stays silent: the handshake must time out or the
  // fixture must observe an absent initialization rather than assume success.
  record();
  setInterval(() => {}, 1000);
} else {
  void main();
}
