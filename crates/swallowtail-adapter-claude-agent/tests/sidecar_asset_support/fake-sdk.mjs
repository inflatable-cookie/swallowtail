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

const observed = { options: null, admissions: {}, permissionModes: [], writes: [], bash: [] };

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
  return { apiProvider: "firstParty", subscriptionType: "max" };
}

export function query({ prompt, options }) {
  state.permissionMode = options.permissionMode ?? "default";
  // Only serialisable option keys are recorded; callbacks are noted by name so
  // the test can assert what was and was not passed.
  observed.options = JSON.parse(
    JSON.stringify(options, (key, value) => (typeof value === "function" ? `[fn ${key}]` : value)),
  );
  record();

  const spawnOptions = {
    command: process.execPath,
    args: ["-e", `setTimeout(() => {}, ${NATIVE_LIFETIME_MS})`],
    cwd: options.cwd,
    env: options.env,
    signal: new AbortController().signal,
  };
  const child = options.spawnClaudeCodeProcess(spawnOptions);

  if (SCENARIO === "editing") {
    return editingSession(prompt, options, child);
  }
  if (SCENARIO === "bash") {
    return bashSession(prompt, options, child);
  }

  let settled = false;
  const messages = [initMessage(options)];

  async function admit(name, input) {
    const decision = await options.canUseTool(name, input);
    observed.admissions[name] = decision;
    record();
  }

  const iterator = {
    async next() {
      if (messages.length > 0) {
        return { value: messages.shift(), done: false };
      }
      if (!settled) {
        settled = true;
        // An allowed read-only tool, then a tool outside the read-only set.
        await admit("Read", { file_path: "/fixture/read-me.txt" });
        await admit("Bash", { command: "rm -rf /" });
        return {
          value: { type: "result", subtype: "success", is_error: false },
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
      return accountInfo();
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
      // Deliberately inert. The real SDK's cleanup races a timer, swallows the
      // outcome, and its escalation is unreferenced, so a fixture that killed
      // the child here would prove a stop the SDK does not actually provide.
      void child;
    },
  };
  void prompt;
  return iterator;
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
    yield initMessage(options);
    for await (const message of prompt) {
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
  iterator.accountInfo = async () => accountInfo();
  iterator.interrupt = async () => ({ received: true });
  iterator.setPermissionMode = async (mode) => {
    state.permissionMode = mode;
    observed.permissionModes.push(mode);
    record();
  };
  iterator.close = () => {
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
    yield initMessage(options);
    let index = 0;
    for await (const message of prompt) {
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
  iterator.accountInfo = async () => accountInfo();
  iterator.interrupt = async () => ({ received: true });
  iterator.setPermissionMode = async (mode) => {
    state.permissionMode = mode;
    observed.permissionModes.push(mode);
    record();
  };
  iterator.close = () => {
    // Deliberately inert, exactly like the read-only path.
    void child;
  };
  return iterator;
}
