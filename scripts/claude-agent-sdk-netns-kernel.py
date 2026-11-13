"""UNACTIVATED Linux-only root monitor for one owned-fake namespace proposal.

No arbitrary command, canonical fixture, namespace name or original artifact.
Current tests never call launch(), cleanup_receipt() or live namespace methods.
"""

import ctypes
import errno
import fcntl
import hashlib
import json
import os
from pathlib import Path
import platform
import select
import signal
import socket
import stat
import struct
import time

LIBC = ctypes.CDLL(None, use_errno=True)


def checked(value, name):
    if value < 0:
        number = ctypes.get_errno()
        raise OSError(number, name + ": " + os.strerror(number))
    return value


def ns(pid="self"):
    info = os.stat(f"/proc/{pid}/ns/net")
    return {"device": info.st_dev, "inode": info.st_ino}


def process(pid):
    data = Path(f"/proc/{pid}/stat").read_text().rsplit(") ", 1)[1].split()
    status = Path(f"/proc/{pid}/status").read_text().splitlines()
    uid = next(row for row in status if row.startswith("Uid:")).split()[1:]
    gid = next(row for row in status if row.startswith("Gid:")).split()[1:]
    groups = next(row for row in status if row.startswith("Groups:")).split()[1:]
    caps = {
        row.split(":", 1)[0]: row.split(":", 1)[1].strip()
        for row in status
        if row.startswith(("CapEff:", "CapPrm:", "CapInh:", "CapAmb:", "CapBnd:"))
    }
    return {
        "pid": pid,
        "parent": int(data[1]),
        "start": data[19],
        "namespace": ns(pid),
        "uids": [int(x) for x in uid],
        "gids": [int(x) for x in gid],
        "groups": groups,
        "caps": caps,
    }


def network(pid="self"):
    interfaces = [
        row.split(":", 1)[0].strip()
        for row in Path(f"/proc/{pid}/net/dev").read_text().splitlines()[2:]
    ]
    external = []
    for row in Path(f"/proc/{pid}/net/route").read_text().splitlines()[1:]:
        fields = row.split()
        if fields[0] != "lo" or fields[2] != "00000000" or fields[1] == "00000000":
            external.append(row)
    for row in Path(f"/proc/{pid}/net/ipv6_route").read_text().splitlines():
        fields = row.split()
        # Linux includes unreachable default sentinel routes on lo. They cannot
        # carry traffic; accept only those defaults, never a unicast default.
        default = fields[0] == "0" * 32 and fields[1] == "00"
        rejected = int(fields[-2], 16) & 0x200
        if fields[-1] != "lo" or fields[4] != "0" * 32 or (default and not rejected):
            external.append(row)
    return {
        "namespace": ns(pid),
        "interfaces": sorted(interfaces),
        "externalRoutes": external,
    }


def require_nonroot(info, uid, gid):
    if (
        uid <= 0
        or gid <= 0
        or info["uids"] != [uid] * 4
        or info["gids"] != [gid] * 4
        or info["groups"]
        or set(info["caps"]) != {"CapEff", "CapPrm", "CapInh", "CapAmb", "CapBnd"}
        or any(int(value, 16) for value in info["caps"].values())
    ):
        raise OSError(errno.EPERM, "nonroot_capability_identity")


def loopback_up():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as control:
        request = bytearray(40)
        request[:3] = b"lo\0"
        fcntl.ioctl(control.fileno(), 0x8913, request, True)
        flags = struct.unpack_from("H", request, 16)[0]
        struct.pack_into("H", request, 16, flags | 1)
        fcntl.ioctl(control.fileno(), 0x8914, request)


def receipt_path(args):
    root = Path(os.environ["TMPDIR"]).resolve(strict=True)
    info = root.stat()
    if info.st_uid != args.uid or stat.S_IMODE(info.st_mode) != 0o700:
        raise OSError(errno.EPERM, "owned_private_TMPDIR_required")
    return root / "netns-monitor.json", {"device": info.st_dev, "inode": info.st_ino}


