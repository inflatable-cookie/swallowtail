// Owned SDK wrapper: the shipped sidecar's actual spawn callback creates the child.
import { query as baseQuery } from "./base-sdk.mjs";
import { readFileSync } from "node:fs";
const config = JSON.parse(readFileSync(new URL("./spawn-config.json", import.meta.url)));
if (config.graphAttack) {
  try { await import("./graph-attack.mjs"); }
  catch (error) { refused("graph", error); throw error; }
}
let started = false;
let observedExit;
let nativeChild;
const nativeExit = new Promise((resolve) => { observedExit = resolve; });
export const waitForNativeExit = () => nativeExit;
export async function stopAndWaitForNativeExit() {
  if (!nativeChild) return;
  nativeChild.kill("SIGKILL");
  await nativeExit;
}
function note(kind, data) { console.error(JSON.stringify({ probeEvent: kind, ...data })); }
function refused(at, error) {
  const message = typeof error?.message === "string" ? error.message : "";
  const allowed = ["unapproved module edge", "unapproved child edge", "unapproved process edge", "require parent mismatch"];
  const reason = allowed.includes(message) ? message :
    message.startsWith("Code generation from strings disallowed") ? "code_generation_refused" : "unexpected_guard_error";
  note("guard_refused", { at, fault: config.fault, reason });
}
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
    let child;
    try { child = spawn({ ...request, command: config.entryCommand[0], args: config.launchAttack ? ["-c", "throw Error(\"unapproved\")"] : config.entryCommand.slice(1) }); }
    catch (error) { refused("launch", error); throw error; }
    // Real ChildProcess; no spawn replacement or PID facade.
    nativeChild = child;
    child.stderr.on("data", (data) => process.stderr.write(data));
    child.once("exit", (code, signal) => {
      observedExit({ code, signal });
    });
    return child;
  } } });
}
