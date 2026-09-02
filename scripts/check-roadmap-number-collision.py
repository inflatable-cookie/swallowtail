#!/usr/bin/env python3
"""Fail when numbered roadmap or batch-card files collide.

Canonical pushed-main authority is
``https://github.com/inflatable-cookie/swallowtail.git`` ``refs/heads/main``.
This checker fetches that commit as objects immediately before enforcement
and aborts if the advertised object cannot be retrieved. The fetch has no
destination ref: it does not create, follow, or overwrite
``refs/swallowtail/roadmap-authority`` or any other ref, and it does not
import tags or write ``FETCH_HEAD``. ``origin/main`` is not authority: a
fork's origin, a stale tracking ref, or a failed fetch must not pass.

In-tree uniqueness is required. A number that canonical main already
assigns to a path may not appear on a different path in this tree.
Same-path content edits are allowed. Reuse via delete-and-add, rename, or
edited rename is not; take a new unused number instead.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path


SCRIPT_ROOT = Path(__file__).resolve().parent.parent
CANONICAL_AUTHORITY = "https://github.com/inflatable-cookie/swallowtail.git"
CANONICAL_REF = "refs/heads/main"
ADVERTISED_OBJECT = re.compile(r"^[0-9a-f]{40}(?:[0-9a-f]{24})?$")
NUMBERED_NAME = re.compile(r"^(?P<number>\d{3})-.+\.md$")
GENERATION_DIR = re.compile(r"^g\d{2}$")
NUMBERED_PATH = re.compile(
    r"^docs/roadmaps/(?P<generation>g\d{2})/"
    r"(?:(?P<card_dir>batch-cards/)?(?P<name>\d{3}-[^/]+\.md))$"
)


def fail(message: str) -> None:
    print(f"roadmap number collision check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def classify(relative: str) -> tuple[str, str, str] | None:
    match = NUMBERED_PATH.fullmatch(relative)
    if match is None:
        return None
    name_match = NUMBERED_NAME.fullmatch(match.group("name"))
    if name_match is None:
        return None
    kind = "card" if match.group("card_dir") else "milestone"
    return match.group("generation"), kind, name_match.group("number")


def occupancy_from_paths(paths: list[str], source: str) -> dict[tuple[str, str, str], str]:
    grouped: dict[tuple[str, str, str], list[str]] = defaultdict(list)
    for relative in paths:
        key = classify(relative)
        if key is None:
            continue
        grouped[key].append(relative)
    occupancy: dict[tuple[str, str, str], str] = {}
    for key, files in sorted(grouped.items()):
        unique = sorted(set(files))
        if len(unique) > 1:
            listed = "\n".join(f"  {path}" for path in unique)
            generation, kind, number = key
            fail(
                f"{source} {generation} {kind} {number} occupies multiple files:\n{listed}"
            )
        occupancy[key] = unique[0]
    return occupancy


def working_tree_paths(root: Path) -> list[str]:
    roadmaps = root / "docs" / "roadmaps"
    if not roadmaps.is_dir():
        fail("missing docs/roadmaps")
    paths: list[str] = []
    for generation_dir in sorted(path for path in roadmaps.iterdir() if path.is_dir()):
        if not GENERATION_DIR.fullmatch(generation_dir.name):
            continue
        for child in generation_dir.glob("*.md"):
            if NUMBERED_NAME.fullmatch(child.name):
                paths.append(child.relative_to(root).as_posix())
        cards = generation_dir / "batch-cards"
        if not cards.is_dir():
            continue
        for child in cards.glob("*.md"):
            if NUMBERED_NAME.fullmatch(child.name):
                paths.append(child.relative_to(root).as_posix())
    return sorted(paths)


def git_run(root: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", "-C", str(root), *args],
        check=False,
        capture_output=True,
        text=True,
    )


def git_text(root: Path, *args: str) -> str:
    proc = git_run(root, *args)
    if proc.returncode != 0:
        detail = proc.stderr.strip() or proc.stdout.strip() or f"exit {proc.returncode}"
        fail(f"git {' '.join(args)} failed: {detail}")
    return proc.stdout


def git_paths(root: Path, ref: str) -> list[str]:
    output = git_text(root, "ls-tree", "-r", "--name-only", ref)
    return [line for line in output.splitlines() if line]


def resolve_commit(root: Path, ref: str) -> str | None:
    proc = git_run(root, "rev-parse", "--verify", "--quiet", f"{ref}^{{commit}}")
    if proc.returncode != 0:
        return None
    sha = proc.stdout.strip()
    return sha or None


def format_key(key: tuple[str, str, str]) -> str:
    generation, kind, number = key
    return f"{generation} {kind} {number}"


def fail_refresh(authority: str, ref: str, detail: str) -> None:
    fail(
        f"cannot refresh canonical main from {authority} ({ref}): {detail}. "
        "Refusing to trust a stale or missing snapshot."
    )


def advertised_authority_sha(root: Path, authority: str, ref: str) -> str:
    proc = git_run(root, "ls-remote", "--refs", authority, ref)
    if proc.returncode != 0:
        detail = proc.stderr.strip() or proc.stdout.strip() or f"exit {proc.returncode}"
        fail_refresh(authority, ref, detail)
    lines = [line for line in proc.stdout.splitlines() if line.strip()]
    if len(lines) != 1:
        fail_refresh(
            authority,
            ref,
            f"ls-remote advertised {len(lines)} objects; expected exactly one",
        )
    sha, separator, advertised = lines[0].partition("\t")
    if not separator:
        parts = lines[0].split()
        if len(parts) != 2:
            fail_refresh(authority, ref, f"malformed ls-remote line: {lines[0]!r}")
        sha, advertised = parts
    if not ADVERTISED_OBJECT.fullmatch(sha) or advertised != ref:
        fail_refresh(
            authority,
            ref,
            f"ls-remote advertised {lines[0]!r}; expected {ref}",
        )
    return sha


def refresh_authority(root: Path, authority: str, ref: str) -> str:
    sha = advertised_authority_sha(root, authority, ref)
    proc = git_run(
        root,
        "fetch",
        "--quiet",
        "--no-tags",
        "--no-write-fetch-head",
        authority,
        sha,
    )
    if proc.returncode != 0:
        detail = proc.stderr.strip() or proc.stdout.strip() or f"exit {proc.returncode}"
        fail_refresh(authority, ref, detail)
    kind = git_run(root, "cat-file", "-t", sha)
    if kind.returncode != 0 or kind.stdout.strip() != "commit":
        fail_refresh(
            authority,
            ref,
            f"advertised object {sha} is not a local commit after fetch",
        )
    return sha


def check_against_base(
    root: Path,
    head: dict[tuple[str, str, str], str],
    base_sha: str,
    base_label: str,
) -> None:
    base_occ = occupancy_from_paths(git_paths(root, base_sha), base_label)
    collisions: list[str] = []
    for key, head_path in sorted(head.items()):
        base_path = base_occ.get(key)
        if base_path is None or base_path == head_path:
            continue
        collisions.append(
            f"{format_key(key)}\n  HEAD: {head_path}\n  {base_label}: {base_path}"
        )
    if collisions:
        fail(
            "stale-base number collision; take a new unused number on current "
            f"canonical main ({base_label}):\n" + "\n".join(collisions)
        )


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Reject colliding numbered roadmap and batch-card files."
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
    head = occupancy_from_paths(working_tree_paths(root), "HEAD")
    if args.local_base:
        base_sha = resolve_commit(root, args.local_base)
        if base_sha is None:
            fail(
                f"cannot resolve local base {args.local_base!r}; refusing to "
                "trust a missing snapshot"
            )
        base_label = args.local_base
    else:
        base_sha = refresh_authority(root, args.authority, args.ref)
        base_label = f"{args.authority} {args.ref}"
    check_against_base(root, head, base_sha, base_label)
    print(
        "roadmap number collision check passed: "
        f"{len(head)} numbered milestone/card files unique against {base_label} "
        f"at {base_sha[:12]}"
    )


if __name__ == "__main__":
    main()
