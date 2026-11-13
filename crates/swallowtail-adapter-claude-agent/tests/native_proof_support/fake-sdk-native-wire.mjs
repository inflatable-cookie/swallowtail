// Provider-free stand-in for `@anthropic-ai/claude-agent-sdk` that speaks the
// real native stdio wire to `fake-native.mjs`.
//
// The ordinary fake SDK in `tests/sidecar_asset_support/fake-sdk.mjs` invents
// SDK-level messages, which is the right shape for proving the sidecar's
// projection. This module has a different job: it keeps the same selected
// option surface but drives a *separate native process* over newline-delimited
// JSON, so the native-proof harness observes the boundary the real SDK uses.
//
// It is not evidence about the real SDK or the real native. It exists to prove
// that the harness's observation points, identity checks, egress controls and
// cleanup work, so that an authorised original-artifact run has something
// already exercised to run.

import { spawn } from "node:child_process";
import { closeSync, fsyncSync, openSync, readFileSync, renameSync, writeSync } from "node:fs";
import path from "node:path";
import process from "node:process";
import readline from "node:readline";
import { fileURLToPath } from "node:url";

const PEER_DIRECTORY = path.dirname(fileURLToPath(import.meta.url));
const OBSERVATIONS = process.env.FAKE_SDK_OBSERVATIONS;
// Some regression scenarios need the peer SDK to misbehave in a way the option
// surface cannot express, so they are configured by a sibling file.
const SDK_CONFIG = (() => {
  try {
    return JSON.parse(readFileSync(path.join(PEER_DIRECTORY, "sdk-config.json"), "utf8"));
  } catch {
    return {};
  }
})();
const observed = {
  options: null,
  spawnHookArgument: null,
  spawnHookArgumentCount: 0,
  nativeArgv: [],
  nativeCommand: null,
  controlCalls: [],
  userMessagesWritten: 0,
  nativeMessages: [],
  permissionAnswers: [],
  interruptCalls: 0,
  closeCalls: 0,
  nativeExit: null,
  errors: [],
};

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

function note(name) {
  observed.controlCalls.push(name);
  record();
}

/// The native argv this route produces, restricted to the surfaces the sidecar
/// actually sets. Kept literal so the harness can assert on exact flags.
function buildArgs(options) {
  const args = ["--output-format", "stream-json", "--verbose", "--input-format", "stream-json"];
  if (options.effort) args.push("--effort", options.effort);
  if (options.model) args.push("--model", options.model);
  if (options.canUseTool) args.push("--permission-prompt-tool=stdio");
  const allowedTools = options.allowedTools ?? [];
  if (allowedTools.length > 0) args.push("--allowedTools", allowedTools.join(","));
  if ((options.disallowedTools ?? []).length > 0)
    args.push("--disallowedTools", options.disallowedTools.join(","));
  if (options.tools !== undefined) {
    if (Array.isArray(options.tools)) args.push("--tools", options.tools.join(","));
    else args.push("--tools=default");
  }
  if (options.mcpServers && Object.keys(options.mcpServers).length > 0)
    args.push("--mcp-config", JSON.stringify({ mcpServers: options.mcpServers }));
  if (options.settingSources !== undefined)
    args.push("--setting-sources", options.settingSources.join(","));
  if (options.strictMcpConfig) args.push("--strict-mcp-config");
  if (options.permissionMode) args.push("--permission-mode", options.permissionMode);
  if (options.managedSettings) args.push("--managed-settings", JSON.stringify(options.managedSettings));
  // The real SDK emits --await-initialize only when both the native path and
  // the spawn hook are unset; this route sets both.
  if (options.pluginsViaInitialize === true) args.push("--await-initialize");
  return args;
}

