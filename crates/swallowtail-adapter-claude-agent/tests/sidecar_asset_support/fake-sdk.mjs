// Provider-free stand-in for the official `@anthropic-ai/claude-agent-sdk`
// default entry point. It records exactly what the shipped sidecar asset does
// with the SDK's option surface and `canUseTool` contract, then writes those
// observations where the test can read them. No provider, credential, network,
// or official package is involved.

import { spawn, spawnSync } from "node:child_process";
import {
  closeSync,
  fsyncSync,
  openSync,
  renameSync,
  writeFileSync,
  writeSync,
} from "node:fs";
import path from "node:path";
import process from "node:process";

const OBSERVATIONS = process.env.FAKE_SDK_OBSERVATIONS;
const TEMP_OBSERVATIONS = `${OBSERVATIONS}.${process.pid}.tmp`;
const NATIVE_LIFETIME_MS = Number(process.env.FAKE_SDK_NATIVE_LIFETIME_MS ?? "50");
// "read-only" reproduces the layer-1 shape exactly. "editing" drives a
// multi-turn write session so the host's admission decision can be checked
// against the filesystem itself.
const SCENARIO = process.env.FAKE_SDK_SCENARIO ?? "read-only";

const observed = {
  options: null,
  controlCalls: [],
  firstInputConsumed: false,
  spawnHookArgument: null,
  spawnHookArgumentCount: null,
  admissions: {},
  permissionModes: [],
  closeCalls: 0,
  promptStreamState: null,
  writes: [],
  bash: [],
};

function sanitizedEnvironment(environment) {
  return { keys: Object.keys(environment ?? {}).sort() };
}

function record() {
  const descriptor = openSync(TEMP_OBSERVATIONS, "w");
  try {
    writeSync(descriptor, JSON.stringify(observed));
    fsyncSync(descriptor);
  } finally {
    closeSync(descriptor);
  }
  renameSync(TEMP_OBSERVATIONS, OBSERVATIONS);
}

function observeControl(name) {
  observed.controlCalls.push(name);
  record();
}

// The live session mode, which the fixture's own admission modelling reads.
const state = { permissionMode: "default" };

function initMessage(options) {
  const message = {
    type: "system",
    subtype: "init",
    cwd: options.cwd,
    model: options.model,
    apiKeySource: "oauth",
    capabilities: ["interrupt_receipt_v1"],
  };
  if (SCENARIO === "canonical-model") {
    message.model = "claude-sonnet-5-20250929";
    message.supportedModels = ["claude-sonnet-5-20250929"];
  } else if (SCENARIO === "canonical-cwd") {
    if (options.cwd.startsWith("/private/")) {
      message.cwd = options.cwd.slice("/private".length);
    } else if (options.cwd.startsWith("/var/")) {
      message.cwd = `/private${options.cwd}`;
    } else {
      message.cwd = `${options.cwd}/../${path.basename(options.cwd)}`;
    }
  } else if (SCENARIO === "cwd-mismatch") {
    message.cwd = "/fixture/elsewhere";
  } else if (SCENARIO === "missing-model") {
    delete message.model;
  } else if (SCENARIO === "unsupported-model") {
    message.model = "claude-sonnet-5-20250929";
    message.supportedModels = ["claude-opus-5"];
  }
  return message;
}

function accountInfo() {
  if (SCENARIO === "account-not-first-party") {
    return { apiProvider: "bedrock", subscriptionType: "max" };
  }
  if (SCENARIO === "account-not-subscription") {
    return { apiProvider: "firstParty" };
  }
  if (SCENARIO === "account-api-key-source") {
    return { apiProvider: "firstParty", subscriptionType: "max", apiKeySource: "oauth" };
  }
  if (SCENARIO === "account-token-source") {
    return { apiProvider: "firstParty", subscriptionType: "max", tokenSource: "oauth" };
  }
  return { apiProvider: "firstParty", subscriptionType: "max" };
}

function modelRows(options) {
  if (SCENARIO === "empty-supported-models") {
    return [];
  }
  if (SCENARIO === "unsupported-model") {
    return [{ value: "claude-opus-5", displayName: "Opus" }];
  }
  if (SCENARIO === "canonical-model") {
    return [
      {
        value: options.model,
        resolvedModel: "claude-sonnet-5-20250929",
        displayName: "Sonnet",
      },
    ];
  }
  return [{ value: options.model, displayName: "Fixture model" }];
}

function initializeResponse(options) {
  return {
    commands: [{ name: "help", description: "fixture command" }],
    agents: [],
    output_style: "",
    available_output_styles: [],
    models: modelRows(options),
    account: accountInfo(),
  };
}

function resultMessage({ subtype = "success", isError = false, error } = {}) {
  const result = {
    type: "result",
    subtype,
    is_error: isError,
    num_turns: 1,
    duration_ms: 7,
  };
  if (error !== undefined) {
    result.error = error;
  }
  return result;
}

