"""SDK240 image-specific admission gate; no candidate opens, execution or repairs.

The selected published image has an unsafe tool-cache ancestor. This first gate
is usable source for the producer, not a completed host-package producer.
"""

import argparse
import json
import os
from pathlib import Path
import platform
import runpy
import stat
import sys

ROOT = Path(__file__).resolve().parents[1]
CANDIDATE = "/opt/hostedtoolcache/node/22.23.4/x64/bin/node"
IMAGE = {
    "imageOS": "ubuntu26",
    "imageVersion": "20261111.196.1",
    "system": "Linux",
    "machine": "x86_64",
}
MAX_OUTPUT = 16384


class Refused(OSError):
    def __init__(self, code, name=None, mode=None):
        super().__init__(code)
        self.code, self.name, self.mode = code, name, mode


def identity(info):
    return (info.st_dev, info.st_ino, info.st_uid, info.st_gid, info.st_mode)


class Ancestry:
    """Hold and recheck the exact directory chain before any candidate open."""

    def __init__(self, observed, *, root=Path("/"), uid=1001):
        self.root, self.rows = Path(root), []
        if observed != IMAGE:
            raise Refused("selected_image_mismatch")
        if self.root.resolve(strict=True) != self.root.absolute():
            raise Refused("inspection_root_alias")
        self.root = self.root.absolute()
        self.uid = uid
        try:
            self.acquire(self.root, "/")
            current = self.root
            for part in Path(CANDIDATE).parent.parts[1:]:
                current = current / part
                literal = "/" + str(current.relative_to(self.root))
                self.acquire(current, literal, part=part)
            self.check()
        except BaseException:
            self.close()
            raise

    def acquire(self, actual, literal, *, part=None):
        flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
        try:
            if part is None:
                fd = os.open(actual, flags)
            else:
                fd = os.open(part, flags, dir_fd=self.rows[-1][2])
        except OSError as error:
            raise Refused("ancestor_alias_or_open_refused", literal) from error
        try:
            info = os.fstat(fd)
        except BaseException:
            os.close(fd)
            raise
        self.rows.append((actual, literal, fd, identity(info)))
        if not stat.S_ISDIR(info.st_mode) or info.st_uid not in (0, self.uid):
            raise Refused("ancestor_type_or_owner_refused", literal)
        if info.st_mode & 0o022:
            raise Refused(
                "unsafe_ancestor", literal, format(stat.S_IMODE(info.st_mode), "04o")
            )

    def check(self):
        for actual, literal, fd, expected in self.rows:
            if (
                identity(os.fstat(fd)) != expected
                or identity(actual.lstat()) != expected
            ):
                raise Refused("ancestor_identity_changed", literal)

    def close(self):
        while self.rows:
            os.close(self.rows.pop()[2])


def emit(stream, value):
    raw = (
        json.dumps(
            {
                **value,
                "hostPackageProduced": False,
                "candidateOpened": False,
                "runtimeExecuted": False,
                "activationAuthorized": False,
            },
            sort_keys=True,
        )
        + "\n"
    )
    if len(raw.encode()) >= MAX_OUTPUT:
        raise Refused("admission_output_bound")
    stream.write(raw)
    stream.flush()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--record-sha256", required=True)
    parser.add_argument("--image-os", required=True)
    parser.add_argument("--image-version", required=True)
    args = parser.parse_args()
    try:
        if os.geteuid() != 0 or not (
            sys.flags.isolated and sys.flags.no_site and sys.dont_write_bytecode
        ):
            raise Refused("trusted_root_isolated_administrative_stage_required")
        session = runpy.run_path(
            str(ROOT / "scripts/claude-agent-sdk-bootstrap-session.py")
        )
        session["source_packet"](session["RECORD"], args.record_sha256)
        observed = {
            "imageOS": args.image_os,
            "imageVersion": args.image_version,
            "system": platform.system(),
            "machine": platform.machine(),
        }
        held = Ancestry(observed)
        try:
            held.check()
            emit(
                sys.stdout,
                {
                    "state": "ancestry_only",
                    "next": "complete_package_producer_still_required",
                },
            )
        finally:
            held.close()
    except Refused as error:
        emit(
            sys.stdout,
            {
                "state": "unsupported",
                "impediment": error.code,
                "ancestor": error.name,
                "observedMode": error.mode,
            },
        )
        return 1
    except OSError:
        emit(
            sys.stdout,
            {"state": "unsupported", "impediment": "administrative_input_refused"},
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
