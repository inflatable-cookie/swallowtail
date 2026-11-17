// Runs the shipped sidecar unchanged. Only fixture transport is injected into
// the exact native spawn; permission/mapping/lifecycle guards stay in product.
import cp from "node:child_process";
import { syncBuiltinESMExports } from "node:module";
import { PassThrough } from "node:stream";
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { pathToFileURL } from "node:url";
const config = JSON.parse(readFileSync(process.argv[2], "utf8"));
const events = [], children = [];
const originalSpawn = cp.spawn;
cp.spawn = (command, args, options) => {
  if (command !== config.native) throw new Error("unexpected sidecar child");
  events.push({ event: "spawn", command, args, envKeys: Object.keys(options.env).sort() });
  const child = originalSpawn(command, args, { ...options, env: { ...options.env,
    ANTHROPIC_API_KEY: "owned-fixture-key",
    ANTHROPIC_BASE_URL: config.environment.ANTHROPIC_BASE_URL,
    CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC: "1", DISABLE_AUTOUPDATER: "1" } });
  const observed = { child, closed: false, exited: false, stderr: "" };
  child.once("exit", (code, signal) => {
    observed.exited = true;
    events.push({event:"native_exit",code,signal,pid:child.pid});
  });
  observed.wait = new Promise((resolve) => child.once("close", (code, signal) => {
    observed.closed = true; events.push({ event: "native_close", code, signal, pid: child.pid }); resolve();
  }));
  child.stderr.on("data", (data) => { if (observed.stderr.length < 32768) observed.stderr += data.toString(); });
  children.push(observed);
  return child;
};
syncBuiltinESMExports();
if (process.argv[3] === "--prepared") {
  const output = process.stdout.write.bind(process.stdout);
  let pending = "";
  process.stdout.write = (chunk, ...args) => {
    pending += chunk.toString();
    while (pending.includes("\n")) {
      const end = pending.indexOf("\n");
      events.push({ event: "wire", ...JSON.parse(pending.slice(0,end)) });
      pending = pending.slice(end+1);
    }
    return output(chunk, ...args);
  };
  process.on("exit", () => writeFileSync(config.observation, JSON.stringify({events,
    children:children.length, allWaited:children.every(c=>c.exited)})));
  await import(pathToFileURL(config.sidecar).href);
} else {
const input = new PassThrough();
Object.defineProperty(process, "stdin", { value: input });
const output = process.stdout.write.bind(process.stdout);
let pending = "", failure, closeResponse, cancellation;
let complete;
const finished = new Promise((resolve) => { complete = resolve; });
const send = (id, command, params = {}) => input.write(JSON.stringify({ type: "command", id, command, params }) + "\n");
process.stdout.write = (chunk, ...args) => {
  pending += chunk.toString();
  while (pending.includes("\n")) {
    const end = pending.indexOf("\n");
    const record = JSON.parse(pending.slice(0, end)); pending = pending.slice(end + 1);
    events.push({ event: "wire", ...record });
    process.stderr.write(JSON.stringify(record) + "\n");
    queueMicrotask(() => {
      if (record.type === "response" && record.id === "open") {
        if (!record.success) { failure = JSON.stringify(record); send("close", "close", { joinBoundMs: 5000 }); }
        else {
          events.push({ event: "initialized" });
          send("query", "query", { text: "Use the owned fixture tool and continue." });
          if (config.case === "cancel") cancellation = setTimeout(() => send("cancel", "interrupt"), 1000);
        }
      }
      if (record.type === "response" && !record.success && !["open", "close"].includes(record.id)) {
        failure = JSON.stringify(record); send("close", "close", { joinBoundMs: 5000 });
      }
      if (record.type === "callback") {
        const effectBeforePermission = existsSync(config.effect);
        events.push({ event: "callback", toolName: record.toolName, effectBeforePermission });
        if (effectBeforePermission) failure = "effect before permission";
        input.write(JSON.stringify({ type: "callback_response", id: record.id,
          decision: config.case === "allow" ? "allow" : "deny" }) + "\n");
      }
      if (record.type === "event" && record.event === "turn_ended") {
        clearTimeout(cancellation);
        send("close", "close", { joinBoundMs: 5000 });
      }
      if (record.type === "response" && record.id === "close") {
        closeResponse = record;
        input.end(); complete();
      }
    });
  }
  const callback = args.at(-1);
  if (typeof callback === "function") queueMicrotask(callback);
  return true;
};
process.env.CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE = config.sdk;
process.env.CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY = config.native;
process.env.CLAUDE_AGENT_SDK_SIDECAR_MANIFEST = config.manifest;
process.exit = (code) => { process.exitCode = code; input.end(); complete(); };
await import(pathToFileURL(config.sidecar).href);
send("open", "open", { cwd: config.workspace, model: "claude-sonnet-5-20250929",
  tools: ["Write"], permissionMode: "default", expectedSdkVersion: config.sdkVersion,
  expectedNativeVersion: config.nativeVersion });
await finished;
for (const observed of children) {
  if (!observed.closed) {
    const timer = setTimeout(() => observed.child.kill("SIGKILL"), 5000);
    await observed.wait; clearTimeout(timer);
  }
  events.push({ event: "stderr", text: observed.stderr });
}
output(JSON.stringify({ events, failure, closeResponse, children: children.length,
  allClosed: children.every((child) => child.closed) }) + "\n");
if (failure) process.exitCode = 1;

}
