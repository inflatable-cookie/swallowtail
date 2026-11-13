// Owned-fake startup carrier, never an original native or administrator policy.
import fs from "node:fs";
import net from "node:net";
import { syncBuiltinESMExports } from "node:module";
import { createHash } from "node:crypto";
const config = JSON.parse(fs.readFileSync(new URL("./native-config.json", import.meta.url)));
if (config.inheritedEndpointFd !== null) {
  let used = false;
  const connect = (options, callback) => {
    if (used || options.host !== "127.0.0.1" ||
        String(options.port) !== config.providerEndpoint.split(":")[1]) {
      const socket = new net.Socket();
      queueMicrotask(() => socket.destroy(Object.assign(new Error("endpoint refused"), { code: "EPERM" })));
      return socket;
    }
    used = true;
    const socket = new net.Socket({ fd: config.inheritedEndpointFd, readable: true, writable: true });
    queueMicrotask(() => { socket.emit("connect"); callback?.(); });
    return socket;
  };
  net.connect = connect;
  net.createConnection = connect;
  syncBuiltinESMExports();
}
console.error(JSON.stringify({ boundaryRuntime: {
  platform: process.platform, arch: process.arch, node: process.version,
} }));
if (config.providerProtocol) {
  const bytes = fs.readFileSync(config.administratorSettings);
  const settings = JSON.parse(bytes);
  const deny = settings.permissions?.denyInsteadOfPrompting;
  if (typeof deny !== "boolean" || deny !== config.expectedManagedDeny) throw new Error("fake_settings_substitution");
  console.error(JSON.stringify({ fakeAdministratorInjection: {
    sha256: createHash("sha256").update(bytes).digest("hex"), denyInsteadOfPrompting: deny,
    userAndProjectSources: [], originalAdministratorDiscovery: false,
  } }));
  const response = await fetch(`http://${config.providerEndpoint}/v1/messages`, {
    method: "POST", headers: { "content-type": "application/json", "anthropic-version": "2023-06-01" },
    body: JSON.stringify({ model: process.argv[process.argv.indexOf("--model") + 1], max_tokens: 64, stream: true,
      messages: [{ role: "user", content: "owned protocol probe" }] }),
  });
  if (response.status !== 200 || !response.headers.get("content-type")?.startsWith("text/event-stream")) {
    throw new Error("owned_provider_protocol_response_refused");
  }
  let buffer = "";
  const frames = [];
  for await (const chunk of response.body) {
    buffer += new TextDecoder().decode(chunk);
    if (buffer.length > 65536) throw new Error("owned_provider_protocol_output_limit");
  }
  for (const block of buffer.split("\n\n").filter(Boolean)) {
    const lines = block.split("\n");
    const event = lines.find((line) => line.startsWith("event: "))?.slice(7);
    const frame = JSON.parse(lines.find((line) => line.startsWith("data: "))?.slice(6));
    if (frame.type !== event) throw new Error("owned_provider_event_mismatch");
    frames.push(frame);
  }
  const expected = ["message_start", "content_block_start", "content_block_delta",
    "content_block_stop", "message_delta", "message_stop"];
  if (JSON.stringify(frames.map((frame) => frame.type)) !== JSON.stringify(expected) ||
      frames[0].message.usage.input_tokens !== 21 || frames[4].usage.output_tokens !== 5 ||
      frames[4].delta.stop_reason !== "end_turn") throw new Error("owned_provider_stream_refused");
  console.error(JSON.stringify({ ownedProviderStream: {
    frameTypes: expected, inputTokens: 21, outputTokens: 5, originalNativeTransport: false,
  } }));
}