function earlyCompletingIterator(promptIterator) {
  let first = true;
  return {
    async next() {
      if (first) {
        first = false;
        return promptIterator.next();
      }
      return { value: undefined, done: true };
    },
    async return() {
      return { value: undefined, done: true };
    },
    [Symbol.asyncIterator]() {
      return this;
    },
  };
}

async function recordPromptLifetime(promptIterator) {
  const pending = promptIterator.next();
  const outcome = await Promise.race([
    pending.then((value) => ({ kind: value.done ? "early-eof" : "message" })),
    new Promise((resolve) => setTimeout(() => resolve({ kind: "open" }), 10)),
  ]);
  observed.promptStreamState = outcome.kind;
  record();
}

export function query({ prompt, options }) {
  state.permissionMode = options.permissionMode ?? "default";
  // Only serialisable option keys are recorded; callbacks are noted by name so
  // the test can assert what was and was not passed.
  observed.options = JSON.parse(
    JSON.stringify(
      { ...options, env: sanitizedEnvironment(options.env) },
      (key, value) => (typeof value === "function" ? `[fn ${key}]` : value),
    ),
  );
  record();

  const spawnOptions = {
    command: process.execPath,
    args: ["-e", `setTimeout(() => {}, ${NATIVE_LIFETIME_MS})`],
    cwd: options.cwd,
    env: options.env,
    signal: new AbortController().signal,
  };
  const child = invokeSpawnHook(options.spawnClaudeCodeProcess, spawnOptions);

  if (SCENARIO === "editing") {
    return editingSession(prompt, options, child);
  }
  if (SCENARIO === "bash") {
    return bashSession(prompt, options, child);
  }

  let settled = false;
  let firstInputConsumed = false;
  const rawPromptIterator = prompt[Symbol.asyncIterator]();
  const promptIterator =
    SCENARIO === "early-input-eof"
      ? earlyCompletingIterator(rawPromptIterator)
      : rawPromptIterator;

  async function admit(name, input) {
    const decision = await options.canUseTool(name, input);
    observed.admissions[name] = decision;
    record();
  }

  const iterator = {
    async next() {
      if (!firstInputConsumed) {
        const firstInput = await promptIterator.next();
        if (firstInput.done) {
          return { value: undefined, done: true };
        }
        firstInputConsumed = true;
        observed.firstInputConsumed = true;
        record();
        if (SCENARIO === "init-throws") {
          throw new Error("fixture init failure");
        }
        if (SCENARIO === "init-missing") {
          return { value: { type: "assistant", message: { content: [] } }, done: false };
        }
        if (SCENARIO === "init-not-first") {
          return { value: { type: "result", subtype: "success", is_error: false }, done: false };
        }
        return { value: initMessage(options), done: false };
      }
      if (!settled) {
        settled = true;
        if (SCENARIO === "early-input-eof") {
          const nextInput = await promptIterator.next();
          if (!nextInput.done) {
            throw new Error("early-input-eof fixture did not complete its input");
          }
          observed.promptStreamState = "early-eof";
          record();
          return {
            value: resultMessage({
              subtype: "error_during_execution",
              isError: true,
              error: "fixture early input EOF",
            }),
            done: false,
          };
        }
        if (SCENARIO === "input-stream-lifetime") {
          await recordPromptLifetime(rawPromptIterator);
          return {
            value: resultMessage(),
            done: false,
          };
        }
        // An allowed read-only tool, then a tool outside the read-only set.
        await admit("Read", { file_path: "/fixture/read-me.txt" });
        await admit("Bash", { command: "rm -rf /" });
        return {
          value: resultMessage(),
          done: false,
        };
      }
      return { value: undefined, done: true };
    },
    async return() {
      return { value: undefined, done: true };
    },
    [Symbol.asyncIterator]() {
      return iterator;
    },
    async accountInfo() {
      observeControl("accountInfo");
      return accountInfo();
    },
    async initializationResult() {
      observeControl("initializationResult");
      return initializeResponse(options);
    },
    async supportedModels() {
      observeControl("supportedModels");
      return modelRows(options);
    },
    async interrupt() {
      return { received: true };
    },
    async setPermissionMode(mode) {
      // Upstream `Query.setPermissionMode` resolves without a value, so the
      // sidecar's confirmation is that the change was accepted.
      state.permissionMode = mode;
      observed.permissionModes.push(mode);
      record();
    },
    close() {
      observed.closeCalls += 1;
      record();
      // Deliberately inert. The real SDK's cleanup races a timer, swallows the
      // outcome, and its escalation is unreferenced, so a fixture that killed
      // the child here would prove a stop the SDK does not actually provide.
      void child;
    },
  };
  void prompt;
  return iterator;
}

function invokeSpawnHook(hook, ...hookArguments) {
  const [spawnOptions] = hookArguments;
  // Record the arguments actually received by this SDK-side invocation before
  // forwarding them. This is a sanitized shape projection, not the query
  // options object: all five 0.3.259 SpawnOptions keys remain visible,
  // including the forwarded signal, and the count catches positional calls.
  observed.spawnHookArgumentCount = hookArguments.length;
  observed.spawnHookArgument = {
    command: spawnOptions?.command ?? null,
    args: spawnOptions?.args ?? null,
    cwd: spawnOptions?.cwd ?? null,
    env: sanitizedEnvironment(spawnOptions?.env),
    signal: spawnOptions?.signal instanceof AbortSignal,
  };
  record();
  return hook(...hookArguments);
}

