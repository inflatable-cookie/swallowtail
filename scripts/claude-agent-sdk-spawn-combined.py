"""Inactive exact-source combined Linux owned-fake host. No original mode.

The only root entrypoints belong to a future separately approved manual job.
Ordinary tests exercise the same state machine with owned host models, never
namespaces, tracing, cgroups or canonical administrator setup.
"""

import argparse
import ctypes
import errno
import hashlib
import json
import os
from pathlib import Path
import platform
import select
import signal
import stat
import sys
import time
import types
import threading
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
RECORD = Path("docs/knowledge/operations/claude-agent-sdk-spawn.sha256")
NODE22 = "3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327"
FRAME = 16384
RESULT = 131072
RECEIPT = 262144


class Refused(OSError):
    pass


def verify_settlement(journal):
    """Cross-check distinct observations; a summary boolean cannot attest it."""
    if not journal.get("treeSettled") or journal.get("failure"):
        return False
    processes = journal.get("processes", [])
    pids = {row["pid"] for row in processes}
    if not pids or set(journal.get("pidfdReaped", [])) != pids:
        return False
    waits = journal.get("waits", [])
    if not any(
        row["pid"] == journal["init"]["pid"] and row["status"] == 0 for row in waits
    ):
        return False
    software = journal.get("softwareResult") or {}
    if software.get("failure") is not None or software.get("driverWait") != 0:
        return False
    actors = software.get("actorGraph", {})
    native_actor = actors.get("native", {})
    driver_actor = actors.get("driver", {})
    monitor_actor = actors.get("monitor", {})
    if not all(
        "pid" in actor and "hostPid" in actor
        for actor in (native_actor, driver_actor, monitor_actor)
    ):
        return False
    native = native_actor["hostPid"]
    terminals = {
        row["pid"]
        for row in journal.get("traceWaits", [])
        if os.WIFEXITED(row["status"]) or os.WIFSIGNALED(row["status"])
    }
    if (
        native not in pids
        or native not in terminals
        or driver_actor["hostPid"] not in terminals
    ):
        return False
    native_waits = software.get("nativeWaits", [])
    if len(native_waits) != 1 or any(
        row.get("code") != 0
        or row.get("pid") != native_actor["pid"]
        or row.get("actor") != driver_actor["pid"]
        for row in native_waits
    ):
        return False
    owner = actors.get("group-owner")
    child = owner or driver_actor
    if not any(
        row.get("kind") == "driver_wait"
        and row.get("pid") == child["pid"]
        and row.get("actor") == monitor_actor["pid"]
        and row.get("code") == 0
        for row in software.get("events", [])
    ):
        return False
    if owner and not any(
        row.get("pid") == driver_actor["pid"]
        and row.get("actor") == owner["pid"]
        and row.get("code") == 0
        for row in software.get("actorWaits", [])
    ):
        return False
    resources = journal.get("resourceEvents", {})
    return bool(
        resources.get("empty") is True
        and resources.get("tasks") == []
        and not resources.get("observationFailure")
        and journal.get("outsideCanaryUnreached") is True
    )


def sha(data):
    return hashlib.sha256(data).hexdigest()


def prepare_fixture(parent, owner, state, run, helper, store):
    """Existing helper's exact bytes, with a durable identity before writes."""
    data = helper.payload(state)
    parent_fd = helper.directory(parent, owner)
    try:
        os.mkdir("claude-code", 0o755, dir_fd=parent_fd)
        fd = os.open(
            "claude-code",
            os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
            dir_fd=parent_fd,
        )
        try:
            os.fchmod(fd, 0o755)
            info = os.fstat(fd)
            if info.st_uid != owner:
                raise Refused(errno.EPERM, "fixture_owner")
            receipt = {
                "run": run,
                "state": state,
                "owner": owner,
                "device": info.st_dev,
                "inode": info.st_ino,
                "policySha256": sha(data) if data is not None else None,
            }
            store(receipt)
            if data is not None:
                helper.write_file(fd, helper.POLICY, data, 0o444)
            helper.write_file(
                fd, helper.LEASE, (json.dumps(receipt) + "\n").encode(), 0o600
            )
            return receipt
        finally:
            os.close(fd)
    finally:
        os.close(parent_fd)


def cleanup_partial_fixture(parent, owner, receipt, helper):
    """Only this root-created inode and exact known helper files may be removed."""
    parent_fd = helper.directory(parent, owner)
    try:
        fd = os.open(
            "claude-code",
            os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
            dir_fd=parent_fd,
        )
        try:
            info = os.fstat(fd)
            if (
                info.st_uid != owner
                or stat.S_IMODE(info.st_mode) != 0o755
                or (info.st_dev, info.st_ino) != (receipt["device"], receipt["inode"])
            ):
                raise Refused(errno.EPERM, "partial_fixture_identity")
            expected = {helper.LEASE: ((json.dumps(receipt) + "\n").encode(), 0o600)}
            data = helper.payload(receipt["state"])
            if data is not None:
                expected[helper.POLICY] = (data, 0o444)
            names = set(os.listdir(fd))
            if not names <= set(expected):
                raise Refused(errno.EPERM, "partial_fixture_unknown_file")
            for name in names:
                data, mode = expected[name]
                if helper.read_file(fd, name, owner, mode) != data:
                    raise Refused(errno.EPERM, "partial_fixture_changed_bytes")
            for name in names:
                os.unlink(name, dir_fd=fd)
            after = os.stat("claude-code", dir_fd=parent_fd, follow_symlinks=False)
            if (after.st_dev, after.st_ino) != (info.st_dev, info.st_ino):
                raise Refused(errno.EPERM, "partial_fixture_replaced")
            os.rmdir("claude-code", dir_fd=parent_fd)
        finally:
            os.close(fd)
    finally:
        os.close(parent_fd)


def captured_core(expected):
    source = ROOT / "scripts/claude-agent-sdk-spawn-proposal.py"
    record = (ROOT / RECORD).read_bytes()
    if sha(record) != expected:
        raise Refused(errno.EPERM, "combined_source_record")
    rows = dict(row.split("  ", 1)[::-1] for row in record.decode().splitlines())
    data = source.read_bytes()
    if rows.get(str(source.relative_to(ROOT))) != sha(data):
        raise Refused(errno.EPERM, "combined_core_source")
    namespace = {"__name__": "private_combined_core", "__file__": str(source)}
    exec(compile(data, str(source), "exec"), namespace)
    core = types.SimpleNamespace(**namespace)
    files = core.captured(expected)
    return core, files