def send(fd, message):
    data = (json.dumps(message) + "\n").encode()
    if len(data) > 16384:
        raise OSError(errno.EFBIG, "monitor_message_limit")
    view = memoryview(data)
    while view:
        view = view[os.write(fd, view) :]


def read_line(fd, deadline, pending, cancelled=lambda: False):
    while b"\n" not in pending:
        if cancelled():
            raise OSError(errno.ECANCELED, "monitor_cancelled")
        if time.monotonic() >= deadline:
            raise OSError(errno.ETIMEDOUT, "monitor_deadline")
        if not select.select(
            [fd], [], [], min(0.05, max(0, deadline - time.monotonic()))
        )[0]:
            continue
        data = os.read(fd, 16384)
        if not data:
            raise OSError(errno.EPIPE, "monitor_peer_closed")
        pending.extend(data)
        if len(pending) > 16384:
            raise OSError(errno.EFBIG, "monitor_message_limit")
    line, rest = pending.split(b"\n", 1)
    pending[:] = rest
    return json.loads(line)


class Guest:
    kernel = True
    missing_join = False

    def __init__(self, args, host, write, read):
        self.args = args
        self.host = host
        self.write = write
        self.read = read
        self.pending = bytearray()
        self.deadline = time.monotonic() + 25
        self.namespace = None
        self.gates = {}
        self.serial = 0

    def clock(self):
        return time.monotonic()

    def cancelled(self, phase):
        return False

    def protocol_cancelled(self):
        return False  # Root monitor enforces cancellation and the outer deadline.

    def exchange(self, phase, **extra):
        send(self.write, {"phase": phase, **extra})
        reply = read_line(self.read, self.deadline, self.pending)
        if reply.get("continue") is not True:
            raise OSError(errno.ECANCELED, "monitor_cancelled")
        return reply

    def endpoint_ready(self, port):
        return self.exchange("endpoint", port=port)["canaryPort"]

    def stage(self, phase):
        if phase == "namespace":
            checked(LIBC.unshare(0x40000000), "CLONE_NEWNET")
            self.namespace = ns()
            if self.namespace == self.host:
                raise OSError(errno.EPERM, "namespace_not_created")
        elif phase == "loopback":
            loopback_up()
        elif phase == "drop":
            # Drop all bounding/ambient capabilities before leaving root.
            last = int(Path("/proc/sys/kernel/cap_last_cap").read_text())
            if not 0 <= last <= 63:
                raise OSError(errno.ENOSYS, "unknown_capability_set")
            for number in range(last + 1):
                checked(LIBC.prctl(24, number, 0, 0, 0), "PR_CAPBSET_DROP")
            checked(LIBC.prctl(47, 4, 0, 0, 0), "CLEAR_AMBIENT_CAPABILITIES")
            os.setgroups([])
            os.setgid(self.args.gid)
            os.setuid(self.args.uid)
            checked(LIBC.prctl(38, 1, 0, 0, 0), "NO_NEW_PRIVILEGES")
            checked(LIBC.prctl(1, signal.SIGKILL, 0, 0, 0), "PARENT_DEATH_SIGNAL")
            require_nonroot(process(os.getpid()), self.args.uid, self.args.gid)
        self.exchange(phase)

    def entry(self, command, kwargs, root, node, files):
        read, write = os.pipe()
        config = {
            "root": str(root),
            "node": str(node),
            "writable": str(root / "workspace/storage.bin"),
            "runtimeRoots": sorted(
                {
                    str(Path(name).resolve())
                    for name in (
                        "/usr/lib",
                        "/lib",
                        "/lib64",
                        str(node.parent.parent / "lib"),
                    )
                    if Path(name).is_dir()
                }
            ),
            "readFiles": ["/dev/urandom", "/dev/random"]
            + (["/etc/ld.so.cache"] if Path("/etc/ld.so.cache").is_file() else []),
            "gateFd": read,
            "namespace": self.namespace,
            "hostNamespace": self.host,
            "uid": self.args.uid,
            "gid": self.args.gid,
        }
        name = root / ("entry-" + str(self.serial) + ".json")
        self.serial += 1
        name.write_text(json.dumps(config))
        target = root / "netns-entry.py"
        if not target.exists():
            target.write_bytes(files["scripts/claude-agent-sdk-netns-entry.py"])
            target.chmod(0o444)
        elif target.read_bytes() != files["scripts/claude-agent-sdk-netns-entry.py"]:
            raise OSError(errno.EPERM, "staged_entry_changed")
        kwargs["pass_fds"] = (*kwargs.get("pass_fds", ()), read)
        self.gates[str(name)] = (read, write)
        return [os.sys.executable, "-B", str(target), str(name), *command], kwargs

    def confirm_child(self, child):
        self.exchange("child", pid=child.pid)
        read, write = next(reversed(self.gates.values()))
        os.write(write, b"1")
        os.close(write)
        os.close(read)
        self.gates.pop(next(reversed(self.gates)))

    def cleanup(self):
        for read, write in self.gates.values():
            os.close(read)
            os.close(write)
        self.gates.clear()
        return {"observed": True, "namespace": self.namespace, "kernelEnforced": True}


