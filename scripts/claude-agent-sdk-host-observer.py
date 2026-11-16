"""Private observer bridge for the proposed same-live-root host transaction.

No CLI, host activation or serialized bootstrap attestation. The caller supplies
its existing live monitor and source-bound ordinary supervisor. Root holds the
same package and each staged input until the outer transaction settles.
"""

import errno
import hashlib
import json
import os
from pathlib import Path
import stat
import sys
import tempfile

ENTRY = "scripts/claude-agent-sdk-observer-entry.py"
POLICY = "scripts/claude-agent-sdk-spawn-policy.py"
KERNEL = "scripts/claude-agent-sdk-netns-entry.py"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value, limit=65536):
    data = json.dumps(value, separators=(",", ":")).encode()
    if len(data) > limit:
        raise OSError(errno.EFBIG, "observer_packet_bound")
    return data


def terminal_code(status):
    if os.WIFEXITED(status):
        return os.WEXITSTATUS(status)
    if os.WIFSIGNALED(status):
        return -os.WTERMSIG(status)
    raise OSError(errno.EPERM, "actual_terminal_wait_required")


def configuration(root, command, environment, read_files, files, loader):
    return {
        "originalActivation": False,
        "command": command,
        "environment": environment,
        "inputs": {str(root / "entry.py"): sha(files[ENTRY])},
        "policy": {
            "file": str(root / "policy.py"),
            "sha256": sha(files[POLICY]),
            "inputs": {
                "root": str(root),
                "canonicalDirectory": str(root),
                "writable": "/dev/null",
                "readFiles": sorted(read_files),
                "runtimeRoots": [],
                "execFiles": [],
                "nativeExecFiles": [command[0], loader],
                "entrySource": str(root / "kernel.py"),
                "entrySha256": sha(files[KERNEL]),
            },
        },
    }


class GuestObserver:
    """One observer per case; inherit only one private read pipe into the child."""

    def __init__(self, context, files):
        self.context, self.files = context, files
        self.root = self.read = self.write = self.child = None
        self.command = None
        self.pending = False
        self.settled = False

    def prepare(self, command, kwargs, bootstrap):
        if self.pending or self.child is not None or bootstrap is None:
            raise OSError("observer_already_pending_or_capture_missing")
        if command != self.context.observer_command(command[0]):
            raise OSError("observer_command_tail_changed")
        if set(kwargs) != {"env", "cwd", "stdout", "stderr"}:
            raise OSError("observer_launch_options_changed")
        self.root = Path(
            tempfile.mkdtemp(prefix="sdk-host-observer-", dir=os.environ["TMPDIR"])
        )
        for source, name in (
            (ENTRY, "entry.py"),
            (POLICY, "policy.py"),
            (KERNEL, "kernel.py"),
        ):
            target = self.root / name
            target.write_bytes(self.files[source])
            target.chmod(0o600)
            bootstrap.bind(target, sha(self.files[source]))
        self.read, self.write = os.pipe()
        for fd in (self.read, self.write):
            os.set_inheritable(fd, False)
        config = configuration(
            self.root,
            command,
            kwargs["env"],
            bootstrap.files,
            self.files,
            self.context.loader,
        )
        data = encoded(config)
        file = self.root / "config.json"
        file.write_bytes(data)
        file.chmod(0o600)
        bootstrap.bind(file, sha(data))
        bootstrap.check()
        self.command = [
            str(Path(sys.executable).resolve(strict=True)),
            "-I",
            "-S",
            "-B",
            str(self.root / "entry.py"),
            str(file),
            sha(data),
            str(self.read),
        ]
        self.pending = True  # Before the request or any possible Popen.
        self.context.request(
            "observer_intent",
            packet={
                "root": str(self.root),
                "entry": self.command,
                "command": command,
                "environment": kwargs["env"],
                "cwd": str(kwargs["cwd"]),
                "readFiles": sorted(bootstrap.files),
                "configSha256": sha(data),
            },
        )
        return self.command, {**kwargs, "close_fds": True, "pass_fds": (self.read,)}

    def started(self, child):
        if not self.pending or self.child is not None:
            raise OSError("observer_start_without_intent")
        self.child = child  # Keep the actual object before any fallible request.
        os.close(self.read)
        self.read = None
        reply = self.context.request("observer_started", namespacePid=child.pid)
        data = encoded({"kernel": reply["policy"]}, limit=4096)
        # <= PIPE_BUF, no payload can inherit the writable end (close_fds).
        if os.write(self.write, data) != len(data):
            raise OSError("observer_gate_short_write")
        os.close(self.write)
        self.write = None

    def joined(self, child):
        if child is not self.child or child.returncode is None:
            raise OSError("actual_owned_observer_wait_required")
        self.context.request(
            "observer_joined", namespacePid=child.pid, code=child.returncode
        )
        self.pending = False
        self.settled = True
        # Derived files remain bound in the runtime capture through its fake
        # case. Only the outer transaction may close/unlink after root's ack.


