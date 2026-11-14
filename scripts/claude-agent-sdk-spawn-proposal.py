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
    "scripts/claude-agent-sdk-spawn-sdk.mjs",
    "scripts/claude-agent-sdk-spawn-native.mjs",
    "scripts/claude-agent-sdk-spawn-driver.mjs",
    "scripts/tests/claude-agent-sdk-spawn-proposal.py",
    "scripts/claude-agent-sdk-probe-custody.py",
    "scripts/claude-agent-sdk-confined-probe.py",
    "scripts/claude-agent-sdk-admin-fixture.py",
    "docs/knowledge/operations/claude-agent-sdk-spawn-proposal.md",
    "docs/knowledge/operations/claude-agent-sdk-spawn-plan.json",
    "effigy.toml",
)
SCHEMA = "swallowtail.sdk234.private-spawn.v1"
STARTUP = ["--no-addons", "--openssl-config=/dev/null"]


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
        return {
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


def validate_entry(observed, driver, command):
    if observed["parent"] != driver or observed["group"] != driver:
        raise Refused("native_parent_or_group")
    if "command" in observed:
        valid = observed["command"] == command
    else:
        valid = observed["commandText"] == " ".join(command)
    if not valid:
        raise Refused("native_command_changed")


def stop_owned(driver, registered):
    """Signal only this rehearsal's directly owned driver and checked native."""
    for owned in registered:
        try:
            actual = process_identity(owned["pid"])
        except (ProcessLookupError, FileNotFoundError, subprocess.CalledProcessError):
            continue  # No signal and no reaping claim follow from disappearance.
        if any(actual[key] != owned[key] for key in ("pid", "start", "group")):
            raise Refused("cleanup_native_identity_changed")
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
    bound=3,
    output_limit=1000000,
    storage_limit=1000000,
    cancel=None,
    custody_hook=None,
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
    runtime = custody.runtime_inventory(node, node_sha)
    events, registered, requests = [], [], []
    failure, driver, pidfd = None, None, None
    captured_output = {"stdout": bytearray(), "stderr": bytearray()}
    phase = "prepare"
    started = time.monotonic()

    def emit(kind, **data):
        events.append({"seq": len(events), "phase": phase, "kind": kind, **data})

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
            native_command = [str(node), *STARTUP, *args]
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
                str(root / "entry.py"),
                str(root / "spawn-config.json"),
            ]
            driver_command = [str(node), *STARTUP, str(root / "driver.mjs")]
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
                        if self.path == "/native-entry":
                            phase = "native-entry"
                            if (
                                registered
                                or value.get("token") != config["token"]
                                or type(value.get("pid")) is not int
                            ):
                                raise Refused("native_registration")
                            observed = process_identity(value["pid"])
                            validate_entry(
                                observed, holder["driver"].pid, entry_command
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
                            self.send_response(200)
                            self.end_headers()
                            self.wfile.write(b"{}")
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
                "entry": {
                    "command": native_command,
                    "env": native_env,
                    "pythonSha256": sha(python_entry.read_bytes()),
                    "sourceSha256": sha(
                        files["scripts/claude-agent-sdk-spawn-entry.py"]
                    ),
                    "nodeSha256": node_sha,
                    "nativeSha256": sha((root / "native.mjs").read_bytes()),
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
            }
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
            for entry in root.rglob("*"):
                if entry.is_file() and entry.name != "storage.bin":
                    lease.bind(entry, sha(entry.read_bytes()))
            lease.configure([driver_command, native_command], [driver_env, native_env])
            phase = "driver-start"
            if custody_hook:
                custody_hook(phase, lease, root)
            if cancel and cancel(phase):
                raise Refused("cancel_driver-start")
            lease.permit(driver_command, driver_env)
            driver = subprocess.Popen(
                driver_command,
                cwd=config["workspace"],
                env=driver_env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                start_new_session=True,
            )
            holder["driver"] = driver
            emit("driver_started", identity=process_identity(driver.pid))
            thread = threading.Thread(
                target=server.serve_forever, kwargs={"poll_interval": 0.01}
            )
            thread.start()
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
                    if (
                        sum(p.stat().st_size for p in root.rglob("*") if p.is_file())
                        > storage_limit
                    ):
                        failure = "live_storage_limit"
                        break
                    for key, _ in selector.select(0.02):
                        data = os.read(key.fileobj.fileno(), 8192)
                        if not data:
                            selector.unregister(key.fileobj)
                            continue
                        captured_output[key.data].extend(data)
                        if sum(len(x) for x in captured_output.values()) > output_limit:
                            failure = "live_output_limit"
                            break
                    if failure:
                        break
            if driver.poll() is None and failure:
                driver.terminate()
            try:
                code = driver.wait(timeout=0.5)
            except subprocess.TimeoutExpired:
                stop_owned(driver, registered)
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
                    "fixture": {
                        "state": state,
                        "path": str(fixture),
                        "sha256": sha(fixture.read_bytes())
                        if fixture.exists()
                        else None,
                        "canonical": False,
                    },
                    "kernelAdmission": "unavailable",
                    "originalActivation": False,
                },
                "requests": requests,
                "stdout": captured_output["stdout"].decode(errors="replace"),
                "peerEvents": software,
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
                        stop_owned(driver, registered)
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
