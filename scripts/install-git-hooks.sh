#!/usr/bin/env bash
# Point this clone's local core.hooksPath at scripts/git-hooks.
# Idempotent. Safe in worktrees: the path is relative to each worktree root.
# Wired from `effigy bootstrap` and `effigy doctor` via the hooks:install task.
set -euo pipefail

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  printf '%s: not inside a git work tree\n' "$0" >&2
  exit 1
fi

repo_root=$(git rev-parse --show-toplevel)
hooks_relpath=scripts/git-hooks
hook_file=$repo_root/$hooks_relpath/pre-push

if [[ ! -f "$hook_file" ]]; then
  printf '%s: missing hook %s/pre-push\n' "$0" "$hooks_relpath" >&2
  exit 1
fi

if [[ ! -x "$hook_file" ]]; then
  chmod +x "$hook_file"
fi

current=$(git config --local --get core.hooksPath || true)
if [[ -n "$current" && "$current" != "$hooks_relpath" ]]; then
  printf '%s: local core.hooksPath is %s; not overwriting. Point it at %s\n' \
    "$0" "$current" "$hooks_relpath" >&2
  exit 1
fi

if [[ "$current" == "$hooks_relpath" ]]; then
  printf 'git hooks already installed: core.hooksPath=%s\n' "$hooks_relpath"
  exit 0
fi

git config --local core.hooksPath "$hooks_relpath"
printf 'installed git hooks: core.hooksPath=%s\n' "$hooks_relpath"
