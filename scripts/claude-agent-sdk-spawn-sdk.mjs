// Owned SDK wrapper: the shipped sidecar's actual spawn callback creates the child.
import { query as baseQuery } from "./base-sdk.mjs";
import { readFileSync } from "node:fs";
const config = JSON.parse(readFileSync(new URL("./spawn-config.json", import.meta.url)));
let started = false;
let observedExit;
const nativeExit = new Promise((resolve) => { observedExit = resolve; });
export const waitForNativeExit = () => nativeExit;
const note = (kind, data) => console.error(JSON.stringify({ probeEvent: kind, ...data }));
export function query({ prompt, options }) {
  const spawn = options.spawnClaudeCodeProcess;
  return baseQuery({ prompt, options: { ...options, spawnClaudeCodeProcess: (request) => {
    if (started) { note("start_refused", { reason: "second_native_start" }); throw new Error("second_native_start"); }
    const keys = Object.keys(request.env).sort();
    if (request.command !== process.execPath || request.cwd !== config.workspace ||
        JSON.stringify(request.args) !== JSON.stringify(config.nativeArgs) ||
        JSON.stringify(keys) !== JSON.stringify(config.nativeEnvKeys)) {
      note("spawn_binding_refused", { command: request.command, args: request.args, cwd: request.cwd, envKeys: keys });
      throw new Error("spawn_request_binding");
    }
    note("spawn_requested", { command: request.command, args: request.args, cwd: request.cwd });
    started = true;
    const child = spawn({ ...request, command: config.entryCommand[0], args: config.entryCommand.slice(1) }); // Real ChildProcess; no spawn replacement or PID facade.
    note("native_started", { pid: child.pid, parent: process.pid });
    child.stderr.on("data", (data) => process.stderr.write(data));
    child.once("exit", (code, signal) => {
      note("native_wait", { actor: process.pid, pid: child.pid, code, signal, source: "ChildProcess.exit" });
      observedExit({ code, signal });
    });
    return child;
  } } });
}
