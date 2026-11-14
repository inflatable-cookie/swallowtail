"""Inactive exclusive cgroupv2 custody for one SDK234 combined fake job.

No delegation/controller is changed on a parent. Tests use an independent
owned-filesystem model; ordinary selectors never instantiate this root adapter.
"""

import ctypes
import errno
import os
from pathlib import Path
import re
import stat
import time

LIMITS = {
    "pids.max": "32",
    "memory.max": "268435456",
    "memory.swap.max": "0",
    "cpu.max": "100000 100000",
}
REQUIRED = {"pids", "memory", "cpu"}
CGROUP2_MAGIC = 0x63677270


class Refused(OSError):
    pass


def admit_delegation(owner, mode, filesystem, controllers, enabled):
    if owner != 0 or mode & 0o022 or filesystem != CGROUP2_MAGIC:
        raise Refused(errno.EPERM, "host_owned_cgroup2_delegation_required")
    if not REQUIRED <= set(controllers) or not REQUIRED <= set(enabled):
        raise Refused(errno.ENOSYS, "pre_enabled_pids_memory_cpu_delegation_required")


def read_at(directory, name):
    fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=directory)
    try:
        data = os.read(fd, 65537)
        if len(data) > 65536:
            raise Refused(errno.EFBIG, "cgroup_record_bound")
        return data.decode().strip()
    finally:
        os.close(fd)


def values(text):
    return dict(row.split(None, 1) for row in text.splitlines())


