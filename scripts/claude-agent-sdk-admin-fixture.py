"""Private fixture lifecycle; no canonical-path or privileged command entrypoint.

The proposed CI adapter is documented separately and is not activated here.
Callers supply an already owned parent. Tests use only their fresh TMPDIR root.
Same-user interpreter/source custody is outside this fixture contract.
"""

import hashlib
import json
import os
from pathlib import Path
import stat

POLICY = "managed-settings.json"
LEASE = ".swallowtail-sdk234.json"
STATES = {"absent": None, "disabled": False, "managed-deny": True}


class Refused(Exception):
    """Preserve an unexpected object for owner disposal; never restore over it."""


def payload(state):
    if state not in STATES:
        raise Refused("unknown_state")
    if state == "absent":
        return None
    return (
        json.dumps({"permissions": {"denyInsteadOfPrompting": STATES[state]}}) + "\n"
    ).encode()


def directory(parent, owner):
    parent = Path(parent).absolute()
    if parent.resolve(strict=True) != parent:
        raise Refused("symlink_parent")
    fd = os.open(parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    info = os.fstat(fd)
    if info.st_uid != owner or stat.S_IMODE(info.st_mode) & 0o022:
        os.close(fd)
        raise Refused("parent_owner_or_mode")
    return fd


def read_file(fd, name, owner, mode):
    child = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=fd)
    try:
        info = os.fstat(child)
        if (
            not stat.S_ISREG(info.st_mode)
            or info.st_uid != owner
            or info.st_nlink != 1
            or stat.S_IMODE(info.st_mode) != mode
            or info.st_size > 4096
        ):
            raise Refused("file_owner_type_mode_or_links")
        with os.fdopen(child, "rb", closefd=False) as stream:
            data = stream.read(4097)
        if len(data) > 4096:
            raise Refused("file_size")
        return data
    finally:
        os.close(child)


def write_file(fd, name, data, mode):
    child = os.open(
        name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=fd
    )
    try:
        with os.fdopen(child, "wb", closefd=False) as stream:
            stream.write(data)
            stream.flush()
        os.fchmod(child, mode)
    finally:
        os.close(child)


def prepare(parent, name, state, owner, run):
    data = payload(state)
    if name not in {"claude-code", "ClaudeCode"} or not run:
        raise Refused("fixture_identity")
    parent_fd = directory(parent, owner)
    try:
        # Exclusive creation refuses any existing directory, file or symlink.
        os.mkdir(name, mode=0o755, dir_fd=parent_fd)
        fd = os.open(
            name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=parent_fd
        )
        try:
            os.fchmod(fd, 0o755)
            info = os.fstat(fd)
            if info.st_uid != owner:
                raise Refused("fixture_owner")
            lease = {
                "run": run,
                "state": state,
                "owner": owner,
                "device": info.st_dev,
                "inode": info.st_ino,
                "policySha256": hashlib.sha256(data).hexdigest() if data else None,
            }
            if data is not None:
                write_file(fd, POLICY, data, 0o444)
            write_file(fd, LEASE, (json.dumps(lease) + "\n").encode(), 0o600)
            return lease
        finally:
            os.close(fd)
    finally:
        os.close(parent_fd)


def cleanup(parent, name, state, owner, run):
    data = payload(state)
    if name not in {"claude-code", "ClaudeCode"}:
        raise Refused("fixture_identity")
    parent_fd = directory(parent, owner)
    try:
        fd = os.open(
            name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=parent_fd
        )
        try:
            info = os.fstat(fd)
            if info.st_uid != owner or stat.S_IMODE(info.st_mode) != 0o755:
                raise Refused("fixture_owner_or_mode")
            expected = {
                "run": run,
                "state": state,
                "owner": owner,
                "device": info.st_dev,
                "inode": info.st_ino,
                "policySha256": hashlib.sha256(data).hexdigest() if data else None,
            }
            if json.loads(read_file(fd, LEASE, owner, 0o600)) != expected:
                raise Refused("lease_mismatch")
            if set(os.listdir(fd)) != ({LEASE, POLICY} if data else {LEASE}):
                raise Refused("unexpected_entries")
            if data is not None and read_file(fd, POLICY, owner, 0o444) != data:
                raise Refused("policy_substitution")
            # No recursive removal; all checks precede any deletion.
            if data is not None:
                os.unlink(POLICY, dir_fd=fd)
            os.unlink(LEASE, dir_fd=fd)
        finally:
            os.close(fd)
        os.rmdir(name, dir_fd=parent_fd)
    finally:
        os.close(parent_fd)
