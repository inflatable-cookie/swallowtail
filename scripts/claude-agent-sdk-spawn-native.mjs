// Owned fake entry only. Kernel admission belongs to a separately activated host.
import { readFileSync, writeFileSync, statSync } from "node:fs";
import http from "node:http";
const config = JSON.parse(readFileSync(new URL("./spawn-config.json", import.meta.url)));
async function post(route, value) {
  const data = JSON.stringify(value);
  return new Promise((resolve, reject) => {
    const request = http.request({ host: "127.0.0.1", port: config.port,
      path: route, method: "POST", agent: false,
      headers: { "content-type": "application/json", "content-length": Buffer.byteLength(data) } },
    (response) => {
      let text = "";
      response.setEncoding("utf8");
      response.on("data", (part) => {
        text += part;
        if (text.length > 65536) response.destroy(new Error("response_limit"));
      });
      response.once("error", reject);
      response.once("end", () => response.statusCode === 200 ? resolve(text) : reject(new Error("entry_refused")));
    });
    request.setTimeout(1000, () => request.destroy(new Error("entry_deadline")));
    request.once("error", reject); request.end(data);
  });
}
// The trusted host entry already gated and execed this payload.
if (config.fault === "storage-excess") {
  writeFileSync(new URL("./storage.bin", import.meta.url), Buffer.alloc(4096));
  await new Promise((resolve) => setTimeout(resolve, 500));
}
let settings;
try { settings = JSON.parse(readFileSync(config.fixturePath, "utf8")); }
catch (error) { if (error.code !== "ENOENT") throw error; settings = null; }
const managed = settings?.permissions?.denyInsteadOfPrompting === true;
const expected = config.fixtureState === "absent" ? null :
  { permissions: { denyInsteadOfPrompting: config.managed } };
if (JSON.stringify(settings) !== JSON.stringify(expected)) throw new Error("fixture_state");
if (config.kernelPolicy) {
  let outsideDenied = false;
  try { readFileSync(config.outsidePath); }
  catch (error) { if (["EACCES", "EPERM"].includes(error.code)) outsideDenied = true; else throw error; }
  if (!outsideDenied) throw new Error("outside_read_succeeded");
  if (settings !== null) {
    const info = statSync(config.fixturePath);
    if (info.uid !== 0 || (info.mode & 0o777) !== 0o444) throw new Error("canonical_owner_or_mode");
  }
  let writeDenied = false;
  try { writeFileSync(config.fixturePath, "x", { flag: "a" }); }
  catch (error) { if (["EACCES", "EPERM"].includes(error.code)) writeDenied = true; else throw error; }
  if (!writeDenied) throw new Error("canonical_write_succeeded");
  const networkError = await new Promise((resolve, reject) => {
    const request = http.request({ host: "127.0.0.1", port: config.forbiddenHostPort, path: "/", agent: false });
    request.once("response", () => reject(new Error("outside_host_reached")));
    request.once("error", (error) => resolve(error.code));
    request.setTimeout(500, () => request.destroy(new Error("canary_attempt_timeout")));
    request.end();
  });
  if (!["ECONNREFUSED", "ENETUNREACH", "EHOSTUNREACH"].includes(networkError)) throw new Error("canary_denial_unproved");
  console.error(JSON.stringify({ probeEvent: "boundary_attempts", pid: process.pid,
    outsideDenied, writeDenied, networkError, canonicalOwnedFixture: true,
    originalActivation: false }));
}
const stream = await post("/v1/messages", { model: "owned-model", stream: true,
  max_tokens: 64, messages: [{ role: "user", content: "owned child-spawn proof" }] });
if (!stream.includes('"output_tokens":5') || !stream.includes("message_stop")) throw new Error("stream_result");
console.error(JSON.stringify({ probeEvent: "native_entry", pid: process.pid, parent: process.ppid,
  ordinaryTcp: true, managed, fixtureState: config.fixtureState,
  nativeAdministratorDiscovery: false, kernelEnforced: false, originalActivation: false }));
await import("./fake-native.mjs");