def private_modules(core, files):
    return {
        name: core.module(
            "scripts/claude-agent-sdk-spawn-" + name + ".py",
            files["scripts/claude-agent-sdk-spawn-" + name + ".py"],
        )
        for name in ("trace", "cgroup", "policy")
    }


def proposed_workflow(data):
    name = ".github/workflows/sdk234-combined-fake.yml"
    lines = data.decode().splitlines()
    prefix = [
        "diff --git a/" + name + " b/" + name,
        "new file mode 100644",
        "--- /dev/null",
        "+++ b/" + name,
    ]
    if (
        lines[:4] != prefix
        or len(lines) < 6
        or not lines[4].startswith("@@ -0,0 +1,")
        or any(not row.startswith("+") for row in lines[5:])
    ):
        raise Refused(errno.EPERM, "one_unapplied_workflow_only")
    body = "\n".join(row[1:] for row in lines[5:]) + "\n"
    if lines[4] != "@@ -0,0 +1," + str(len(lines) - 5) + " @@":
        raise Refused(errno.EPERM, "workflow_patch_length")
    required = (
        "  workflow_dispatch:\n",
        "  contents: read\n",
        "    timeout-minutes: 5\n",
        "        state: [absent, disabled, managed-deny]\n",
        "          persist-credentials: false\n",
        "          combined_node=/opt/hostedtoolcache/node/22.23.2/x64/bin/node\n",
        "        if: always()\n",
        "--kill-after=2s 35s",
        "--kill-after=2s 5s",
        "--profile acp-actor-shape --cgroup-parent /sys/fs/cgroup",
    )
    if any(item not in body for item in required) or any(
        item in body
        for item in (
            "setup-node",
            "contents: write",
            "pull_request:",
            "  push:",
            "--rerun",
        )
    ):
        raise Refused(errno.EPERM, "combined_workflow_scope")
    return body


def check_workflow(files):
    import tempfile

    body = proposed_workflow(
        files["docs/knowledge/operations/claude-agent-sdk-combined-workflow.patch"]
    )
    with tempfile.TemporaryDirectory(
        prefix="sdk234-workflow-", dir=os.environ["TMPDIR"]
    ) as temporary:
        file = Path(temporary) / "sdk234-combined-fake.yml"
        file.write_text(body)
        subprocess.run(["actionlint", str(file)], check=True, timeout=10)
    return {
        "workflowApplied": False,
        "workflowSha256": sha(body.encode()),
        "originalActivation": False,
    }


def identity(pid):
    base = Path(f"/proc/{pid}")
    fields = (base / "stat").read_text().rsplit(") ", 1)[1].split()
    status = {
        line.split(":", 1)[0]: line.split(":", 1)[1].strip()
        for line in (base / "status").read_text().splitlines()
        if ":" in line
    }
    net = (base / "ns/net").stat()
    pidns = (base / "ns/pid").stat()
    return {
        "pid": pid,
        "parent": int(fields[1]),
        "group": int(fields[2]),
        "start": fields[19],
        "tgid": int(status["Tgid"]),
        "namespacePids": [int(x) for x in status["NSpid"].split()],
        "namespace": {"device": net.st_dev, "inode": net.st_ino},
        "pidNamespace": {"device": pidns.st_dev, "inode": pidns.st_ino},
        "uids": [int(x) for x in status["Uid"].split()],
        "gids": [int(x) for x in status["Gid"].split()],
        "groups": status["Groups"].split(),
        "caps": {
            key: status[key]
            for key in ("CapEff", "CapPrm", "CapAmb", "CapInh", "CapBnd")
        },
        "tracer": int(status["TracerPid"]),
        "membership": (base / "cgroup").read_text(),
        "command": [
            x.decode()
            for x in (base / "cmdline").read_bytes().rstrip(b"\0").split(b"\0")
        ],
    }


def socket_peer(group, init_pid, client_port, server_port):
    """Authenticate the live TCP caller from kernel socket ownership, not JSON PID."""
    wanted = ("0100007F:%04X" % client_port, "0100007F:%04X" % server_port)
    rows = Path(f"/proc/{init_pid}/net/tcp").read_text().splitlines()[1:]
    inodes = {
        row.split()[9]
        for row in rows
        if tuple(row.split()[1:3]) == wanted and row.split()[3] == "01"
    }
    if len(inodes) != 1:
        raise Refused(errno.EPERM, "control_socket_identity")
    wanted_link = "socket:[" + next(iter(inodes)) + "]"
    owners = set()
    for pid in group.members():
        for file in Path(f"/proc/{pid}/fd").iterdir():
            try:
                if os.readlink(file) == wanted_link:
                    owners.add(pid)
            except FileNotFoundError:
                pass
    if len(owners) != 1:
        raise Refused(errno.EPERM, "control_socket_owner_not_unique")
    return next(iter(owners))


def write_frame(fd, value):
    data = (json.dumps(value, separators=(",", ":")) + "\n").encode()
    if len(data) > FRAME:
        raise Refused(errno.EFBIG, "combined_control_frame")
    while data:
        data = data[os.write(fd, data) :]


def read_frame(fd, deadline, pending):
    while b"\n" not in pending:
        if time.monotonic() >= deadline:
            raise Refused(errno.ETIMEDOUT, "combined_peer_deadline")
        if not select.select(
            [fd], [], [], min(0.02, max(0, deadline - time.monotonic()))
        )[0]:
            continue
        data = os.read(fd, FRAME)
        if not data:
            raise Refused(errno.EPIPE, "combined_peer_closed")
        pending.extend(data)
        if len(pending) > FRAME:
            raise Refused(errno.EFBIG, "combined_pending_frame")
    line, rest = pending.split(b"\n", 1)
    pending[:] = rest
    return json.loads(line)


