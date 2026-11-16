// Private prepared command driver around the unchanged shipped sidecar/probe.
import { PassThrough } from "node:stream";
import { readFileSync, writeSync } from "node:fs";
import { stopAndWaitForNativeExit } from "./probe-sdk.mjs";
// macOS may add this at exec despite env -i; exclude it from the private SDK
// handoff explicitly. This rehearsal still grants no Darwin runtime custody.
delete process.env.__CF_USER_TEXT_ENCODING;
const config = JSON.parse(readFileSync(new URL("./spawn-config.json", import.meta.url)));
const exitImmediately = process.exit.bind(process);
let exiting = false;
function exitNote(probeEvent, code) {
  writeSync(2, JSON.stringify({ probeEvent, actor: process.pid, code, source: "owned parent exit gate" }) + "\n");
}
Object.defineProperty(process, "exit", { configurable: false, writable: false, value(code) {
  if (exiting) return;
  exiting = true;
  const status = code ?? process.exitCode ?? 0;
  process.exitCode = status;
  exitNote("driver_exit_requested", status);
  // Normal sidecar/probe close can race the SIGTERM handler. Quiesce starts
  // before the first await and never let either path bypass the actual wait.
  stopAndWaitForNativeExit().then(() => {
    exitNote("driver_exit_ready", status);
    exitImmediately(status);
  }, () => {
    // Keep the real child referenced; the existing host deadline remains the
    // termination authority. Failure is not permission to exit past its wait.
    exitNote("driver_exit_wait_failed", status);
  });
} });
if (config.mode === "probe") {
  process.once("SIGTERM", async () => {
    await stopAndWaitForNativeExit();
    process.exit(1);
  });
  await import("./prepared-probe.mjs");
} else {
  const input = new PassThrough();
  Object.defineProperty(process, "stdin", { value: input });
  const send = (id, command, params) => input.write(JSON.stringify({ type: "command", id, command, params }) + "\n");
  let closing = false;
  const close = () => { if (!closing) { closing = true; send("close", "close", { joinBoundMs: 300 }); } };
  const output = process.stdout.write.bind(process.stdout);
  let pending = "";
  process.stdout.write = (chunk, ...args) => {
    const result = output(chunk, ...args);
    pending += chunk.toString();
    if (pending.length > 1000000) throw new Error("driver_output_limit");
    while (pending.includes("\n")) {
      const end = pending.indexOf("\n");
      const record = JSON.parse(pending.slice(0, end)); pending = pending.slice(end + 1);
      queueMicrotask(() => {
        if (record.type === "response" && record.id === "open") {
          if (record.success && config.fault === "close-before-native-wait") close();
          else if (record.success) send("query", "query", { text: "owned spawn turn" });
          else close();
        }
        if (record.type === "callback") input.write(JSON.stringify({ type: "callback_response", id: record.id,
          decision: "deny" }) + "\n");
        if (record.type === "event" && record.event === "turn_ended") close();
        if (record.type === "response" && record.id === "close") input.end();
      });
    }
    return result;
  };
  process.once("SIGTERM", async () => {
    // Retain the real handle through its actual wait before sidecar close can
    // exit this parent. This includes children still in the Python entry gate.
    await stopAndWaitForNativeExit();
    close();
  });
  await import("./claude-agent-sdk-sidecar.mjs");
  send("open", "open", { cwd: config.workspace, model: config.model,
    tools: ["Read", "Glob", "Grep"], permissionMode: "default",
    expectedSdkVersion: "0.3.317", expectedNativeVersion: "2.1.317" });
}