class RootObserver:
    """Uses the existing monitor's live tracer, group, custody and pin handles."""

    def __init__(self, monitor, observer_command, identity_reader):
        self.monitor, self.observer_command = monitor, observer_command
        self.identity_reader = identity_reader
        self.packet = self.actor = self.gate = None
        self.attempted = self.settled = False

    def intent(self, packet):
        if self.attempted:
            raise OSError("duplicate_observer_intent")
        self.attempted = True  # Missing start notification remains an obligation.
        self.packet = packet
        root = Path(packet["root"])
        scratch = Path(os.environ["TMPDIR"]).resolve(strict=True)
        if (
            scratch not in root.parents
            or root.resolve(strict=True) != root
            or root.lstat().st_uid != self.monitor.args.uid
            or stat.S_IMODE(root.lstat().st_mode) != 0o700
        ):
            raise OSError("observer_owned_root_required")
        if packet["command"] != self.observer_command(self.monitor.args.node):
            raise OSError("observer_exact_command_required")
        expected_env = {
            "PATH": "/usr/bin:/bin",
            "HOME": str(scratch),
            "TMPDIR": str(scratch),
            "LANG": "C",
            "LC_ALL": "C",
        }
        if packet["environment"] != expected_env:
            raise OSError("observer_environment_changed")
        config = root / "config.json"
        for name in ("entry.py", "policy.py", "kernel.py", "config.json"):
            self.monitor.lease.bind(root / name)
        with config.open("rb") as stream:
            data = stream.read(65537)
        if len(data) > 65536 or sha(data) != packet["configSha256"]:
            raise OSError("observer_config_changed")
        value = json.loads(data)
        expected_paths = {
            root / name
            for name in ("entry.py", "policy.py", "kernel.py", "config.json")
        }
        if set(root.iterdir()) != expected_paths:
            raise OSError("observer_stage_members_changed")
        for source, name in (
            (ENTRY, "entry.py"),
            (POLICY, "policy.py"),
            (KERNEL, "kernel.py"),
        ):
            if (root / name).read_bytes() != self.monitor.files[source]:
                raise OSError("observer_captured_source_changed")
        if value != configuration(
            root,
            packet["command"],
            expected_env,
            set(packet["readFiles"]) - {str(config)},
            self.monitor.files,
            self.monitor.loader,
        ):
            raise OSError("observer_config_command_changed")
        entry = packet["entry"]
        if (
            len(entry) != 8
            or entry[1:4] != ["-I", "-S", "-B"]
            or entry[0] != str(Path(sys.executable).resolve(strict=True))
            or entry[4:7] != [str(root / "entry.py"), str(config), sha(data)]
            or not entry[7].isdigit()
            or not 3 <= int(entry[7]) < 4096
        ):
            # The exact source entry has eight argv elements, not a caller tail.
            raise OSError("observer_entry_command_changed")
        cwd = Path(packet["cwd"])
        if (
            scratch not in cwd.parents
            or cwd.resolve(strict=True) != cwd
            or cwd.stat().st_uid != self.monitor.args.uid
            or cwd.stat().st_mode & 0o077
        ):
            raise OSError("observer_private_cwd_required")
        for path in [*expected_paths, *packet["readFiles"]]:
            self.monitor.lease.bind(path)
        self.monitor.check_custody()
        trace = self.monitor.modules["trace"]
        image = trace.image_binding(packet["command"], expected_env, packet["cwd"])
        self.gate = trace.ExecGate(
            self.monitor.trace,
            self.identity_reader,
            self.monitor.member,
            {"driver": image},
            self.monitor.pin,
            self.monitor.check_custody,
            allow_native=False,
        )
        if self.monitor.gate is not None and not self.monitor.gate.complete():
            raise OSError("previous_phase_wait_missing")
        self.monitor.gate = self.gate
        return {}

    def started(self, namespace_pid):
        if not self.attempted or self.gate is None or self.actor is not None:
            raise OSError("observer_start_without_pre_entry_intent")
        info = self.monitor.find(namespace_pid)
        self.actor = self.monitor.pin(info["pid"])
        if (
            info["parent"] != self.monitor.init
            or info["command"] != self.packet["entry"]
        ):
            raise OSError("observer_actual_entry_or_parent_changed")
        self.monitor.check_custody()
        self.gate.driver(info["pid"])
        return {"policy": self.monitor.policy(info)}

    def joined(self, namespace_pid, code):
        if (
            self.actor is None
            or namespace_pid != self.actor["namespacePids"][-1]
            or type(code) is not int
        ):
            raise OSError("observer_wait_identity_changed")
        self.monitor.drain()
        if not self.gate.complete():
            raise OSError("observer_trace_wait_missing")
        actual = {
            (row["actor"], row["pid"], row["status"])
            for row in self.monitor.trace.waits
            if os.WIFEXITED(row["status"]) or os.WIFSIGNALED(row["status"])
        }
        for task in self.gate.tasks.values():
            row = task["terminal"]
            if (row["actor"], row["pid"], row["status"]) not in actual:
                raise OSError("observer_live_trace_wait_missing")
        terminal = self.gate.tasks[self.actor["pid"]]["terminal"]
        if terminal_code(terminal["status"]) != code:
            raise OSError("observer_parent_and_tracer_wait_disagree")
        self.monitor.check_custody()
        self.settled = True
        return {}


