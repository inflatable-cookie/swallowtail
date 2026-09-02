// Provider-free stand-in for the official `@anthropic-ai/claude-agent-sdk`
// default entry point. It records exactly what the shipped sidecar asset does
// with the SDK's option surface and `canUseTool` contract, then writes those
// observations where the test can read them. No provider, credential, network,
// or official package is involved.

import { spawn } from "node:child_process";
import { writeFileSync } from "node:fs";
import process from "node:process";

const OBSERVATIONS = process.env.FAKE_SDK_OBSERVATIONS;
const NATIVE_LIFETIME_MS = Number(process.env.FAKE_SDK_NATIVE_LIFETIME_MS ?? "50");

const observed = { options: null, admissions: {} };

function record() {
  writeFileSync(OBSERVATIONS, JSON.stringify(observed));
}

export function query({ prompt, options }) {
  // Only serialisable option keys are recorded; callbacks are noted by name so
  // the test can assert what was and was not passed.
  observed.options = JSON.parse(
    JSON.stringify(options, (key, value) => (typeof value === "function" ? `[fn ${key}]` : value)),
  );
  record();

  const child = options.spawnClaudeCodeProcess("node", [
    "-e",
    `setTimeout(() => {}, ${NATIVE_LIFETIME_MS})`,
  ]);

  let settled = false;
  const messages = [
    {
      type: "system",
      subtype: "init",
      cwd: options.cwd,
      model: options.model,
      apiKeySource: "oauth",
      capabilities: ["interrupt_receipt_v1"],
    },
  ];

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
      return { apiProvider: "firstParty", subscriptionType: "max" };
    },
    async interrupt() {
      return { received: true };
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