def root_required(args):
    if (
        platform.system() != "Linux"
        or platform.machine() != "x86_64"
        or os.geteuid() != 0
    ):
        raise OSError(errno.EPERM, "future_root_Linux_x64_authority_required")
    if not args.uid or args.uid <= 0 or args.gid is None or args.gid <= 0:
        raise OSError(errno.EPERM, "nonroot_runner_required")
    if not hasattr(os, "pidfd_open") or not hasattr(signal, "pidfd_send_signal"):
        raise OSError(errno.ENOSYS, "pidfd_support_missing")
    if (
        args.node_sha256
        != "3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327"
    ):
        raise OSError(errno.EPERM, "exact_Node22_linux_identity_required")
    for name, expected in (
        ("/proc/self/exe", args.python_sha256),
        (f"/proc/{os.getppid()}/exe", args.timeout_sha256),
    ):
        if (
            not expected
            or hashlib.sha256(Path(name).read_bytes()).hexdigest() != expected
        ):
            raise OSError(errno.EPERM, "startup_runtime_changed")


def launch(args, files, core):
    root_required(args)
    path, identity = receipt_path(args)
    descriptor = os.open(
        path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600
    )
    record = {
        "sourceRecord": args.record_sha256,
        "directory": identity,
        "uid": args.uid,
        "monitor": process(os.getpid()),
        "children": [],
        "settled": False,
        "originalActivation": False,
    }
    info = os.fstat(descriptor)
    record["receipt"] = {"device": info.st_dev, "inode": info.st_ino}
    handles = {}
    host = ns()
    deadline = time.monotonic() + 27
    result = None
    failure = None
    cancelled = False

    def save():
        data = json.dumps(record).encode()
        os.lseek(descriptor, 0, 0)
        os.ftruncate(descriptor, 0)
        os.write(descriptor, data)
        os.fsync(descriptor)

    def register(pid, parent, expected_ns=None):
        handle = os.pidfd_open(pid)
        observed = process(pid)
        if observed["parent"] != parent or (
            expected_ns is not None and observed["namespace"] != expected_ns
        ):
            os.close(handle)
            raise OSError(errno.EPERM, "foreign_child_identity")
        handles[pid] = handle
        record["children"].append(observed)
        save()
        return observed

    def cancel(signum, frame):
        nonlocal cancelled
        cancelled = True

    previous = {
        sig: signal.signal(sig, cancel) for sig in (signal.SIGTERM, signal.SIGINT)
    }
    previous[signal.SIGCHLD] = signal.signal(signal.SIGCHLD, signal.SIG_DFL)
    checked(LIBC.prctl(36, 1, 0, 0, 0), "CHILD_SUBREAPER")
    read, write = os.pipe()
    answer_read, answer_write = os.pipe()
    core.reject_inherited_sockets([0, 1, 2])
    pid = os.fork()
    if pid == 0:
        try:
            os.close(read)
            os.close(answer_write)
            allowed = {0, 1, 2, write, answer_read}
            for name in os.listdir("/proc/self/fd"):
                fd = int(name)
                if fd not in allowed:
                    try:
                        os.close(fd)
                    except OSError:
                        pass
            core.reject_inherited_sockets(allowed)
            checked(LIBC.prctl(1, signal.SIGKILL, 0, 0, 0), "PARENT_DEATH_SIGNAL")
            if os.getppid() != record["monitor"]["pid"]:
                raise OSError(errno.ESRCH, "monitor_died")
            ops = Guest(args, host, write, answer_read)

            def protocol(selected):
                with core.tempfile.TemporaryDirectory(
                    prefix="sdk-netns-payload-", dir=os.environ["TMPDIR"]
                ) as temporary:
                    return core.protocol(
                        args.node, args.node_sha256, 10, files, selected, temporary
                    )

            response = core.execute(ops, protocol)
            if response["protocol"]:
                response["protocol"]["observations"] = [
                    json.loads(row)
                    for row in response["protocol"]["stderr"].splitlines()
                    if row.startswith("{")
                ]
                for key in ("native", "driver", "stderr"):
                    response["protocol"].pop(key, None)
            send(write, {"phase": "done", "result": response})
            os._exit(0 if not response["failure"] else 78)
        except BaseException as error:
            try:
                send(write, {"phase": "error", "error": str(error)})
            except BaseException:
                pass
            os._exit(78)
    os.close(write)
    os.close(answer_read)
    guest_namespace = None
    canary = None
    phases = []
    try:
        register(pid, os.getpid())
        pending = bytearray()
        while True:
            if cancelled:
                raise OSError(errno.ECANCELED, "root_monitor_cancelled")
            message = read_line(read, deadline, pending, lambda: cancelled)
            phase = message["phase"]
            if phase == "error":
                raise OSError(errno.EPERM, message["error"])
            if phase == "done":
                result = message["result"]
                break
            if phase == "endpoint":
                if not phases or phases[-1] != "protocol" or canary is not None:
                    raise OSError(errno.EPERM, "endpoint_phase")
                canary = socket.socket()
                canary.bind(("127.0.0.1", 0))
                canary.listen(1)
                port = canary.getsockname()[1]
                if port == message["port"]:
                    raise OSError(errno.EADDRINUSE, "canary_port_collision")
                send(answer_write, {"continue": True, "canaryPort": port})
                continue
            if phase == "child":
                info = register(message["pid"], pid, guest_namespace)
                require_nonroot(info, args.uid, args.gid)
                if len(handles) > 3:
                    raise OSError(errno.EPERM, "process_bound")
            else:
                if len(phases) >= len(core.PHASES) or phase != core.PHASES[len(phases)]:
                    raise OSError(errno.EPERM, "phase_order")
                phases.append(phase)
                info = process(pid)
                if phase == "namespace":
                    guest_namespace = info["namespace"]
                    if guest_namespace == host:
                        raise OSError(errno.EPERM, "wrong_namespace")
                if phase in ("loopback", "identity", "drop", "protocol", "cleanup"):
                    core.check_network(network(pid), host)
                    if info["namespace"] != guest_namespace:
                        raise OSError(errno.EPERM, "namespace_drift")
                if phase in ("drop", "protocol", "cleanup"):
                    require_nonroot(info, args.uid, args.gid)
                    record["children"][0] = info
                    save()
            send(answer_write, {"continue": True})
    except (OSError, ValueError, KeyError) as error:
        failure = str(error)
    finally:
        # Normal completion gets a finite chance to exit without a forced kill.
        waited = []
        if result is not None and not failure:
            until = time.monotonic() + 0.5
            while time.monotonic() < until:
                joined, code = os.waitpid(pid, os.WNOHANG)
                if joined:
                    waited.append({"pid": joined, "waitStatus": code})
                    if code != 0:
                        failure = "guest_exit_failed"
                    break
                time.sleep(0.01)
        # Stop the direct guest first, then kill only registered pidfd identities.
        if pid in handles and not waited:
            try:
                signal.pidfd_send_signal(handles[pid], signal.SIGSTOP)
            except ProcessLookupError:
                pass
        for handle in reversed(list(handles.values())):
            try:
                signal.pidfd_send_signal(handle, signal.SIGKILL)
            except ProcessLookupError:
                pass
        until = time.monotonic() + 2
        while time.monotonic() < until:
            try:
                joined, code = os.waitpid(-1, os.WNOHANG)
            except ChildProcessError:
                record["settled"] = True
                break
            if joined:
                waited.append({"pid": joined, "waitStatus": code})
            else:
                time.sleep(0.01)
        # pidfd HUP separately observes that each registered process was reaped,
        # rather than crediting a guest's SDK cleanup promise or exit marker.
        reaped = []
        for child, handle in handles.items():
            poll = select.poll()
            poll.register(handle, select.POLLIN | select.POLLHUP)
            events = poll.poll(0)
            if not events or not events[0][1] & select.POLLHUP:
                record["settled"] = False
            else:
                reaped.append(child)
        record.update(
            {
                "waits": waited,
                "pidfdReaped": reaped,
                "phases": phases,
                "failure": failure,
                "kernelResult": result,
            }
        )
        if canary is not None:
            record["outsideCanaryUnreached"] = not bool(
                select.select([canary], [], [], 0)[0]
            )
            canary.close()
            if not record["outsideCanaryUnreached"]:
                failure = "outside_canary_reached"
                record["failure"] = failure
        save()
        os.close(descriptor)
        os.close(read)
        os.close(answer_write)
        for handle in handles.values():
            os.close(handle)
        for sig, handler in previous.items():
            signal.signal(sig, handler)
    if (
        failure
        or not record["settled"]
        or not result
        or result["failure"]
        or phases != list(core.PHASES)
        or len(handles) != 3
        or not record.get("outsideCanaryUnreached")
    ):
        raise OSError(errno.EPERM, "namespace_attempt_failed; inspect root receipt")
    return {
        "kernelEnforced": True,
        "namespace": guest_namespace,
        "phases": phases,
        "joins": record["waits"],
        "pidfdReaped": record["pidfdReaped"],
        "protocol": result["protocol"],
        "originalActivation": False,
        "productionAdmission": False,
    }


