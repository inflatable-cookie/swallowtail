// Test-only replacement for one SDK spawn hook. The supervisor has already
// started the native under its own no-fork policy. No child is created here.
import childProcess from "node:child_process";
import { syncBuiltinESMExports } from "node:module";
import { EventEmitter } from "node:events";
import { Socket } from "node:net";
import readline from "node:readline";
import { readFileSync } from "node:fs";

const config = JSON.parse(readFileSync(new URL("./confined-config.json", import.meta.url)));
let used = false;
childProcess.spawn = (command, args, options) => {
  if (used || command !== process.execPath ||
      JSON.stringify(args) !== JSON.stringify(config.nativeArgs) ||
      options.cwd !== config.workspace) {
    throw new Error("confined_spawn_refused");
  }
  used = true;
  const child = new EventEmitter();
  child.pid = config.nativePid;
  child.stdin = new Socket({ fd: config.inputFd, readable: false, writable: true });
  child.stdout = new Socket({ fd: config.outputFd, readable: true, writable: false });
  child.stderr = new Socket({ fd: config.errorFd, readable: true, writable: false });
  child.kill = () => {
    // Only the supervisor owns signalling and observes waitpid. EOF requests
    // cleanup; a noncooperative peer reaches the supervisor's finite deadline.
    child.stdin.end();
    return true;
  };
  const exits = readline.createInterface({
    input: new Socket({ fd: config.exitFd, readable: true, writable: false }),
  });
  exits.once("line", (line) => {
    const exit = JSON.parse(line);
    child.exitCode = exit.code;
    child.signalCode = exit.signal;
    child.emit("exit", exit.code, exit.signal);
    child.emit("close", exit.code, exit.signal);
  });
  return child;
};
syncBuiltinESMExports();
