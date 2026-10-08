#!/usr/bin/env python3
"""Fail when numbered research records collide with a different slug."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

# Keep sibling imports from writing __pycache__ under scripts/.
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
from canonical_main_authority import (  # noqa: E402
    CANONICAL_AUTHORITY,
    CANONICAL_REF,
    AuthorityError,
    decode_z_paths,
    refresh_authority,
)


SCRIPT_ROOT = Path(__file__).resolve().parent.parent
NUMBERED_NAME = re.compile(r"^(?P<number>\d{3})-(?P<slug>.+)\.(?:md|tsv|csv)$")
RESEARCH_PATH = re.compile(r"^docs/research/(?P<name>[^/]+)$")
ALLOWLISTED_COLLISIONS = {
    "328": frozenset(
        {
            "goose-acp-1-50-1-failure-binding-reopen",
            "qoder-headless-1-1-54-identity",
        }
    ),
    "337": frozenset(
        {
            "opencode-acp-1-18-32-identity",
            "opencode-acp-unavailable-cells",
        }
    ),
}


def fail(message: str) -> None:
    print(f"research number collision check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def classify(relative: str) -> tuple[str, str] | None:
    path_match = RESEARCH_PATH.fullmatch(relative)
    if path_match is None:
        return None
    name_match = NUMBERED_NAME.fullmatch(path_match.group("name"))
    if name_match is None:
        return None
    return name_match.group("number"), name_match.group("slug")


def format_slugs(slugs: set[str] | frozenset[str]) -> str:
    return ", ".join(sorted(slugs))


def occupancy_from_paths(
    paths: list[str], source: str, *, reject_collisions: bool = True
) -> dict[str, set[str]]:
    grouped: dict[str, set[str]] = defaultdict(set)
    for relative in paths:
        record = classify(relative)
        if record is None:
            continue
        number, slug = record
        grouped[number].add(slug)

    occupancy: dict[str, set[str]] = {}
    for number, slugs in sorted(grouped.items()):
        if (
            reject_collisions
            and len(slugs) > 1
            and ALLOWLISTED_COLLISIONS.get(number) != slugs
        ):
            fail(
                f"{source} research number {number} is assigned to different "
                f"records ({format_slugs(slugs)})"
            )
        occupancy[number] = slugs
    return occupancy


def record_identity(relative: str) -> tuple[str, str, str] | None:
    record = classify(relative)
    if record is None:
        return None
    number, slug = record
    return number, slug, Path(relative).suffix


def working_tree_paths(root: Path) -> list[str]:
    research = root / "docs" / "research"
    if not research.is_dir():
        fail("missing docs/research")
    paths = [
        child.relative_to(root).as_posix()
        for child in research.iterdir()
        if child.is_file() and classify(child.relative_to(root).as_posix()) is not None
    ]
    return sorted(paths)


def git_run(root: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", "-C", str(root), *args],
        check=False,
        capture_output=True,
        text=True,
    )


def git_paths(root: Path, ref: str) -> list[str]:
    proc = subprocess.run(
        [
            "git",
            "-C",
            str(root),
            "-c",
            "core.quotePath=false",
            "ls-tree",
            "-z",
            "-r",
            "--name-only",
            "--",
            ref,
        ],
        check=False,
        capture_output=True,
    )
    if proc.returncode != 0:
        detail = (
            proc.stderr.decode("utf-8", "replace").strip()
            or proc.stdout.decode("utf-8", "replace").strip()
            or f"exit {proc.returncode}"
        )
        fail(f"git ls-tree failed: {detail}")
    try:
        return decode_z_paths(proc.stdout)
    except AuthorityError as exc:
        fail(str(exc))


def resolve_commit(root: Path, ref: str) -> str | None:
    proc = git_run(root, "rev-parse", "--verify", "--quiet", f"{ref}^{{commit}}")
    if proc.returncode != 0:
        return None
    sha = proc.stdout.strip()
    return sha or None


def git_file_contents(root: Path, commit: str, relative: str) -> bytes:
    proc = subprocess.run(
        ["git", "-C", str(root), "show", f"{commit}:{relative}"],
        check=False,
        capture_output=True,
    )
    if proc.returncode != 0:
        detail = (
            proc.stderr.decode("utf-8", "replace").strip()
            or proc.stdout.decode("utf-8", "replace").strip()
            or f"exit {proc.returncode}"
        )
        fail(f"cannot read base research record {relative}: {detail}")
    return proc.stdout


def renumbered_record_matches(
    original: bytes, candidate: bytes, original_number: str, new_number: str
) -> bool:
    if original == candidate:
        return True
    original_heading = f"# Research {original_number}:".encode()
    if not original.startswith(original_heading):
        return False
    candidate_heading = f"# Research {new_number}:".encode()
    expected = candidate_heading + original[len(original_heading) :]
    return candidate == expected


def base_record_contents(
    root: Path,
    relative: str,
    contents: dict[str, bytes],
    commit: str | None,
) -> bytes:
    original = contents.get(relative)
    if original is not None:
        return original
    if commit is None:
        fail(
            "canonical authority omitted bytes needed to prove retention of "
            f"{relative}"
        )
    return git_file_contents(root, commit, relative)


def check_against_base(
    root: Path,
    head: dict[str, set[str]],
    head_paths: list[str],
    base_paths: list[str],
    base_contents: dict[str, bytes],
    base_label: str,
    base_commit: str | None = None,
) -> None:
    base = occupancy_from_paths(base_paths, base_label, reject_collisions=False)
    unapproved_base_collisions = {
        number
        for number, slugs in base.items()
        if len(slugs) > 1 and ALLOWLISTED_COLLISIONS.get(number) != slugs
    }
    head_path_set = set(head_paths)
    head_identities = [
        (relative, record_identity(relative)) for relative in head_paths
    ]

    for number in sorted(unapproved_base_collisions):
        colliding_paths = [
            relative
            for relative in base_paths
            if (identity := record_identity(relative)) is not None
            and identity[0] == number
        ]
        for original_path in colliding_paths:
            if original_path in head_path_set:
                original = base_record_contents(
                    root, original_path, base_contents, base_commit
                )
                if (root / original_path).read_bytes() != original:
                    fail(
                        f"canonical collision record {original_path} changed in "
                        "place; preserve its evidence bytes"
                    )
                continue
            identity = record_identity(original_path)
            assert identity is not None
            original_number, slug, extension = identity
            candidates = [
                relative
                for relative, candidate_identity in head_identities
                if candidate_identity is not None
                and candidate_identity[0] != original_number
                and candidate_identity[1] == slug
                and candidate_identity[2] == extension
            ]
            if len(candidates) != 1:
                fail(
                    f"canonical collision repair must retain {original_path}; "
                    "keep it or move the same record to one fresh unused number"
                )

            candidate_path = candidates[0]
            candidate_identity = record_identity(candidate_path)
            assert candidate_identity is not None
            new_number = candidate_identity[0]
            if new_number in base:
                fail(
                    f"canonical collision repair target {new_number} is already "
                    f"occupied in {base_label}"
                )
            original = base_record_contents(
                root, original_path, base_contents, base_commit
            )
            candidate = (root / candidate_path).read_bytes()
            if not renumbered_record_matches(
                original, candidate, original_number, new_number
            ):
                fail(
                    f"canonical collision repair changed {original_path}; only "
                    "its numbered heading and path may change"
                )

    collisions: list[str] = []
    for number, head_slugs in sorted(head.items()):
        base_slugs = base.get(number)
        if base_slugs is None:
            continue
        new_slugs = head_slugs - base_slugs
        if not new_slugs:
            continue
        allowlisted = ALLOWLISTED_COLLISIONS.get(number)
        if allowlisted == base_slugs and head_slugs <= allowlisted:
            continue
        collisions.append(
            f"research number {number}\n"
            f"  HEAD: {format_slugs(head_slugs)}\n"
            f"  {base_label}: {format_slugs(base_slugs)}"
        )
    if collisions:
        fail(
            "number already assigned in the base; take a new unused "
            f"number ({base_label}):\n" + "\n".join(collisions)
        )


def fail_refresh(authority: str, ref: str, detail: str) -> None:
    fail(
        f"cannot refresh canonical main from {authority} ({ref}): {detail}. "
        "Refusing to trust a stale or missing snapshot."
    )


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Reject numbered research records assigned to different slugs."
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=SCRIPT_ROOT,
        help="repository root to scan (default: this checkout)",
    )
    parser.add_argument(
        "--authority",
        default=CANONICAL_AUTHORITY,
        help=(
            "canonical git URL or path to fetch for pushed main "
            f"(default: {CANONICAL_AUTHORITY})"
        ),
    )
    parser.add_argument(
        "--ref",
        default=CANONICAL_REF,
        help=f"ref to fetch from authority (default: {CANONICAL_REF})",
    )
    parser.add_argument(
        "--local-base",
        default=None,
        help=(
            "skip fetch and compare against this existing ref; diagnostics "
            "and hermetic tests only, never the worker or CI enforcement path"
        ),
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> None:
    args = parse_args(argv)
    root = args.root.resolve()
    head_paths = working_tree_paths(root)
    head = occupancy_from_paths(head_paths, "working tree")
    if args.local_base:
        base_sha = resolve_commit(root, args.local_base)
        if base_sha is None:
            fail(
                f"cannot resolve local base {args.local_base!r}; refusing to "
                "trust a missing snapshot"
            )
        base_label = f"local base {args.local_base}"
        base_paths = git_paths(root, base_sha)
        base_contents: dict[str, bytes] = {}
    else:
        try:
            _base_sha, base_paths, base_contents = refresh_authority(
                args.authority, args.ref
            )
        except AuthorityError as exc:
            fail_refresh(args.authority, args.ref, str(exc))
        base_label = f"canonical main ({args.authority} {args.ref})"
    check_against_base(
        root,
        head,
        head_paths,
        base_paths,
        base_contents,
        base_label,
        base_sha if args.local_base else None,
    )


if __name__ == "__main__":
    main()
