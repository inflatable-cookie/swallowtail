"""Private SDK240 trusted-host transaction proposal; no activation authority.

The independently provisioned manifest is the trust anchor for loader selection
and the supervisor, not a report emitted by the candidate or by this reader.
Only owned fixtures exercise mutation/entry under ordinary validation.
"""

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import stat
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
RECORD = ROOT / "docs/knowledge/operations/claude-agent-sdk-bootstrap-session.sha256"
PATCH = ROOT / "docs/knowledge/operations/claude-agent-sdk-runtime-permissions.patch"
CANDIDATE = "/opt/hostedtoolcache/node/22.23.4/x64/bin/node"
CANDIDATE_SHA = "94efd062ed066c1a0b332e18249859c905e920eeada5df55d24c99b919629a90"
PATCH_SHA = "087d937c796759e1baf5916e5852ce1fe86789a8a62826fb5fa924200230cc99"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def directory_entries(name):
    entries = []
    with os.scandir(name) as scan:
        for entry in scan:
            if len(entries) >= 8192 or len(entry.name) > 255:
                raise OSError("loader_directory_inventory_bound")
            entries.append(entry.name)
    return sorted(entries)


def host_identity():
    result = {
        "system": platform.system(),
        "machine": platform.machine(),
        "release": platform.release(),
        "uid": os.getuid(),
        "python": str(Path(sys.executable).resolve(strict=True)),
        "bootId": None,
        "loaderAuxv": {},
    }
    if sys.platform == "linux":
        boot = Path("/proc/sys/kernel/random/boot_id").read_bytes()
        if len(boot) != 37 or boot[-1:] != b"\n":
            raise OSError("fresh_host_boot_identity_required")
        result["bootId"] = boot[:-1].decode("ascii")
        with open("/proc/self/auxv", "rb") as stream:
            raw = stream.read(8193)
        if len(raw) > 8192 or len(raw) % 16:
            raise OSError("bounded_x86_64_auxiliary_vector_required")
        for offset in range(0, len(raw), 16):
            tag = int.from_bytes(raw[offset : offset + 8], "little")
            value = int.from_bytes(raw[offset + 8 : offset + 16], "little")
            if tag in (16, 23, 26, 29, 30):
                if str(tag) in result["loaderAuxv"]:
                    raise OSError("duplicate_loader_auxiliary_input")
                result["loaderAuxv"][str(tag)] = value
        if result["loaderAuxv"].get("23") != 0 or "16" not in result["loaderAuxv"]:
            raise OSError("ordinary_host_loader_auxiliary_inputs_required")
    return result


def load(name, data):
    spec = importlib.util.spec_from_loader(name, loader=None)
    module = importlib.util.module_from_spec(spec)
    module.__file__ = str(ROOT / name)
    exec(compile(data, module.__file__, "exec"), module.__dict__)
    return module


