"""Inactive Linux child policy for the exact private combined fake proposal.

Tests inspect BPF and ordered exec state without loading filters or namespaces.
The proposed root tracer checks actual committed exec images before user entry.
An execveat FD/flag scalar alone cannot constrain an absolute pathname. No
filter, namespace or tracing action is activated by ordinary selectors.
"""

import ast
import ctypes
import errno
import hashlib
import os
from pathlib import Path
import platform
import resource
import signal

FD_LIMIT = 64
DENY = 0x00050000 | errno.EPERM
ALLOW = 0x7FFF0000


def instructions(role, pid, blocked):
    """Scalar syscall restrictions; committed-image entry is gated by ptrace."""
    if role not in {"driver", "native"} or pid <= 0:
        raise ValueError("policy_role_or_pid")
    rows, labels = [], {}

    def label(name):
        labels[name] = len(rows)

    def load(offset):
        rows.append((0x20, None, None, offset))

    def jump(op, value, yes, no):
        rows.append((op, yes, no, value))

    def ret(value):
        rows.append((0x06, None, None, value))

    load(4)
    jump(0x15, 0xC000003E, "nr", "deny")
    label("nr")
    load(0)
    jump(0x45, 0x40000000, "deny", "blocked0")
    numbers = [x for x in blocked if not (role == "driver" and x in (57, 58))]
    for index, nr in enumerate(numbers):
        label("blocked" + str(index))
        jump(0x15, nr, "deny", "blocked" + str(index + 1))
    label("blocked" + str(len(numbers)))
    jump(0x15, 435, "enosys", "check_clone")
    label("check_clone")
    jump(0x15, 56, "clone", "check_kill")
    label("check_kill")
    jump(0x15, 62, "kill", "check_tkill")
    label("check_tkill")
    jump(0x15, 200, "kill", "check_tgkill")
    label("check_tgkill")
    jump(0x15, 234, "kill", "allow")
    label("clone")
    load(16)
    # All namespaces and CLONE_UNTRACED are forbidden. Every task remains
    # enrolled in the inherited cgroup, PID namespace and kernel tracing.
    jump(0x45, 0x7E820080, "deny", "clone_kind")
    label("clone_kind")
    if role == "native":
        jump(0x45, 0x10000, "allow", "deny")
    else:
        ret(ALLOW)
    label("kill")
    load(16)
    if role == "native":
        jump(0x15, pid, "allow", "deny")
    else:
        jump(0x15, 0, "deny", "kill_sign")
        label("kill_sign")
        jump(0x45, 0x80000000, "deny", "allow")
    label("enosys")
    ret(0x00050000 | errno.ENOSYS)
    label("deny")
    ret(DENY)
    label("allow")
    ret(ALLOW)
    result = []
    for index, (op, yes, no, value) in enumerate(rows):
        jt = 0 if yes is None else labels[yes] - index - 1
        jf = 0 if no is None else labels[no] - index - 1
        if not 0 <= jt <= 255 or not 0 <= jf <= 255:
            raise ValueError("BPF_branch")
        result.append((op, jt, jf, value))
    return result


def load_entry(path, expected):
    source = Path(path).read_bytes()
    if hashlib.sha256(source).hexdigest() != expected:
        raise OSError(errno.EPERM, "entry_source_changed")
    namespace = {"__name__": "private_policy_entry", "__file__": str(path)}
    exec(compile(source, str(path), "exec"), namespace)
    tree = ast.parse(source)
    func = next(
        n
        for n in tree.body
        if isinstance(n, ast.FunctionDef) and n.name == "restrict_syscalls"
    )
    blocked = next(
        ast.literal_eval(n.value)
        for n in func.body
        if isinstance(n, ast.Assign)
        and any(isinstance(t, ast.Name) and t.id == "blocked" for t in n.targets)
    )
    return namespace, blocked


