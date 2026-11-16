// Private one-shot accounting; only the trusted host releases the spawn edge.
import { writeSync, openSync, readSync, closeSync, fstatSync, constants } from "node:fs";
const exitImmediately = process.exit.bind(process);
let launchId, control;
let attempted = false, permitted = false, stopping = false;
let childPid = null, child, terminal, waited;
function note(probeEvent, data) {
  writeSync(2, JSON.stringify({ probeEvent, launchId, actor: process.pid, ...data }) + "\n");
}
async function stop() {
  stopping = true;
  if (attempted && !child) return; // Unknown spawn return never authorizes exit.
  if (child && !terminal) { child.kill("SIGKILL"); await waited; }
  exitImmediately(1);
}
export function arm(id, binding) {
  if (launchId !== undefined || typeof id !== "string" || !/^[a-f0-9]{32}$/.test(id)) throw Error("launch_slot_binding");
  launchId = id; control = binding;
  process.once("exit", () => note("native_launch_closed", {
    attempted, pid: childPid, source: "owned one-shot parent exit",
  }));
  process.once("SIGTERM", () => { stop().catch(() => {}); });
}
export async function awaitPermit() {
  const fd = openSync(control.path, constants.O_RDONLY | constants.O_NOFOLLOW);
  try {
    const info = fstatSync(fd, { bigint: true });
    if (!info.isFile() || info.nlink !== 1n || info.size > 1n || [info.dev, info.ino, info.uid, info.gid, info.mode].map(String).join(":") !== control.identity.join(":")) throw Error("launch_control_identity");
    note("native_launch_ready", { source: "owned bootstrap before payload" });
    const byte = Buffer.alloc(2);
    const until = Date.now() + 1000;
    while (!stopping) {
      const count = readSync(fd, byte, 0, 2, 0);
      if (count === 1 && byte[0] === 1) { permitted = true; return; }
      if (count !== 0 || Date.now() > until) throw Error("launch_control_refused");
      await new Promise((resolve) => setTimeout(resolve, 5));
    }
    throw Error("parent_closing");
  } finally { closeSync(fd); }
}
export function beforeSpawn() {
  if (launchId === undefined || attempted || !permitted || stopping) throw Error("launch_slot_reused_or_unreleased");
  attempted = true;
  note("native_launch_intent", { source: "owned one-shot before spawn" });
}
export function spawned(actualChild) {
  if (!attempted || childPid !== null) throw Error("launch_slot_return");
  child = actualChild; childPid = child.pid ?? null;
  note("native_started", { pid: childPid, parent: process.pid, source: "ChildProcess.spawn" });
  waited = new Promise((resolve) => child.once("exit", (code, signal) => {
    terminal = { code, signal };
    note("native_wait", { pid: childPid, code, signal, source: "ChildProcess.exit" });
    resolve();
  }));
}