def source_packet(record, expected, *, root=ROOT):
    with record.open("rb") as stream:
        raw = stream.read(65537)
    if len(raw) > 65536 or sha(raw) != expected:
        raise OSError("source_record_changed")
    sources = {}
    for row in raw.decode().splitlines():
        digest, name = row.split("  ", 1)
        if (
            Path(name).is_absolute()
            or ".." in Path(name).parts
            or name in sources
            or len(sources) >= 128
        ):
            raise OSError("source_record_path")
        file = root / name
        if file.resolve(strict=True) != file.absolute():
            raise OSError("source_input_symlink")
        descriptor = os.open(file, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            info = os.fstat(descriptor)
            if not stat.S_ISREG(info.st_mode) or info.st_size > 8388608:
                raise OSError("source_input_bound")
            data = os.read(descriptor, info.st_size + 1)
            if len(data) != info.st_size or sha(data) != digest:
                raise OSError("source_input_changed")
            sources[name] = data
        finally:
            os.close(descriptor)
    return sources


def permission_source():
    data = PATCH.read_bytes()
    if sha(data) != PATCH_SHA:
        raise OSError("historical_permission_source_changed")
    body = (
        "\n".join(
            line[1:]
            for line in data.decode().splitlines()
            if line.startswith("+") and not line.startswith("+++")
        )
        + "\n"
    )
    return load("scripts/claude-agent-sdk-runtime-permissions.py", body.encode())


class HostInputs:
    """Held read-only input graph from an independently provisioned manifest."""

    def __init__(self, custody, elf, manifest, *, recovery=False):
        self.custody, self.manifest = custody, manifest
        self.capture = custody.Inputs(reserve_pipes=False)
        self.aliases = {}
        self.lookup = manifest["lookup"]
        try:
            if (
                len(self.lookup["directories"]) > 32
                or len(self.lookup["absent"]) > 128
                or len(manifest["aliases"]) > 256
            ):
                raise OSError("loader_lookup_manifest_bound")
            self.expected_host = manifest["host"]
            if self.expected_host != host_identity():
                raise OSError("independently_provisioned_host_mismatch")
            inputs = manifest["bootstrapInputs"]
            for role in (
                "libraries",
                "loaderInputs",
                "interpreters",
                "supervisorInputs",
            ):
                if not isinstance(inputs[role], dict):
                    raise OSError("bootstrap_role_missing")
                for name, digest in inputs[role].items():
                    self.capture.bind(name, digest)
            if not {str(p) for p in custody.python_inputs()} <= set(
                inputs["supervisorInputs"]
            ):
                raise OSError("independent_supervisor_input_missing")
            if (
                os.path.lexists("/etc/ld.so.cache")
                and "/etc/ld.so.cache" not in inputs["loaderInputs"]
            ):
                raise OSError("independent_loader_cache_missing")
            if sys.platform == "linux":
                self.capture.bind_ancestors(Path("/etc/ld.so.preload"))
            # Provisioning names the ordered loader search and hwcaps closure;
            # this reader cannot derive that authority from a candidate report.
            if (
                not self.lookup["directories"]
                or not self.lookup["absent"]
                or self.lookup["environment"]
                != {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"}
            ):
                raise OSError("independent_loader_search_inputs_required")
            for name, entries in self.lookup["directories"].items():
                self.capture.bind_ancestors(Path(name) / ".loader-search")
                if not isinstance(entries, list) or entries != sorted(set(entries)):
                    raise OSError("loader_search_inventory_shape")
            for name in self.lookup["absent"]:
                if not Path(name).is_absolute():
                    raise OSError("loader_absence_path")
                self.capture.bind_ancestors(Path(name))
            for alias, target in manifest["aliases"].items():
                self.bind_alias(alias, target)
            node = Path(manifest["path"])
            self.capture.bind_ancestors(node)
            fd = os.open(node, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
            self.node_fd = fd
            info = os.fstat(fd)
            if (
                not stat.S_ISREG(info.st_mode)
                or info.st_uid != os.getuid()
                or stat.S_IMODE(info.st_mode)
                not in ((0o755, 0o777) if recovery else (0o777,))
                or info.st_size != manifest["bytes"]
                or info.st_size > 268435456
            ):
                raise OSError("exact_candidate_identity_mode_or_size")
            self.node_identity = custody.identity(info)
            data = os.pread(fd, info.st_size + 1, 0)
            if len(data) != info.st_size or sha(data) != manifest["sha256"]:
                raise OSError("exact_candidate_bytes")
            objects = {str(node): data}
            total = len(data)
            for name in set(inputs["libraries"]) | set(inputs["interpreters"]):
                descriptor, identity, _ = self.capture.files[name]
                total += identity[5]
                if total > 536870912:
                    raise OSError("elf_graph_byte_bound")
                objects[name] = os.pread(descriptor, identity[5] + 1, 0)
            if not set(manifest["interpreterAliases"].items()) <= set(
                manifest["aliases"].items()
            ):
                raise OSError("interpreter_alias_not_held")
            elf.verify_graph(
                str(node),
                objects,
                manifest["resolutions"],
                interpreter_paths=manifest["interpreterAliases"],
            )
            self.check()
        except BaseException:
            self.close()
            raise

    def bind_alias(self, name, target):
        path = Path(name)
        if (
            not path.is_absolute()
            or str(path.resolve(strict=True)) != target
            or target not in self.capture.files
        ):
            raise OSError("independent_loader_alias_missing")
        rows = []
        for part in (*reversed(path.parents), path):
            info = part.lstat()
            if info.st_uid not in self.capture.owners:
                raise OSError("loader_alias_owner")
            if stat.S_ISLNK(info.st_mode):
                rows.append((str(part), self.custody.identity(info), os.readlink(part)))
            elif stat.S_ISDIR(info.st_mode):
                if info.st_mode & 0o022:
                    raise OSError("loader_alias_writable_ancestor")
                self.capture.bind_ancestors(part.resolve(strict=True) / ".alias-input")
        self.aliases[name] = (target, rows)

    def check(self):
        self.capture.check()
        if hasattr(self, "expected_host") and self.expected_host != host_identity():
            raise OSError("fresh_host_or_loader_auxiliary_inputs_changed")
        if os.path.lexists("/etc/ld.so.preload"):
            raise OSError("ambient_loader_preload_refused")
        for name, entries in self.lookup["directories"].items():
            if directory_entries(name) != entries:
                raise OSError("loader_search_directory_changed")
        for name in self.lookup["absent"]:
            if os.path.lexists(name):
                raise OSError("loader_search_absence_changed")
        for name, (target, links) in self.aliases.items():
            if str(Path(name).resolve(strict=True)) != target:
                raise OSError("loader_alias_changed")
            for link, identity, value in links:
                if (
                    self.custody.identity(Path(link).lstat()) != identity
                    or os.readlink(link) != value
                ):
                    raise OSError("loader_alias_identity_changed")
        if hasattr(self, "node_identity"):
            before = self.node_identity
            current = self.custody.identity(os.fstat(self.node_fd))
            # fchmod changes mode/ctime; all other original identity stays fixed.
            stable = lambda row: row[:4] + row[5:7]
            if stable(current) != stable(before) or current != self.custody.identity(
                Path(self.manifest["path"]).lstat()
            ):
                raise OSError("candidate_descriptor_or_path_changed")

    def close(self):
        if hasattr(self, "node_fd"):
            os.close(self.node_fd)
            del self.node_fd
        self.capture.close()


class Session:
    """One intent -> prepare -> observe/fakes -> actual joins -> checked restore."""

    def __init__(self, custody, elf, manifest, receipt, *, source_inputs=None):
        self.custody, self.elf, self.manifest = custody, elf, manifest
        self.helper = permission_source()
        self.preparation = self.helper.Preparation(custody, manifest, receipt)
        self.preparation.deadline = time.monotonic() + 80
        self.host = None
        self.child = None
        self.unsettled = False
        self.results = []
        self.source_inputs = {} if source_inputs is None else dict(source_inputs)

    def hold_source(self):
        for name, digest in self.source_inputs.items():
            self.host.capture.bind(name, digest)
        self.host.check()

    def store(self, phase, *, unresolved):
        self.unsettled = unresolved
        self.preparation.state.update(phase=phase, children=[-1] if unresolved else [])
        self.helper.persist(self.preparation.receipt, self.preparation.state)

    def start(self, command, **kwargs):
        self.host.check()
        # The crash window before a Popen handle exists is explicitly unknown.
        self.preparation.state["runtimeEntryAttempted"] = True
        self.store("observer_entry_intent", unresolved=True)
        self.child = subprocess.Popen(command, **kwargs)
        self.preparation.state["children"] = [self.child.pid]
        self.helper.persist(self.preparation.receipt, self.preparation.state)
        return self.child

    def joined(self, child):
        if child is not self.child or child.returncode is None:
            raise OSError("actual_owned_observer_wait_required")
        self.preparation.state.setdefault("observerWaits", []).append(
            {"pid": child.pid, "exit": child.returncode}
        )
        child.stdout.close()
        child.stderr.close()
        self.child = None
        self.store("observer_joined", unresolved=False)

    def runtime(self, custody, node, digest):
        # rehearsal loads its own source-bound custody module: keep its exact type.
        host = self.host
        manifest = self.manifest

        class HostBootstrap(custody.Bootstrap):
            def check(self):
                host.check()
                super().check()

            def resolve_library(self, name):
                self.check()
                if name in host.aliases:
                    return host.aliases[name][0]
                return super().resolve_library(name)

        inputs = manifest["bootstrapInputs"]
        bootstrap = HostBootstrap(
            node,
            digest,
            version=manifest["version"],
            libraries=inputs["libraries"],
            loader_inputs=inputs["loaderInputs"],
            interpreters=inputs["interpreters"],
            supervisor_inputs=inputs["supervisorInputs"],
            reserve_pipes=False,
        )
        observed = custody.runtime_inventory(
            node, digest, bootstrap=bootstrap, observer_owner=self
        )
        try:
            self.preparation.state["runtimeExecuted"] = True
            self.preparation.state.setdefault("runtimeObservations", []).append(
                {"nodeSha256": observed["nodeSha256"], "version": observed["version"]}
            )
            self.store("runtime_observed", unresolved=False)
        except BaseException:
            observed.close()
            raise
        return observed

    def run(self, core, record):
        try:
            self.host = HostInputs(self.custody, self.elf, self.manifest)
            self.hold_source()
            self.preparation.prepare()
            if self.preparation.state["identity"] != self.helper.stable(
                os.fstat(self.host.node_fd)
            ):
                raise OSError("permission_descriptor_binding_changed")
            for mode, state in (
                ("probe", "absent"),
                ("sidecar", "absent"),
                ("sidecar", "managed-deny"),
            ):
                self.host.check()
                # A crash anywhere in rehearsal retains unknown child settlement.
                self.store("fake_entry_intent", unresolved=True)

                def capture(custody, node, digest):
                    observed = self.runtime(custody, node, digest)
                    self.store("fake_entry_intent", unresolved=True)
                    return observed

                result = core.rehearsal(
                    record,
                    self.manifest["path"],
                    self.manifest["sha256"],
                    mode=mode,
                    state=state,
                    bound=10,
                    runtime_factory=capture,
                )
                if (
                    result.get("failure") is not None
                    or result.get("driverWait") != 0
                    or len(result.get("nativeWaits", [])) != 1
                    or result["nativeWaits"][0].get("code") != 0
                ):
                    raise OSError("owned_fake_settlement_failed; receipt retained")
                self.store("fake_joined", unresolved=False)
                self.results.append(
                    {
                        "mode": mode,
                        "state": state,
                        "driverWait": result["driverWait"],
                        "nativeWaits": result["nativeWaits"],
                    }
                )
            self.preparation.state["fakeCases"] = self.results
            return self.results
        finally:
            if self.preparation.state is not None:
                if self.child is not None:
                    if self.child.poll() is None:
                        self.child.kill()
                    self.child.wait(timeout=2)
                    self.joined(self.child)
                if self.unsettled:
                    raise OSError(
                        "child_settlement_unknown; receipt and custody retained"
                    )
                self.host.check()
                self.preparation.cleanup()
            else:
                self.preparation.close()
            if self.host is not None:
                self.host.close()

    def recover(self):
        # Recovery reacquires authority; it never pretends to inherit dead fds.
        # The historical helper refuses observer/fake entry intents (children
        # nonempty), so no receipt flag or PID reuse can stand in for an owned wait.
        try:
            self.host = HostInputs(self.custody, self.elf, self.manifest, recovery=True)
            self.hold_source()
            self.preparation.recover()
            self.host.check()
            return self.preparation.cleanup()
        finally:
            if self.host is not None:
                self.host.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--host-manifest", required=True)
    parser.add_argument("--host-manifest-sha256", required=True)
    parser.add_argument("--record-sha256", required=True)
    parser.add_argument("--receipt", required=True)
    parser.add_argument("--recover-only", action="store_true")
    args = parser.parse_args()
    if not (sys.flags.isolated and sys.flags.no_site and sys.dont_write_bytecode):
        raise OSError("isolated_no_site_no_bytecode_supervisor_required")
    if (
        set(os.environ) - {"PATH", "LANG", "LC_ALL", "HOME", "TMPDIR", "TMP", "TEMP"}
        or os.environ.get("PATH") != "/usr/bin:/bin"
        or os.environ.get("LANG") != "C"
        or os.environ.get("LC_ALL") != "C"
    ):
        raise OSError("credential_free_explicit_supervisor_environment_required")
    sources = source_packet(RECORD, args.record_sha256)
    raw = Path(args.host_manifest).read_bytes()
    if len(raw) > 1048576 or sha(raw) != args.host_manifest_sha256:
        raise OSError("independently_provisioned_manifest_binding")
    manifest = json.loads(raw)
    if (
        platform.system() != "Linux"
        or platform.machine() != "x86_64"
        or manifest["path"] != CANDIDATE
        or manifest["sha256"] != CANDIDATE_SHA
        or manifest["bytes"] != 124902584
        or manifest["version"] != "v22.23.4"
        or manifest["originalActivation"] is not False
    ):
        raise OSError("fixed_fake_only_candidate_required")
    core = load(
        "scripts/claude-agent-sdk-spawn-proposal.py",
        sources["scripts/claude-agent-sdk-spawn-proposal.py"],
    )
    custody = load(
        "scripts/claude-agent-sdk-probe-custody.py",
        sources["scripts/claude-agent-sdk-probe-custody.py"],
    )
    elf = load(
        "scripts/claude-agent-sdk-bootstrap-elf.py",
        sources["scripts/claude-agent-sdk-bootstrap-elf.py"],
    )
    source_inputs = {str(ROOT / name): sha(data) for name, data in sources.items()}
    source_inputs[str(RECORD)] = args.record_sha256
    session = Session(custody, elf, manifest, args.receipt, source_inputs=source_inputs)
    result = (
        session.recover()
        if args.recover_only
        else session.run(core, sha((ROOT / core.RECORD).read_bytes()))
    )
    print(
        json.dumps(
            {
                "fakeCases": result,
                "originalActivation": False,
                "kernelEnforced": False,
                "qualified": False,
            }
        )
    )


if __name__ == "__main__":
    main()
