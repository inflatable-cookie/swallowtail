#!/usr/bin/env bash
# Hermetic evidence for research-number collisions against local and pushed main.
set -euo pipefail

test_root=$(cd "$(dirname "$0")/../.." && pwd)
checker=(python3 "$test_root/scripts/check-research-number-collision.py")
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT

git_test() {
  local cwd=$1
  shift
  git -C "$cwd" -c user.email=research-test@example.com \
    -c user.name='Research Collision Test' "$@"
}

write_record() {
  local cwd=$1
  local name=$2
  mkdir -p "$cwd/docs/research"
  printf '# %s\n' "$name" >"$cwd/docs/research/$name"
}

expect_failure() {
  local expected=$1
  shift
  local output exit_status=0
  output=$("$@" 2>&1) || exit_status=$?
  if [[ "$exit_status" -eq 0 || "$output" != *"$expected"* ]]; then
    printf 'expected failure containing %s (exit %s):\n%s\n' \
      "$expected" "$exit_status" "$output" >&2
    exit 1
  fi
}

expect_pass() {
  local output
  if ! output=$("$@" 2>&1); then
    printf 'expected checker pass:\n%s\n' "$output" >&2
    exit 1
  fi
}

seed=$scratch/seed
canonical=$scratch/canonical.git
worker=$scratch/worker
updater=$scratch/updater
mkdir -p "$seed/docs/research"
git_test "$seed" init -q -b main
write_record "$seed" 100-seed.md
write_record "$seed" 328-goose-acp-1-50-1-failure-binding-reopen.md
write_record "$seed" 328-qoder-headless-1-1-54-identity.md
write_record "$seed" 337-opencode-acp-1-18-32-identity.md
write_record "$seed" 337-opencode-acp-unavailable-cells.tsv
git_test "$seed" add docs
git_test "$seed" commit -q -m 'seed research records'
git_test "$scratch" clone -q --bare "$seed" "$canonical"
git_test "$scratch" clone -q "$canonical" "$worker"
git_test "$scratch" clone -q "$canonical" "$updater"

# Same-slug companions and the exact 328 and 337 pairs remain valid.
write_record "$worker" 101-companion.md
write_record "$worker" 101-companion.tsv
write_record "$worker" 101-companion.csv
expect_pass "${checker[@]}" --root "$worker" --local-base main
expect_pass "${checker[@]}" --root "$worker" --authority "$canonical"

# Other differing slugs fail, including a substituted member of the 337 pair.
write_record "$worker" 102-first.md
write_record "$worker" 102-second.csv
expect_failure 'working tree research number 102' \
  "${checker[@]}" --root "$worker" --local-base main
rm "$worker/docs/research/102-first.md" \
  "$worker/docs/research/102-second.csv"
write_record "$worker" 328-unlisted-third-record.md
expect_failure 'working tree research number 328' \
  "${checker[@]}" --root "$worker" --local-base main
rm "$worker/docs/research/328-unlisted-third-record.md"
rm "$worker/docs/research/337-opencode-acp-unavailable-cells.tsv"
write_record "$worker" 337-opencode-unlisted-cells.tsv
expect_failure 'working tree research number 337' \
  "${checker[@]}" --root "$worker" --local-base main
rm "$worker/docs/research/337-opencode-unlisted-cells.tsv"
write_record "$worker" 337-opencode-acp-unavailable-cells.tsv

# A stale worker ref cannot hide a record already on canonical main.
write_record "$updater" 104-canonical-record.md
git_test "$updater" add docs/research/104-canonical-record.md
git_test "$updater" commit -q -m 'allocate research 104 on canonical main'
git_test "$updater" push -q origin HEAD:main
write_record "$worker" 104-stale-worker-record.md
expect_failure 'canonical main' \
  "${checker[@]}" --root "$worker" --authority "$canonical"

printf 'research number collision tests passed\n'
