#!/usr/bin/env python3
"""Bounded fake for a passing research check made stale by a base update."""

from __future__ import annotations

import os
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts" / "check-research-number-collision.py"
COMMAND_TIMEOUT_SECONDS = 120


def isolated_git_env() -> dict[str, str]:
    env = {
        key: value
        for key, value in os.environ.items()
        if not key.startswith("GIT_")
        or key in {"GIT_EXEC_PATH", "GIT_SSL_CAINFO", "GIT_SSL_CAPATH"}
    }
    env["GIT_CONFIG_GLOBAL"] = os.devnull
    env["GIT_CONFIG_NOSYSTEM"] = "1"
    env["GIT_TERMINAL_PROMPT"] = "0"
    return env


def git(root: Path | None, *args: str) -> str:
    command = [
        "git",
        "-c",
        "user.name=Research Freshness Test",
        "-c",
        "user.email=research-freshness-test@example.com",
    ]
    if root is not None:
        command.extend(["-C", str(root)])
    command.extend(args)
    result = subprocess.run(
        command,
        check=False,
        capture_output=True,
        text=True,
        timeout=COMMAND_TIMEOUT_SECONDS,
        env=isolated_git_env(),
    )
    if result.returncode != 0:
        raise AssertionError(
            f"git {' '.join(args)} failed ({result.returncode}):\n"
            f"{result.stdout}{result.stderr}"
        )
    return result.stdout.strip()


def checker(root: Path, authority: Path, *, expected: str) -> str:
    result = subprocess.run(
        [
            "python3",
            str(CHECKER),
            "--root",
            str(root),
            "--authority",
            str(authority),
        ],
        check=False,
        capture_output=True,
        text=True,
        timeout=COMMAND_TIMEOUT_SECONDS,
        env=isolated_git_env(),
    )
    output = result.stdout + result.stderr
    if expected == "pass" and result.returncode != 0:
        raise AssertionError(f"expected checker pass:\n{output}")
    if expected != "pass" and (result.returncode == 0 or expected not in output):
        raise AssertionError(
            f"expected checker failure containing {expected!r}:\n{output}"
        )
    return output.strip()


def add_record(root: Path, filename: str, heading: str) -> None:
    path = root / "docs" / "research" / filename
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(f"# {heading}\nFrozen fake evidence.\n", encoding="utf-8")
    git(root, "add", path.relative_to(root).as_posix())
    git(root, "commit", "-q", "-m", f"add {filename}")


def merged_candidate(
    head: str, authority: Path, branch: str, destination: Path
) -> tuple[str, str, str]:
    git(None, "clone", "-q", str(authority), str(destination))
    base = git(destination, "rev-parse", "HEAD")
    git(destination, "fetch", "-q", "origin", f"refs/heads/{branch}")
    git(destination, "merge", "--no-edit", "--no-ff", "FETCH_HEAD")
    candidate = git(destination, "rev-parse", "HEAD")
    tree = git(destination, "rev-parse", "HEAD^{tree}")
    parents = git(destination, "rev-list", "--parents", "-n", "1", "HEAD").split()
    if parents != [candidate, base, head]:
        raise AssertionError(f"candidate parents changed: {parents!r}")
    return base, candidate, tree