class GuestContext:
    """Trusted fork's private pipe; payload children never inherit this authority."""

    def __init__(self, runtime, args, read, write, metadata, files):
        self.runtime, self.args = runtime, args
        self.read, self.write = read, write
        self.metadata, self.files = metadata, files
        self.deadline = time.monotonic() + 23
        self.pending = bytearray()
        self.layout = None
        self.lock = threading.Lock()

    def request(self, kind, **values):
        with self.lock:
            write_frame(self.write, {"kind": kind, **values})
            result = read_frame(self.read, self.deadline, self.pending)
        if "error" in result:
            raise OSError(result.get("errno", errno.EPERM), result["error"])
        return result

    def observe(self, namespace_pid):
        result = self.request("identity", namespacePid=namespace_pid)
        return result["identity"]

    def driver_gate(self, pid, client, server):
        return self.request(
            "driver_gate", namespacePid=pid, clientPort=client, serverPort=server
        )["policy"]

    def native_gate(self, pid, client, server):
        return self.request(
            "native_gate", namespacePid=pid, clientPort=client, serverPort=server
        )["policy"]

    def prepare(
        self,
        root,
        config,
        driver_command,
        native_command,
        driver_entry,
        native_entry,
        owner_command,
        driver_env,
        native_env,
        lease,
    ):
        # All derived source/config bytes are bound before the first Node start.
        (root / "kernel-policy.py").write_bytes(
            self.files["scripts/claude-agent-sdk-spawn-policy.py"]
        )
        (root / "kernel-entry-source.py").write_bytes(
            self.files["scripts/claude-agent-sdk-netns-entry.py"]
        )
        canonical = "/etc/claude-code/managed-settings.json"
        config["fixturePath"] = canonical
        config["kernelPolicy"] = {
            "file": str(root / "kernel-policy.py"),
            "sha256": sha(self.files["scripts/claude-agent-sdk-spawn-policy.py"]),
            "inputs": {
                "root": str(root),
                "canonicalDirectory": "/etc/claude-code",
                "writable": str(root / "storage.bin"),
                "readFiles": list(lease.files) + ["/dev/random", "/dev/urandom"],
                "runtimeRoots": [],
                "execFiles": [
                    driver_command[0],
                    native_entry[0],
                    str(Path("/lib64/ld-linux-x86-64.so.2").resolve(strict=True)),
                ],
                "nativeExecFiles": [
                    native_command[0],
                    str(Path("/lib64/ld-linux-x86-64.so.2").resolve(strict=True)),
                ],
                "entrySource": str(root / "kernel-entry-source.py"),
                "entrySha256": sha(
                    self.files["scripts/claude-agent-sdk-netns-entry.py"]
                ),
            },
        }
        config["outsidePath"] = self.metadata["outsidePath"]
        config["forbiddenHostPort"] = self.metadata["canaryPort"]
        self.layout = {
            "root": str(root),
            "config": str(root / "spawn-config.json"),
            "driver": driver_command,
            "native": native_command,
            "driverEntry": driver_entry,
            "nativeEntry": native_entry,
            "owner": owner_command,
            "driverEnv": driver_env,
            "nativeEnv": native_env,
            "readFiles": list(lease.files),
            "canonicalDirectory": "/etc/claude-code",
        }

    def ready(self, lease):
        self.layout["readFiles"] = list(lease.files)
        import base64

        data = json.dumps(self.layout, separators=(",", ":")).encode()
        if len(data) > 65536:
            raise Refused(errno.EFBIG, "combined_layout_bound")
        self.request("layout_begin", length=len(data), sha256=sha(data))
        for index, offset in enumerate(range(0, len(data), 6000)):
            self.request(
                "layout_chunk",
                index=index,
                data=base64.b64encode(data[offset : offset + 6000]).decode(),
            )
        self.request("layout_end")

    def stop(self, observed):
        self.request(
            "stop_actor", namespacePid=observed["pid"], start=observed["start"]
        )

    def finish(self, result):
        import base64

        data = json.dumps(result, separators=(",", ":")).encode()
        if len(data) > RESULT:
            raise Refused(errno.EFBIG, "combined_result_bound")
        self.request("result_begin", length=len(data), sha256=sha(data))
        for index, offset in enumerate(range(0, len(data), 6000)):
            self.request(
                "result_chunk",
                index=index,
                data=base64.b64encode(data[offset : offset + 6000]).decode(),
            )
        self.request("done")


