#!/usr/bin/env bash
# Hermetic mutation evidence for canonical-main roadmap card collisions.
# Throwaway repos and remotes only. Does not read the live checkout or its remotes.
set -euo pipefail

collision_checker_root=$(cd "$(dirname "$0")/../.." && pwd)
collision_checker=(
  python3 "$collision_checker_root/scripts/check-roadmap-number-collision.py"
)
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

collision_write_card() {
  local collision_cwd=$1
  local collision_file=$2
  local collision_body=$3
  mkdir -p "$(dirname "$collision_cwd/$collision_file")"
  printf '%s\n' "$collision_body" >"$collision_cwd/$collision_file"
}

collision_seed_work() {
  local collision_cwd=$1
  mkdir -p "$collision_cwd/docs/roadmaps/g04/batch-cards"
  collision_git "$collision_cwd" init -q -b main
  collision_write_card \
    "$collision_cwd" \
    docs/roadmaps/g04/batch-cards/075-ollama-0-32-15-claim.md \
    '# 075 Ollama'
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

collision_expect_pass() {
  local collision_output
  if ! collision_output=$("$@" 2>&1); then
    printf 'expected collision checker pass\n%s\n' "$collision_output" >&2
    exit 1
  fi
}

# Canonical and fork remotes start at 075. Canonical later gains 076-kimi;
# the fork does not. Worker clones the fork, so origin is not authority.
collision_seed=$collision_scratch/seed
collision_canonical=$collision_scratch/canonical.git
collision_fork=$collision_scratch/fork.git
collision_seed_work "$collision_seed"
collision_git "$collision_scratch" clone -q --bare "$collision_seed" "$collision_canonical"
collision_git "$collision_scratch" clone -q --bare "$collision_seed" "$collision_fork"

collision_updater=$collision_scratch/updater
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_updater"
collision_write_card \
  "$collision_updater" \
  docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md \
  '# 076 Kimi Platform'
collision_write_card \
  "$collision_updater" \
  docs/roadmaps/g04/batch-cards/077-kimi-platform-chat-admission-and-prepare.md \
  '# 077 Kimi Platform'
collision_write_card \
  "$collision_updater" \
  docs/roadmaps/g04/batch-cards/078-kimi-platform-chat-refresh-catalogue-and-047.md \
  '# 078 Kimi Platform'
collision_git "$collision_updater" add docs
collision_git "$collision_updater" commit -q -m 'g04.024 compiles cards 076-078'
collision_git "$collision_updater" push -q origin HEAD:main

collision_worker=$collision_scratch/worker
collision_git "$collision_scratch" clone -q "$collision_fork" "$collision_worker"
collision_git "$collision_worker" checkout -q -b stale-currentness
collision_write_card \
  "$collision_worker" \
  docs/roadmaps/g04/batch-cards/076-claude-code-2-1-238-identity.md \
  '# 076 Claude Code identity'
collision_git "$collision_worker" add docs
collision_git "$collision_worker" commit -q -m 'stale currentness allocates 076'

# Stale/wrong origin snapshot would pass; canonical refresh fails.
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --local-base origin/main
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --authority "$collision_fork"
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --authority "$collision_canonical"
collision_expect_failure \
  '076-claude-code-2-1-238-identity.md' \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --authority "$collision_canonical"
collision_expect_failure \
  '076-kimi-platform-chat-addable-descriptor.md' \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --authority "$collision_canonical"

# Detached HEAD still fails after canonical refresh.
collision_git "$collision_worker" checkout -q --detach HEAD
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --authority "$collision_canonical"
collision_git "$collision_worker" checkout -q stale-currentness

# Missing authority and failed fetch fail closed.
collision_expect_failure \
  'cannot refresh canonical main' \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --authority "$collision_scratch/missing.git"
collision_expect_failure \
  'cannot resolve local base' \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --local-base 'refs/heads/does-not-exist'

# In-tree untracked duplicate of the same number.
collision_intree=$collision_scratch/intree
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_intree"
collision_write_card \
  "$collision_intree" \
  docs/roadmaps/g04/batch-cards/076-claude-code-2-1-238-identity.md \
  '# 076 Claude Code identity'
collision_expect_failure \
  'occupies multiple files' \
  "${collision_checker[@]}" \
  --root "$collision_intree" \
  --authority "$collision_canonical"

# Same-path content edit remains allowed.
collision_edit=$collision_scratch/same-path-edit
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_edit"
printf '\n# edited in place\n' >>"$collision_edit/docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md"
collision_git "$collision_edit" add docs
collision_git "$collision_edit" commit -q -m 'edit 076 in place'
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_edit" \
  --authority "$collision_canonical"

# Delete plus unrelated add reuses the number.
collision_unrelated=$collision_scratch/unrelated
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_unrelated"
collision_git "$collision_unrelated" rm -q \
  docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md
collision_write_card \
  "$collision_unrelated" \
  docs/roadmaps/g04/batch-cards/076-unrelated.md \
  '# unrelated 076'
collision_git "$collision_unrelated" add docs
collision_git "$collision_unrelated" commit -q -m 'delete 076-kimi, add unrelated 076'
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_unrelated" \
  --authority "$collision_canonical"

# Exact-content rename reuses the number.
collision_rename=$collision_scratch/exact-rename
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_rename"
collision_git "$collision_rename" mv \
  docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md \
  docs/roadmaps/g04/batch-cards/076-kimi-platform-renamed.md
collision_git "$collision_rename" commit -q -m 'exact-content rename 076'
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_rename" \
  --authority "$collision_canonical"

# Edited rename reuses the number.
collision_edited=$collision_scratch/edited-rename
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_edited"
collision_git "$collision_edited" mv \
  docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md \
  docs/roadmaps/g04/batch-cards/076-kimi-platform-retitled.md
printf '\n# retitled and edited\n' >>"$collision_edited/docs/roadmaps/g04/batch-cards/076-kimi-platform-retitled.md"
collision_git "$collision_edited" add docs
collision_git "$collision_edited" commit -q -m 'edited rename 076'
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_edited" \
  --authority "$collision_canonical"

# Serial restack onto current canonical main (079-080).
collision_serial=$collision_scratch/serial
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_serial"
collision_git "$collision_serial" checkout -q -b serial-currentness
collision_write_card \
  "$collision_serial" \
  docs/roadmaps/g04/batch-cards/079-claude-code-2-1-238-identity.md \
  '# 079 Claude Code identity'
collision_write_card \
  "$collision_serial" \
  docs/roadmaps/g04/batch-cards/080-claude-code-2-1-238-claim.md \
  '# 080 Claude Code claim'
collision_git "$collision_serial" add docs
collision_git "$collision_serial" commit -q -m 'restacked currentness 079-080'
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_serial" \
  --authority "$collision_canonical"

# Current-tree analogue: unique numbered files matching refreshed canonical main.
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_updater" \
  --authority "$collision_canonical"

printf 'roadmap number collision tests passed\n'