def run() -> None:
    with tempfile.TemporaryDirectory(prefix="swallowtail-research-freshness-") as raw:
        scratch = Path(raw)
        seed = scratch / "seed"
        canonical = scratch / "canonical.git"
        worker_a = scratch / "worker-a"
        worker_b = scratch / "worker-b"
        seed.mkdir()
        git(None, "init", "-q", "-b", "main", str(seed))
        (seed / "docs" / "research").mkdir(parents=True)
        (seed / "docs" / "research" / "100-seed.md").write_text(
            "# Research 100: Seed\nSeed evidence.\n", encoding="utf-8"
        )
        git(seed, "add", "docs/research/100-seed.md")
        git(seed, "commit", "-q", "-m", "seed canonical main")
        git(None, "clone", "-q", "--bare", str(seed), str(canonical))
        git(None, "clone", "-q", str(canonical), str(worker_a))
        git(None, "clone", "-q", str(canonical), str(worker_b))

        base_zero = git(canonical, "rev-parse", "refs/heads/main")
        add_record(worker_a, "900-alpha.md", "Research 900: Alpha")
        add_record(worker_b, "900-beta.md", "Research 900: Beta")
        head_a = git(worker_a, "rev-parse", "HEAD")
        head_b = git(worker_b, "rev-parse", "HEAD")
        git(worker_a, "push", "-q", "origin", "HEAD:refs/heads/pr/alpha")
        git(worker_b, "push", "-q", "origin", "HEAD:refs/heads/pr/beta")

        base_a, candidate_a, tree_a = merged_candidate(
            head_a, canonical, "pr/alpha", scratch / "candidate-a"
        )
        base_b, candidate_b, tree_b = merged_candidate(
            head_b, canonical, "pr/beta", scratch / "candidate-b"
        )
        if base_a != base_zero or base_b != base_zero:
            raise AssertionError("initial candidates did not share canonical base")
        checker(scratch / "candidate-a", canonical, expected="pass")
        checker(scratch / "candidate-b", canonical, expected="pass")

        # Model the published PR-head check: it remains green for this exact
        # head even after its tested canonical base moves. This ledger is a
        # finite fake, not an assertion about GitHub's internal storage.
        published = {
            head_a: {"base": base_a, "candidate": candidate_a, "tree": tree_a, "result": "success"},
            head_b: {"base": base_b, "candidate": candidate_b, "tree": tree_b, "result": "success"},
        }
        git(worker_a, "push", "-q", "origin", "HEAD:refs/heads/main")
        base_one = git(canonical, "rev-parse", "refs/heads/main")
        if base_one == base_zero or published[head_b]["result"] != "success":
            raise AssertionError("the stale green-check interval was not created")
        if published[head_b]["base"] != base_zero:
            raise AssertionError("the recorded check unexpectedly followed main")

        # Queue's inspected loose-base path admits this configuration. These
        # values are explicit fixture inputs; this fake does not execute
        # Queue's gate or simulate GitHub's check publication.
        stale_path_fixture = {
            "merge_state": "BEHIND",
            "mergeable": True,
            "update_branch_required": False,
            "published_check": published[head_b],
        }

        stale_worker_error = checker(
            worker_b,
            canonical,
            expected="number already assigned in the base",
        )
        latest = scratch / "candidate-b-on-current-main"
        base_latest, candidate_latest, tree_latest = merged_candidate(
            head_b, canonical, "pr/beta", latest
        )
        if base_latest != base_one or candidate_latest == candidate_b or tree_latest == tree_b:
            raise AssertionError("the current candidate did not bind the advanced base")
        current_tree_error = checker(
            latest,
            canonical,
            expected="working tree research number 900",
        )

        print(
            "initial strict checks passed independently against "
            f"base {base_zero[:12]} (heads {head_a[:12]}, {head_b[:12]})"
        )
        print(
            "after alpha merged, beta retained a green check tested at "
            f"{published[head_b]['base'][:12]} while main was {base_one[:12]}"
        )
        print(
            "stale-path fixture inputs: "
            f"merge_state={stale_path_fixture['merge_state']}, "
            f"mergeable={stale_path_fixture['mergeable']}, "
            "update_branch_required="
            f"{stale_path_fixture['update_branch_required']}"
        )
        print(
            "strict allocation rejected beta against refreshed main: "
            f"{stale_worker_error.splitlines()[0]}"
        )
        print(
            "strict candidate check rejected the current merged tree: "
            f"{current_tree_error.splitlines()[0]}"
        )


if __name__ == "__main__":
    run()
