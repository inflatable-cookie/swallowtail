"""Inactive exact-actor Linux exec-stop gate; tests supply owned event models.

A successful exec is checked at PTRACE_EVENT_EXEC before user-space return.
This checks the committed image, not mutable pathname pointers followed by a
seccomp CONTINUE. Models never establish actual kernel admission.
"""

import ctypes
import errno
import hashlib
import os
from pathlib import Path
import platform
import signal

SEIZE = 0x4206
INTERRUPT = 0x4207
GETEVENTMSG = 0x4201
CONT = 7
OPTIONS = 0x00100000 | 0x00000002 | 0x00000004 | 0x00000008 | 0x00000010 | 0x00000040
FORK, VFORK, CLONE, EXEC, EXIT, STOP = 1, 2, 3, 4, 6, 128
WALL = 0x40000000


class Refused(OSError):
    pass


class LinuxTrace:
    kernel_enforced = True

    def __init__(self):
        if (
            platform.system() != "Linux"
            or platform.machine() != "x86_64"
            or os.geteuid() != 0
        ):
            raise Refused(errno.EPERM, "root_Linux_x64_trace_required")
        self.libc = ctypes.CDLL(None, use_errno=True)
        self.libc.ptrace.restype = ctypes.c_long
        self.waits = []

    def call(self, operation, pid, data=0):
        result = self.libc.ptrace(operation, pid, None, ctypes.c_void_p(data))
        if result < 0:
            number = ctypes.get_errno()
            raise Refused(number, "ptrace_operation_" + str(operation))
        return result

    def seize(self, pid):
        self.call(SEIZE, pid, OPTIONS)
        self.call(INTERRUPT, pid)

    def resume(self, pid, sig=0):
        self.call(CONT, pid, sig)

    def event_message(self, pid):
        value = ctypes.c_ulonglong()
        self.call(GETEVENTMSG, pid, ctypes.addressof(value))
        return value.value

    def image(self, pid):
        return observe_image(pid)

    def wait(self):
        try:
            pid, status = os.waitpid(-1, os.WNOHANG | WALL)
        except ChildProcessError:
            return None
        if pid:
            row = {
                "actor": os.getpid(),
                "pid": pid,
                "status": status,
                "source": "waitpid.__WALL",
            }
            self.waits.append(row)
            return row
        return None


def image_binding(command, environment, cwd):
    file = Path(command[0]).resolve(strict=True)
    info = file.stat()
    return {
        "path": str(file),
        "device": info.st_dev,
        "inode": info.st_ino,
        "bytes": info.st_size,
        "sha256": hashlib.sha256(file.read_bytes()).hexdigest(),
        "argv": command,
        "environment": environment,
        "cwd": str(cwd),
    }


def observe_image(pid):
    file = Path(f"/proc/{pid}/exe")
    info = file.stat()
    args = Path(f"/proc/{pid}/cmdline").read_bytes().rstrip(b"\0").split(b"\0")
    environment = Path(f"/proc/{pid}/environ").read_bytes().rstrip(b"\0").split(b"\0")
    return {
        "path": str(file.resolve(strict=True)),
        "device": info.st_dev,
        "inode": info.st_ino,
        "bytes": info.st_size,
        "sha256": hashlib.sha256(file.read_bytes()).hexdigest(),
        "argv": [x.decode() for x in args],
        "environment": dict(x.decode().split("=", 1) for x in environment if x),
        "cwd": str(Path(f"/proc/{pid}/cwd").resolve(strict=True)),
    }


