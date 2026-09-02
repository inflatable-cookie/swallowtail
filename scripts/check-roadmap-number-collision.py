#!/usr/bin/env python3
"""Fail when numbered roadmap or batch-card files collide.

In-tree uniqueness is required. A number that this tree assigns to a
different file than the pushed base (`origin/main` by default) is the
stale-planning-base collision from PRs 24-30: g04.024 kept cards 076-078
while parallel currentness branches reused those ids from older bases.

A same-number retitle against an unchanged base path is allowed. Fetch
the base before allocating numbers; this checker does not fetch.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path


SCRIPT_ROOT = Path(__file__).resolve().parent.parent
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


def check_against_base(
    root: Path,
    head: dict[tuple[str, str, str], str],
    base_ref: str,
) -> None:
    base_sha = resolve_commit(root, base_ref)
    if base_sha is None:
        fail(
            f"cannot resolve base {base_ref!r}; fetch origin main before "
            "allocating numbered roadmaps or cards"
        )
    head_sha = resolve_commit(root, "HEAD")
    if head_sha is None:
        fail("cannot resolve HEAD")
    if head_sha == base_sha:
        return

    merge_base = git_text(root, "merge-base", "HEAD", base_sha).strip()
    base_occ = occupancy_from_paths(git_paths(root, base_sha), base_ref)
    merge_occ = occupancy_from_paths(git_paths(root, merge_base), "merge-base")

    collisions: list[str] = []
    for key, head_path in sorted(head.items()):
        base_path = base_occ.get(key)
        if base_path is None or base_path == head_path:
            continue
        merge_path = merge_occ.get(key)
        if merge_path == base_path:
            continue
        collisions.append(
            f"{format_key(key)}\n  HEAD: {head_path}\n  {base_ref}: {base_path}"
        )
    if collisions:
        fail(
            "stale-base number collision; restack onto current pushed main:\n"
            + "\n".join(collisions)
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
        "--base",
        default="origin/main",
        help="pushed base ref that owns allocated numbers (default: origin/main)",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> None:
    args = parse_args(argv)
    root = args.root.resolve()
    head = occupancy_from_paths(working_tree_paths(root), "HEAD")
    check_against_base(root, head, args.base)
    print(
        "roadmap number collision check passed: "
        f"{len(head)} numbered milestone/card files unique against {args.base}"
    )


if __name__ == "__main__":
    main()