class Monitor:
    """One exact fake invocation; privileged entry is separately owner-gated."""

    def __init__(self, args, core, files, modules, runtime, lease, group, journal):
        self.args, self.core, self.files, self.modules = args, core, files, modules
        self.runtime, self.lease, self.group, self.journal = (
            runtime,
            lease,
            group,
            journal,
        )
        self.trace = modules["trace"].LinuxTrace()
        self.gate = None
        self.host = identity(os.getpid())
        self.init = None
        self.packet = None
        self.handles = {}
        self.seen = {}
        self.result = None
        self.result_data = bytearray()
        self.result_header = None
        self.layout_data = bytearray()
        self.layout_header = None
        self.cancelled = False
        self.network = []
        self.waits = []
        self.setup_fds = []
        self.previous_signals = {}
        self.parent_fd = os.pidfd_open(os.getpid())
        self.journal_fd = args.journal_fd
        import socket

        self.canary = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.canary.bind(("127.0.0.1", 0))
        self.canary.listen(1)

    def member(self, info):
        self.group.assert_limits()
        if (
            info["tgid"] not in self.group.members()
            or info["membership"] != self.journal["membership"]
        ):
            raise Refused(errno.EPERM, "cgroup_member_changed")
        if self.init is not None and info["namespace"] != self.journal["namespace"]:
            raise Refused(errno.EPERM, "network_namespace_changed")
        if (
            self.init is not None
            and info["pidNamespace"] != self.journal["pidNamespace"]
        ):
            raise Refused(errno.EPERM, "pid_namespace_changed")
        if (
            info["pidNamespace"] == self.host["pidNamespace"]
            or info["uids"] != [self.args.uid] * 4
            or info["gids"] != [self.args.gid] * 4
            or info["groups"]
            or any(int(x, 16) for x in info["caps"].values())
        ):
            raise Refused(errno.EPERM, "payload_namespace_or_credentials")

    def pin(self, pid):
        info = identity(pid)
        if pid not in self.handles:
            self.handles[pid] = os.pidfd_open(pid)
            self.seen[pid] = info
        return info

    def find(self, namespace_pid):
        candidates = []
        for pid in self.group.members():
            info = identity(pid)
            if info["namespacePids"][-1] == namespace_pid:
                candidates.append(info)
        if len(candidates) != 1:
            raise Refused(errno.ESRCH, "namespace_actor_missing_or_ambiguous")
        self.member(candidates[0])
        return candidates[0]

    def normalized(self, info):
        def local(host_pid):
            try:
                value = identity(host_pid)
            except FileNotFoundError:
                return 0
            return (
                value["namespacePids"][-1]
                if value["pidNamespace"] == info["pidNamespace"]
                else 0
            )

        return {
            **info,
            "hostPid": info["pid"],
            "pid": info["namespacePids"][-1],
            "parent": local(info["parent"]),
            "group": local(info["group"]),
        }

    def policy(self, info):
        return {
            "uid": self.args.uid,
            "gid": self.args.gid,
            "parent": self.normalized(info)["parent"],
            "namespace": info["namespace"],
            "pidNamespace": info["pidNamespace"],
            "membership": self.journal["membership"],
            "tracerHostPid": os.getpid(),
        }

    def handle(self, request):
        kind = request["kind"]
        if kind == "layout_begin":
            if (
                self.layout_header
                or type(request["length"]) is not int
                or not 0 < request["length"] <= 65536
            ):
                raise Refused(errno.EFBIG, "layout_header")
            self.layout_header = request
            return {}
        if kind == "layout_chunk":
            import base64

            if (
                not self.layout_header
                or request["index"] != len(self.layout_data) // 6000
            ):
                raise Refused(errno.EPERM, "layout_sequence")
            data = base64.b64decode(request["data"], validate=True)
            if len(data) != min(
                6000, self.layout_header["length"] - len(self.layout_data)
            ):
                raise Refused(errno.EFBIG, "layout_chunk_length")
            self.layout_data.extend(data)
            return {}
        if kind == "layout_end":
            if (
                not self.layout_header
                or len(self.layout_data) != self.layout_header["length"]
                or sha(self.layout_data) != self.layout_header["sha256"]
            ):
                raise Refused(errno.EPERM, "incomplete_layout")
            return self.handle(
                {"kind": "layout", "packet": json.loads(self.layout_data)}
            )
        if kind == "layout":
            if self.packet is not None:
                raise Refused(errno.EPERM, "duplicate_layout")
            packet = request["packet"]
            root = Path(packet["root"])
            run = Path(os.environ["TMPDIR"]).resolve(strict=True)
            if (
                run not in root.parents
                or root.resolve(strict=True) != root
                or root.stat().st_uid != self.args.uid
            ):
                raise Refused(errno.EPERM, "owned_staged_layout")
            # The trusted guest alone creates this layout; the root owns the
            # source record/runtime/config identities through every exec stop.
            # Every launch/config is produced by this captured trusted guest;
            # match its main scripts back to the exact source record as well.
            if (root / "entry.py").read_bytes() != self.files[
                "scripts/claude-agent-sdk-spawn-entry.py"
            ] or (root / "driver.mjs").read_bytes() != self.files[
                "scripts/claude-agent-sdk-spawn-driver.mjs"
            ]:
                raise Refused(errno.EPERM, "captured_entry_or_driver_changed")
            for path in root.rglob("*"):
                if path.is_file() and path.name != "storage.bin":
                    self.lease.bind(path)
            for path in packet["readFiles"]:
                self.lease.bind(path)
            self.lease.check()
            self.packet = packet
            trace = self.modules["trace"]
            cwd = root / "workspace"
            images = {
                "driver": trace.image_binding(
                    packet["driver"], packet["driverEnv"], cwd
                ),
                "entry": trace.image_binding(
                    packet["nativeEntry"], packet["nativeEnv"], cwd
                ),
                "native": trace.image_binding(
                    packet["native"], packet["nativeEnv"], cwd
                ),
            }
            self.gate = trace.ExecGate(
                self.trace, identity, self.member, images, self.pin, self.lease.check
            )
            return {}
        if kind == "identity":
            info = self.find(request["namespacePid"])
            return {"identity": self.normalized(info)}
        if kind in {"driver_gate", "native_gate"}:
            self.lease.check()
            self.check_network()
            info = self.find(request["namespacePid"])
            peer = socket_peer(
                self.group, self.init, request["clientPort"], request["serverPort"]
            )
            if peer != info["pid"]:
                raise Refused(errno.EPERM, "forged_entry_peer")
            self.pin(info["pid"])
            if kind == "driver_gate":
                if info["command"] != self.packet["driverEntry"]:
                    raise Refused(errno.EPERM, "driver_entry_argv")
                parent = identity(info["parent"])
                if self.args.profile == "sdk":
                    if parent["pid"] != self.init or info["group"] != info["pid"]:
                        raise Refused(errno.EPERM, "driver_parent_or_group")
                else:
                    self.member(parent)
                    if (
                        parent["parent"] != self.init
                        or parent["command"] != self.packet["owner"]
                        or parent["group"] != parent["pid"]
                        or info["group"] != parent["pid"]
                    ):
                        raise Refused(errno.EPERM, "group_owner_actor_chain")
                    self.pin(parent["pid"])
                    self.journal["groupOwnerActor"] = parent
                self.journal["driverActor"] = info
                self.gate.driver(info["pid"])
            else:
                if info["command"] != self.packet["nativeEntry"]:
                    raise Refused(errno.EPERM, "native_entry_argv")
                driver = self.journal["driverActor"]
                if info["parent"] != driver["pid"] or info["group"] != driver["group"]:
                    raise Refused(errno.EPERM, "native_actor_chain")
                self.gate.arm_native(info["pid"])
            return {"policy": self.policy(info)}
        if kind == "stop_actor":
            info = self.find(request["namespacePid"])
            if info["start"] != request["start"] or info["pid"] not in self.handles:
                raise Refused(errno.EPERM, "unowned_stop_identity")
            signal.pidfd_send_signal(self.handles[info["pid"]], signal.SIGKILL)
            return {"stopped": True}
        if kind == "result_begin":
            if (
                self.result_header
                or type(request["length"]) is not int
                or not 0 < request["length"] <= RESULT
            ):
                raise Refused(errno.EFBIG, "result_header")
            self.result_header = request
            return {}
        if kind == "result_chunk":
            import base64

            if (
                not self.result_header
                or request["index"] != len(self.result_data) // 6000
            ):
                raise Refused(errno.EPERM, "result_sequence")
            data = base64.b64decode(request["data"], validate=True)
            remaining = self.result_header["length"] - len(self.result_data)
            if len(data) != min(6000, remaining):
                raise Refused(errno.EFBIG, "result_chunk_length")
            self.result_data.extend(data)
            return {}
        if kind == "done":
            if (
                not self.result_header
                or len(self.result_data) != self.result_header["length"]
                or sha(self.result_data) != self.result_header["sha256"]
            ):
                raise Refused(errno.EPERM, "incomplete_result")
            self.result = json.loads(self.result_data)
            return {}
        raise Refused(errno.EPERM, "unapproved_monitor_message")

    def save(self):
        self.journal["custody"] = self.lease.observation()
        self.journal["traceEvents"] = self.gate.events if self.gate else []
        self.journal["traceWaits"] = self.trace.waits
        data = json.dumps(self.journal, separators=(",", ":")).encode()
        if len(data) > RECEIPT:
            raise Refused(errno.EFBIG, "combined_receipt_bound")
        fd = self.journal_fd
        os.lseek(fd, 0, 0)
        os.ftruncate(fd, 0)
        if os.write(fd, data) != len(data):
            raise Refused(errno.EIO, "journal_short_write")
        os.fsync(fd)

    def drain(self):
        for _ in range(128):
            row = self.trace.wait()
            if row is None:
                return
            handled = self.gate.process(row) if self.gate else False
            if not handled:
                if not (os.WIFEXITED(row["status"]) or os.WIFSIGNALED(row["status"])):
                    raise Refused(errno.EPERM, "unexpected_untraced_stop")
                self.waits.append({**row, "source": "parent_or_adoptee_waitpid"})
            if self.gate:
                for tid, task in self.gate.tasks.items():
                    if (
                        tid == task["tgid"]
                        and tid not in self.handles
                        and "terminal" not in task
                    ):
                        self.pin(tid)
        raise Refused(errno.EFBIG, "trace_event_burst_bound")

    def run(self):
        try:
            return self.run_owned()
        except BaseException as error:
            # Even failure before cgroup attachment owns this direct child.
            # Stop by held pidfd and actually wait it; never signal a PID found
            # later by pattern, nor call disappearance a successful join.
            if self.init is not None and not any(
                row["pid"] == self.init for row in self.waits
            ):
                handle = self.handles.get(self.init)
                if handle is not None:
                    try:
                        signal.pidfd_send_signal(handle, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    until = time.monotonic() + 1
                    while time.monotonic() < until:
                        row = self.trace.wait()
                        if row is None:
                            time.sleep(0.002)
                            continue
                        if self.gate and self.gate.process(row):
                            continue
                        if os.WIFEXITED(row["status"]) or os.WIFSIGNALED(row["status"]):
                            self.waits.append(
                                {**row, "source": "parent_or_adoptee_waitpid"}
                            )
                        if row["pid"] == self.init:
                            break
                else:
                    # This is the still-unwaited direct os.fork child; its PID
                    # cannot be reused before our parent wait. A pidfd setup
                    # refusal must not abandon it behind the initial barrier.
                    try:
                        os.kill(self.init, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    until = time.monotonic() + 1
                    while time.monotonic() < until:
                        row = self.trace.wait()
                        if row is None:
                            time.sleep(0.002)
                            continue
                        if os.WIFEXITED(row["status"]) or os.WIFSIGNALED(row["status"]):
                            self.waits.append(
                                {**row, "source": "parent_or_adoptee_waitpid"}
                            )
                        if row["pid"] == self.init:
                            break
            self.journal.update({"failure": str(error)[:2000], "waits": self.waits})
            self.save()
            raise
        finally:
            for fd in set([*self.setup_fds, self.parent_fd, *self.handles.values()]):
                try:
                    os.close(fd)
                except OSError as error:
                    if error.errno != errno.EBADF:
                        raise
            self.canary.close()
            for sig, handler in self.previous_signals.items():
                signal.signal(sig, handler)

    def run_owned(self):
        libc = ctypes.CDLL(None, use_errno=True)

        def checked(value, label):
            if value < 0:
                raise Refused(ctypes.get_errno(), label)

        read, write = os.pipe()
        answer_read, answer_write = os.pipe()
        self.setup_fds = [read, write, answer_read, answer_write]
        self.journal_fd = self.args.journal_fd
        deadline = time.monotonic() + 30
        previous = {
            sig: signal.signal(sig, lambda sig, frame: setattr(self, "cancelled", True))
            for sig in (signal.SIGTERM, signal.SIGINT)
        }
        previous[signal.SIGCHLD] = signal.signal(signal.SIGCHLD, signal.SIG_DFL)
        self.previous_signals = previous
        checked(libc.prctl(36, 1, 0, 0, 0), "ROOT_SUBREAPER")
        checked(libc.unshare(0x20000000), "CLONE_NEWPID")
        pid = os.fork()
        if pid == 0:
            try:
                keep = {0, 1, 2, write, answer_read, self.parent_fd}
                for name in os.listdir("/proc/self/fd"):
                    fd = int(name)
                    if fd not in keep:
                        try:
                            os.close(fd)
                        except OSError:
                            pass
                read_frame(answer_read, deadline, bytearray())
                checked(libc.unshare(0x40000000), "CLONE_NEWNET")
                util = self.core.module(
                    "scripts/claude-agent-sdk-netns-kernel.py",
                    self.files["scripts/claude-agent-sdk-netns-kernel.py"],
                )
                util.loopback_up()
                last = int(Path("/proc/sys/kernel/cap_last_cap").read_text())
                if not 0 <= last <= 63:
                    raise Refused(errno.EPERM, "capability_inventory")
                for number in range(last + 1):
                    checked(libc.prctl(24, number, 0, 0, 0), "DROP_BOUNDING_CAP")
                checked(libc.prctl(47, 4, 0, 0, 0), "CLEAR_AMBIENT")
                os.setgroups([])
                os.setresgid(self.args.gid, self.args.gid, self.args.gid)
                os.setresuid(self.args.uid, self.args.uid, self.args.uid)
                checked(libc.prctl(38, 1, 0, 0, 0), "NO_NEW_PRIVS")
                checked(libc.prctl(1, signal.SIGKILL, 0, 0, 0), "INIT_PARENT_DEATH")
                parent = select.poll()
                parent.register(self.parent_fd, select.POLLIN | select.POLLHUP)
                if parent.poll(0):
                    raise Refused(errno.ESRCH, "root_monitor_died_during_drop")
                context = GuestContext(
                    self.runtime, self.args, answer_read, write, {}, self.files
                )
                context.metadata = context.request("ready")
                result = self.core.rehearsal(
                    self.args.record_sha256,
                    self.args.node,
                    NODE22,
                    state=self.args.state,
                    composition=self.args.profile,
                    bound=20,
                    context=context,
                )
                context.finish(result)
                os._exit(0 if result["failure"] is None else 78)
            except BaseException as error:
                try:
                    write_frame(
                        write, {"kind": "guest_failure", "message": str(error)[:1000]}
                    )
                except OSError:
                    pass
                os._exit(78)
        os.close(write)
        os.close(answer_read)
        self.setup_fds = [read, answer_write]
        self.init = pid
        self.pin(pid)
        self.group.attach(pid)
        self.journal["membership"] = identity(pid)["membership"]
        self.journal["init"] = self.seen[pid]
        self.save()
        write_frame(answer_write, {"begin": True})
        pending = bytearray()
        failure = None
        try:
            while time.monotonic() < deadline - 3:
                self.drain()
                if self.cancelled:
                    raise Refused(errno.ECANCELED, "combined_cancelled")
                if self.result is not None and any(
                    row["pid"] == pid for row in self.waits
                ):
                    break
                if not select.select([read], [], [], 0.005)[0]:
                    continue
                data = os.read(read, FRAME)
                if not data:
                    if self.result is None:
                        raise Refused(errno.EPIPE, "guest_result_missing")
                    continue
                pending.extend(data)
                if len(pending) > FRAME:
                    raise Refused(errno.EFBIG, "combined_pending_frame")
                if b"\n" not in pending:
                    continue
                line, rest = pending.split(b"\n", 1)
                pending[:] = rest
                request = json.loads(line)
                if request["kind"] == "ready":
                    info = identity(pid)
                    self.journal["namespace"] = info["namespace"]
                    self.journal["pidNamespace"] = info["pidNamespace"]
                    self.member(info)
                    self.journal["init"] = info
                    self.seen[pid] = info
                    if info["namespace"] == self.host["namespace"]:
                        raise Refused(errno.EPERM, "host_network_inherited")
                    self.check_network()
                    reply = {
                        "canaryPort": self.canary.getsockname()[1],
                        "outsidePath": self.journal["outsidePath"],
                    }
                elif request["kind"] == "guest_failure":
                    raise Refused(errno.EPERM, "guest_" + request["message"])
                else:
                    reply = self.handle(request)
                self.save()
                write_frame(answer_write, reply)
            else:
                raise Refused(errno.ETIMEDOUT, "combined_inner_deadline")
            self.check_network(allow_gone=True)
            if self.result is None or self.result["failure"] is not None:
                raise Refused(errno.EPERM, "combined_protocol_refused")
        except (OSError, ValueError, KeyError) as error:
            failure = str(error)
        finally:
            # Record any not-yet-registered process identities before group stop.
            try:
                allowed = {self.init}
                for key in ("groupOwnerActor", "driverActor"):
                    if key in self.journal:
                        allowed.add(self.journal[key]["pid"])
                if self.gate:
                    allowed.update(self.gate.roles.values())
                for member in self.group.members():
                    if member not in allowed:
                        failure = failure or "unexpected_cgroup_member"
                    if member not in self.handles:
                        self.pin(member)
                if not self.group.empty():
                    self.group.stop(deadline)
                until = min(deadline, time.monotonic() + 3)
                while time.monotonic() < until:
                    self.drain()
                    if self.group.empty() and all(
                        self.reaped(fd) for fd in self.handles.values()
                    ):
                        break
                    time.sleep(0.002)
                complete = self.group.empty() and all(
                    self.reaped(fd) for fd in self.handles.values()
                )
                complete = complete and (self.gate is None or self.gate.complete())
                if not complete:
                    failure = failure or "independent_tree_join_unknown"
            except (OSError, ValueError) as error:
                complete = False
                failure = failure or str(error)
            try:
                events = self.group.events()
            except OSError as error:
                events = {"pids": {}, "memory": {}, "observationFailure": str(error)}
                complete = False
                failure = failure or str(error)
            if int(events["pids"].get("max", "0")) or int(
                events["memory"].get("oom_kill", "0")
            ):
                failure = failure or "kernel_resource_exhaustion"
            if select.select([self.canary], [], [], 0)[0]:
                failure = failure or "outside_canary_reached"
            self.journal.update(
                {
                    "failure": failure,
                    "treeSettled": complete,
                    "waits": self.waits,
                    "pidfdReaped": [
                        key for key, fd in self.handles.items() if self.reaped(fd)
                    ],
                    "processes": list(self.seen.values()),
                    "resourceEvents": events,
                    "networkSnapshots": self.network,
                    "softwareResult": self.result,
                    "outsideCanaryUnreached": not bool(
                        select.select([self.canary], [], [], 0)[0]
                    ),
                }
            )
            self.save()
        if (
            failure
            or not complete
            or not self.result
            or not verify_settlement(self.journal)
        ):
            raise Refused(errno.EPERM, failure or "combined_result_missing")
        return {
            "kernelEnforced": True,
            "originalActivation": False,
            "productionAdmission": False,
            "treeSettled": True,
            "sourceRecord": self.args.record_sha256,
            "traceEvents": self.journal["traceEvents"],
            "traceWaits": self.trace.waits,
            "waits": self.waits,
            "pidfdReaped": self.journal["pidfdReaped"],
            "softwareResult": self.result,
            "canonicalDiscovery": False,
        }

    @staticmethod
    def reaped(fd):
        poll = select.poll()
        poll.register(fd, select.POLLIN | select.POLLHUP)
        return any(flags & select.POLLHUP for _, flags in poll.poll(0))

    def check_network(self, allow_gone=False):
        if not Path(f"/proc/{self.init}").exists() and allow_gone:
            return
        util = self.core.module(
            "scripts/claude-agent-sdk-netns-kernel.py",
            self.files["scripts/claude-agent-sdk-netns-kernel.py"],
        )
        observed = util.network(self.init)
        if (
            observed["namespace"] == self.host["namespace"]
            or observed["interfaces"] != ["lo"]
            or observed["externalRoutes"]
        ):
            raise Refused(errno.EPERM, "network_routes_or_interfaces")
        self.network.append(
            {
                "observed": observed,
                "route": Path(f"/proc/{self.init}/net/route").read_text(),
                "ipv6Route": Path(f"/proc/{self.init}/net/ipv6_route").read_text(),
            }
        )


def inspect(args, core, files):
    if os.geteuid() == 0:
        raise Refused(errno.EPERM, "ordinary_uid_preflight_required")
    node = Path(args.node).resolve(strict=True)
    actual = sha(node.read_bytes())
    if platform.system() == "Linux" and actual != NODE22:
        raise Refused(errno.EPERM, "preflight_Node22_identity_before_execution")
    custody = core.module(
        "scripts/claude-agent-sdk-probe-custody.py",
        files["scripts/claude-agent-sdk-probe-custody.py"],
    )
    runtime = custody.runtime_inventory(node, actual)
    result = {
        "host": platform.uname()._asdict(),
        "custodyRuntime": runtime,
        "nodeVersion": runtime["version"],
        "sourceRecord": args.record_sha256,
        "tools": {},
        "kernelEnforced": False,
        "originalActivation": False,
    }
    for name in ("sudo", "timeout", "python3"):
        found = shutil.which(name)
        target = Path(found).resolve(strict=True) if found else None
        result["tools"][name] = (
            {"path": str(target), "sha256": sha(target.read_bytes())}
            if target
            else None
        )
    result["tools"]["node"] = {"path": str(node), "sha256": actual}
    if platform.system() == "Linux" and platform.machine() == "x86_64":
        libc = ctypes.CDLL(None, use_errno=True)
        libc.syscall.restype = ctypes.c_long
        result["landlockAbiReadOnly"] = libc.syscall(444, 0, 0, 1)
        result["namespaceReadable"] = (
            Path("/proc/self/ns/pid").exists() and Path("/proc/self/ns/net").exists()
        )
        fd = os.pidfd_open(os.getpid())
        os.close(fd)
        result["pidfdSelfReadOnly"] = True
    parent = Path(args.cgroup_parent)
    cgroup = {"path": str(parent), "available": False, "controllers": [], "enabled": []}
    if platform.system() == "Linux" and parent.is_dir():
        cgroup.update(
            {
                "available": True,
                "controllers": (parent / "cgroup.controllers").read_text().split(),
                "enabled": (parent / "cgroup.subtree_control").read_text().split(),
            }
        )
    result["combinedCgroup"] = cgroup
    result["unobservedCapabilities"] = [
        "root ptrace seize/exec/fork/thread events",
        "new PID namespace",
        "changed inherited policy",
        "combined canonical discovery",
    ]
    result["originalActivation"] = False
    return result


def journal_file():
    root = Path(os.environ["TMPDIR"]).resolve(strict=True)
    info = root.stat()
    if stat.S_IMODE(info.st_mode) != 0o700:
        raise Refused(errno.EPERM, "private_job_scratch_required")
    return root / "combined-root.json", {"device": info.st_dev, "inode": info.st_ino}


def privileged_tools(args):
    if args.node_sha256 != NODE22 or sha(Path(args.node).read_bytes()) != NODE22:
        raise Refused(errno.EPERM, "exact_SDK_Node22_required")
    if (
        sha(Path("/proc/self/exe").read_bytes()) != args.python_sha256
        or sha(Path(f"/proc/{os.getppid()}/exe").read_bytes()) != args.timeout_sha256
    ):
        raise Refused(errno.EPERM, "privileged_interpreter_or_timeout_changed")


def activate(args, core, files):
    if (
        platform.system() != "Linux"
        or platform.machine() != "x86_64"
        or os.geteuid() != 0
        or args.uid <= 0
        or args.gid <= 0
    ):
        raise Refused(errno.EPERM, "separate_root_Linux_combined_authority_required")
    privileged_tools(args)
    path, directory = journal_file()
    preflight = path.parent / "combined-preflight.json"
    st = preflight.lstat()
    if not stat.S_ISREG(st.st_mode) or st.st_uid != args.uid or st.st_mode & 0o022:
        raise Refused(errno.EPERM, "preflight_ownership")
    runtime = json.loads(preflight.read_bytes())["custodyRuntime"]
    if (
        runtime["version"] != "v22.23.2"
        or runtime["nodeSha256"] != NODE22
        or runtime["ownerUid"] != args.uid
    ):
        raise Refused(errno.EPERM, "runtime_preflight_changed")
    custody = core.module(
        "scripts/claude-agent-sdk-probe-custody.py",
        files["scripts/claude-agent-sdk-probe-custody.py"],
    )
    modules = private_modules(core, files)
    fixture = core.module(
        "scripts/claude-agent-sdk-admin-fixture.py",
        files["scripts/claude-agent-sdk-admin-fixture.py"],
    )
    lease = custody.Lease(
        runtime,
        args.node,
        NODE22,
        {"schema": core.SCHEMA, "scope": "combined-owned-fake"},
        reserve_pipes=False,
    )
    group = None
    journal = {
        "sourceRecord": args.record_sha256,
        "directory": directory,
        "state": args.state,
        "run": args.run,
        "fixture": None,
        "cgroup": None,
        "treeSettled": False,
        "originalActivation": False,
        "monitor": identity(os.getpid()),
    }
    descriptor = os.open(
        path, os.O_CREAT | os.O_EXCL | os.O_WRONLY | os.O_NOFOLLOW, 0o600
    )
    args.journal_fd = descriptor

    def save():
        data = json.dumps(journal, separators=(",", ":")).encode()
        if len(data) > RECEIPT:
            raise Refused(errno.EFBIG, "receipt_bound")
        os.lseek(descriptor, 0, 0)
        os.ftruncate(descriptor, 0)
        os.write(descriptor, data)
        os.fsync(descriptor)

    try:
        for name, data in files.items():
            lease.bind(ROOT / name, sha(data))
        lease.bind(ROOT / RECORD, args.record_sha256)
        lease.bind(preflight)
        lease.bind(
            Path(f"/proc/{os.getppid()}/exe").resolve(strict=True), args.timeout_sha256
        )
        group = modules["cgroup"].LinuxCgroup(
            args.cgroup_parent, "sdk234-" + os.urandom(12).hex()
        )
        journal["cgroup"] = group.create()
        save()

        def fixture_created(receipt):
            journal["fixture"] = receipt
            journal["fixtureStage"] = "created"
            save()

        prepare_fixture(Path("/etc"), 0, args.state, args.run, fixture, fixture_created)
        journal["fixtureStage"] = "ready"
        save()
        outside = path.parent / "outside-readable-owned-sentinel"
        fd = os.open(
            outside, os.O_CREAT | os.O_EXCL | os.O_WRONLY | os.O_NOFOLLOW, 0o444
        )
        os.write(fd, b"owned boundary sentinel")
        os.close(fd)
        journal["outsidePath"] = str(outside)
        save()
        monitor = Monitor(args, core, files, modules, runtime, lease, group, journal)
        return monitor.run()
    except BaseException as error:
        journal["failure"] = str(error)[:2000]
        if group is not None and group.created:
            journal["cgroup"] = group.receipt()
        save()
        raise
    finally:
        os.close(descriptor)
        if group is not None:
            group.close()
        lease.close()


def cleanup(args, core, files):
    if os.geteuid() != 0 or platform.system() != "Linux":
        raise Refused(errno.EPERM, "root_cleanup_authority_required")
    privileged_tools(args)
    path, directory = journal_file()
    st = path.lstat()
    if (
        not stat.S_ISREG(st.st_mode)
        or st.st_uid != 0
        or stat.S_IMODE(st.st_mode) != 0o600
        or st.st_nlink != 1
    ):
        raise Refused(errno.EPERM, "root_journal_identity")
    data = path.read_bytes()
    if len(data) > RECEIPT:
        raise Refused(errno.EFBIG, "root_journal_bound")
    journal = json.loads(data)
    if (
        journal["sourceRecord"] != args.record_sha256
        or journal["directory"] != directory
        or journal["state"] != args.state
        or journal["run"] != args.run
    ):
        raise Refused(errno.EPERM, "cleanup_binding")
    try:
        monitor = identity(journal["monitor"]["pid"])
    except FileNotFoundError:
        monitor = None
    if monitor is not None and monitor["start"] == journal["monitor"]["start"]:
        raise Refused(errno.EBUSY, "setup_monitor_still_live")
    # A missing full-tree observation is never repaired by emptiness/disposal.
    tree = verify_settlement(journal)
    modules = private_modules(core, files)
    errors = []
    cgroup_cleaned = journal["cgroup"] is None
    fixture_cleaned = journal["fixture"] is None
    if journal["cgroup"] is not None:
        try:
            modules["cgroup"].cleanup(journal["cgroup"])
            cgroup_cleaned = True
        except OSError as error:
            errors.append(str(error))
    fixture = core.module(
        "scripts/claude-agent-sdk-admin-fixture.py",
        files["scripts/claude-agent-sdk-admin-fixture.py"],
    )
    if journal["fixture"] is not None:
        try:
            if journal.get("fixtureStage") == "ready":
                fixture.cleanup(Path("/etc"), "claude-code", args.state, 0, args.run)
            else:
                cleanup_partial_fixture(Path("/etc"), 0, journal["fixture"], fixture)
            fixture_cleaned = True
        except (OSError, fixture.Refused) as error:
            errors.append(str(error))
    after = path.lstat()
    if (after.st_dev, after.st_ino) != (st.st_dev, st.st_ino):
        raise Refused(errno.EPERM, "journal_replaced")
    if not errors:
        path.unlink()
    return {
        "fixtureCleanup": fixture_cleaned,
        "cgroupCleanup": cgroup_cleaned,
        "independentTreeSettlement": tree,
        "originalActivation": False,
        "failure": errors or (None if tree else "tree_settlement_unproved"),
        "journal": journal,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inspect", action="store_true")
    parser.add_argument("--kernel-combined", action="store_true")
    parser.add_argument("--cleanup-combined", action="store_true")
    parser.add_argument("--check-workflow", action="store_true")
    parser.add_argument("--record-sha256", required=True)
    parser.add_argument("--node")
    parser.add_argument("--node-sha256")
    parser.add_argument("--python-sha256")
    parser.add_argument("--timeout-sha256")
    parser.add_argument("--uid", type=int, default=-1)
    parser.add_argument("--gid", type=int, default=-1)
    parser.add_argument("--cgroup-parent", default="/sys/fs/cgroup")
    parser.add_argument(
        "--state", choices=("absent", "disabled", "managed-deny"), default="absent"
    )
    parser.add_argument(
        "--profile", choices=("sdk", "acp-actor-shape"), default="acp-actor-shape"
    )
    parser.add_argument("--run", default="")
    args = parser.parse_args()
    if (
        sum(
            (
                args.inspect,
                args.kernel_combined,
                args.cleanup_combined,
                args.check_workflow,
            )
        )
        != 1
    ):
        raise Refused(errno.EPERM, "one_explicit_fake_mode_required")
    core, files = captured_core(args.record_sha256)
    result = (
        check_workflow(files)
        if args.check_workflow
        else inspect(args, core, files)
        if args.inspect
        else cleanup(args, core, files)
        if args.cleanup_combined
        else activate(args, core, files)
    )
    print(json.dumps(result))
    return 1 if result.get("failure") else 0


if __name__ == "__main__":
    raise SystemExit(main())
