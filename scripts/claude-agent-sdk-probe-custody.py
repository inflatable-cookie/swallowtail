"""Private trusted-host custody for the SDK fake probe, decision89db0f25 v2.

The trusted host owns this object and its descriptors through launch. This is
observed byte/identity custody, not hostile-same-user verified execution.
"""

import hashlib
import json
import os
from pathlib import Path
import stat
import subprocess
import sys

DECISION = "89db0f25-dc94-4feb-b447-8b9fabe52561"
STARTUP = ("--no-addons", "--openssl-config=/dev/null")


class Refused(OSError):
    pass


def digest(data):
    return hashlib.sha256(data).hexdigest()


def identity(info):
    return (
        info.st_dev,
        info.st_ino,
        info.st_uid,
        info.st_gid,
        info.st_mode,
        info.st_size,
        info.st_mtime_ns,
        info.st_ctime_ns,
    )


def python_inputs():
    paths = {Path(sys.executable).resolve(strict=True)}
    for module in tuple(sys.modules.values()):
        name = getattr(module, "__file__", None)
        if name and Path(name).is_file():
            paths.add(Path(name).resolve(strict=True))
    if sys.platform == "linux":
        for row in Path("/proc/self/maps").read_text().splitlines():
            fields = row.split(maxsplit=5)
            if len(fields) == 6 and fields[5].startswith("/"):
                paths.add(Path(fields[5]).resolve(strict=True))
    return paths


def runtime_inventory(node, expected):
    """One credential-free, non-root host-runtime observation, before payloads."""
    node = Path(node).resolve(strict=True)
    if os.geteuid() == 0 or digest(node.read_bytes()) != expected:
        raise Refused("nonroot_exact_runtime_preflight_required")
    command = [
        str(node),
        *STARTUP,
        "-e",
        "console.log(JSON.stringify({node:process.execPath,version:process.version,"
        "libraries:process.report.getReport().sharedObjects}))",
    ]
    result = subprocess.run(
        command,
        env={
            "PATH": "/usr/bin:/bin",
            "HOME": os.environ["TMPDIR"],
            "TMPDIR": os.environ["TMPDIR"],
            "LANG": "C",
            "LC_ALL": "C",
        },
        capture_output=True,
        timeout=5,
        check=True,
    )
    if len(result.stdout) > 65536 or result.stderr:
        raise Refused("runtime_observation_limit")
    report = json.loads(result.stdout)
    if Path(report["node"]).resolve(strict=True) != node:
        raise Refused("runtime_observation_identity")
    unresolved = []
    libraries = set()
    for name in report["libraries"]:
        try:
            libraries.add(str(Path(name).resolve(strict=True)))
        except FileNotFoundError:
            if sys.platform != "darwin" or not name.startswith(
                ("/usr/lib/", "/System/Library/")
            ):
                raise
            unresolved.append(name)
    libraries = sorted(libraries)
    return {
        "node": str(node),
        "nodeSha256": expected,
        "version": report["version"],
        "unresolvedSystemLibraries": sorted(unresolved),
        "libraries": {name: digest(Path(name).read_bytes()) for name in libraries},
        "startup": list(STARTUP),
        "decision": DECISION,
        "ownerUid": os.getuid(),
    }