def cleanup_receipt(args):
    root_required(args)
    path, identity = receipt_path(args)
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        info = os.fstat(fd)
        if (
            not stat.S_ISREG(info.st_mode)
            or info.st_uid != 0
            or stat.S_IMODE(info.st_mode) != 0o600
            or info.st_nlink != 1
            or info.st_size > 65536
        ):
            raise OSError(errno.EPERM, "receipt_identity")
        with os.fdopen(fd, "rb", closefd=False) as stream:
            record = json.loads(stream.read(65537))
    finally:
        os.close(fd)
    if (
        record["directory"] != identity
        or record["sourceRecord"] != args.record_sha256
        or record["uid"] != args.uid
    ):
        raise OSError(errno.EPERM, "receipt_source_identity")
    if record["receipt"] != {"device": info.st_dev, "inode": info.st_ino}:
        raise OSError(errno.EPERM, "receipt_replaced")
    # This independent step never converts hard-monitor failure into success.
    # It refuses live/reused PIDs, leaving whole-job disposal as the owner gate.
    for expected in (record["monitor"], *record["children"]):
        try:
            actual = process(expected["pid"])
        except FileNotFoundError:
            continue
        if (
            actual["start"] != expected["start"]
            or actual["namespace"] != expected["namespace"]
        ):
            raise OSError(errno.EPERM, "cleanup_PID_identity_changed")
        raise OSError(errno.EBUSY, "cleanup_live_or_unreaped_process")
    if not record["settled"]:
        raise OSError(errno.ECHILD, "independent_full_tree_join_unobserved")
    current = path.lstat()
    if (current.st_dev, current.st_ino) != (info.st_dev, info.st_ino):
        raise OSError(errno.EPERM, "receipt_changed_before_unlink")
    os.unlink(path)
    return {
        "settled": True,
        "record": record,
        "originalActivation": False,
        "authorityConsumedByAttempt": True,
    }
