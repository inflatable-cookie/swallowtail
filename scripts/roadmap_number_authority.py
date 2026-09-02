"""Fetch canonical roadmap occupancy from an isolated Git store."""

from __future__ import annotations

import os
import re
import subprocess
import tempfile
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path

CANONICAL_AUTHORITY = "https://github.com/inflatable-cookie/swallowtail.git"
CANONICAL_REF = "refs/heads/main"
ADVERTISED_OBJECT = re.compile(r"^[0-9a-f]{40}(?:[0-9a-f]{24})?$")
KEEP_GIT_ENV = frozenset({"GIT_EXEC_PATH", "GIT_SSL_CAINFO", "GIT_SSL_CAPATH"})
ISOLATED_CONFIG = (
    "http.sslVerify=true",
    "http.followRedirects=false",
    "core.quotePath=false",
)


class AuthorityError(Exception):
    """Canonical discovery or fetch failed closed."""


def isolated_git_env(home: Path) -> dict[str, str]:
    env = {
        key: value
        for key, value in os.environ.items()
        if not key.startswith("GIT_") or key in KEEP_GIT_ENV
    }
    env["HOME"] = str(home)
    env["XDG_CONFIG_HOME"] = str(home / "xdg")
    env["GIT_CONFIG_NOSYSTEM"] = "1"
    env["GIT_CONFIG_GLOBAL"] = str(home / "gitconfig")
    env["GIT_CONFIG_SYSTEM"] = str(home / "gitconfig")
    env["GIT_TERMINAL_PROMPT"] = "0"
    return env


def isolated_git(
    store: Path,
    env: dict[str, str],
    *args: str,
    text: bool = True,
) -> subprocess.CompletedProcess[str] | subprocess.CompletedProcess[bytes]:
    command = ["git", "--git-dir", str(store)]
    for setting in ISOLATED_CONFIG:
        command.extend(["-c", setting])
    command.extend(args)
    return subprocess.run(
        command,
        check=False,
        capture_output=True,
        text=text,
        env=env,
        cwd=str(store.parent),
    )


def git_detail(
    proc: subprocess.CompletedProcess[str] | subprocess.CompletedProcess[bytes],
) -> str:
    err = proc.stderr
    out = proc.stdout
    if isinstance(err, bytes):
        err = err.decode("utf-8", "replace")
    if isinstance(out, bytes):
        out = out.decode("utf-8", "replace")
    return err.strip() or out.strip() or f"exit {proc.returncode}"


def decode_z_paths(payload: bytes) -> list[str]:
    if not payload:
        return []
    parts = payload.split(b"\0")
    if parts and parts[-1] == b"":
        parts = parts[:-1]
    paths: list[str] = []
    for raw in parts:
        if not raw:
            continue
        try:
            paths.append(raw.decode("utf-8"))
        except UnicodeDecodeError as exc:
            raise AuthorityError(f"authority path is not utf-8: {raw!r}") from exc
    return paths


@contextmanager
def isolated_git_store() -> Iterator[tuple[Path, dict[str, str]]]:
    with tempfile.TemporaryDirectory(prefix="swallowtail-roadmap-auth-") as raw:
        root = Path(raw)
        home = root / "home"
        store = root / "store.git"
        home.mkdir()
        (home / "xdg").mkdir()
        (home / "gitconfig").write_text("", encoding="utf-8")
        env = isolated_git_env(home)
        proc = subprocess.run(
            ["git", "init", "--bare", "-q", "--", str(store)],
            check=False,
            capture_output=True,
            text=True,
            env=env,
            cwd=str(root),
        )
        if proc.returncode != 0:
            raise AuthorityError(f"isolated git init failed: {git_detail(proc)}")
        yield store, env


def resolved_authority_url(store: Path, env: dict[str, str], authority: str) -> str:
    proc = isolated_git(store, env, "ls-remote", "--get-url", "--", authority)
    if proc.returncode != 0:
        raise AuthorityError(git_detail(proc))
    resolved = proc.stdout.strip()
    if resolved != authority:
        raise AuthorityError(
            f"canonical URL rewritten to {resolved!r}; expected {authority!r}"
        )
    return resolved


def advertised_authority_sha(
    store: Path, env: dict[str, str], authority: str, ref: str
) -> str:
    proc = isolated_git(store, env, "ls-remote", "--refs", "--", authority, ref)
    if proc.returncode != 0:
        raise AuthorityError(git_detail(proc))
    lines = [line for line in proc.stdout.splitlines() if line.strip()]
    if len(lines) != 1:
        raise AuthorityError(
            f"ls-remote advertised {len(lines)} objects; expected exactly one"
        )
    sha, separator, advertised = lines[0].partition("\t")
    if not separator:
        parts = lines[0].split()
        if len(parts) != 2:
            raise AuthorityError(f"malformed ls-remote line: {lines[0]!r}")
        sha, advertised = parts
    if not ADVERTISED_OBJECT.fullmatch(sha) or advertised != ref:
        raise AuthorityError(f"ls-remote advertised {lines[0]!r}; expected {ref}")
    return sha


def fetch_authority_sha(
    store: Path, env: dict[str, str], authority: str, sha: str
) -> None:
    proc = isolated_git(
        store,
        env,
        "fetch",
        "--quiet",
        "--no-tags",
        "--no-write-fetch-head",
        "--",
        authority,
        sha,
    )
    if proc.returncode != 0:
        raise AuthorityError(git_detail(proc))
    kind = isolated_git(store, env, "cat-file", "-t", "--", sha)
    if kind.returncode != 0 or kind.stdout.strip() != "commit":
        raise AuthorityError(f"advertised object {sha} is not a commit after fetch")


def occupancy_paths(store: Path, env: dict[str, str], sha: str) -> list[str]:
    proc = isolated_git(
        store,
        env,
        "ls-tree",
        "-z",
        "-r",
        "--name-only",
        "--",
        sha,
        text=False,
    )
    if proc.returncode != 0:
        raise AuthorityError(git_detail(proc))
    return decode_z_paths(proc.stdout)


def refresh_authority(authority: str, ref: str) -> tuple[str, list[str]]:
    with isolated_git_store() as (store, env):
        resolved_authority_url(store, env, authority)
        sha = advertised_authority_sha(store, env, authority, ref)
        fetch_authority_sha(store, env, authority, sha)
        return sha, occupancy_paths(store, env, sha)
