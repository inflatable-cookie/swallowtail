// Private, source-bound one-shot accounting. This grants no spawn capability.
import { writeSync } from "node:fs";
let launchId;
let attempted = false;
let childPid = null;
function note(probeEvent, data) {
  writeSync(2, JSON.stringify({ probeEvent, launchId, actor: process.pid, ...data }) + "\n");
}
export function arm(id) {
  if (launchId !== undefined || typeof id !== "string" || !/^[a-f0-9]{32}$/.test(id)) throw Error("launch_slot_binding");
  launchId = id;
  process.once("exit", () => note("native_launch_closed", {
    attempted, pid: childPid, source: "owned one-shot parent exit",
  }));
}
export function beforeSpawn() {
  if (launchId === undefined || attempted) throw Error("launch_slot_reused");
  attempted = true;
  note("native_launch_intent", { source: "owned one-shot before spawn" });
}
export function spawned(child) {
  if (!attempted || childPid !== null) throw Error("launch_slot_return");
  childPid = child.pid ?? null;
  note("native_started", { pid: childPid, parent: process.pid, source: "ChildProcess.spawn" });
  child.once("exit", (code, signal) => note("native_wait", {
    pid: childPid, code, signal, source: "ChildProcess.exit",
  }));
}
