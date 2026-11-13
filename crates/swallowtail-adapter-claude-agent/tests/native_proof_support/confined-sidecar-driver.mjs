// Drive the unchanged shipped sidecar inside the same confined driver process.
import { PassThrough } from "node:stream";
import { readFileSync } from "node:fs";
const config = JSON.parse(readFileSync(new URL("./confined-config.json", import.meta.url)));
const template = JSON.parse(readFileSync(new URL("./probe-options.json", import.meta.url)));
const input = new PassThrough();
Object.defineProperty(process, "stdin", { value: input });
const send = (id, command, params) => input.write(JSON.stringify({ type: "command", id, command, params }) + "\n");
const output = process.stdout.write.bind(process.stdout);
let pending = "";
let closing = false;
process.stdout.write = (chunk, ...args) => {
  const result = output(chunk, ...args);
  pending += chunk.toString();
  while (pending.includes("\n")) {
    const end = pending.indexOf("\n");
    const record = JSON.parse(pending.slice(0, end));
    pending = pending.slice(end + 1);
    queueMicrotask(() => {
      if (record.type === "response" && record.id === "open") {
        if (record.success) send("query", "query", { text: "owned sidecar turn" });
        else process.exit(1);
      }
      if (record.type === "callback") {
        input.write(JSON.stringify({ type: "callback_response", id: record.id,
          decision: { behavior: "deny", message: "owned callback denial" } }) + "\n");
      }
      if (record.type === "event" && record.event === "turn_ended" && !closing) {
        closing = true;
        send("close", "close", { joinBoundMs: 1000 });
      }
      if (record.type === "response" && record.id === "close") input.end();
    });
  }
  return result;
};
await import("./claude-agent-sdk-sidecar.mjs");
send("open", "open", {
  cwd: config.workspace, model: template.options.model, tools: template.options.tools,
  permissionMode: "default", expectedSdkVersion: "0.3.317", expectedNativeVersion: "2.1.317",
});
