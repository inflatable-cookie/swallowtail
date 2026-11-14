// Private fake-only driver. The admitted option template stays the selected
// surface; the spawn hook attaches supervisor-owned pipes rather than forking.
import { readFileSync } from "node:fs";
import { spawn } from "node:child_process";
import { query } from "./probe-sdk.mjs";

const config = JSON.parse(readFileSync(new URL("./confined-config.json", import.meta.url)));
const template = JSON.parse(readFileSync(new URL("./probe-options.json", import.meta.url)));
const events = [];
let finishPrompt;
const promptFinished = new Promise((resolve) => { finishPrompt = resolve; });
const emit = (event) => {
  events.push(event);
  console.log(JSON.stringify(event));
};
const options = {
  ...template.options,
  cwd: config.workspace,
  env: { ...process.env },
  pathToClaudeCodeExecutable: config.nativeArgs[0],
  canUseTool: async (name, input) => {
    emit({ event: "permission", name, input });
    return { behavior: "deny", message: "owned callback denial" };
  },
  spawnClaudeCodeProcess: (request) => spawn(request.command, request.args, request),
};
const probe = query({ prompt: prompt(), options });
if (config.withoutSdkJoin) {
  delete probe.joinNative;
  delete probe.killNative;
  emit({ event: "sdk_join_unavailable" });
}
const initialized = await probe.initializationResult();
emit({ event: "initialized", models: initialized.models?.length ?? 0 });
emit({ event: "models", count: (await probe.supportedModels()).length });
emit({ event: "account", provider: (await probe.accountInfo())?.apiProvider ?? null });
for await (const message of probe) {
  emit({ event: "message", message });
  if (config.scenario === "abortable-turn" && message.type === "assistant") await probe.interrupt();
  if (message.type === "result") break;
}
finishPrompt();
await probe.close();
emit({ event: "iterator_done", permissions: events.filter((row) => row.event === "permission").length });
// Cleanup truth comes only from the supervisor, never an SDK join promise.
// A hanging native is deliberately left for the supervisor to detect.
process.exit(0);

async function* prompt() {
  yield { type: "user", message: { role: "user", content: "owned probe turn" } };
  await promptFinished;
}
