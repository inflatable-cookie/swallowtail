"""Private SDK child-spawn rehearsal; no kernel/canonical/original entrypoint.

The Linux combined-host plan remains inactive. Ordinary rehearsal evidence
reports software waits separately from independent kernel reaping/admission.
"""

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import selectors
import re
import signal
import stat
import subprocess
import sys
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, HTTPServer

ROOT = Path(__file__).resolve().parents[1]
RECORD = Path("docs/knowledge/operations/claude-agent-sdk-spawn.sha256")
INPUTS = (
    "scripts/claude-agent-sdk-spawn-proposal.py",
    "scripts/claude-agent-sdk-spawn-entry.py",
    "scripts/claude-agent-sdk-spawn-owner.py",
    "scripts/claude-agent-sdk-spawn-policy.py",
    "scripts/claude-agent-sdk-spawn-trace.py",
    "scripts/claude-agent-sdk-spawn-cgroup.py",
    "scripts/claude-agent-sdk-spawn-combined.py",
    "scripts/claude-agent-sdk-netns-entry.py",
    "scripts/claude-agent-sdk-netns-kernel.py",
    "scripts/claude-agent-sdk-spawn-loader.mjs",
    "scripts/claude-agent-sdk-spawn-bootstrap.mjs",
    "scripts/claude-agent-sdk-spawn-sdk.mjs",
    "scripts/claude-agent-sdk-spawn-native.mjs",
    "scripts/claude-agent-sdk-spawn-driver.mjs",
    "scripts/tests/claude-agent-sdk-spawn-proposal.py",
    "scripts/tests/claude-agent-sdk-combined-proposal.py",
    "scripts/claude-agent-sdk-probe-custody.py",
    "scripts/claude-agent-sdk-confined-probe.py",
    "scripts/claude-agent-sdk-admin-fixture.py",
    "docs/knowledge/operations/claude-agent-sdk-spawn-proposal.md",
    "docs/knowledge/operations/claude-agent-sdk-spawn-plan.json",
    "docs/knowledge/operations/claude-agent-sdk-combined-workflow.patch",
    "effigy.toml",
)
SCHEMA = "swallowtail.sdk234.private-spawn.v1"
STARTUP = ["--no-addons", "--openssl-config=/dev/null"]


GRAPH_ATTACKS = {
    "direct-exec": 'import {spawnSync} from "node:child_process"; spawnSync(process.execPath,["-e","require(\'fs\').writeFileSync("+JSON.stringify(OWNED_MARKER)+",\'bad\')"]);',
    "alternate-interpreter": 'import {spawnSync} from "node:child_process"; spawnSync("/usr/bin/env",["node","-e","require(\'fs\').writeFileSync("+JSON.stringify(OWNED_MARKER)+",\'bad\')"]);',
    "create-require": 'import {createRequire} from "node:module"; createRequire(import.meta.url)("node:child_process").spawnSync(process.execPath,["-e","require(\'fs\').writeFileSync("+JSON.stringify(OWNED_MARKER)+",\'bad\')"]);',
    "worker": 'import {Worker} from "node:worker_threads"; new Worker("require(\'fs\').writeFileSync("+JSON.stringify(OWNED_MARKER)+",\'bad\')",{eval:true});',
    "dynamic-code": 'import {writeFileSync} from "node:fs"; Function("file","writeFileSync(file,\'bad\')")(OWNED_MARKER);',
    "vm": 'import vm from "node:vm"; vm.runInNewContext("effect()",{effect(){throw Error("effect")}});',
    "addon": "process.dlopen({},OWNED_MARKER);",
    "process-binding": 'process.binding("spawn_sync");',
    "data-import": 'await import("data:text/javascript,globalThis.bad=true");',
}


class Refused(Exception):
    pass


def sha(data):
    return hashlib.sha256(data).hexdigest()


