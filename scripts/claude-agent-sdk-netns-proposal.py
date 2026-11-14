"""Private namespace proposal. Rehearsal models namespace operations explicitly.

The kernel branch belongs to a future exact-source fake-only authority request.
No original mode or production admission is implemented.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import socket
import stat
import subprocess
import sys
import tempfile
import threading
import time
import types
from http.server import BaseHTTPRequestHandler, HTTPServer

ROOT = Path(__file__).resolve().parents[1]
RECORD = Path("docs/knowledge/operations/claude-agent-sdk-netns.sha256")
CUSTODY = "scripts/claude-agent-sdk-probe-custody.py"
ADMISSION = "docs/knowledge/operations/claude-agent-sdk-netns-admission.json"
INPUTS = (
    CUSTODY,
    ".github/workflows/sdk234-netns-proof.yml",
    "scripts/tests/oxide-selector-routing.py",
    "scripts/claude-agent-sdk-netns-proposal.py",
    "scripts/claude-agent-sdk-netns-kernel.py",
    "scripts/claude-agent-sdk-netns-entry.py",
    "scripts/claude-agent-sdk-netns-native.mjs",
    "scripts/tests/claude-agent-sdk-netns-proposal.py",
    "scripts/claude-agent-sdk-confined-probe.py",
    "crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-native-proof/confined-execution.sha256",
    ADMISSION,
    "docs/knowledge/operations/claude-agent-sdk-loopback-namespace.md",
    "docs/knowledge/operations/claude-agent-sdk-netns-ci.patch",
    "effigy.toml",
)
PHASES = (
    "preflight",
    "namespace",
    "loopback",
    "identity",
    "drop",
    "protocol",
    "cleanup",
)
OLD_RECORD = "aa24aac1b8d8a223d83982103c2e18823877822911215eb3c3c30dcf695d08cb"


class Refused(Exception):
    """No admitted result follows from a missing observation or capability."""


def sha(data):
    return hashlib.sha256(data).hexdigest()


def snapshots(root, expected):
    record = root / RECORD
    if record.is_symlink() or sha(record.read_bytes()) != expected:
        raise Refused("record_mismatch")
    result = {}
    for row in record.read_text().splitlines():
        digest, name = row.split("  ", 1)
        relative = Path(name)
        if relative.is_absolute() or ".." in relative.parts or name in result:
            raise Refused("source_path")
        candidate = root / relative
        if candidate.resolve(strict=True) != candidate.absolute():
            raise Refused("source_symlink")
        fd = os.open(candidate, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            if not stat.S_ISREG(os.fstat(fd).st_mode):
                raise Refused("source_type")
            with os.fdopen(fd, "rb", closefd=False) as stream:
                data = stream.read()
        finally:
            os.close(fd)
        if sha(data) != digest:
            raise Refused("source_mismatch_before_import")
        result[name] = data
    if tuple(result) != INPUTS:
        raise Refused("record_file_set")
    admission = json.loads(result[ADMISSION])
    if (
        admission["fakeScope"] != "loopback-transport-only"
        or admission["kernelProofAccepted"] is not False
        or admission["sdkCustodyDecision"] != "89db0f25-dc94-4feb-b447-8b9fabe52561"
        or admission["custodyModel"] != "explicit-trusted-host"
        or admission["hostileSameUserReplacementCovered"] is not False
        or admission["productionAdmission"] is not False
        or admission["originalActivation"] is not False
        or admission["canonicalFixtureAttempt"] is not False
        or admission["requiresSeparateSetupAuthority"] is not True
        or admission["processes"] != {"monitor": 1, "guest": 1, "nodeChildren": 2}
        or admission["bounds"]
        != {
            "wallSeconds": 30,
            "nodeCpuSeconds": 10,
            "nodeDescriptors": 64,
            "outputBytes": 1000000,
            "writableBytes": 1000000,
        }
    ):
        raise Refused("fake_admission_scope_mismatch")
    return result


def load_verified(name, data, root=ROOT):
    module = types.ModuleType(name)
    module.__file__ = str(root / name)
    exec(compile(data, module.__file__, "exec"), module.__dict__)
    return module


def host_custody(files, node, node_sha, record_sha, runtime=None):
    custody = load_verified(CUSTODY, files[CUSTODY])
    runtime = (
        runtime if runtime is not None else custody.runtime_inventory(node, node_sha)
    )
    lease = custody.Lease(runtime, node, node_sha, json.loads(files[ADMISSION]))
    try:
        for name, data in files.items():
            lease.bind(ROOT / name, sha(data))
        lease.bind(ROOT / RECORD, record_sha)
        return lease
    except BaseException:
        lease.close()
        raise


def reject_inherited_sockets(descriptors):
    for fd in descriptors:
        if stat.S_ISSOCK(os.fstat(fd).st_mode):
            raise Refused("inherited_egress_socket")


def check_network(observed, host):
    if observed["namespace"] == host or observed["interfaces"] != ["lo"]:
        raise Refused("wrong_namespace_or_interfaces")
    if observed["externalRoutes"]:
        raise Refused("external_route")


class Model:
    """Only namespace/tool effects are simulated; processes/TCP/waits are real."""

    kernel = False

    def __init__(
        self,
        fail=None,
        cancel=None,
        expire=None,
        missing_join=False,
        cancel_after_children=False,
    ):
        self.fail = fail
        self.cancel = cancel
        self.expire = expire
        self.missing_join = missing_join
        self.cancel_after_children = cancel_after_children
        self.started = 0
        self.now = 0
        self.events = []
        self.created = False
        self.cleaned = False
        self.host = {"device": 0, "inode": 1}
        self.network = {
            "namespace": {"device": 0, "inode": 2},
            "interfaces": ["lo"],
            "externalRoutes": [],
        }

    def stage(self, phase):
        self.events.append(phase)
        if self.fail == phase:
            raise Refused("simulated_" + phase + "_refusal")
        if phase == "namespace":
            self.created = True
        if phase == "identity":
            check_network(self.network, self.host)
        if phase == self.expire:
            self.now += 31

    def clock(self):
        return self.now

    def cancelled(self, phase):
        return phase == self.cancel

    def protocol_cancelled(self):
        return self.cancel_after_children and self.started == 2

    def endpoint_ready(self, port):
        return None  # No simulated outside-denial claim in an ordinary host netns.

    def confirm_child(self, child):
        if child.poll() is not None:
            raise Refused("child_exited_before_admission")
        self.started += 1

    def cleanup(self):
        self.events.append("cleanup_observation")
        if self.fail == "cleanup_observation":
            raise Refused("simulated_cleanup_failure")
        self.cleaned = True
        return {"created": self.created, "observed": True, "kernelEnforced": False}


def execute(ops, protocol):
    result = None
    failure = None
    cleanup = None
    deadline = ops.clock() + 30

    def phase(name, limit):
        ops.stage(name)
        if ops.cancelled(name):
            raise Refused("cancelled_at_" + name)
        if ops.clock() >= limit:
            raise Refused("deadline_at_" + name)

    try:
        for name in PHASES[:-1]:
            phase(name, deadline)
        result = protocol(ops)
        if result["failure"] is not None:
            raise Refused("protocol_" + result["failure"])
        if len(result["exits"]) != 2 or getattr(ops, "missing_join", False):
            raise Refused("missing_join_observation")
    except (Refused, OSError) as error:
        failure = str(error)
    finally:
        # Cancellation/deadline cannot skip teardown. Its failure is retained too.
        try:
            phase("cleanup", ops.clock() + 2)
        except (Refused, OSError) as error:
            failure = failure or str(error)
        try:
            cleanup = ops.cleanup()
            if not cleanup or cleanup.get("observed") is not True:
                raise Refused("cleanup_join_unknown")
        except (Refused, OSError) as error:
            failure = (failure + "; " if failure else "") + str(error)
    return {
        "failure": failure,
        "protocol": result,
        "cleanup": cleanup,
        "kernelEnforced": ops.kernel,
        "originalActivation": False,
        "productionAdmission": False,
    }


def protocol(
    node, node_sha, bound, files, ops, temporary, mode="probe", scenario="permission"
):
    old = load_verified(
        "scripts/claude-agent-sdk-confined-probe.py",
        files["scripts/claude-agent-sdk-confined-probe.py"],
    )
    try:
        accepted = old.verified_sources(ROOT, OLD_RECORD)
    except old.Refused as error:
        raise Refused("accepted_peer_or_admission_mismatch") from error
    admission = json.loads(accepted[old.FIXTURE / "probe-admission.json"])
    if (
        admission["production_admission"]
        or admission["authority"]["original_run_authorised"]
    ):
        raise Refused("production_or_original_admission")
    lease = getattr(ops, "lease", None)
    if lease is None:
        raise Refused("host_custody_missing")
    lease.check()
    for name, data in accepted.items():
        lease.bind(ROOT / name, sha(data))
    node = Path(node).resolve(strict=True)
    if sha(old.read_regular(node)) != node_sha:
        raise Refused("node_identity")
    root = Path(temporary) / "payload"
    root.mkdir(mode=0o700)
    for name in ("home", "tmp", "workspace"):
        (root / name).mkdir(mode=0o700)
    (root / "workspace/storage.bin").touch()
    mapping = {
        "confined-runner.mjs": "confined-runner.mjs",
        "confined-spawn.mjs": "confined-spawn.mjs",
        "fake-sdk-native-wire.mjs": "probe-sdk.mjs",
        "fake-native.mjs": "fake-native.mjs",
        "confined-sidecar-driver.mjs": "confined-sidecar-driver.mjs",
    }
    for source, target in mapping.items():
        (root / target).write_bytes(accepted[old.SUPPORT / source])
    (root / "probe-options.json").write_bytes(
        accepted[old.FIXTURE / "probe-options.json"]
    )
    (root / "claude-agent-sdk-sidecar.mjs").write_bytes(
        accepted[
            Path(
                "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
            )
        ]
    )
    (root / "netns-native.mjs").write_bytes(
        files["scripts/claude-agent-sdk-netns-native.mjs"]
    )
    sdk_version, native_version = (
        ("0.3.317", "2.1.317") if mode == "sidecar" else ("0.3.322", "2.1.322")
    )
    (root / "package.json").write_text(
        json.dumps({"name": "@anthropic-ai/claude-agent-sdk", "version": sdk_version})
    )
    (root / "manifest.json").write_text(json.dumps({"version": native_version}))
    (root / "sdk-config.json").write_text("{}")
    frames = [
        {"type": "message_start", "message": {"usage": {"input_tokens": 21}}},
        {
            "type": "content_block_start",
            "index": 0,
            "content_block": {"type": "text", "text": ""},
        },
        {
            "type": "content_block_delta",
            "index": 0,
            "delta": {"type": "text_delta", "text": "owned"},
        },
        {"type": "content_block_stop", "index": 0},
        {
            "type": "message_delta",
            "delta": {"stop_reason": "end_turn"},
            "usage": {"output_tokens": 5},
        },
        {"type": "message_stop"},
    ]
    stream = "".join(
        f"event: {row['type']}\ndata: {json.dumps(row)}\n\n" for row in frames
    ).encode()
    requests = []

    class Endpoint(BaseHTTPRequestHandler):
        def do_POST(self):
            self.connection.settimeout(1)
            length = int(self.headers.get("Content-Length", "0"))
            if self.path != "/v1/messages" or not 0 < length <= 4096:
                self.send_error(400)
                return
            data = json.loads(self.rfile.read(length))
            if any(
                name.lower() in {"authorization", "x-api-key"} for name in self.headers
            ):
                self.send_error(403)
                return
            if requests:
                self.send_error(429)
                return
            requests.append(data)
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream")
            self.send_header("Content-Length", str(len(stream)))
            self.end_headers()
            self.wfile.write(stream)

        def log_message(self, *args):
            pass

    class BoundedServer(HTTPServer):
        def get_request(self):
            connection, address = super().get_request()
            connection.settimeout(1)  # Bound headers as well as the POST body.
            return connection, address

        def handle_error(self, request, client_address):
            pass  # The failed bounded response fails the owned native/protocol.

    server = BoundedServer(("127.0.0.1", 0), Endpoint)
    server.timeout = 1
    thread = threading.Thread(
        target=server.serve_forever, kwargs={"poll_interval": 0.01}
    )
    owned = []
    host_events = []
    started_at = time.monotonic()
    entered_supervision = False
    fds = []
    native_gate = lease.take_pipe(3)
    try:
        canary_port = ops.endpoint_ready(server.server_port)
        (root / "native-config.json").write_text(
            json.dumps(
                {
                    "port": server.server_port,
                    "forbiddenHostPort": canary_port,
                    "scenario": scenario,
                    "startFd": native_gate[0],
                }
            )
        )
        template = json.loads((root / "probe-options.json").read_bytes())["options"]
        native_args = [
            str(root / "fake-native.mjs"),
            "--output-format",
            "stream-json",
            "--verbose",
            "--input-format",
            "stream-json",
            "--model",
            template["model"],
            "--permission-prompt-tool=stdio",
            "--disallowedTools",
            ",".join(template["disallowedTools"]),
            "--tools",
            ",".join(template["tools"]),
            "--setting-sources",
            "",
            "--strict-mcp-config",
            "--permission-mode",
            "default",
        ]
        fds = [lease.take_pipe(index) for index in range(3)]
        native_fds = (fds[0][0], fds[1][1], fds[2][1])
        driver_fds = (fds[0][1], fds[1][0], fds[2][0])
        (root / "confined-config.json").write_text(
            json.dumps(
                {
                    "workspace": str(root / "workspace"),
                    "nativeArgs": native_args,
                    "inputFd": driver_fds[0],
                    "outputFd": driver_fds[1],
                    "errorFd": driver_fds[2],
                    "scenario": scenario,
                }
            )
        )
        env = {
            "PATH": "/usr/bin:/bin",
            "HOME": str(root / "home"),
            "TMPDIR": str(root / "tmp"),
            "TMP": str(root / "tmp"),
            "TEMP": str(root / "tmp"),
            "LANG": "C",
            "LC_ALL": "C",
        }
        prefix = [str(node), "--no-addons", "--openssl-config=/dev/null"]
        commands = [
            prefix + ["--import", str(root / "netns-native.mjs"), *native_args],
            prefix
            + [
                "--import",
                str(root / "confined-spawn.mjs"),
                str(
                    root
                    / (
                        "confined-sidecar-driver.mjs"
                        if mode == "sidecar"
                        else "confined-runner.mjs"
                    )
                ),
            ],
        ]
        driver_env = dict(env)
        if mode == "sidecar":
            driver_env.update(
                {
                    "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE": str(root / "probe-sdk.mjs"),
                    "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY": str(
                        root / "fake-native.mjs"
                    ),
                    "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST": str(root / "manifest.json"),
                }
            )

        lease.configure(commands, [env, driver_env])
        for staged in root.iterdir():
            if staged.is_file() and staged.name != "confined-config.json":
                lease.bind(staged, sha(staged.read_bytes()))

        def spawn(command, **kwargs):
            hook = getattr(ops, "custody_event", None)
            if hook is not None:
                hook(lease.launched, lease, command)
            lease.permit(command, kwargs["env"])
            if lease.launched == 2:
                lease.bind(
                    root / "confined-config.json",
                    sha((root / "confined-config.json").read_bytes()),
                )
            if lease.launched == 1:
                kwargs["pass_fds"] = (*kwargs.get("pass_fds", ()), native_gate[0])
            reject_inherited_sockets(kwargs.get("pass_fds", ()))
            if ops.kernel:
                command, kwargs = ops.entry(command, kwargs, root, node, files)
            if ops.kernel:
                for entry in root.glob("entry-*.json"):
                    lease.bind(entry, sha(entry.read_bytes()))
                lease.bind(
                    root / "netns-entry.py",
                    sha(files["scripts/claude-agent-sdk-netns-entry.py"]),
                )
            lease.check()
            child = subprocess.Popen(command, **kwargs)
            owned.append(child)
            original_poll = child.poll
            noted = False

            def observed_poll():
                nonlocal noted
                code = original_poll()
                if code is not None and not noted:
                    noted = True
                    host_events.append(
                        {
                            "pid": child.pid,
                            "exitCode": code,
                            "at": time.monotonic() - started_at,
                        }
                    )
                return code

            child.poll = observed_poll
            ops.confirm_child(child)
            if lease.launched == 2:
                os.write(native_gate[1], b"1")
            return child

        original_write = os.write

        def observed_write(fd, data):
            if data.startswith(b'{"code":'):
                host_events.append(
                    {"exitNotificationFd": fd, "at": time.monotonic() - started_at}
                )
            return original_write(fd, data)

        old.os = types.SimpleNamespace(
            **{**vars(os), "write": observed_write, "pipe": lambda: lease.take_pipe(4)}
        )
        old.subprocess = types.SimpleNamespace(
            Popen=spawn,
            PIPE=subprocess.PIPE,
            DEVNULL=subprocess.DEVNULL,
            TimeoutExpired=subprocess.TimeoutExpired,
        )
        thread.start()
        try:
            entered_supervision = True
            result = old.supervise(
                commands,
                root / "workspace",
                env,
                native_fds,
                driver_fds,
                bound,
                1_000_000,
                driver_environment=driver_env,
                cancelled=ops.protocol_cancelled,
            )
            result["hostExitEvents"] = host_events
            result["stdioDescriptors"] = list(native_fds + driver_fds + native_gate)
            result["custody"] = lease.observation()
            result["requests"] = requests
            result["ordinaryTcp"] = (
                len(requests) == 1 and '"ordinaryTcp":true' in result["stderr"]
            )
            if result["failure"] is None and not result["ordinaryTcp"]:
                raise Refused("ordinary_tcp_protocol_unobserved")
            return result
        finally:
            for child in owned:
                if child.poll() is None:
                    child.kill()
                child.wait(timeout=2)
                for stream in (child.stdout, child.stderr):
                    if stream is not None:
                        stream.close()
    finally:
        for fd in native_gate:
            os.close(fd)
        if not entered_supervision:
            for pair in fds:
                for fd in pair:
                    os.close(fd)
        if thread.ident is not None:
            server.shutdown()
        server.server_close()
        if thread.ident is not None:
            thread.join(timeout=2)
            if thread.is_alive():
                raise Refused("endpoint_join_unobserved")


def rehearse(
    expected, node, node_sha, ops=None, mode="probe", scenario="permission", bound=10
):
    if mode not in ("probe", "sidecar"):
        raise Refused("original_or_unknown_mode")
    files = snapshots(ROOT, expected)
    if not 0 < bound <= 10:
        raise Refused("bound_may_only_tighten")
    selected_ops = ops or Model()
    lease = host_custody(files, node, node_sha, expected)
    selected_ops.lease = lease
    try:
        with tempfile.TemporaryDirectory(
            prefix="sdk-netns-model-", dir=os.environ["TMPDIR"]
        ) as temporary:
            return execute(
                selected_ops,
                lambda selected: protocol(
                    node, node_sha, bound, files, selected, temporary, mode, scenario
                ),
            )
    finally:
        lease.close()


def inspect_host(node=None):
    import platform
    import shutil

    observed = {
        "host": platform.uname()._asdict(),
        "python": sys.executable,
        "pythonSha256": sha(Path(sys.executable).read_bytes()),
        "kernelNamespaceObserved": False,
        "originalActivation": False,
        "tools": {},
    }
    for name in ("sudo", "timeout", "python3", "node"):
        found = shutil.which(name)
        if found:
            target = Path(found).resolve(strict=True)
            observed["tools"][name] = {
                "path": str(target),
                "sha256": sha(target.read_bytes()),
            }
        else:
            observed["tools"][name] = None
    if node:
        # The hosted preflight runs as the ordinary runner, before any privilege.
        record = snapshots(ROOT, os.environ["NETNS_RECORD"])
        custody = load_verified(CUSTODY, record[CUSTODY])
        observed_node_sha = sha(Path(node).resolve(strict=True).read_bytes())
        if (
            sys.platform == "linux"
            and observed_node_sha
            != "3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327"
        ):
            raise Refused("preflight_Node22_identity_before_execution")
        observed["custodyRuntime"] = custody.runtime_inventory(node, observed_node_sha)
        observed["nodeVersion"] = (
            subprocess.check_output(
                [node, "--version"], env={"PATH": "/usr/bin:/bin"}, timeout=5
            )
            .decode()
            .strip()
        )
    if sys.platform == "linux":
        import ctypes

        libc = ctypes.CDLL(None, use_errno=True)
        libc.syscall.restype = ctypes.c_long
        if platform.machine() == "x86_64":
            abi = libc.syscall(444, 0, 0, 1)  # VERSION query only; no ruleset.
            observed["landlockAbiReadOnly"] = abi
            observed["landlockQueryErrno"] = ctypes.get_errno() if abi < 0 else 0
        observed["namespaceReadable"] = Path("/proc/self/ns/net").exists()
        observed["pidfdApi"] = hasattr(os, "pidfd_open")
        observed["capabilities"] = [
            row
            for row in Path("/proc/self/status").read_text().splitlines()
            if row.startswith(("CapEff:", "CapPrm:", "CapAmb:"))
        ]
        if hasattr(os, "pidfd_open"):
            descriptor = os.pidfd_open(os.getpid())
            os.close(descriptor)
            observed["pidfdSelfReadOnly"] = True
    return observed


def kernel_with_custody(args, files, kernel):
    kernel.root_required(args)
    args.custody_started = time.monotonic()
    preflight = Path(os.environ["TMPDIR"]) / "preflight.json"
    info = preflight.lstat()
    if preflight.is_symlink() or info.st_uid != args.uid or info.st_mode & 0o022:
        raise Refused("host_preflight_ownership")
    runtime = json.loads(preflight.read_bytes())["custodyRuntime"]
    if runtime["version"] != "v22.23.2" or runtime["ownerUid"] != args.uid:
        raise Refused("host_preflight_runtime_version")
    lease = host_custody(
        files, args.node, args.node_sha256, args.record_sha256, runtime
    )
    try:
        lease.bind(preflight, sha(preflight.read_bytes()))
        lease.bind(
            Path(f"/proc/{os.getppid()}/exe").resolve(strict=True), args.timeout_sha256
        )
        return kernel.launch(args, files, sys.modules[__name__], lease)
    finally:
        lease.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rehearse", action="store_true")
    parser.add_argument("--kernel-proposal", action="store_true")
    parser.add_argument("--cleanup-proposal", action="store_true")
    parser.add_argument("--inspect", action="store_true")
    parser.add_argument("--record-sha256")
    parser.add_argument("--node")
    parser.add_argument("--node-sha256")
    parser.add_argument("--uid", type=int)
    parser.add_argument("--gid", type=int)
    parser.add_argument("--python-sha256")
    parser.add_argument("--timeout-sha256")
    args = parser.parse_args()
    if args.inspect:
        print(json.dumps(inspect_host(args.node)))
    elif args.kernel_proposal or args.cleanup_proposal:
        files = snapshots(ROOT, args.record_sha256)
        kernel = load_verified(
            "scripts/claude-agent-sdk-netns-kernel.py",
            files["scripts/claude-agent-sdk-netns-kernel.py"],
        )
        result = (
            kernel.cleanup_receipt(args)
            if args.cleanup_proposal
            else kernel_with_custody(args, files, kernel)
        )
        print(json.dumps(result))
    elif args.rehearse:
        result = rehearse(args.record_sha256, args.node, args.node_sha256)
        print(json.dumps(result))
        if result["failure"]:
            raise SystemExit(78)
    else:
        raise Refused("explicit_fake_mode_required; original mode unavailable")


if __name__ == "__main__":
    try:
        main()
    except (Refused, OSError, ValueError) as error:
        print(json.dumps({"refused": str(error), "originalActivation": False}))
        raise SystemExit(78)