class ExecGate:
    """Two exact payload actors and their threads; no arbitrary process API."""

    def __init__(self, io, snapshot, membership, images, pin=None, before_release=None):
        self.io = io
        self.snapshot = snapshot
        self.membership = membership
        self.images = images
        self.pin = pin or (lambda pid: None)
        self.before_release = before_release or (lambda: None)
        self.tasks = {}
        self.roles = {}
        self.events = []
        self.native_armed = False
        self.seen_native = False
        self.pending_stops = set()

    def log(self, kind, **values):
        self.events.append(
            {
                "seq": len(self.events),
                "phase": "kernel-supervision",
                "kind": kind,
                "actor": os.getpid(),
                **values,
            }
        )

    def driver(self, pid):
        if self.roles:
            raise Refused(errno.EPERM, "duplicate_driver_trace")
        info = self.snapshot(pid)
        self.membership(info)
        self.pin(pid)
        self.io.seize(pid)
        self.tasks[pid] = {
            "role": "driver",
            "state": "driver-entry",
            "tgid": pid,
            "identity": info,
        }
        self.roles["driver"] = pid
        self.log("driver_trace_requested", pid=pid)

    def arm_native(self, pid):
        task = self.tasks.get(pid)
        if (
            self.native_armed
            or task is None
            or task["state"] != "native-entry"
            or self.roles.get("native") != pid
        ):
            raise Refused(errno.EPERM, "missing_duplicate_or_early_native_approval")
        self.membership(self.snapshot(pid))
        self.native_armed = True
        self.log("native_exec_armed", pid=pid)

    def process(self, row):
        pid, status = row["pid"], row["status"]
        if os.WIFEXITED(status) or os.WIFSIGNALED(status):
            if pid not in self.tasks:
                return False  # The monitor separately owns direct/adoptee waits.
            self.tasks[pid]["terminal"] = row
            self.log("trace_terminal_wait", **row)
            return True
        if not os.WIFSTOPPED(status):
            raise Refused(errno.EPERM, "unrecognized_wait_status")
        event = status >> 16
        if pid not in self.tasks:
            if event != STOP:
                raise Refused(errno.EPERM, "unregistered_trace_event")
            # Kernel child stop may precede its parent's creation event. Hold it;
            # a deadline rejects a child with no actual parent event.
            self.pending_stops.add(pid)
            return True
        task = self.tasks[pid]
        info = self.snapshot(pid)
        self.membership(info)
        if event in (FORK, VFORK, CLONE):
            child = self.io.event_message(pid)
            new = self.snapshot(child)
            self.membership(new)
            if len(self.tasks) >= 32:
                raise Refused(errno.EAGAIN, "kernel_task_budget")
            if new["tgid"] == task["tgid"]:
                role, state = task["role"], "thread"
            else:
                if (
                    task["role"] != "driver"
                    or task["state"] not in {"driver", "thread"}
                    or self.seen_native
                ):
                    raise Refused(errno.EPERM, "extra_descendant_before_entry")
                if new["parent"] != self.roles["driver"]:
                    raise Refused(errno.EPERM, "native_kernel_parent")
                role, state = "native", "native-candidate"
                self.seen_native = True
                self.roles["native"] = child
            self.tasks[child] = {
                "role": role,
                "state": state,
                "tgid": new["tgid"],
                "identity": new,
            }
            if child == new["tgid"]:
                # Pin while kernel-stopped, before either child or parent can
                # resume. Disappearance after release cannot replace a pidfd.
                self.pin(child)
            self.log(
                "kernel_child_registered", pid=child, parent=pid, role=role, state=state
            )
            if child in self.pending_stops:
                self.pending_stops.remove(child)
                self.io.resume(child)
            self.io.resume(pid)
            return True
        if event == EXEC:
            former = self.io.event_message(pid)
            if former != pid or task["state"] == "thread":
                raise Refused(errno.EPERM, "nonleader_exec_before_entry")
            expected = None
            if task["state"] == "driver-entry":
                expected, next_state = self.images["driver"], "driver"
            elif task["state"] == "native-candidate":
                expected, next_state = self.images["entry"], "native-entry"
            elif task["state"] == "native-entry" and self.native_armed:
                expected, next_state = self.images["native"], "native"
            if expected is None or self.io.image(pid) != expected:
                raise Refused(errno.EPERM, "unapproved_committed_exec_before_entry")
            self.before_release()
            task["state"] = next_state
            self.log(
                "exec_image_released",
                pid=pid,
                role=task["role"],
                state=next_state,
                image=expected["sha256"],
            )
            self.io.resume(pid)
            return True
        if event == EXIT:
            self.log("trace_exit_stop", pid=pid)
            self.io.resume(pid)
            return True  # An EXIT stop is not a terminal wait or a parent reap.
        if event == STOP:
            self.io.resume(pid)
            return True
        # Deliver actual ordinary signals; do not turn a signal into an exec
        # approval or suppress fatal errors. Kernel SIGTRAP without an event is
        # not one of the admitted protocol events.
        sig = os.WSTOPSIG(status)
        if sig == signal.SIGTRAP:
            raise Refused(errno.EPERM, "unknown_trace_trap")
        self.io.resume(pid, sig)
        return True

    def complete(self):
        return (
            bool(self.tasks)
            and not self.pending_stops
            and all("terminal" in task for task in self.tasks.values())
        )