class NativeWire {
  constructor(child, options) {
    this.child = child;
    this.options = options;
    this.nextRequestId = 0;
    this.pending = new Map();
    this.inbox = [];
    this.waiters = [];
    this.exit = new Promise((resolve) => {
      child.once("exit", (code, signal) => {
        observed.nativeExit = { code, signal: signal ?? null };
        record();
        resolve({ code, signal: signal ?? null });
      });
    });
    this.stdinClosed = false;
    child.on("error", (error) => {
      observed.errors.push(String(error?.code ?? error?.message ?? error));
      record();
    });
    const reader = readline.createInterface({ input: child.stdout });
    reader.on("line", (line) => {
      let message;
      try {
        message = JSON.parse(line);
      } catch {
        observed.errors.push("native wrote a non-JSON line");
        record();
        return;
      }
      this.receive(message);
    });
  }

  receive(message) {
    observed.nativeMessages.push(message.type);
    if (message.type === "control_response") {
      const resolve = this.pending.get(message.response?.request_id);
      this.pending.delete(message.response?.request_id);
      resolve?.(message);
      record();
      return;
    }
    if (message.type === "control_request") {
      void this.answerControlRequest(message);
      return;
    }
    this.inbox.push(message);
    record();
    const waiter = this.waiters.shift();
    waiter?.();
  }

  async answerControlRequest(message) {
    const subtype = message.request?.subtype;
    if (subtype !== "can_use_tool") {
      this.write({
        type: "control_response",
        response: {
          subtype: "error",
          request_id: message.request_id,
          error: `fixture SDK does not answer ${String(subtype)}`,
        },
      });
      return;
    }
    if (typeof this.options.canUseTool !== "function") {
      // No callback exists on this route's option set only when the caller
      // removed it. Answering with a denial keeps the native from hanging.
      this.write({
        type: "control_response",
        response: {
          subtype: "success",
          request_id: message.request_id,
          response: { behavior: "deny", message: "no canUseTool callback" },
        },
      });
      return;
    }
    let decision;
    try {
      decision = await this.options.canUseTool(message.request.tool_name, message.request.input);
    } catch (error) {
      decision = { behavior: "deny", message: String(error?.message ?? "callback failed") };
    }
    observed.permissionAnswers.push(decision);
    record();
    this.write({
      type: "control_response",
      response: {
        subtype: "success",
        request_id: message.request_id,
        response: decision,
      },
    });
  }

  write(message) {
    if (this.stdinClosed) return;
    this.child.stdin.write(`${JSON.stringify(message)}\n`);
  }

  request(subtype, request) {
    const requestId = `fixture-sdk-${this.nextRequestId++}`;
    this.write({ type: "control_request", request_id: requestId, request: { subtype, ...request } });
    return new Promise((resolve, reject) => {
      this.pending.set(requestId, (message) => {
        const response = message.response;
        if (response?.subtype === "error") {
          reject(new Error(String(response.error ?? "control request failed")));
          return;
        }
        resolve(response?.response ?? {});
      });
    });
  }

  async next() {
    if (this.inbox.length > 0) return this.inbox.shift();
    if (this.stdinClosed) {
      // The peer may still be emitting frames for work that started before the
      // input ended, so the exit sentinel is only returned once the inbox is
      // drained *after* the process has gone.
      const exit = await this.exit;
      if (this.inbox.length > 0) return this.inbox.shift();
      return exit;
    }
    await new Promise((resolve) => this.waiters.push(resolve));
    return this.next();
  }

  async endInput() {
    if (this.stdinClosed) return;
    this.stdinClosed = true;
    this.child.stdin.end();
    for (const waiter of this.waiters.splice(0)) waiter();
  }

  /// Sends one prompt message and waits for this route's reply shape, which the
  /// sidecar receives as a streamed assistant message plus a result.
  async writePromptMessage(message) {
    this.write({ type: "user", message: { role: "user", content: [{ type: "text", text: message }] } });
    observed.userMessagesWritten += 1;
    record();
  }
}

