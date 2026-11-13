// Owned hostile peer. Every path and endpoint it attacks belongs to the test.
import fs from "node:fs";
import net from "node:net";
import { spawnSync } from "node:child_process";
const config = JSON.parse(fs.readFileSync(new URL("./native-config.json", import.meta.url)));
const attempt = (name, action) => {
  try {
    action();
    console.log(JSON.stringify({ name, denied: false }));
  } catch (error) {
    console.log(JSON.stringify({ name, denied: true, code: error.code }));
  }
};
if (config.attack === "filesystem") {
  attempt("read_outside", () => fs.readFileSync(config.outside));
  attempt("stat_outside", () => fs.statSync(config.outside));
  attempt("write_outside", () => fs.writeFileSync(config.outside, "changed"));
  attempt("symlink_escape", () => fs.readFileSync(config.escape));
  attempt("source_write", () => fs.writeFileSync(new URL("./confined-runner.mjs", import.meta.url), "changed"));
  attempt("extra_file", () => fs.writeFileSync("extra.bin", "changed"));
  attempt("new_directory", () => fs.mkdirSync("extra"));
  attempt("hardlink", () => fs.linkSync("storage.bin", "extra-link"));
} else if (config.attack === "fork") {
  const child = spawnSync(process.execPath, ["-e", "process.exit(73)"], { timeout: 1000 });
  console.log(JSON.stringify({ name: "fork", code: child.error?.code, status: child.status }));
} else if (config.attack === "network") {
  const socket = net.connect({ host: "127.0.0.1", port: config.forbiddenPort });
  socket.once("connect", () => { console.log('"forbidden_connected"'); socket.end(); });
  socket.once("error", (error) => console.log(JSON.stringify({ name: "network", code: error.code })));
} else if (config.attack === "storage") {
  attempt("storage", () => fs.writeFileSync("storage.bin", Buffer.alloc(2_000_000)));
} else if (config.attack === "output") {
  for (;;) fs.writeSync(1, Buffer.alloc(65536, 120));
} else if (config.attack === "hang") {
  console.log(JSON.stringify({ name: "barrier_ready" }));
  setInterval(() => {}, 1000);
}
