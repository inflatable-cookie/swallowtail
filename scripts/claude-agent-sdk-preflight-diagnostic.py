"""Read-only observations, never runtime execution or host admission."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import selectors
import stat
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
RECORD = "docs/knowledge/operations/claude-agent-sdk-preflight-diagnostic.sha256"
INPUTS = (
    "scripts/claude-agent-sdk-preflight-diagnostic.py",
    "scripts/tests/claude-agent-sdk-preflight-diagnostic.py",
    "docs/knowledge/operations/claude-agent-sdk-preflight-diagnostic.md",
    "docs/knowledge/operations/claude-agent-sdk-preflight-diagnostic.json",
    "docs/knowledge/operations/claude-agent-sdk-preflight-workflow.patch",
    "docs/knowledge/operations/claude-agent-sdk-preflight-routing.patch",
    "effigy.toml",
)
NODE = Path("/opt/hostedtoolcache/node/22.23.2/x64/bin/node")
NODE_SHA = "3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327"
PHASES = (
    "source_identity",
    "clean_worktree",
    "scratch_creation",
    "cached_executable",
    "inventory_production",
    "host_capabilities",
)


class Refused(Exception):
    pass


def bounded_read(file, maximum=8192):
    descriptor = os.open(file, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        if not stat.S_ISREG(os.fstat(descriptor).st_mode):
            raise Refused("read_type")
        data = os.read(descriptor, maximum + 1)
    finally:
        os.close(descriptor)
    if len(data) > maximum:
        raise Refused("read_bound")
    return data


class Diagnostic:
    def __init__(self, root, source, record, temporary, output, node=NODE, cgroup=None):
        self.root, self.source, self.record = Path(root), source, record
        self.temporary = Path(temporary) if temporary else None
        self.output, self.node = output, Path(node)
        self.cgroup = Path(cgroup or "/sys/fs/cgroup")
        self.deadline, self.output_bytes = time.monotonic() + 10, 0
        self.inventory = None

    def remaining(self):
        remaining = self.deadline - time.monotonic()
        if remaining <= 0:
            raise Refused("deadline")
        return remaining

    def emit(self, data):
        data = {
            **data,
            "hostAdmission": False,
            "kernelEnforced": False,
            "originalActivation": False,
        }
        raw = json.dumps(data, sort_keys=True) + "\n"
        if len(raw.encode()) > 4096 or self.output_bytes + len(raw.encode()) > 32768:
            raise Refused("report_bound")
        self.output.write(raw)
        self.output.flush()
        self.output_bytes += len(raw.encode())

    def git(self, *args):
        command = [
            "/usr/bin/git",
            "-c",
            "core.fsmonitor=false",
            "-c",
            "core.hooksPath=/dev/null",
            *args,
        ]
        environment = {
            "PATH": "/usr/bin:/bin",
            "LANG": "C",
            "LC_ALL": "C",
            "GIT_CONFIG_NOSYSTEM": "1",
            "GIT_CONFIG_GLOBAL": "/dev/null",
            "GIT_OPTIONAL_LOCKS": "0",
        }
        child = subprocess.Popen(
            command,
            cwd=self.root,
            env=environment,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
        )
        data = bytearray()
        try:
            with selectors.DefaultSelector() as poll:
                poll.register(child.stdout, selectors.EVENT_READ)
                while poll.get_map():
                    events = poll.select(self.remaining())
                    if not events:
                        raise Refused("git_deadline")
                    chunk = os.read(child.stdout.fileno(), 8193)
                    if not chunk:
                        poll.unregister(child.stdout)
                    data.extend(chunk)
                    if len(data) > 8192:
                        raise Refused("git_output_bound")
            if child.wait(timeout=self.remaining()) != 0:
                raise Refused("git_failed")
            return bytes(data)
        finally:
            if child.poll() is None:
                child.kill()
            child.wait(timeout=2)
            child.stdout.close()

    def file_identity(self, file, maximum=268435456):
        descriptor = os.open(file, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        try:
            before = os.fstat(descriptor)
            if not stat.S_ISREG(before.st_mode) or before.st_size > maximum:
                raise Refused("file_type_or_bound")
            digest, total = hashlib.sha256(), 0
            while True:
                self.remaining()
                chunk = os.read(descriptor, 65536)
                if not chunk:
                    break
                total += len(chunk)
                if total > maximum:
                    raise Refused("file_read_bound")
                digest.update(chunk)
            after = os.fstat(descriptor)
            fields = ("st_dev", "st_ino", "st_size", "st_mtime_ns", "st_ctime_ns")
            if (
                any(getattr(before, key) != getattr(after, key) for key in fields)
                or total != before.st_size
            ):
                raise Refused("file_changed")
            return {
                "sha256": digest.hexdigest(),
                "bytes": total,
                "device": before.st_dev,
                "inode": before.st_ino,
                "ownerUid": before.st_uid,
                "mode": stat.S_IMODE(before.st_mode),
            }
        finally:
            os.close(descriptor)

    def source_identity(self):
        if os.geteuid() == 0:
            raise Refused("ordinary_uid_required")
        if not re.fullmatch(r"[a-f0-9]{40}", self.source) or not re.fullmatch(
            r"[a-f0-9]{64}", self.record
        ):
            raise Refused("source_arguments")
        if self.git("rev-parse", "HEAD").decode().strip() != self.source:
            raise Refused("source_mismatch")
        record = self.root / RECORD
        if record.resolve(strict=True) != record.absolute():
            raise Refused("record_symlink")
        raw = bounded_read(record)
        if hashlib.sha256(raw).hexdigest() != self.record:
            raise Refused("record_mismatch")
        rows = [line.split("  ", 1) for line in raw.decode().splitlines()]
        if (
            any(len(row) != 2 for row in rows)
            or tuple(row[1] for row in rows) != INPUTS
        ):
            raise Refused("record_input_set")
        for digest, name in rows:
            file = self.root / name
            if file.resolve(strict=True) != file.absolute():
                raise Refused("source_symlink")
            if self.file_identity(file, 2097152)["sha256"] != digest:
                raise Refused("source_bytes")
        return {"source": self.source, "recordSha256": self.record}

    def clean_worktree(self):
        if self.git("status", "--porcelain", "--untracked-files=normal"):
            raise Refused("worktree_dirty")
        return {"clean": True}

    def scratch_creation(self):
        if self.temporary is None:
            raise Refused("temporary_root_required")
        leaf = Path(tempfile.mkdtemp(prefix="sdk234-readonly-", dir=self.temporary))
        before = leaf.lstat()
        if not stat.S_ISDIR(before.st_mode) or stat.S_IMODE(before.st_mode) != 0o700:
            raise Refused("scratch_private_mode")
        after = leaf.lstat()
        if (before.st_dev, before.st_ino) != (after.st_dev, after.st_ino):
            raise Refused("scratch_replaced")
        leaf.rmdir()
        return {"createdPrivateLeaf": True, "removedOwnLeaf": True}

    def cached_executable(self):
        if not self.node.is_file() or not os.access(self.node, os.X_OK):
            raise Refused("cached_executable_unavailable")
        return {"availableExecutable": True, "executed": False}

    def inventory_production(self):
        self.inventory = {
            "cachedNode": self.file_identity(self.node),
            "currentPython": self.file_identity(Path(sys.executable).resolve()),
            "system": platform.system(),
            "machine": platform.machine(),
            "kernelRelease": platform.release(),
            "runtimeLibrariesAttested": False,
            "candidateExecuted": False,
        }
        return self.inventory

    def host_capabilities(self):
        info = self.cgroup.lstat()
        controllers = bounded_read(self.cgroup / "cgroup.controllers").decode().split()
        enabled = bounded_read(self.cgroup / "cgroup.subtree_control").decode().split()
        required = {"pids", "memory", "cpu"}
        checks = {
            "linuxX64": self.inventory["system"] == "Linux"
            and self.inventory["machine"] == "x86_64",
            "nodeBytesMatch": self.inventory["cachedNode"]["sha256"] == NODE_SHA,
            "rootOwnedParent": info.st_uid == 0 and not info.st_mode & 0o022,
            "controllersAvailable": required <= set(controllers),
            "controllersEnabled": required <= set(enabled),
        }
        namespaces = {}
        for name in ("net", "pid"):
            file = Path("/proc/self/ns", name)
            if file.exists():
                entry = file.stat()
                namespaces[name] = {"device": entry.st_dev, "inode": entry.st_ino}
        return {
            "checks": checks,
            "cgroupParent": {
                "device": info.st_dev,
                "inode": info.st_ino,
                "ownerUid": info.st_uid,
                "mode": stat.S_IMODE(info.st_mode),
            },
            "requiredControllersPresent": sorted(required & set(controllers)),
            "requiredControllersEnabled": sorted(required & set(enabled)),
            "namespaceIdentities": namespaces,
            "readOnlyChecksPassed": all(checks.values()),
            "rootCapabilities": "unobserved; no privileged methods invoked",
        }

    def run(self):
        failed, produced = None, False
        try:
            for phase in PHASES:
                if failed:
                    self.emit({"phase": phase, "status": "skipped"})
                    continue
                try:
                    self.remaining()
                    facts = getattr(self, phase)()
                    if (
                        phase == "host_capabilities"
                        and not facts["readOnlyChecksPassed"]
                    ):
                        failed = phase
                    self.emit(
                        {
                            "phase": phase,
                            "status": "refused" if failed else "observed",
                            "facts": facts,
                        }
                    )
                    if phase == "inventory_production":
                        produced = True
                except (
                    OSError,
                    ValueError,
                    Refused,
                    subprocess.TimeoutExpired,
                ) as error:
                    failed = phase
                    reason = (
                        str(error) if isinstance(error, Refused) else "operation_failed"
                    )
                    self.emit({"phase": phase, "status": "refused", "reason": reason})
            self.emit(
                {
                    "schema": "swallowtail.sdk234.readonly-preflight.v1",
                    "failedPhase": failed,
                    "reportProduced": produced,
                    "readOnlyChecksPassed": failed is None,
                    "hostAdmission": False,
                    "kernelEnforced": False,
                    "originalActivation": False,
                }
            )
        except (OSError, Refused):
            return 1
        return 1 if failed else 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-sha", required=True)
    parser.add_argument("--record-sha256", required=True)
    args = parser.parse_args()
    return Diagnostic(
        ROOT, args.source_sha, args.record_sha256, os.environ.get("TMPDIR"), sys.stdout
    ).run()


if __name__ == "__main__":
    raise SystemExit(main())
