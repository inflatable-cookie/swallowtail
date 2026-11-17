// Original SDK/native test driver. Runs only in a disposable trusted CI job.
import { readFileSync, existsSync } from "node:fs";
import { spawn } from "node:child_process";
import { pathToFileURL } from "node:url";
const config = JSON.parse(readFileSync(process.argv[2], "utf8"));
const { query } = await import(pathToFileURL(config.sdk).href);
const events = [], children = [];
const abortController = new AbortController();
let cancellation;
const prompt = async function* () {
  yield { type: "user", message: { role: "user", content: "Use the requested owned fixture tool, then report continued." } };
};
const options = {
  cwd: config.workspace, model: "claude-sonnet-5-20250929", executable: "node",
  pathToClaudeCodeExecutable: config.native, env: config.environment,
  settingSources: [], skills: [], plugins: [], agents: {}, mcpServers: {},
  strictMcpConfig: true, hooks: {}, persistSession: false,
  includePartialMessages: false, tools: ["Write"],
  disallowedTools: ["Read", "Glob", "Grep", "Edit", "MultiEdit", "Bash", "BashOutput", "KillShell", "NotebookEdit", "WebFetch", "WebSearch", "Task"],
  permissionMode: "default", abortController,
  canUseTool: async (toolName, input) => {
    const effectBeforePermission = config.effect ? existsSync(config.effect) : false;
    events.push({ event: "callback", toolName, input, effectBeforePermission });
    if (effectBeforePermission) throw new Error("effect before permission");
    return config.case === "allow"
      ? { behavior: "allow", updatedInput: input }
      : { behavior: "deny", message: "owned fixture denial" };
  },
  spawnClaudeCodeProcess: (request) => {
    events.push({ event: "spawn", command: request.command, args: request.args,
      envKeys: Object.keys(request.env).sort() });
    const child = spawn(request.command, request.args, { cwd: request.cwd,
      env: request.env, signal: request.signal, stdio: ["pipe", "pipe", "pipe"] });
    const observed = { child, stderr: "", closed: false };
    observed.wait = new Promise((resolve) => child.once("close", (code, signal) => {
      observed.closed = true;
      events.push({ event: "native_close", code, signal, pid: child.pid });
      resolve();
    }));
    child.stderr.on("data", (chunk) => {
      if (observed.stderr.length < 32768) observed.stderr += chunk.toString();
    });
    children.push(observed);
    return child;
  },
};
let handle, failure;
try {
  handle = query({ prompt: prompt(), options });
  await handle.initializationResult();
  events.push({ event: "initialized" });
  if (["cancel", "deadline"].includes(config.case)) cancellation = setTimeout(() => abortController.abort(), config.case === "cancel" ? 1000 : 500);
  for await (const message of handle) {
    events.push({ event: "message", type: message.type, subtype: message.subtype,
      result: message.type === "result" ? message.result : undefined,
      is_error: message.type === "result" ? message.is_error : undefined });
    if (message.type === "result") break;
  }
} catch (error) {
  failure = String(error);
  events.push({ event: "error", message: failure });
} finally {
  clearTimeout(cancellation);
  handle?.close();
  for (const observed of children) {
    if (!observed.closed) {
      const timer = setTimeout(() => observed.child.kill("SIGKILL"), 5000);
      await observed.wait;
      clearTimeout(timer);
    }
    events.push({ event: "stderr", text: observed.stderr });
  }
}
console.log(JSON.stringify({ events, failure, children: children.length,
  allClosed: children.every((child) => child.closed) }));
if (failure && !["cancel", "deadline", "error"].includes(config.case)) process.exitCode = 1;