export function query({ prompt, options }) {
  observed.options = JSON.parse(
    JSON.stringify(
      { ...options, env: Object.keys(options.env ?? {}).sort() },
      (key, value) => (typeof value === "function" ? `[fn ${key}]` : value),
    ),
  );
  observed.spawnHookArgumentCount += 1;
  record();

  const nativePath = options.pathToClaudeCodeExecutable;
  if (typeof nativePath !== "string") {
    throw new Error("fixture SDK requires pathToClaudeCodeExecutable");
  }
  const args = buildArgs(options);
  const spawnOptions = {
    command: process.execPath,
    args: [nativePath, ...args],
    cwd: options.cwd,
    env: options.env,
    signal: new AbortController().signal,
  };
  observed.spawnHookArgument = {
    command: spawnOptions.command,
    args: spawnOptions.args,
    cwd: spawnOptions.cwd,
    envKeys: Object.keys(spawnOptions.env ?? {}).sort(),
  };
  observed.nativeArgv = args;
  observed.nativeCommand = spawnOptions.command;
  record();

  const child =
    typeof options.spawnClaudeCodeProcess === "function"
      ? options.spawnClaudeCodeProcess(spawnOptions)
      : spawn(spawnOptions.command, spawnOptions.args, {
          cwd: spawnOptions.cwd,
          env: spawnOptions.env,
          stdio: ["pipe", "pipe", "pipe"],
        });
  if (SDK_CONFIG.doubleSpawn === true) {
    // A second start must be refused by the probe's process bound.
    try {
      options.spawnClaudeCodeProcess?.(spawnOptions);
    } catch (error) {
      observed.errors.push(String(error?.message ?? error));
      record();
    }
  }
  const wire = new NativeWire(child, options);
  observed.closeCalls = 0;

  let cleanupPerformed = false;
  let permissionMode = options.permissionMode;
  let initialization;
  let aborted = false;

  const start = wire
    .request("initialize", {
      hooks: undefined,
      systemPrompt: typeof options.systemPrompt === "string" ? [options.systemPrompt] : undefined,
    })
    .then((response) => response)
    .catch((error) => {
      observed.errors.push(`initialize failed: ${String(error?.message ?? error)}`);
      record();
      throw error;
    });
  initialization = start;
  initialization.catch(() => {});

  const iterator = {
    async next() {
      for (;;) {
        if (aborted) return { value: undefined, done: true };
        const message = await wire.next();
        if (message === undefined) return { value: undefined, done: true };
        if (message.code !== undefined && message.signal !== undefined) {
          return { value: undefined, done: true };
        }
        if (message.type === "control_request") continue;
        return { value: message, done: false };
      }
    },
    async return() {
      aborted = true;
      await wire.endInput();
      return { value: undefined, done: true };
    },
    [Symbol.asyncIterator]() {
      return this;
    },
  };

  // The sidecar writes prompt messages into the async iterable; this pumps them
  // onto the native wire, then closes the native's stdin.
  void (async () => {
    try {
      for await (const message of prompt) {
        await wire.writePromptMessage(message);
      }
    } catch (error) {
      observed.errors.push(`prompt pump failed: ${String(error?.message ?? error)}`);
      record();
    } finally {
      await wire.endInput();
    }
  })();

  return {
    [Symbol.asyncIterator]() {
      return iterator;
    },
    next: () => iterator.next(),
    initializationResult: () => initialization,
    supportedModels: async () => (await initialization).models ?? [],
    accountInfo: async () => (await initialization).account,
    setPermissionMode: async (mode) => {
      note("setPermissionMode");
      permissionMode = mode;
      await wire.request("set_permission_mode", { mode });
      return mode;
    },
    setModel: async (model) => {
      note("setModel");
      await wire.request("set_model", { model });
      return model;
    },
    interrupt: async () => {
      observed.interruptCalls += 1;
      note("interrupt");
      const response = await wire.request("interrupt", {});
      return response;
    },
    mcpServerStatus: async () => [],
    /// The probe runner joins the peer explicitly so a peer that never exits is
    /// a cleanup failure rather than a hang.
    joinNative: async () => wire.exit,
    killNative: () => {
      child.kill("SIGKILL");
    },
    close: async () => {
      observed.closeCalls += 1;
      record();
      cleanupPerformed = true;
      await wire.endInput();
    },
    get cleanupPerformed() {
      return cleanupPerformed;
    },
    get permissionMode() {
      return permissionMode;
    },
  };
}