def captured(expected, root=ROOT):
    record = (root / RECORD).read_bytes()
    if sha(record) != expected:
        raise Refused("source_record_changed")
    result = {}
    for row in record.decode().splitlines():
        digest, name = row.split("  ", 1)
        file = root / name
        if name not in INPUTS or file.resolve(strict=True) != file.absolute():
            raise Refused("source_path")
        fd = os.open(file, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            if not stat.S_ISREG(os.fstat(fd).st_mode):
                raise Refused("source_type")
            with os.fdopen(fd, "rb", closefd=False) as stream:
                data = stream.read(8 * 1024 * 1024 + 1)
            if len(data) > 8 * 1024 * 1024 or sha(data) != digest:
                raise Refused("source_bytes_changed")
            result[name] = data
        finally:
            os.close(fd)
    if tuple(result) != INPUTS:
        raise Refused("source_file_set")
    return result


def module(name, data):
    spec = importlib.util.spec_from_file_location(name, ROOT / name)
    value = importlib.util.module_from_spec(spec)
    exec(compile(data, str(ROOT / name), "exec"), value.__dict__)
    return value


def process_identity(pid):
    """Read OS parent/group/start/command; never grants wait rights."""
    if sys.platform == "linux":
        base = Path(f"/proc/{pid}")
        fields = (base / "stat").read_text().split(") ", 1)[1].split()
        command = (base / "cmdline").read_bytes().rstrip(b"\0").split(b"\0")
        status = {
            line.split(":", 1)[0]: line.split(":", 1)[1].strip()
            for line in (base / "status").read_text().splitlines()
            if ":" in line
        }
        ns = (base / "ns/net").stat()
        return {
            "namespace": {"device": ns.st_dev, "inode": ns.st_ino},
            "cgroup": (base / "cgroup").read_text(),
            "uids": [int(x) for x in status["Uid"].split()],
            "gids": [int(x) for x in status["Gid"].split()],
            "pid": pid,
            "parent": int(fields[1]),
            "group": int(fields[2]),
            "start": fields[19],
            "command": [x.decode() for x in command],
        }
    observed = (
        subprocess.check_output(
            [
                "ps",
                "-ww",
                "-p",
                str(pid),
                "-o",
                "ppid=",
                "-o",
                "pgid=",
                "-o",
                "lstart=",
                "-o",
                "command=",
            ],
            text=True,
            timeout=1,
            env={"PATH": "/usr/bin:/bin", "LC_ALL": "C"},
        )
        .strip()
        .split(None, 7)
    )
    if len(observed) != 8:
        raise Refused("process_identity_missing")
    return {
        "pid": pid,
        "parent": int(observed[0]),
        "group": int(observed[1]),
        "start": " ".join(observed[2:7]),
        "commandText": observed[7],
    }


def validate_entry(observed, driver_actor, command):
    # This actor record comes from the trusted host's real launch/observation,
    # not from a route's trace or a public runtime handle.
    if (
        observed["parent"] != driver_actor["pid"]
        or observed["group"] != driver_actor["group"]
    ):
        raise Refused("native_parent_or_group")
    for key in ("namespace", "cgroup", "uids", "gids"):
        if key in driver_actor and observed.get(key) != driver_actor[key]:
            raise Refused("native_" + key)
    if "command" in observed:
        valid = observed["command"] == command
    else:
        valid = observed["commandText"] == " ".join(command)
    if not valid:
        raise Refused("native_command_changed")


class ActorRegistry:
    """Host-owned finite actor graph; JSON receipts cannot create custody."""

    def __init__(self, profile, observer=None):
        if profile not in {"sdk", "acp-actor-shape"}:
            raise Refused("actor_profile")
        self.owner = os.getpid()
        self.profile = profile
        self.observe = observer or process_identity
        self.actors = {"monitor": self.observe(self.owner)}

    def bind(self, role, pid, command, parent, group=None):
        if (
            os.getpid() != self.owner
            or role in self.actors
            or parent not in self.actors
        ):
            raise Refused("actor_owner_duplicate_or_parent")
        allowed = {
            "group-owner": "monitor",
            "driver": "group-owner" if self.profile == "acp-actor-shape" else "monitor",
            "native": "driver",
        }
        if allowed.get(role) != parent or (
            role == "group-owner" and self.profile != "acp-actor-shape"
        ):
            raise Refused("actor_graph_edge")
        observed = self.observe(pid)
        wanted_group = pid if group is None else self.actors[group]["group"]
        expected = {
            **{
                key: value
                for key, value in self.actors[parent].items()
                if key in ("namespace", "cgroup", "uids", "gids")
            },
            "pid": self.actors[parent]["pid"],
            "group": wanted_group,
        }
        validate_entry(observed, expected, command)
        self.actors[role] = observed
        return observed

    def driver(self):
        if os.getpid() != self.owner or "driver" not in self.actors:
            raise Refused("driver_actor_missing")
        actual = self.observe(self.actors["driver"]["pid"])
        if any(
            actual[key] != self.actors["driver"][key]
            for key in ("pid", "parent", "group", "start")
        ):
            raise Refused("driver_actor_changed")
        return actual


def graph_config(root, role, entry_command, native_env):
    """Close the selected first-party graph, not an arbitrary JS program."""
    names = [p for p in root.iterdir() if p.suffix in {".mjs", ".cjs"}]
    files = [
        {
            "url": p.as_uri(),
            "source": p.read_text(),
            "format": "commonjs" if p.suffix == ".cjs" else "module",
        }
        for p in names
    ]
    targets = {x["url"] for x in files}
    builtins = {
        "node:fs",
        "node:fs/promises",
        "node:path",
        "node:process",
        "node:url",
        "node:readline",
        "node:stream",
        "node:child_process",
        "node:module",
        "node:crypto",
        "node:http",
        "node:net",
        "node:events",
    }
    edges = {}
    pattern = r"(?:from\s+|import\s*\(\s*|require\s*\(\s*|import\s+)[\"']([^\"']+)[\"']"
    for file in files:
        for specifier in re.findall(pattern, file["source"]):
            if specifier in builtins:
                target = specifier
            elif specifier.startswith("./"):
                target = (root / specifier[2:]).as_uri()
                if target not in targets:
                    continue
            else:
                continue
            edges[(file["url"], specifier)] = target
    entry = (root / ("driver.mjs" if role == "driver" else "native.mjs")).as_uri()
    edges[(None, entry)] = entry
    sdk = (root / "probe-sdk.mjs").as_uri()
    edges[((root / "claude-agent-sdk-sidecar.mjs").as_uri(), sdk)] = sdk
    python = Path(entry_command[0])
    info = python.stat()
    ancestors = []
    for ancestor in python.parents:
        st = ancestor.stat()
        ancestors.append(
            {
                "path": str(ancestor),
                "dev": str(st.st_dev),
                "ino": str(st.st_ino),
                "uid": str(st.st_uid),
                "mode": str(st.st_mode),
            }
        )
    native = {
        "path": str(python),
        "sha256": sha(python.read_bytes()),
        "uid": str(info.st_uid),
        "bytes": str(info.st_size),
        "dev": str(info.st_dev),
        "ino": str(info.st_ino),
        "ancestors": ancestors,
        "argv": entry_command[1:],
        "cwd": str(root / "workspace"),
        "env": native_env,
        "callers": [],
    }
    if role == "driver":
        native["callers"] = [
            (root / name).as_uri()
            for name in ("claude-agent-sdk-sidecar.mjs", "prepared-probe.mjs")
        ]
    return {
        "files": files,
        "edges": [
            {"parent": p, "specifier": x, "target": y} for (p, x), y in edges.items()
        ],
        "native": native,
        "requireParents": [],
        "platform": __import__("platform").system().lower().replace("darwin", "darwin")
        + "-"
        + ("arm64" if __import__("platform").machine() == "arm64" else "x64"),
    }


def stop_owned(driver, registered, observer=None, stop=None):
    """Signal only this rehearsal's directly owned driver and checked native."""
    for owned in registered:
        try:
            actual = (observer or process_identity)(owned["pid"])
        except (ProcessLookupError, FileNotFoundError, subprocess.CalledProcessError):
            continue  # No signal and no reaping claim follow from disappearance.
        if any(actual[key] != owned[key] for key in ("pid", "start", "group")):
            raise Refused("cleanup_native_identity_changed")
        if stop is not None:
            stop(owned)
        else:
            os.kill(owned["pid"], signal.SIGKILL)
    if driver.poll() is None:
        driver.kill()


def parse_events(data):
    rows = []
    for line in data.splitlines():
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    return rows


def rehearsal(
    expected,
    node,
    node_sha,
    *,
    state="absent",
    mode="sidecar",
    scenario="permission",
    composition="sdk",
    bound=10,
    output_limit=1000000,
    storage_limit=1000000,
    cancel=None,
    custody_hook=None,
    context=None,
    fault=None,
):
    if state not in {"absent", "disabled", "managed-deny"} or mode not in {
        "sidecar",
        "probe",
    }:
        raise Refused("rehearsal_scope")
    if (
        not 0 < bound <= 30
        or not 0 < output_limit <= 1000000
        or not 0 < storage_limit <= 1000000
    ):
        raise Refused("bounds")
    registry = ActorRegistry(composition, context.observe if context else None)
    files = captured(expected)
    custody = module(
        "scripts/claude-agent-sdk-probe-custody.py",
        files["scripts/claude-agent-sdk-probe-custody.py"],
    )
    old = module(
        "scripts/claude-agent-sdk-confined-probe.py",
        files["scripts/claude-agent-sdk-confined-probe.py"],
    )
    accepted = old.verified_sources(
        ROOT, "aa24aac1b8d8a223d83982103c2e18823877822911215eb3c3c30dcf695d08cb"
    )
    node = Path(node).resolve(strict=True)
    runtime = context.runtime if context else custody.runtime_inventory(node, node_sha)
    events, registered, requests = [], [], []
    event_lock = threading.Lock()
    peer_pending = bytearray()
    failure, driver, pidfd = None, None, None
    captured_output = {"stdout": bytearray(), "stderr": bytearray()}
    phase = "prepare"
    started = time.monotonic()

    def emit(kind, **data):
        with event_lock:
            events.append({"seq": len(events), "phase": phase, "kind": kind, **data})

    def observe_peer(data):
        peer_pending.extend(data)
        while b"\n" in peer_pending:
            line, rest = peer_pending.split(b"\n", 1)
            peer_pending[:] = rest
            try:
                row = json.loads(line)
            except (ValueError, UnicodeError):
                continue
            if isinstance(row, dict) and row.get("probeEvent") in {
                "spawn_requested",
                "native_started",
                "native_entry",
                "native_wait",
                "actor_wait",
                "boundary_attempts",
            }:
                emit(
                    row["probeEvent"],
                    source="peer-stream; source-bound software observation, not kernel attestation",
                    observation=row,
                )

    with tempfile.TemporaryDirectory(
        prefix="sdk234-spawn-", dir=os.environ["TMPDIR"]
    ) as temporary:
        root = Path(temporary)
        lease = custody.Lease(
            runtime,
            node,
            node_sha,
            {"schema": SCHEMA, "fakeOnly": True},
            reserve_pipes=False,
        )
        server, thread = None, None
        try:
            for name, data in accepted.items():
                lease.bind(ROOT / name, sha(data))
            for name, data in files.items():
                lease.bind(ROOT / name, sha(data))
            for name in ("home", "tmp", "workspace", "fixture"):
                (root / name).mkdir(mode=0o700)
            for source, target in {
                "fake-sdk-native-wire.mjs": "base-sdk.mjs",
                "fake-native.mjs": "fake-native.mjs",
                "confined-runner.mjs": "prepared-probe.mjs",
            }.items():
                data = accepted[old.SUPPORT / source]
                if source == "confined-runner.mjs":
                    anchor = b"process.exit(0);"
                    if data.count(anchor) != 1:
                        raise Refused("prepared_driver_binding")
                    data = data.replace(
                        b"spawn(request.command, request.args, request)",
                        b'spawn(request.command, request.args, {...request, detached:false, stdio:["pipe","pipe","pipe"]})',
                    )
                    data = data.replace(
                        anchor,
                        b'await import("./probe-sdk.mjs").then((m) => m.waitForNativeExit());\nprocess.exit(0);',
                    )
                (root / target).write_bytes(data)
            for source, target in {
                "sdk": "probe-sdk.mjs",
                "native": "native.mjs",
                "driver": "driver.mjs",
            }.items():
                (root / target).write_bytes(
                    files[f"scripts/claude-agent-sdk-spawn-{source}.mjs"]
                )
            (root / "group-owner.py").write_bytes(
                files["scripts/claude-agent-sdk-spawn-owner.py"]
            )
            (root / "captured-loader.mjs").write_bytes(
                files["scripts/claude-agent-sdk-spawn-loader.mjs"]
            )
            for role in ("driver", "native"):
                (root / f"bootstrap-{role}.mjs").write_bytes(
                    files["scripts/claude-agent-sdk-spawn-bootstrap.mjs"]
                )
            (root / "entry.py").write_bytes(
                files["scripts/claude-agent-sdk-spawn-entry.py"]
            )
            module(
                "scripts/claude-agent-sdk-spawn-entry.py",
                files["scripts/claude-agent-sdk-spawn-entry.py"],
            )
            sidecar = Path(
                "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
            )
            (root / "claude-agent-sdk-sidecar.mjs").write_bytes(accepted[sidecar])
            template = json.loads(accepted[old.FIXTURE / "probe-options.json"])
            (root / "probe-options.json").write_text(json.dumps(template))
            (root / "package.json").write_text(
                json.dumps(
                    {"name": "@anthropic-ai/claude-agent-sdk", "version": "0.3.317"}
                )
            )
            (root / "manifest.json").write_text('{"version":"2.1.317"}')
            fixture = root / "fixture/managed-settings.json"
            if state != "absent":
                fixture.write_text(
                    json.dumps(
                        {
                            "permissions": {
                                "denyInsteadOfPrompting": state == "managed-deny"
                            }
                        }
                    )
                )
                fixture.chmod(0o400)
            # Opposite controls are owned fixture data, never canonical settings.
            for name in ("home", "workspace"):
                (root / name / ".claude").mkdir(mode=0o700)
                (root / name / ".claude/settings.json").write_text(
                    json.dumps(
                        {
                            "permissions": {
                                "denyInsteadOfPrompting": state != "managed-deny"
                            }
                        }
                    )
                )
            (root / "storage.bin").touch()
            native_scenario = "managed-deny" if state == "managed-deny" else scenario
            (root / "native-config.json").write_text(
                json.dumps({"scenario": native_scenario})
            )
            (root / "sdk-config.json").write_text(
                json.dumps({"doubleSpawn": fault == "double-start"})
            )
            args = [
                str(root / "native.mjs"),
                "--output-format",
                "stream-json",
                "--verbose",
                "--input-format",
                "stream-json",
                "--model",
                template["options"]["model"],
                "--permission-prompt-tool=stdio",
                "--disallowedTools",
                ",".join(template["options"]["disallowedTools"]),
                "--tools",
                ",".join(template["options"]["tools"]),
                "--setting-sources",
                "",
                "--strict-mcp-config",
                "--permission-mode",
                "default",
            ]
            native_command = [
                str(node),
                *STARTUP,
                "--disallow-code-generation-from-strings",
                "--import",
                str(root / "bootstrap-native.mjs"),
                *args,
            ]
            python_entry = Path(sys.executable).resolve(strict=True)
            if sys.platform == "darwin":
                python_entry = Path(
                    subprocess.check_output(
                        ["ps", "-p", str(os.getpid()), "-o", "comm="],
                        text=True,
                        timeout=1,
                        env={"PATH": "/usr/bin:/bin", "LC_ALL": "C"},
                    ).strip()
                ).resolve(strict=True)
            lease.bind(python_entry, sha(python_entry.read_bytes()))
            entry_command = [
                str(python_entry),
                "-I",
                "-B",
                "-S",
                str(root / "entry.py"),
                str(root / "spawn-config.json"),
                "native",
            ]
            driver_command = [
                str(node),
                *STARTUP,
                "--disallow-code-generation-from-strings",
                "--import",
                str(root / "bootstrap-driver.mjs"),
                str(root / "driver.mjs"),
            ]
            native_env = {
                "PATH": "/usr/bin:/bin",
                "HOME": str(root / "home"),
                "TMPDIR": str(root / "tmp"),
                "LANG": "C",
                "LC_ALL": "C",
            }
            driver_env = {
                **native_env,
                "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE": str(root / "probe-sdk.mjs"),
                "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY": str(root / "native.mjs"),
                "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST": str(root / "manifest.json"),
            }
            if mode == "probe":
                driver_env = dict(native_env)
            driver_entry_command = [
                str(python_entry),
                "-I",
                "-B",
                "-S",
                str(root / "entry.py"),
                str(root / "spawn-config.json"),
                "driver",
            ]
            owner_command = [
                str(python_entry),
                "-I",
                "-B",
                "-S",
                str(root / "group-owner.py"),
                str(root / "spawn-config.json"),
            ]
            errors = []
            holder = {}

            class Endpoint(BaseHTTPRequestHandler):
                def do_POST(self):
                    nonlocal phase, pidfd
                    self.connection.settimeout(1)
                    try:
                        length = int(self.headers.get("Content-Length", "0"))
                        if not 0 < length <= 8192 or any(
                            x.lower() in {"authorization", "x-api-key"}
                            for x in self.headers
                        ):
                            raise Refused("endpoint_input")
                        value = json.loads(self.rfile.read(length))
                        if self.path == "/driver-entry":
                            phase = "driver-entry"
                            if (
                                value.get("token") != config["token"]
                                or type(value.get("pid")) is not int
                            ):
                                raise Refused("driver_registration")
                            actor = registry.bind(
                                "driver",
                                value["pid"],
                                driver_entry_command,
                                "group-owner"
                                if composition == "acp-actor-shape"
                                else "monitor",
                                "group-owner"
                                if composition == "acp-actor-shape"
                                else None,
                            )
                            lease.permit(driver_command, driver_env)
                            holder["actor"] = actor
                            emit("driver_released", identity=actor)
                            answer = (
                                {
                                    "kernel": context.driver_gate(
                                        value["pid"],
                                        self.client_address[1],
                                        server.server_port,
                                    )
                                }
                                if context
                                else {}
                            )
                            self.send_response(200)
                            self.end_headers()
                            self.wfile.write(json.dumps(answer).encode())
                            phase = "protocol"
                        elif self.path == "/native-entry":
                            phase = "native-entry"
                            if (
                                registered
                                or value.get("token") != config["token"]
                                or type(value.get("pid")) is not int
                            ):
                                raise Refused("native_registration")
                            registry.driver()
                            observed = registry.bind(
                                "native",
                                value["pid"],
                                entry_command,
                                "driver",
                                "driver",
                            )
                            if (
                                value.get("entrySourceSha256")
                                != config["entry"]["sourceSha256"]
                            ):
                                raise Refused("native_entry_source")
                            if fault == "wrong-parent":
                                raise Refused("native_parent_or_group")
                            if custody_hook:
                                custody_hook("native-entry", lease, root)
                            lease.permit(native_command, native_env)
                            if cancel and cancel("native-entry"):
                                raise Refused("cancel_native-entry")
                            if hasattr(os, "pidfd_open"):
                                pidfd = os.pidfd_open(value["pid"])
                            registered.append(observed)
                            emit("native_released", identity=observed)
                            answer = (
                                {
                                    "kernel": context.native_gate(
                                        value["pid"],
                                        self.client_address[1],
                                        server.server_port,
                                    )
                                }
                                if context
                                else {}
                            )
                            self.send_response(200)
                            self.end_headers()
                            self.wfile.write(json.dumps(answer).encode())
                            phase = "protocol"
                        elif self.path == "/v1/messages":
                            if len(registered) != 1 or requests:
                                raise Refused("endpoint_before_entry_or_repeat")
                            requests.append(value)
                            body = b'event: message_delta\ndata: {"type":"message_delta","usage":{"output_tokens":5}}\n\nevent: message_stop\ndata: {"type":"message_stop"}\n\n'
                            self.send_response(200)
                            self.send_header("Content-Length", str(len(body)))
                            self.end_headers()
                            self.wfile.write(body)
                        else:
                            raise Refused("endpoint_route")
                    except (OSError, ValueError, KeyError, Refused) as error:
                        errors.append(str(error))
                        self.send_error(403)

                def log_message(self, *args):
                    pass

            class BoundedServer(HTTPServer):
                def get_request(self):
                    connection, address = super().get_request()
                    connection.settimeout(1)
                    return connection, address

            server = BoundedServer(("127.0.0.1", 0), Endpoint)
            server.timeout = 1
            config = {
                "workspace": str(root / "workspace"),
                "originalActivation": False,
                "entryCommand": entry_command,
                "driverEntryCommand": driver_entry_command,
                "driverEnv": driver_env,
                "driverEntry": {
                    "command": driver_command,
                    "env": driver_env,
                    "pythonSha256": sha(python_entry.read_bytes()),
                    "sourceSha256": sha(
                        files["scripts/claude-agent-sdk-spawn-entry.py"]
                    ),
                    "nodeSha256": node_sha,
                    "nativePath": str(root / "driver.mjs"),
                    "nativeSha256": sha((root / "driver.mjs").read_bytes()),
                },
                "entry": {
                    "command": native_command,
                    "env": native_env,
                    "pythonSha256": sha(python_entry.read_bytes()),
                    "sourceSha256": sha(
                        files["scripts/claude-agent-sdk-spawn-entry.py"]
                    ),
                    "nodeSha256": node_sha,
                    "nativeSha256": sha((root / "native.mjs").read_bytes()),
                    "nativePath": str(root / "native.mjs"),
                },
                "nativeArgs": args,
                "nativeStartup": STARTUP,
                "nativeEnvKeys": sorted(native_env),
                "mode": mode,
                "model": template["options"]["model"],
                "fixtureState": state,
                "fixturePath": str(fixture),
                "managed": state == "managed-deny",
                "port": server.server_port,
                "token": os.urandom(16).hex(),
                "fault": fault,
                "graphAttack": fault in GRAPH_ATTACKS,
                "launchAttack": fault == "alternate-flags",
            }
            if context:
                for name in custody.python_inputs():
                    lease.bind(name)
                context.prepare(
                    root,
                    config,
                    driver_command,
                    native_command,
                    driver_entry_command,
                    entry_command,
                    owner_command,
                    driver_env,
                    native_env,
                    lease,
                )
            (root / "spawn-config.json").write_text(json.dumps(config))
            (root / "confined-config.json").write_text(
                json.dumps(
                    {
                        "workspace": config["workspace"],
                        "nativeArgs": args,
                        "scenario": native_scenario,
                    }
                )
            )
            attack = GRAPH_ATTACKS.get(fault, "export const owned = true;")
            attack = attack.replace(
                "OWNED_MARKER", json.dumps(str(root / "bypass-effect"))
            )
            (root / "graph-attack.mjs").write_text(attack)
            for role in ("driver", "native"):
                (root / f"{role}-graph.json").write_text(
                    json.dumps(graph_config(root, role, entry_command, native_env))
                )
            for entry in root.rglob("*"):
                if entry.is_file() and entry.name != "storage.bin":
                    lease.bind(entry, sha(entry.read_bytes()))
            lease.configure([driver_command, native_command], [driver_env, native_env])
            if context:
                context.ready(lease)
            phase = "driver-start"
            if custody_hook:
                custody_hook(phase, lease, root)
            if cancel and cancel(phase):
                raise Refused("cancel_driver-start")
            thread = threading.Thread(
                target=server.serve_forever, kwargs={"poll_interval": 0.01}
            )
            thread.start()
            host_command = (
                owner_command
                if composition == "acp-actor-shape"
                else driver_entry_command
            )
            driver = subprocess.Popen(
                host_command,
                cwd=config["workspace"],
                env=driver_env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                start_new_session=True,
            )
            holder["driver"] = driver
            if composition == "acp-actor-shape":
                emit(
                    "group_owner_started",
                    identity=registry.bind(
                        "group-owner", driver.pid, owner_command, "monitor"
                    ),
                )
            phase = "protocol"
            with selectors.DefaultSelector() as selector:
                for name in captured_output:
                    stream = getattr(driver, name)
                    os.set_blocking(stream.fileno(), False)
                    selector.register(stream, selectors.EVENT_READ, name)
                while selector.get_map():
                    if time.monotonic() - started > bound:
                        failure = "deadline_" + phase
                        break
                    if cancel and cancel(phase):
                        failure = "cancel_" + phase
                        break
                    if errors:
                        failure = errors[0]
                        break
                    if (root / "storage.bin").stat().st_size > storage_limit:
                        failure = "live_storage_limit"
                        break
                    for key, _ in selector.select(0.02):
                        data = os.read(key.fileobj.fileno(), 8192)
                        if not data:
                            selector.unregister(key.fileobj)
                            continue
                        captured_output[key.data].extend(data)
                        if key.data == "stderr":
                            observe_peer(data)
                        if sum(len(x) for x in captured_output.values()) > output_limit:
                            failure = "live_output_limit"
                            break
                    if failure:
                        break
            if driver.poll() is None and failure:
                emit(
                    "stop_requested", actor=os.getpid(), pid=driver.pid, reason=failure
                )
                driver.terminate()
                emit(
                    "stop_acknowledged",
                    actor=os.getpid(),
                    pid=driver.pid,
                    source="Popen.terminate; not a wait",
                )
            try:
                code = driver.wait(timeout=0.5)
            except subprocess.TimeoutExpired:
                stop_owned(
                    driver,
                    registered,
                    context.observe if context else None,
                    context.stop if context else None,
                )
                code = driver.wait(timeout=1)
            emit("driver_wait", pid=driver.pid, actor=os.getpid(), code=code)
            software = parse_events(captured_output["stderr"].decode(errors="replace"))
            native_waits = [x for x in software if x.get("probeEvent") == "native_wait"]
            if fault == "missing-wait":
                native_waits = []
            if not failure and (
                code != 0
                or errors
                or len(native_waits) != 1
                or native_waits[0].get("code") != 0
            ):
                failure = errors[0] if errors else "native_or_driver_wait_failed"
            if any(x.get("probeEvent") == "start_refused" for x in software):
                failure = "second_native_start"
            kernel_reaped = False
            if pidfd is not None:
                import select

                poll = select.poll()
                poll.register(pidfd, select.POLLHUP)
                kernel_reaped = bool(
                    any(flags & select.POLLHUP for _, flags in poll.poll(100))
                )
            phase = "cleanup"
            if fault == "cleanup":
                failure = "cleanup_failed"
            observation = lease.observation()
            return {
                "schema": SCHEMA,
                "kernelEnforced": False,
                "originalActivation": False,
                "productionAdmission": False,
                "nativeAdministratorDiscovery": False,
                "failure": failure,
                "events": events,
                "registered": registered,
                "nativeWaits": native_waits,
                "actorGraph": registry.actors,
                "actorWaits": [
                    x for x in software if x.get("probeEvent") == "actor_wait"
                ],
                "driverWait": code,
                "independentNativeReapingObserved": kernel_reaped,
                "fullKernelTreeSettlement": False,
                "custody": observation,
                "invocation": {
                    "schema": SCHEMA,
                    "sourceRecord": expected,
                    "runtime": runtime,
                    "driver": {"argv": driver_command, "env": driver_env},
                    "native": {"entryArgv": entry_command, **config["entry"]},
                    "actorProfile": composition,
                    "actors": registry.actors,
                    "fixture": {
                        "state": state,
                        "path": config["fixturePath"],
                        "sha256": sha(Path(config["fixturePath"]).read_bytes())
                        if Path(config["fixturePath"]).exists()
                        else None,
                        "canonical": context is not None,
                    },
                    "kernelAdmission": "unavailable",
                    "originalActivation": False,
                },
                "requests": requests,
                "stdout": captured_output["stdout"].decode(errors="replace"),
                "peerEvents": software,
                "bypassEffectObserved": (root / "bypass-effect").exists(),
                "stderr": captured_output["stderr"].decode(errors="replace"),
                "fixtureState": state,
                "fixtureCleanup": {"ownedScratchOnly": True, "canonical": False},
            }
        except (OSError, ValueError, Refused) as error:
            return {
                "schema": SCHEMA,
                "kernelEnforced": False,
                "originalActivation": False,
                "failure": str(error),
                "events": events,
                "registered": registered,
                "fullKernelTreeSettlement": False,
            }
        finally:
            try:
                if driver is not None:
                    try:
                        stop_owned(
                            driver,
                            registered,
                            context.observe if context else None,
                            context.stop if context else None,
                        )
                    finally:
                        driver.wait(timeout=1)
                        driver.stdout.close()
                        driver.stderr.close()
            finally:
                try:
                    if server is not None:
                        try:
                            if thread is not None:
                                server.shutdown()
                                thread.join(timeout=1)
                                if thread.is_alive():
                                    raise Refused("endpoint_join_missing")
                        finally:
                            server.server_close()
                finally:
                    if pidfd is not None:
                        os.close(pidfd)
                    lease.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rehearse", action="store_true")
    parser.add_argument("--record-sha256", required=True)
    parser.add_argument("--node", required=True)
    parser.add_argument("--node-sha256", required=True)
    parser.add_argument(
        "--state", choices=("absent", "disabled", "managed-deny"), default="absent"
    )
    parser.add_argument("--mode", choices=("sidecar", "probe"), default="sidecar")
    args = parser.parse_args()
    if not args.rehearse:
        raise Refused("kernel_canonical_and_original_modes_unavailable")
    result = rehearsal(
        args.record_sha256,
        args.node,
        args.node_sha256,
        state=args.state,
        mode=args.mode,
    )
    print(json.dumps(result))
    return 1 if result["failure"] else 0


if __name__ == "__main__":
    raise SystemExit(main())