/// A two-turn Bash session. The first command is denied; the second carries
/// oversized command and description fields so the shipped sidecar must expose
/// a bounded, truncation-flagged view while retaining the full input privately.
function bashSession(prompt, options, child) {
  async function attempt(input) {
    const decision = await options.canUseTool("Bash", input);
    const allowed = decision.behavior === "allow";
    const unchanged =
      allowed && JSON.stringify(decision.updatedInput) === JSON.stringify(input);
    let exitStatus = null;
    if (allowed) {
      const result = spawnSync(decision.updatedInput.command, {
        cwd: options.cwd,
        shell: true,
        stdio: "ignore",
      });
      exitStatus = result.status;
    }
    observed.bash.push({
      command: input.command,
      description: input.description,
      allowed,
      inputUnchanged: unchanged,
      ran: allowed,
      exitStatus,
    });
    record();
  }

  async function* messages() {
    let first = true;
    for await (const message of prompt) {
      if (first) {
        first = false;
        yield initMessage(options);
      }
      void message;
      await attempt({
        command: `node -e "require('fs').writeFileSync('denied.txt','denied')"`,
        description: "write a denied marker",
      });
      await attempt({
        command: `node -e "require('fs').writeFileSync('allowed.txt','allowed')" ${"x".repeat(180)}`,
        description: "d".repeat(180),
      });
      yield { type: "result", subtype: "success", is_error: false };
    }
  }

  const iterator = messages();
  iterator.accountInfo = async () => {
    observeControl("accountInfo");
    return accountInfo();
  };
  iterator.initializationResult = async () => {
    observeControl("initializationResult");
    return initializeResponse(options);
  };
  iterator.supportedModels = async () => {
    observeControl("supportedModels");
    return modelRows(options);
  };
  iterator.interrupt = async () => ({ received: true });
  iterator.setPermissionMode = async (mode) => {
    state.permissionMode = mode;
    observed.permissionModes.push(mode);
    record();
  };
  iterator.close = () => {
    observed.closeCalls += 1;
    record();
    void child;
  };
  return iterator;
}

/// A multi-turn editing session.
///
/// Each prompt message produces one write attempt against the leased cwd. The
/// file is created only when the write was admitted, so the filesystem itself
/// is the evidence that nothing ran unadmitted.
///
/// `acceptEdits` models the documented upstream behaviour: edits run without a
/// per-call decision while every other tool still goes through `canUseTool`.
/// That is upstream's stated contract, not a runtime observation made here.
function editingSession(prompt, options, child) {
  async function admit(name, input) {
    const decision = await options.canUseTool(name, input);
    observed.admissions[name] = decision;
    record();
    return decision.behavior === "allow" ? decision.updatedInput : null;
  }

  async function attemptWrite(index) {
    const input = {
      file_path: path.join(options.cwd, `turn-${index}.txt`),
      content: `turn ${index}\n`,
    };
    let admitted = input;
    if (state.permissionMode === "acceptEdits") {
      observed.writes.push({ turn: index, admitted: "skipped" });
    } else {
      admitted = await admit("Write", input);
      observed.writes.push({ turn: index, admitted: admitted === null ? "denied" : "allowed" });
    }
    record();
    if (admitted !== null) {
      writeFileSync(admitted.file_path, admitted.content);
    }
  }

  async function* messages() {
    // record flushes the SDK observations before the wire event that proves
    // the turn completed, so the Rust fixture can read them after turn_ended.
    let first = true;
    let index = 0;
    for await (const message of prompt) {
      if (first) {
        first = false;
        yield initMessage(options);
      }
      index += 1;
      void message;
      // A read is always mediated, whatever the mode, so the acceptEdits
      // narrowing is visible: edits skip admission, reads never do.
      await admit("Read", { file_path: path.join(options.cwd, "read-me.txt") });
      await attemptWrite(index);
      yield { type: "result", subtype: "success", is_error: false };
    }
  }

  const iterator = messages();
  iterator.accountInfo = async () => {
    observeControl("accountInfo");
    return accountInfo();
  };
  iterator.initializationResult = async () => {
    observeControl("initializationResult");
    return initializeResponse(options);
  };
  iterator.supportedModels = async () => {
    observeControl("supportedModels");
    return modelRows(options);
  };
  iterator.interrupt = async () => ({ received: true });
  iterator.setPermissionMode = async (mode) => {
    state.permissionMode = mode;
    observed.permissionModes.push(mode);
    record();
  };
  iterator.close = () => {
    observed.closeCalls += 1;
    record();
    // Deliberately inert, exactly like the read-only path.
    void child;
  };
  return iterator;
}
