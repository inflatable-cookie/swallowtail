#!/usr/bin/env python3
"""Verify the reviewed ACP startup receipt using source bytes only."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
RECORD = "docs/knowledge/operations/claude-agent-acp-startup-integration.json"
REVIEWED_HEAD = "5923c113c9e4ee4be42c3345148b212b1d8733cf"
# Review5920367664 confirmed this receipt's values; its coverage defect is
# repaired here without promoting that source/fake review to qualification.
REVIEWED_SHA = "38eac917e03e8b0888767e5edc9005e44f0647188d8deba0c9d56ce426411634"
MAX_BYTES = 2 * 1024 * 1024


class Refused(ValueError):
    pass


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)


def decode(raw):
    if len(raw) > MAX_BYTES:
        raise Refused("record_size")

    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise Refused("duplicate_key")
            result[key] = value
        return result

    def invalid_number(_):
        raise Refused("non_json_number")

    try:
        return json.loads(raw, object_pairs_hook=unique, parse_constant=invalid_number)
    except (ValueError, UnicodeError) as error:
        raise Refused(str(error)) from error


def regular(file):
    if file.is_symlink() or not file.is_file() or file.stat().st_size > MAX_BYTES:
        raise Refused(f"regular_bounded_file_required:{file.name}")
    return file.read_bytes()


def git_blob(head, name):
    # All names come from the digest-pinned reference, never the candidate.
    identity = f"{head}:{name}"
    try:
        size = subprocess.run(
            ["git", "cat-file", "-s", identity],
            cwd=ROOT,
            capture_output=True,
            check=True,
            timeout=10,
        )
        if not 0 <= int(size.stdout) <= MAX_BYTES:
            raise Refused(f"git_blob_size:{identity}")
        return subprocess.run(
            ["git", "show", identity],
            cwd=ROOT,
            capture_output=True,
            check=True,
            timeout=10,
        ).stdout
    except (subprocess.SubprocessError, ValueError) as error:
        raise Refused(f"missing_or_invalid_git_binding:{identity}") from error


def bound(raw, expected, label):
    actual = digest(raw)
    if actual != expected:
        raise Refused(f"source_binding_drift:{label}")
    return actual


def verify(record, root=ROOT):
    reference = decode(bound_reference())
    # Recompute every byte binding independently of the candidate JSON. The
    # reviewed reference also freezes metadata, lookup policy and closed gates.
    shared = reference["acceptedShared"]
    for name, expected in shared["files"].items():
        bound(git_blob(shared["head"], name), expected, f"acceptedShared:{name}")
    for name, expected in reference["before"].items():
        bound(git_blob(reference["baseHead"], name), expected, f"before:{name}")
    for name, expected in reference["currentSources"].items():
        file = root / name
        if not file.resolve().is_relative_to(root.resolve()):
            raise Refused(f"source_path_escape:{name}")
        bound(regular(file), expected, f"currentSources:{name}")
    for key in ("unchangedInvocation", "sourceFlow"):
        item = reference[key]
        bound(git_blob(item["head"], item["path"]), item["sha256"], f"{key}:git")
        bound(regular(root / item["path"]), item["sha256"], f"{key}:current")
    if canonical(record) != canonical(reference):
        raise Refused("receipt_content_drift")
    return {"reviewedReceiptSha256": REVIEWED_SHA, "admission": "closed"}


def bound_reference():
    raw = git_blob(REVIEWED_HEAD, RECORD)
    bound(raw, REVIEWED_SHA, "reviewed_receipt")
    return raw


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--record", type=Path, default=ROOT / RECORD)
    args = parser.parse_args()
    try:
        print(json.dumps(verify(decode(regular(args.record))), sort_keys=True))
    except (Refused, OSError) as error:
        parser.exit(1, f"ACP startup receipt refused: {error}\n")


if __name__ == "__main__":
    main()