class LinuxCgroup:
    kernel_enforced = True

    def __init__(self, parent, name):
        if os.geteuid() != 0 or not re.fullmatch(r"sdk234-[a-f0-9]{24}", name):
            raise Refused(errno.EPERM, "exclusive_root_cgroup_name_required")
        self.parent = Path(parent).absolute()
        if self.parent.resolve(strict=True) != self.parent:
            raise Refused(errno.EPERM, "cgroup_parent_symlink")
        self.parent_fd = os.open(
            self.parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
        )
        self.fd = None
        self.name = name
        self.created = False
        try:
            info = os.fstat(self.parent_fd)
            buffer = ctypes.create_string_buffer(256)
            libc = ctypes.CDLL(None, use_errno=True)
            if libc.fstatfs(self.parent_fd, buffer) != 0:
                raise Refused(ctypes.get_errno(), "cgroup_fstatfs")
            admit_delegation(
                info.st_uid,
                stat.S_IMODE(info.st_mode),
                ctypes.c_long.from_buffer(buffer).value,
                read_at(self.parent_fd, "cgroup.controllers").split(),
                read_at(self.parent_fd, "cgroup.subtree_control").split(),
            )
            self.parent_identity = {"device": info.st_dev, "inode": info.st_ino}
        except BaseException:
            os.close(self.parent_fd)
            raise

    @property
    def path(self):
        return self.parent / self.name

    def create(self):
        os.mkdir(self.name, 0o700, dir_fd=self.parent_fd)
        info = os.stat(self.name, dir_fd=self.parent_fd, follow_symlinks=False)
        self.identity = {"device": info.st_dev, "inode": info.st_ino}
        self.created = True
        self.fd = os.open(
            self.name,
            os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
            dir_fd=self.parent_fd,
        )
        os.chmod(self.name, 0o700, dir_fd=self.parent_fd)
        info = os.fstat(self.fd)
        self.identity = {"device": info.st_dev, "inode": info.st_ino}
        for name, value in LIMITS.items():
            self.write(name, value)
            if read_at(self.fd, name) != value:
                raise Refused(errno.EPERM, "cgroup_limit_not_applied_" + name)
        for name in (
            "cgroup.kill",
            "cgroup.freeze",
            "cgroup.events",
            "pids.events",
            "memory.events",
        ):
            os.stat(name, dir_fd=self.fd, follow_symlinks=False)
        return self.receipt()

    def write(self, name, data):
        if (
            name not in {*LIMITS, "cgroup.procs", "cgroup.kill", "cgroup.freeze"}
            or self.fd is None
        ):
            raise Refused(errno.EPERM, "cgroup_control_not_owned")
        fd = os.open(name, os.O_WRONLY | os.O_NOFOLLOW, dir_fd=self.fd)
        try:
            raw = (data + "\n").encode()
            if os.write(fd, raw) != len(raw):
                raise Refused(errno.EIO, "cgroup_short_write")
        finally:
            os.close(fd)

    def attach(self, pid):
        if not self.created or type(pid) is not int or pid <= 1 or self.members():
            raise Refused(errno.EPERM, "initial_task_before_release_required")
        self.write("cgroup.procs", str(pid))
        if self.members() != [pid]:
            raise Refused(errno.EPERM, "initial_cgroup_membership")
        self.assert_limits()

    def assert_limits(self):
        for name, value in LIMITS.items():
            if read_at(self.fd, name) != value:
                raise Refused(errno.EPERM, "cgroup_limit_changed_" + name)

    def members(self):
        return sorted(int(x) for x in read_at(self.fd, "cgroup.procs").split())

    def tasks(self):
        result = sorted(int(x) for x in read_at(self.fd, "cgroup.threads").split())
        if len(result) > 32:
            raise Refused(errno.EAGAIN, "task_limit_observation")
        return result

    def stop(self, deadline):
        self.write("cgroup.freeze", "1")
        while values(read_at(self.fd, "cgroup.events")).get("frozen") != "1":
            if time.monotonic() >= deadline:
                # Killing our identity-bound subtree is still necessary when
                # freeze acknowledgement fails. Do not mark settlement green.
                self.write("cgroup.kill", "1")
                raise Refused(errno.ETIMEDOUT, "cgroup_freeze_unknown")
            time.sleep(0.002)
        owned = self.members()
        self.write("cgroup.kill", "1")
        return owned

    def empty(self):
        return (
            values(read_at(self.fd, "cgroup.events")).get("populated") == "0"
            and not self.members()
        )

    def receipt(self):
        return {
            "parent": str(self.parent),
            "name": self.name,
            "parentIdentity": self.parent_identity,
            "identity": self.identity,
            "limits": LIMITS,
            "created": True,
        }

    def events(self):
        return {
            "pids": values(read_at(self.fd, "pids.events")),
            "memory": values(read_at(self.fd, "memory.events")),
            "tasks": self.tasks(),
            "empty": self.empty(),
        }

    def remove(self, expected):
        current = os.stat(self.name, dir_fd=self.parent_fd, follow_symlinks=False)
        if (
            self.fd is None
            or {"device": current.st_dev, "inode": current.st_ino}
            != expected["identity"]
            or current.st_uid != 0
            or stat.S_IMODE(current.st_mode) != 0o700
            or not self.empty()
        ):
            raise Refused(errno.EPERM, "cgroup_cleanup_identity_or_members")
        os.rmdir(self.name, dir_fd=self.parent_fd)
        self.created = False

    def close(self):
        if self.fd is not None:
            os.close(self.fd)
            self.fd = None
        os.close(self.parent_fd)


def cleanup(receipt):
    group = LinuxCgroup(receipt["parent"], receipt["name"])
    try:
        if (
            group.parent_identity != receipt["parentIdentity"]
            or receipt["limits"] != LIMITS
        ):
            raise Refused(errno.EPERM, "cgroup_parent_or_limit_changed")
        group.fd = os.open(
            group.name,
            os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
            dir_fd=group.parent_fd,
        )
        current = os.fstat(group.fd)
        if (
            {"device": current.st_dev, "inode": current.st_ino} != receipt["identity"]
            or current.st_uid != 0
            or stat.S_IMODE(current.st_mode) != 0o700
        ):
            raise Refused(errno.EPERM, "cgroup_cleanup_replaced")
        if not group.empty():
            deadline = time.monotonic() + 2
            group.stop(deadline)
            while not group.empty():
                if time.monotonic() >= deadline:
                    raise Refused(errno.ETIMEDOUT, "cgroup_cleanup_population_unknown")
                time.sleep(0.002)
        # This disposal cannot manufacture the old monitor's missing waits.
        group.remove(receipt)
    finally:
        group.close()