def filesystem(config, entry, role):
    checked = entry["checked"]
    libc = entry["LIBC"]
    abi = entry["landlock_abi"]()
    if abi < 3:
        raise OSError(errno.ENOSYS, "Landlock_ABI3_required")
    checked(libc.prctl(38, 1, 0, 0, 0), "NO_NEW_PRIVILEGES")
    ruleset = entry["Ruleset"](entry["FS_HANDLED"])
    fd = checked(
        libc.syscall(444, ctypes.byref(ruleset), ctypes.sizeof(ruleset), 0),
        "LANDLOCK_CREATE",
    )
    read = entry["FS_READ"]
    grants = [
        (config["root"], read),
        (config["canonicalDirectory"], read),
        (config["writable"], entry["FS_WRITE"] | (1 << 2)),
        ("/dev/null", (1 << 1) | (1 << 2)),
    ]
    grants += [(name, 1 << 2) for name in config["readFiles"]]
    grants += [(name, read) for name in config["runtimeRoots"]]
    exec_files = config["execFiles"] if role == "driver" else config["nativeExecFiles"]
    grants += [(name, entry["FS_EXECUTE"] | (1 << 2)) for name in exec_files]
    try:
        for name, access in grants:
            candidate = Path(name)
            if candidate.resolve(strict=True) != candidate.absolute():
                raise OSError(errno.EPERM, "policy_path_symlink")
            child = os.open(candidate, os.O_PATH | os.O_CLOEXEC | os.O_NOFOLLOW)
            try:
                if not candidate.is_dir():
                    access &= ~(1 << 3)
                rule = entry["PathRule"](access, child)
                checked(libc.syscall(445, fd, 1, ctypes.byref(rule), 0), "LANDLOCK_ADD")
            finally:
                os.close(child)
        checked(libc.syscall(446, fd, 0), "LANDLOCK_RESTRICT")
    finally:
        os.close(fd)
    return abi


def install(config, role):
    if (
        platform.system() != "Linux"
        or platform.machine() != "x86_64"
        or os.geteuid() == 0
    ):
        raise OSError(errno.EPERM, "nonroot_Linux_x64_policy_required")
    if role not in {"driver", "native"}:
        raise OSError(errno.EPERM, "kernel_policy_role")
    if os.getuid() != config["uid"] or os.getgid() != config["gid"]:
        raise OSError(errno.EPERM, "kernel_membership_or_credentials")
    if role == "driver":
        info = Path("/proc/self/status").read_text().splitlines()
        caps = [
            row.split(":", 1)[1].strip()
            for row in info
            if row.startswith(("CapEff:", "CapPrm:", "CapAmb:", "CapInh:", "CapBnd:"))
        ]
        if len(caps) != 5 or any(int(x, 16) for x in caps):
            raise OSError(errno.EPERM, "capability_regain_path")
        tracer = next(
            int(row.split(":", 1)[1]) for row in info if row.startswith("TracerPid:")
        )
        if tracer != config["tracerHostPid"] or tracer <= 0:
            raise OSError(errno.EPERM, "trusted_kernel_tracer_missing")
        net = os.stat("/proc/self/ns/net")
        pidns = os.stat("/proc/self/ns/pid")
        if (
            {"device": net.st_dev, "inode": net.st_ino} != config["namespace"]
            or {"device": pidns.st_dev, "inode": pidns.st_ino} != config["pidNamespace"]
            or Path("/proc/self/cgroup").read_text() != config["membership"]
        ):
            raise OSError(errno.EPERM, "kernel_membership_or_credentials")
    # Native entry inherits the driver's Landlock/seccomp restriction, which
    # deliberately denies procfs. Its actual namespace/credentials/tracer are
    # independently checked by root from the authenticated live socket owner;
    # it adds restrictions below without granting itself procfs access.
    entry, blocked = load_entry(config["entrySource"], config["entrySha256"])
    # Retain the proved filesystem/socket mechanisms; extra file grants are
    # exact host-captured Python inputs, not a writable runtime search path.
    filesystem(config, entry, role)
    entry["restrict_tcp_sockets"]()
    libc = entry["LIBC"]
    entry["checked"](libc.prctl(1, signal.SIGKILL, 0, 0, 0), "PARENT_DEATH")
    if os.getppid() != config["parent"] or config["parent"] < 1:
        raise OSError(errno.ESRCH, "parent_changed_before_entry")
    resource.setrlimit(resource.RLIMIT_NOFILE, (FD_LIMIT, FD_LIMIT))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_CPU, (10, 10))
    resource.setrlimit(resource.RLIMIT_FSIZE, (1000000, 1000000))
    code = instructions(role, os.getpid(), blocked)
    array = (entry["Instruction"] * len(code))(
        *(entry["Instruction"](*row) for row in code)
    )
    program = entry["Program"](len(code), array)
    entry["checked"](libc.prctl(22, 2, ctypes.byref(program), 0, 0), "CHILD_SECCOMP")
    return entry
