#!/usr/bin/env bash
# Shared helpers for hermetic roadmap-number collision tests.

collision_canonical_https='https://github.com/inflatable-cookie/swallowtail.git'

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

collision_git_dir_file() {
  local collision_cwd=$1
  local collision_name=$2
  printf '%s/%s\n' "$(git -C "$collision_cwd" rev-parse --absolute-git-dir)" "$collision_name"
}
