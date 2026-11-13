#!/usr/bin/env python3
"""Linux x64 fake-probe entry: Landlock plus a fixed-topology seccomp policy.

No privileged helper, namespace setup, vendor entry or installation. The
supervisor supplies connected endpoint/stdio descriptors before this boundary.
"""

import ctypes
import errno
import json
import os
from pathlib import Path
import platform
import resource
import sys

LIBC = ctypes.CDLL(None, use_errno=True)
LIBC.syscall.restype = ctypes.c_long
FS_READ = (1 << 2) | (1 << 3)
FS_EXECUTE = 1
FS_WRITE = (1 << 1) | (1 << 14)
FS_HANDLED = (1 << 15) - 1  # Through TRUNCATE: require Landlock ABI >= 3.


class Ruleset(ctypes.Structure):
    _fields_ = [("handled_access_fs", ctypes.c_uint64)]


class PathRule(ctypes.Structure):
    _pack_ = 1
    _fields_ = [("allowed_access", ctypes.c_uint64), ("parent_fd", ctypes.c_int32)]


class Instruction(ctypes.Structure):
    _fields_ = [
        ("code", ctypes.c_ushort),
        ("jt", ctypes.c_ubyte),
        ("jf", ctypes.c_ubyte),
        ("k", ctypes.c_uint32),
    ]


class Program(ctypes.Structure):
    _fields_ = [("length", ctypes.c_ushort), ("filter", ctypes.POINTER(Instruction))]


def checked(result, operation):
    if result < 0:
        number = ctypes.get_errno()
        raise OSError(number, f"{operation}: {os.strerror(number)}")
    return result


def landlock_abi():
    if platform.system() != "Linux" or platform.machine() != "x86_64":
        raise OSError(errno.ENOSYS, "Linux x64 is required")
    return checked(LIBC.syscall(444, 0, 0, 1), "landlock_query_abi")


def restrict_files(config):
    status = Path("/proc/self/status").read_text().splitlines()
    capabilities = {
        line.split(":", 1)[0]: line.split(":", 1)[1].strip()
        for line in status
        if line.startswith(("CapEff:", "CapPrm:", "CapAmb:"))
    }
    if os.geteuid() == 0 or any(int(value, 16) for value in capabilities.values()):
        raise OSError(errno.EPERM, "unprivileged_linux_host_required")
    abi = landlock_abi()
    if abi < 3:
        raise OSError(errno.ENOSYS, f"Landlock ABI {abi}; ABI 3 with TRUNCATE required")
    checked(LIBC.prctl(38, 1, 0, 0, 0), "PR_SET_NO_NEW_PRIVS")
    ruleset = Ruleset(FS_HANDLED)
    descriptor = checked(
        LIBC.syscall(444, ctypes.byref(ruleset), ctypes.sizeof(ruleset), 0),
        "landlock_create_ruleset",
    )
    try:
        grants = [
            (config["root"], FS_READ),
            (config["node"], (1 << 2) | FS_EXECUTE),
            (config["writable"], (1 << 2) | FS_WRITE),
            ("/dev/null", (1 << 2) | (1 << 1)),
        ]
        grants += [
            (str(Path("/lib64/ld-linux-x86-64.so.2").resolve()), (1 << 2) | FS_EXECUTE)
        ]
        grants += [(name, FS_READ) for name in config["runtimeRoots"]]
        grants += [(name, 1 << 2) for name in config["readFiles"]]
        for name, access in grants:
            file = os.open(name, os.O_PATH | os.O_CLOEXEC)
            try:
                # READ_DIR is invalid on a regular file.
                if not Path(name).is_dir():
                    access &= ~(1 << 3)
                rule = PathRule(access, file)
                checked(
                    LIBC.syscall(445, descriptor, 1, ctypes.byref(rule), 0),
                    "landlock_add_rule",
                )
            finally:
                os.close(file)
        checked(LIBC.syscall(446, descriptor, 0), "landlock_restrict_self")
    finally:
        os.close(descriptor)
    return abi


def restrict_syscalls():
    # Classic BPF seccomp_data offsets: nr=0, arch=4, args[0]=16.
    load = lambda offset: Instruction(0x20, 0, 0, offset)
    equal = lambda value, yes, no: Instruction(0x15, yes, no, value)
    result = lambda value: Instruction(0x06, 0, 0, value)
    denied = 0x00050000 | errno.EPERM
    instructions = [
        load(4),
        equal(0xC000003E, 1, 0),
        result(0x80000000),
        load(0),
        Instruction(0x45, 0, 1, 0x40000000),
        result(denied),
    ]
    # Network cannot escape through io_uring, a new socket, namespaces or a
    # foreign process. No inherited socket except the owned connected endpoint.
    blocked = (
        29,
        30,
        31,
        41,
        42,
        43,
        49,
        50,
        53,
        57,
        58,
        64,
        65,
        66,
        67,
        68,
        69,
        70,
        71,
        101,
        126,
        155,
        165,
        166,
        200,
        248,
        249,
        250,
        272,
        288,
        298,
        304,
        308,
        310,
        311,
        319,
        321,
        323,
        425,
        426,
        427,
        438,
        440,
        90,
        91,
        92,
        93,
        94,
        129,
        132,
        141,
        142,
        144,
        156,
        188,
        189,
        190,
        197,
        198,
        199,
        203,
        235,
        251,
        260,
        261,
        268,
        280,
        297,
        314,
        424,
        452,
    )
    for number in blocked:
        instructions += [equal(number, 0, 1), result(denied)]
    # glibc falls back to clone when clone3 reports ENOSYS. Permit only thread
    # clones, which share the existing process and inherit both restrictions.
    instructions += [
        equal(435, 0, 1),
        result(0x00050000 | errno.ENOSYS),
        equal(56, 0, 4),
        load(16),
        Instruction(0x45, 1, 0, 0x10000),
        result(denied),
        result(0x7FFF0000),
    ]
    for number in (62, 234):  # kill/tgkill: positive self PID only, never a group.
        instructions += [
            equal(number, 0, 4),
            load(16),
            equal(os.getpid(), 1, 0),
            result(denied),
            result(0x7FFF0000),
        ]
    for number in (302,):  # prlimit64: zero denotes this process.
        instructions += [
            equal(number, 0, 5),
            load(16),
            equal(os.getpid(), 2, 0),
            equal(0, 1, 0),
            result(denied),
            result(0x7FFF0000),
        ]
    instructions += [result(0x7FFF0000)]
    array = (Instruction * len(instructions))(*instructions)
    program = Program(len(instructions), array)
    checked(LIBC.prctl(22, 2, ctypes.byref(program), 0, 0), "PR_SET_SECCOMP")


def main():
    config = json.loads(Path(sys.argv[1]).read_text())
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_FSIZE, (1_000_000, 1_000_000))
    resource.setrlimit(resource.RLIMIT_NOFILE, (64, 64))
    resource.setrlimit(resource.RLIMIT_CPU, (10, 10))
    abi = restrict_files(config)
    restrict_syscalls()
    print(
        json.dumps(
            {
                "linuxBoundary": {
                    "landlockAbi": abi,
                    "seccomp": True,
                    "network": "inherited-connected-fd-only",
                }
            }
        ),
        file=sys.stderr,
        flush=True,
    )
    os.execve(config["node"], sys.argv[2:], dict(os.environ))


if __name__ == "__main__":
    try:
        main()
    except OSError as error:
        print(
            json.dumps({"linuxBoundaryRefused": str(error), "errno": error.errno}),
            file=sys.stderr,
            flush=True,
        )
        raise SystemExit(78)
