// Private prepared command driver around the unchanged shipped sidecar/probe.
import { PassThrough } from "node:stream";
import { readFileSync } from "node:fs";
// macOS may add this at exec despite env -i; exclude it from the private SDK
// handoff explicitly. This rehearsal still grants no Darwin runtime custody.
delete process.env.__CF_USER_TEXT_ENCODING;
const config = JSON.parse(readFileSync(new URL("./spawn-config.json", import.meta.url)));
if (config.mode === "probe") {
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
          if (record.success) send("query", "query", { text: "owned spawn turn" });
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
  process.once("SIGTERM", close);
  await import("./claude-agent-sdk-sidecar.mjs");
  send("open", "open", { cwd: config.workspace, model: config.model,
    tools: ["Read", "Glob", "Grep"], permissionMode: "default",
    expectedSdkVersion: "0.3.317", expectedNativeVersion: "2.1.317" });
}
