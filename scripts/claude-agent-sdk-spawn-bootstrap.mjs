// Trusted captured bootstrap for the selected owned graph only.
import { readFileSync } from "node:fs";
import * as loader from "./captured-loader.mjs";
const role = new URL(import.meta.url).pathname.endsWith("bootstrap-native.mjs") ? "native" : "driver";
const graph = JSON.parse(readFileSync(new URL(`./${role}-graph.json`, import.meta.url)));
if (!process.execArgv.includes("--no-addons") ||
    !process.execArgv.includes("--disallow-code-generation-from-strings")) throw new Error("graph_startup_policy");
for (const name of ["binding", "_linkedBinding", "dlopen", "getBuiltinModule", "execve"]) {
  Object.defineProperty(process, name, { value() { throw new Error("unapproved process edge"); },
    configurable: false, writable: false });
}
// This key may be synthesized by macOS at exec. The SDK handoff excludes it.
delete process.env.__CF_USER_TEXT_ENCODING;
loader.install(graph);