def fixture_settlement(monitor, root, proof):
    """Authorize a staged-input release only from the same live case's waits.

    This function does not unlink or close anything. Its caller may release only
    the corresponding stage bindings, after durably recording this acknowledgement.
    The foundational package/admin owner stays held through the final init wait.
    """
    monitor.check_custody()
    if str(root) != monitor.packet["root"] or proof.get("driverIntent") is not True:
        raise OSError("staged_fixture_or_driver_intent_changed")
    monitor.drain()
    gate = monitor.gate
    if gate is None or not gate.complete() or set(gate.roles) != {"driver", "native"}:
        raise OSError("staged_actual_child_waits_missing")
    actual = {
        (row["actor"], row["pid"], row["status"])
        for row in monitor.trace.waits
        if os.WIFEXITED(row["status"]) or os.WIFSIGNALED(row["status"])
    }
    for task in gate.tasks.values():
        row = task["terminal"]
        if (row["actor"], row["pid"], row["status"]) not in actual:
            raise OSError("staged_live_trace_wait_missing")
    driver, native = (gate.tasks[gate.roles[role]] for role in ("driver", "native"))
    driver_pid = driver["identity"]["namespacePids"][-1]
    native_pid = native["identity"]["namespacePids"][-1]
    parent_wait = proof.get("driver")
    if (
        not isinstance(parent_wait, dict)
        or type(parent_wait.get("code")) is not int
        or parent_wait.get("pid") != driver_pid
        or parent_wait["code"] != terminal_code(driver["terminal"]["status"])
    ):
        raise OSError("staged_driver_parent_wait_disagrees")
    launch = proof.get("launch", {})
    wait = launch.get("wait", {})
    config = json.loads(Path(monitor.packet["config"]).read_bytes())
    if (
        launch.get("state") != "settled"
        or launch.get("active") is not True
        or launch.get("settlement") != "exact_parent_handle_wait"
        or launch.get("parent", {}).get("pid") != driver_pid
        or wait.get("pid") != native_pid
        or wait.get("actor") != driver_pid
        or wait.get("launchId") != config["native"]["launchId"]
        or wait.get("source") != "ChildProcess.exit"
    ):
        raise OSError("staged_native_exact_parent_wait_missing")
    code = terminal_code(native["terminal"]["status"])
    import signal

    if code < 0:
        match = (
            wait.get("code") is None
            and wait.get("signal") == signal.Signals(-code).name
        )
    else:
        match = (
            type(wait.get("code")) is int
            and wait["code"] == code
            and wait.get("signal") is None
        )
    if not match:
        raise OSError("staged_native_parent_wait_disagrees")
    monitor.check_custody()
    return {
        "stage": str(root),
        "driver": driver_pid,
        "native": native_pid,
        "waitsSha256": sha(encoded(sorted(actual), limit=65536)),
    }