class Lease:
    """Host-created live custody, no caller boolean or serializable attestation."""

    def __init__(self, runtime, node, expected, policy):
        self.owner = os.getpid()
        self.owners = {0, runtime["ownerUid"]}
        self.revoked = False
        self.closed = False
        self.files = {}
        self.commands = None
        self.launched = 0
        self.unresolved = runtime.get("unresolvedSystemLibraries", [])
        self.policy = digest(json.dumps(policy, sort_keys=True).encode())
        if (
            runtime.get("decision") != DECISION
            or runtime.get("startup") != list(STARTUP)
            or runtime.get("node") != str(Path(node).resolve(strict=True))
            or runtime.get("nodeSha256") != expected
            or not isinstance(runtime.get("libraries"), dict)
            or (
                sys.platform == "linux"
                and (not runtime["libraries"] or self.unresolved)
            )
        ):
            raise Refused("runtime_custody_binding_missing")
        try:
            self.bind(node, expected)
            for name, sha in runtime["libraries"].items():
                self.bind(name, sha)
            if sys.platform == "linux" and Path("/etc/ld.so.preload").exists():
                raise Refused("ambient_loader_preload_refused")
            if Path("/etc/ld.so.cache").is_file():
                self.bind(Path("/etc/ld.so.cache").resolve(strict=True))
            for name in python_inputs():
                self.bind(name)
        except BaseException:
            self.close()
            raise

    @property
    def descriptors(self):
        return {row[0] for row in self.files.values()}

    def bind(self, name, expected=None):
        self.check()
        path = Path(name).absolute()
        if path.resolve(strict=True) != path:
            raise Refused("custody_symlink")
        if str(path) in self.files:
            row = self.files[str(path)]
            if expected is not None and row[2] != expected:
                raise Refused("custody_expected_bytes_changed")
            return
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            info = os.fstat(fd)
            if (
                not stat.S_ISREG(info.st_mode)
                or info.st_mode & 0o022
                or info.st_uid not in self.owners
            ):
                raise Refused("custody_unowned_or_writable_input")
            value = hashlib.sha256()
            while True:
                data = os.read(fd, 1024 * 1024)
                if not data:
                    break
                value.update(data)
            sha = value.hexdigest()
            if expected is not None and sha != expected:
                raise Refused("custody_bytes_changed")
            if identity(info) != identity(os.fstat(fd)):
                raise Refused("custody_changed_during_capture")
            self.files[str(path)] = (fd, identity(info), sha)
        except BaseException:
            os.close(fd)
            raise

    def check(self):
        if self.closed or self.revoked or self.owner != os.getpid():
            raise Refused("custody_absent_revoked_or_wrong_host")
        for name, (fd, expected, _) in self.files.items():
            if (
                identity(os.fstat(fd)) != expected
                or identity(Path(name).lstat()) != expected
            ):
                raise Refused("custody_input_identity_changed")

    def adopt_fork(self, parent):
        if self.owner != parent or os.getppid() != parent:
            raise Refused("custody_unowned_fork")
        self.owner = os.getpid()
        self.check()

    def configure(self, commands, environments):
        self.check()
        if self.commands is not None or len(commands) != 2:
            raise Refused("custody_fixed_topology")
        allowed = {
            "PATH",
            "HOME",
            "TMPDIR",
            "TMP",
            "TEMP",
            "LANG",
            "LC_ALL",
            "CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE",
            "CLAUDE_AGENT_SDK_SIDECAR_NATIVE_BINARY",
            "CLAUDE_AGENT_SDK_SIDECAR_MANIFEST",
        }
        for command, env in zip(commands, environments):
            if (
                command[1:3] != list(STARTUP)
                or command[0] not in self.files
                or not set(env) <= allowed
                or env.get("PATH") != "/usr/bin:/bin"
            ):
                raise Refused("custody_startup_policy")
        self.commands = [
            (tuple(command), dict(env)) for command, env in zip(commands, environments)
        ]

    def permit(self, command, environment):
        self.check()
        if (
            self.commands is None
            or self.launched >= 2
            or (tuple(command), environment) != self.commands[self.launched]
        ):
            raise Refused("custody_launch_mismatch")
        self.launched += 1

    def revoke(self):
        self.revoked = True

    def close(self):
        if not self.closed:
            for fd, _, _ in self.files.values():
                os.close(fd)
            self.closed = True

    def observation(self):
        self.check()
        return {
            "decision": DECISION,
            "hostPid": self.owner,
            "policySha256": self.policy,
            "inputs": {name: sha for name, (_, _, sha) in self.files.items()},
            "launches": self.launched,
            "hostileSameUserReplacementCovered": False,
            "verifiedExecutionPrimitive": False,
            "unresolvedSystemLibraries": self.unresolved,
            "runtimeCustodyComplete": not self.unresolved,
        }
