// Owned fake only: ordinary TCP, no connection patch and no inherited socket.
import fs from "node:fs";
import net from "node:net";
import http from "node:http";
const config = JSON.parse(fs.readFileSync(new URL("./native-config.json", import.meta.url)));
let outsideDenial = null;
if (config.forbiddenHostPort !== null) {
  outsideDenial = await new Promise((resolve, reject) => {
    const socket = net.createConnection({ host: "127.0.0.1", port: config.forbiddenHostPort });
    socket.setTimeout(500);
    socket.once("connect", () => { socket.destroy(); reject(new Error("outside_namespace_listener_reached")); });
    socket.once("timeout", () => { socket.destroy(); reject(new Error("outside_denial_unobserved")); });
    socket.once("error", (error) => {
      socket.destroy();
      if (["ECONNREFUSED", "ENETUNREACH", "EHOSTUNREACH"].includes(error.code)) resolve(error.code);
      else reject(error);
    });
  });
}
const body = JSON.stringify({ model: "owned-model", max_tokens: 64, stream: true,
  messages: [{ role: "user", content: "owned transport probe" }] });
const text = await new Promise((resolve, reject) => {
  const request = http.request({ host: "127.0.0.1", port: config.port,
    path: "/v1/messages", method: "POST", agent: false,
    headers: { "content-type": "application/json", "content-length": Buffer.byteLength(body),
      "anthropic-version": "2023-06-01", "connection": "close" } }, (response) => {
    if (response.statusCode !== 200) {
      response.destroy(); reject(new Error("owned_endpoint_refused")); return;
    }
    let received = "";
    response.setEncoding("utf8");
    response.on("data", (chunk) => {
      received += chunk;
      if (received.length > 65536) response.destroy(new Error("owned_response_limit"));
    });
    response.once("end", () => resolve(received));
    response.once("error", reject);
  });
  request.setTimeout(2000, () => request.destroy(new Error("owned_endpoint_deadline")));
  request.once("error", reject);
  request.end(body);
});
const frames = text.split("\n\n").filter(Boolean).map((block) => {
  const lines = block.split("\n");
  const frame = JSON.parse(lines.find((line) => line.startsWith("data: ")).slice(6));
  if (!lines.includes(`event: ${frame.type}`)) throw new Error("owned_frame_mismatch");
  return frame;
});
const expected = ["message_start", "content_block_start", "content_block_delta",
  "content_block_stop", "message_delta", "message_stop"];
if (JSON.stringify(frames.map((frame) => frame.type)) !== JSON.stringify(expected) ||
    frames[0].message.usage.input_tokens !== 21 || frames[4].usage.output_tokens !== 5 ||
    frames[4].delta.stop_reason !== "end_turn") throw new Error("owned_stream_mismatch");
console.error(JSON.stringify({ ordinaryTcp: true, inheritedConnectedSocket: false,
  outsideNamespaceDenial: outsideDenial,
  runtime: { platform: process.platform, arch: process.arch, node: process.version },
  nativeAdministratorDiscovery: false, originalActivation: false }));
await import("./fake-native.mjs");
