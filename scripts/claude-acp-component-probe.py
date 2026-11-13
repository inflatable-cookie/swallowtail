#!/usr/bin/env python3
"""Confined owner for the idle-class component witness. Never runs ACP/SDK/native.

--fake is the only currently authorized execution. --original requires separate
exact authority naming source, owner, plan, guard, Node and module identities.
Neither mode is whole-route/native proof. No install or host mutation occurs.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import selectors
import time
import shutil
import stat
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SUPPORT = ROOT / "crates/swallowtail-adapter-claude-agent/tests/guard_component_probe"
MAX_OUTPUT = 1024 * 1024


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def runtime_files(node):
    """Read Mach-O linkage without executing the selected interpreter."""
    pending = [node]
    files, aliases = set(), set()
    while pending:
        binary = pending.pop()
        if binary in files or str(binary).startswith(("/System/", "/usr/lib/")):
            continue
        files.add(binary)
        result = subprocess.run(["/usr/bin/otool", "-L", str(binary)], env={"PATH":"/usr/bin:/bin"}, capture_output=True, text=True, timeout=5, check=True)
        for line in result.stdout.splitlines()[1:]:
            name = line.strip().split(" (", 1)[0]
            if name.startswith("@rpath/"):
                name = str(binary.parent.parent / "lib" / name[7:])
            elif name.startswith("@loader_path/"):
                name = str(binary.parent / name[13:])
            elif name.startswith("@"):
                raise ValueError("unclassified runtime dependency")
            candidate = Path(name)
            aliases.add(candidate)
            aliases.add(candidate.parent.resolve() / candidate.name)
            canonical = candidate.resolve()
            if canonical != binary:
                pending.append(canonical)
    return sorted(files), sorted(aliases)


def limits():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_FSIZE, (0, 0))
    resource.setrlimit(resource.RLIMIT_NOFILE, (256, 256))
    resource.setrlimit(resource.RLIMIT_CPU, (30, 30))



def bounded_output(child, wall_seconds=40, output_bytes=MAX_OUTPUT):
    deadline = time.monotonic() + wall_seconds
    output = {child.stdout: bytearray(), child.stderr: bytearray()}
    with selectors.DefaultSelector() as reader:
        for stream in output:
            reader.register(stream, selectors.EVENT_READ)
        try:
            while reader.get_map():
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    raise ValueError("component owner timeout; cleanup unconfirmed")
                for key, _ in reader.select(min(remaining, 1)):
                    block = os.read(key.fileobj.fileno(), 32768)
                    if not block:
                        reader.unregister(key.fileobj)
                    else:
                        output[key.fileobj].extend(block)
                        if len(output[key.fileobj]) > output_bytes:
                            raise ValueError("component output limit; cleanup unconfirmed")
            child.wait(timeout=max(0.01, deadline-time.monotonic()))
        except (ValueError, subprocess.TimeoutExpired):
            child.kill()
            child.wait(timeout=5)
            raise
        finally:
            for stream in output:
                stream.close()
    return bytes(output[child.stdout]), bytes(output[child.stderr])


def bounded_copy(source, destination, limit):
    if source.stat().st_size > limit:
        raise ValueError("component scratch input limit")
    with source.open("rb") as incoming, destination.open("xb") as outgoing:
        count = 0
        while block := incoming.read(32768):
            count += len(block)
            if count > limit:
                raise ValueError("component scratch input grew beyond limit")
            outgoing.write(block)


def main():
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--fake", action="store_true")
    mode.add_argument("--original", action="store_true")
    parser.add_argument("--owner", type=Path, required=True)
    parser.add_argument("--owner-sha256", required=True)
    parser.add_argument("--node", type=Path, required=True)
    parser.add_argument("--hop")
    parser.add_argument("--package", type=Path)
    args = parser.parse_args()
    owner, node = args.owner.resolve(strict=True), args.node.resolve(strict=True)
    if digest(owner) != args.owner_sha256:
        raise ValueError("owner artifact substitution")
    if args.original and (not args.hop or not args.package):
        raise ValueError("exact original component inputs absent")
    if args.fake and (args.hop or args.package):
        raise ValueError("fake mode accepts no original source")
    if platform.system() != "Darwin" or platform.machine() != "arm64":
        # No raw execution fallback. Linux needs the retained confined-executor
        # integration; availability of a namespace alone is not admission.
        raise ValueError("Linux x64 confined component executor is not admitted")
    sandbox = Path("/usr/bin/sandbox-exec")
    info = sandbox.stat()
    if info.st_uid != 0 or info.st_mode & (stat.S_ISUID | stat.S_IWGRP | stat.S_IWOTH):
        raise ValueError("untrusted sandbox owner")
    runtime, aliases = runtime_files(node)
    root = Path(tempfile.mkdtemp(prefix="acp-component-", dir=os.environ["TMPDIR"])).resolve()
    success = False
    try:
        bounded_copy(owner, root / "owner", 32 * 1024 * 1024)
        if digest(root / "owner") != args.owner_sha256:
            raise ValueError("copied owner identity mismatch")
        (root / "owner").chmod(0o700)
        bounded_copy(SUPPORT / "entry.mjs", root / "entry.mjs", 1024 * 1024)
        bounded_copy(SUPPORT / "idle.mjs" if args.fake else args.package / "dist/idle-compaction.js", root / "idle.mjs", 1024 * 1024)
        (root / "outside-denied").write_text("owned denied-read canary\n")
        dependencies = [{"path":str(path),"sha256":digest(path)} for path in runtime if path != node]
        readable = [root/"owner", root/"entry.mjs", root/"idle.mjs", root/"input.json", *runtime, *aliases, Path("/bin/sleep")]
        literals = " ".join(f"(literal {json.dumps(str(path))})" for path in readable)
        ancestors = " ".join(f"(path-ancestors {json.dumps(str(path))})" for path in readable)
        policy = f'''(version 1)
(deny default)
(import "/System/Library/Sandbox/Profiles/dyld-support.sb")
(allow syscall*)
(allow file-read-metadata)
(allow file-read-data {ancestors} {literals} (literal "/dev/null") (literal "/dev/urandom") (literal "/dev/random"))
(allow file-read* (subpath "/System/Library") (subpath "/usr/lib") (subpath "/System/Volumes/Preboot/Cryptexes/OS") (subpath "/System/Cryptexes/OS"))
(allow file-map-executable {literals} (subpath "/System/Library") (subpath "/usr/lib") (subpath "/System/Volumes/Preboot/Cryptexes/OS") (subpath "/System/Cryptexes/OS"))
(allow file-write-data (literal "/dev/null"))
(allow process-fork)
(allow process-exec {literals})
(allow process-info* (target self) (target children))
(allow signal (target children) (target same-sandbox))
(allow sysctl-read (sysctl-name-regex #"^hw\\.") (sysctl-name "kern.osrelease" "kern.ostype" "kern.osversion" "kern.version" "kern.argmax" "kern.hostname" "kern.osproductversion" "kern.osproductversioncompat"))
'''
        (root / "policy.sb").write_text(policy)
        results = []
        for control in (True, False):
            (root / "input.json").write_text(json.dumps({"root":str(root),"node":str(node),"mode":"fake" if args.fake else "original","hop":args.hop,"control":control,"dependencies":dependencies}))
            command = [str(sandbox),"-f",str(root/"policy.sb"),str(root/"owner"),str(root/"input.json")]
            child = subprocess.Popen(command, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env={"PATH":"/usr/bin:/bin","HOME":str(root),"TMPDIR":str(root),"TMP":str(root),"TEMP":str(root)}, preexec_fn=limits)
            stdout, stderr = bounded_output(child)
            if child.returncode or len(stdout)>MAX_OUTPUT or len(stderr)>MAX_OUTPUT:
                raise ValueError("confined component failed: "+stderr.decode(errors="replace")[:500])
            result = json.loads(stdout)
            if result.get("nativeExecuted") is not False or result.get("routeQualified") is not False:
                raise ValueError("component scope widened")
            results.append(result)
        print(json.dumps({"scope":"idle-class-component-only","mode":"fake" if args.fake else "original","hop":args.hop,"ownerSha256":args.owner_sha256,"sandboxSha256":digest(sandbox),"policySha256":hashlib.sha256(policy.encode()).hexdigest(),"host":{"system":platform.system(),"machine":platform.machine(),"kernel":platform.release()},"results":results},sort_keys=True))
        success = True
    finally:
        if success:
            shutil.rmtree(root)
        else:
            print("retained failed owned fixture: "+str(root),file=__import__("sys").stderr)


if __name__ == "__main__":
    main()
