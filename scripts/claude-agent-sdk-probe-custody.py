"""Private trusted-host custody for the SDK fake probe, decision89db0f25 v2.

The trusted host owns this object and its descriptors through launch. This is
observed byte/identity custody, not hostile-same-user verified execution.
"""

import hashlib
import json
import os
import platform
from pathlib import Path
import stat
import subprocess
import sys
import re
import selectors
import time

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
        cache = getattr(module, "__cached__", None)
        if cache and Path(cache).is_file():
            paths.add(Path(cache).resolve(strict=True))
    if sys.platform == "linux":
        for row in Path("/proc/self/maps").read_text().splitlines():
            fields = row.split(maxsplit=5)
            if len(fields) == 6 and fields[5].startswith("/"):
                paths.add(Path(fields[5]).resolve(strict=True))
    return paths


def directory_identity(info):
    return (info.st_dev, info.st_ino, info.st_uid, info.st_gid, info.st_mode)


class Inputs:
    """Live host-owned input descriptors. No runtime execution or attestation."""

    def __init__(self, *, reserve_pipes=True):
        if os.geteuid() == 0:
            raise Refused("ordinary_uid_custody_required")
        self.owner = os.getpid()
        self.owners = {0, os.getuid()}
        self.revoked = self.closed = False
        self.files, self.directories, self.reserved = {}, {}, {}
        self.active_child = None
        self.deadline = time.monotonic() + 10
        self.commands, self.launched = None, 0
        try:
            for index in range(5 if reserve_pipes else 0):
                pair = os.pipe()
                self.reserved[index] = pair
                if max(pair) >= 64:
                    raise Refused("bounded_stdio_descriptors_unavailable")
        except BaseException:
            self.close()
            raise

    @property
    def descriptors(self):
        return {row[0] for row in self.files.values()} | {
            fd for pair in self.reserved.values() for fd in pair
        }

    def take_pipe(self, index):
        self.check()
        if index not in self.reserved:
            raise Refused("owned_pipe_reservation_missing")
        return self.reserved.pop(index)

    def release_reservations(self):
        for pair in self.reserved.values():
            for fd in pair:
                os.close(fd)
        self.reserved.clear()

    def bind(self, name, expected=None):
        self.check()
        path = Path(name).absolute()
        if path.resolve(strict=True) != path:
            raise Refused("custody_symlink")
        self.bind_ancestors(path)
        if str(path) in self.files:
            if expected is not None and self.files[str(path)][2] != expected:
                raise Refused("custody_expected_bytes_changed")
            return
        if len(self.files) >= 512:
            raise Refused("custody_input_bound")
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            info = os.fstat(fd)
            if (
                not stat.S_ISREG(info.st_mode)
                or info.st_mode & 0o6022
                or info.st_uid not in self.owners
            ):
                raise Refused("custody_unowned_or_writable_input")
            if info.st_size > 268435456:
                raise Refused("custody_file_bound")
            value = hashlib.sha256()
            total = 0
            while True:
                if time.monotonic() > self.deadline:
                    raise Refused("custody_capture_deadline")
                data = os.read(fd, 65536)
                if not data:
                    break
                total += len(data)
                if total > 268435456:
                    raise Refused("custody_file_bound")
                value.update(data)
            sha = value.hexdigest()
            if expected is not None and (
                not re.fullmatch(r"[a-f0-9]{64}", expected) or sha != expected
            ):
                raise Refused("custody_bytes_changed")
            if identity(info) != identity(os.fstat(fd)) or identity(
                path.lstat()
            ) != identity(info):
                raise Refused("custody_changed_during_capture")
            self.files[str(path)] = (fd, identity(info), sha)
        except BaseException:
            os.close(fd)
            raise

    def bind_ancestors(self, path):
        for parent in reversed(path.parents):
            name = str(parent)
            if name in self.directories:
                continue
            if len(self.directories) >= 256:
                raise Refused("custody_ancestor_bound")
            fd = os.open(parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
            try:
                info = os.fstat(fd)
                if (
                    not stat.S_ISDIR(info.st_mode)
                    or info.st_mode & 0o022
                    or info.st_uid not in self.owners
                ):
                    raise Refused("custody_unsafe_ancestor")
                self.directories[name] = (fd, directory_identity(info))
            except BaseException:
                os.close(fd)
                raise

    def check(self):
        if self.closed or self.revoked or self.owner != os.getpid():
            raise Refused("custody_absent_revoked_or_wrong_host")
        for name, (fd, expected) in self.directories.items():
            if (
                directory_identity(os.fstat(fd)) != expected
                or directory_identity(Path(name).lstat()) != expected
            ):
                raise Refused("custody_ancestor_identity_changed")
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
        for name in python_inputs():
            self.bind(name)
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
        if self.active_child is not None:
            error = Refused("runtime_observer_join_unknown; custody retained")
            error.capture = self
            raise error
        if not self.closed:
            self.release_reservations()
            for fd, _, _ in self.files.values():
                os.close(fd)
            for fd, _ in self.directories.values():
                os.close(fd)
            self.closed = True


class Bootstrap(Inputs):
    """Pinned inputs from an independently established trusted-host manifest.

    Supplying byte bindings is not evidence that the host's dependency set is
    complete. That exact loader/library closure is a separate host prerequisite.
    """

    def __init__(
        self,
        node,
        expected,
        *,
        version,
        libraries,
        loader_inputs,
        interpreters,
        supervisor_inputs,
    ):
        super().__init__()
        try:
            self.node, self.expected = str(Path(node).absolute()), expected
            self.bind(self.node, expected)
            self.version = version
            self.scratch = str(Path(os.environ["TMPDIR"]).absolute())
            scratch = Path(self.scratch)
            info = scratch.lstat()
            if (
                scratch.resolve() != scratch
                or info.st_uid != os.getuid()
                or stat.S_IMODE(info.st_mode) != 0o700
            ):
                raise Refused("bootstrap_private_scratch_required")
            self.bind_ancestors(scratch / ".observer")
            if not libraries or not isinstance(version, str):
                raise Refused("trusted_host_library_manifest_required")
            self.libraries = dict(libraries)
            for mapping in (libraries, loader_inputs, interpreters, supervisor_inputs):
                if not isinstance(mapping, dict):
                    raise Refused("bootstrap_manifest_shape")
                for name, sha in mapping.items():
                    if not Path(name).is_absolute() or not isinstance(sha, str):
                        raise Refused("bootstrap_manifest_shape")
                    self.bind(name, sha)
            if not {str(p) for p in python_inputs()} <= set(supervisor_inputs):
                raise Refused("bootstrap_supervisor_manifest_missing")
            header = os.pread(self.files[self.node][0], 256, 0)
            if header.startswith(b"#!"):
                words = header.splitlines()[0][2:].decode().split()
                if len(words) != 1 or words[0] not in interpreters:
                    raise Refused("bootstrap_interpreter_manifest_missing")
            if sys.platform == "linux":
                if Path("/etc/ld.so.preload").exists():
                    raise Refused("ambient_loader_preload_refused")
                cache = Path("/etc/ld.so.cache")
                if cache.exists() and str(cache) not in loader_inputs:
                    raise Refused("bootstrap_loader_manifest_missing")
            self.launch_binding = self.launch_inputs()
            self.check()
        except BaseException:
            self.close()
            raise

    def launch_inputs(self):
        return json.dumps(
            {
                "node": self.node,
                "expected": self.expected,
                "version": self.version,
                "libraries": self.libraries,
                "startup": list(STARTUP),
                "scratch": str(Path(os.environ["TMPDIR"]).absolute()),
            },
            sort_keys=True,
        )

    def check(self):
        super().check()
        if (
            hasattr(self, "launch_binding")
            and self.launch_inputs() != self.launch_binding
        ):
            raise Refused("bootstrap_launch_inputs_changed")


class RuntimeObservation(dict):
    """An observation with retained live custody; JSON alone is insufficient."""

    def __init__(self, values, capture):
        super().__init__(values)
        self.capture = capture

    def close(self):
        self.capture.close()


def runtime_inventory(node, expected, *, bootstrap=None):
    """Observe only after eligibility and independently supplied bootstrap inputs."""
    node = Path(node).absolute()
    if bootstrap is None:
        # Apply the same eligibility rule even when the later prerequisite is absent.
        initial = Inputs(reserve_pipes=False)
        try:
            initial.bind(node, expected)
        finally:
            initial.close()
        raise Refused("trusted_host_bootstrap_manifest_required_before_execution")
    if (
        not isinstance(bootstrap, Bootstrap)
        or bootstrap.node != str(node)
        or bootstrap.expected != expected
    ):
        raise Refused("runtime_bootstrap_binding_missing")
    child = None
    try:
        bootstrap.check()
        command = [
            str(node),
            *STARTUP,
            "-e",
            "console.log(JSON.stringify({node:process.execPath,version:process.version,"
            "libraries:process.report.getReport().sharedObjects}))",
        ]
        environment = {
            "PATH": "/usr/bin:/bin",
            "HOME": os.environ["TMPDIR"],
            "TMPDIR": os.environ["TMPDIR"],
            "LANG": "C",
            "LC_ALL": "C",
        }
        child = subprocess.Popen(
            command,
            env=environment,
            cwd=bootstrap.scratch,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        bootstrap.active_child = child
        stdout, stderr = bytearray(), bytearray()
        deadline = time.monotonic() + 5
        with selectors.DefaultSelector() as poll:
            poll.register(child.stdout, selectors.EVENT_READ, stdout)
            poll.register(child.stderr, selectors.EVENT_READ, stderr)
            while poll.get_map():
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    raise Refused("runtime_observer_deadline")
                events = poll.select(remaining)
                if not events:
                    raise Refused("runtime_observer_deadline")
                for key, _ in events:
                    data = os.read(key.fileobj.fileno(), 4096)
                    if not data:
                        poll.unregister(key.fileobj)
                    key.data.extend(data)
                    if len(stdout) + len(stderr) > 65536 or stderr:
                        raise Refused("runtime_observation_limit")
        if child.wait(timeout=max(0.001, deadline - time.monotonic())) != 0:
            raise Refused("runtime_observer_exit")
        bootstrap.active_child = None
        bootstrap.check()
        report = json.loads(stdout)
        if report["node"] != str(node) or report["version"] != bootstrap.version:
            raise Refused("runtime_observation_identity_or_version")
        libraries, kernel = set(), []
        for name in report["libraries"]:
            if sys.platform == "linux" and name == "linux-vdso.so.1":
                kernel.append({"name": name, "kernel": platform.release()})
                continue
            if not Path(name).is_absolute() or Path(name).resolve(strict=True) != Path(
                name
            ):
                raise Refused("unidentified_runtime_library")
            libraries.add(name)
        if libraries != set(bootstrap.libraries):
            raise Refused("runtime_library_manifest_mismatch")
        bootstrap.check()
        return RuntimeObservation(
            {
                "node": str(node),
                "nodeSha256": expected,
                "version": report["version"],
                "unresolvedSystemLibraries": [],
                "kernelMappings": kernel,
                "libraries": dict(bootstrap.libraries),
                "startup": list(STARTUP),
                "decision": DECISION,
                "ownerUid": os.getuid(),
                "bootstrapLibraryClosure": "trusted-host prerequisite; not independently attested by report",
            },
            bootstrap,
        )
    except BaseException as error:
        if child is not None and bootstrap.active_child is not None:
            try:
                if child.poll() is None:
                    child.kill()
                child.wait(timeout=2)
                bootstrap.active_child = None
            except BaseException:
                unknown = Refused("runtime_observer_join_unknown; custody retained")
                unknown.capture = bootstrap
                raise unknown from error
        bootstrap.close()
        raise
    finally:
        if child is not None:
            child.stdout.close()
            child.stderr.close()


class Lease(Inputs):
    """Adopt the observation's live capture through launch; never a JSON flag."""

    def __init__(self, runtime, node, expected, policy, *, reserve_pipes=True):
        if not isinstance(runtime, RuntimeObservation):
            raise Refused("runtime_live_preentry_custody_required")
        capture = runtime.capture
        try:
            capture.check()
        except BaseException:
            capture.close()
            raise
        self.__dict__ = capture.__dict__
        self.capture = capture
        self.kernel_mappings = runtime.get("kernelMappings", [])
        self.unresolved = runtime.get("unresolvedSystemLibraries", [])
        self.software_model = runtime.get("softwareBootstrapModel") is True
        self.policy = digest(json.dumps(policy, sort_keys=True).encode())
        try:
            if any(
                row != {"name": "linux-vdso.so.1", "kernel": platform.release()}
                for row in self.kernel_mappings
            ):
                raise Refused("kernel_runtime_mapping_changed")
            if (
                runtime.get("decision") != DECISION
                or runtime.get("startup") != list(STARTUP)
                or runtime.get("node") != str(Path(node).absolute())
                or runtime.get("nodeSha256") != expected
                or runtime.get("ownerUid") != os.getuid()
                or not isinstance(runtime.get("libraries"), dict)
                or self.unresolved
            ):
                raise Refused("runtime_custody_binding_missing")
            if isinstance(capture, Bootstrap) and (
                runtime.get("version") != capture.version
                or runtime.get("libraries") != capture.libraries
            ):
                raise Refused("runtime_observation_binding_changed")
            if not reserve_pipes:
                self.release_reservations()
            self.bind(node, expected)
            for name, sha in runtime["libraries"].items():
                self.bind(name, sha)
            for name in python_inputs():
                self.bind(name)
            self.check()
        except BaseException:
            self.close()
            raise

    def bind(self, name, expected=None):
        return self.capture.bind(name, expected)

    def check(self):
        return self.capture.check()

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
            "softwareBootstrapModel": self.software_model,
            "runtimeCustodyComplete": False,
            "bootstrapLibraryClosure": "independent trusted-host prerequisite remains required",
        }
