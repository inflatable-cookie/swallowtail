#!/usr/bin/env bash
# Mutation evidence for stale-base currentness card collisions.
# Uses throwaway git repos under mktemp. Does not touch user worktrees.
set -euo pipefail

collision_repo_root=$(cd "$(dirname "$0")/../.." && pwd)
collision_checker=(python3 "$collision_repo_root/scripts/check-roadmap-number-collision.py")
collision_scratch=$(mktemp -d)
trap 'rm -rf "$collision_scratch"' EXIT

collision_git() {
  local collision_cwd=$1
  shift
  git -C "$collision_cwd" \
    -c user.email=collision-test@example.com \
    -c user.name='Collision Test' \
    "$@"
}

collision_init_repo() {
  local collision_cwd=$1
  mkdir -p "$collision_cwd/docs/roadmaps/g04/batch-cards"
  collision_git "$collision_cwd" init -q -b main
  printf '# 075 Ollama\n' >"$collision_cwd/docs/roadmaps/g04/batch-cards/075-ollama-0-32-15-claim.md"
  collision_git "$collision_cwd" add docs
  collision_git "$collision_cwd" commit -q -m '075 on planning base'
}

collision_expect_failure() {
  local collision_expected=$1
  shift
  local collision_output
  local collision_exit_status=0
  collision_output=$("$@" 2>&1) || collision_exit_status=$?
  if [[ "$collision_exit_status" -eq 0 ]]; then
    printf 'expected collision checker failure\n%s\n' "$collision_output" >&2
    exit 1
  fi
  if [[ "$collision_output" != *"$collision_expected"* ]]; then
    printf 'collision checker failure changed:\n%s\n' "$collision_output" >&2
    exit 1
  fi
}

# Reproduce PRs 24-30: stale currentness branch allocates 076-claude while
# pushed main already assigned 076-kimi to g04.024.
collision_stale=$collision_scratch/stale
collision_init_repo "$collision_stale"
collision_git "$collision_stale" checkout -q -b stale-currentness
collision_git "$collision_stale" checkout -q main
printf '# 076 Kimi Platform\n' >"$collision_stale/docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md"
collision_git "$collision_stale" add docs
collision_git "$collision_stale" commit -q -m 'g04.024 compiles cards 076-078'
collision_git "$collision_stale" checkout -q stale-currentness
printf '# 076 Claude Code identity\n' >"$collision_stale/docs/roadmaps/g04/batch-cards/076-claude-code-2-1-238-identity.md"
collision_git "$collision_stale" add docs
collision_git "$collision_stale" commit -q -m 'stale currentness allocates 076-077'
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" --root "$collision_stale" --base main
collision_expect_failure \
  '076-claude-code-2-1-238-identity.md' \
  "${collision_checker[@]}" --root "$collision_stale" --base main
collision_expect_failure \
  '076-kimi-platform-chat-addable-descriptor.md' \
  "${collision_checker[@]}" --root "$collision_stale" --base main

# In-tree duplicate of the same number, as after a naive merge of both files.
collision_intree=$collision_scratch/intree
collision_init_repo "$collision_intree"
printf '# 076 Kimi Platform\n' >"$collision_intree/docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md"
printf '# 076 Claude Code identity\n' >"$collision_intree/docs/roadmaps/g04/batch-cards/076-claude-code-2-1-238-identity.md"
collision_expect_failure \
  'occupies multiple files' \
  "${collision_checker[@]}" --root "$collision_intree" --base main

# Falsification: serial currentness restacked onto current main (079-080).
collision_serial=$collision_scratch/serial
collision_init_repo "$collision_serial"
printf '# 076 Kimi Platform\n' >"$collision_serial/docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md"
printf '# 077 Kimi Platform\n' >"$collision_serial/docs/roadmaps/g04/batch-cards/077-kimi-platform-chat-admission-and-prepare.md"
printf '# 078 Kimi Platform\n' >"$collision_serial/docs/roadmaps/g04/batch-cards/078-kimi-platform-chat-refresh-catalogue-and-047.md"
collision_git "$collision_serial" add docs
collision_git "$collision_serial" commit -q -m 'g04.024 owns 076-078'
collision_git "$collision_serial" checkout -q -b serial-currentness
printf '# 079 Claude Code identity\n' >"$collision_serial/docs/roadmaps/g04/batch-cards/079-claude-code-2-1-238-identity.md"
printf '# 080 Claude Code claim\n' >"$collision_serial/docs/roadmaps/g04/batch-cards/080-claude-code-2-1-238-claim.md"
collision_git "$collision_serial" add docs
collision_git "$collision_serial" commit -q -m 'restacked currentness 079-080'
"${collision_checker[@]}" --root "$collision_serial" --base main >/dev/null

# Falsification: same-number retitle against an unchanged base path.
collision_retitle=$collision_scratch/retitle
collision_init_repo "$collision_retitle"
printf '# 076 old slug\n' >"$collision_retitle/docs/roadmaps/g04/batch-cards/076-old-slug.md"
collision_git "$collision_retitle" add docs
collision_git "$collision_retitle" commit -q -m '076 old slug'
collision_git "$collision_retitle" checkout -q -b retitle
collision_git "$collision_retitle" rm -q docs/roadmaps/g04/batch-cards/076-old-slug.md
printf '# 076 new slug\n' >"$collision_retitle/docs/roadmaps/g04/batch-cards/076-new-slug.md"
collision_git "$collision_retitle" add docs
collision_git "$collision_retitle" commit -q -m 'retitle 076'
"${collision_checker[@]}" --root "$collision_retitle" --base main >/dev/null

# Current Swallowtail tree against origin/main still passes.
"${collision_checker[@]}" --root "$collision_repo_root" --base origin/main >/dev/null

printf 'roadmap number collision tests passed\n'
