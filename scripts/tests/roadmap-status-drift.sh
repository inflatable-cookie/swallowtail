#!/usr/bin/env bash
# Hermetic fixtures for the flattened task-index registry, retired status
# buckets, and nested-dispatch rejection. Injects via --root; does not read
# the live checkout's roadmap indexes. Since g05.057, terminal task state is
# lifecycle-owned: the checker validates the `## Tasks` link registry and the
# nested-dispatch rejection, not hand-maintained status.
set -euo pipefail

status_tests_dir=$(cd "$(dirname "$0")" && pwd)
status_checker_root=$(cd "$status_tests_dir/../.." && pwd)
status_checker=(python3 "$status_checker_root/scripts/check-roadmap-status-drift.py")
status_scratch=$(mktemp -d)
trap 'rm -rf "$status_scratch"' EXIT

status_run() {
  "${status_checker[@]}" --root "$1"
}

status_expect_pass() {
  local status_output
  if ! status_output=$(status_run "$1" 2>&1); then
    printf 'expected status checker pass\n%s\n' "$status_output" >&2
    exit 1
  fi
}

status_expect_failure() {
  local status_expected=$1
  local status_tree=$2
  local status_output
  local status_exit_code=0
  status_output=$(status_run "$status_tree" 2>&1) || status_exit_code=$?
  if [[ "$status_exit_code" -eq 0 ]]; then
    printf 'expected status checker failure\n%s\n' "$status_output" >&2
    exit 1
  fi
  if [[ "$status_output" != *"$status_expected"* ]]; then
    printf 'status checker failure changed:\n%s\n' "$status_output" >&2
    exit 1
  fi
}

status_write() {
  local status_file=$1
  local status_body=$2
  mkdir -p "$(dirname "$status_file")"
  printf '%s\n' "$status_body" >"$status_file"
}

status_seed_tasks() {
  local status_tree=$1
  status_write "$status_tree/docs/roadmaps/generation-index.md" \
'# Roadmap Generation Index

| Generation | Status | Focus |
| --- | --- | --- |
| `g05` | active | fixture |'
  status_write "$status_tree/docs/roadmaps/g05/001-alpha.md" '# 001 alpha'
  status_write "$status_tree/docs/roadmaps/g05/002-beta.md" '# 002 beta'
  status_write "$status_tree/docs/roadmaps/g05/003-gamma.md" '# 003 gamma'
  status_write "$status_tree/docs/roadmaps/g05/004-delta.md" '# 004 delta'
  status_write "$status_tree/docs/roadmaps/g05/005-epsilon.md" '# 005 epsilon'
}

status_write_index() {
  local status_tree=$1
  status_write "$status_tree/docs/roadmaps/g05/README.md" \
'# g05

## Tasks

- [001-alpha.md](./001-alpha.md) — alpha
- [002-beta.md](./002-beta.md) — beta
- [003-gamma.md](./003-gamma.md) — gamma
- [004-delta.md](./004-delta.md) — delta
- [005-epsilon.md](./005-epsilon.md) — epsilon'
}

# A flat link registry listing every task exactly once passes.
status_tree=$status_scratch/tree
status_seed_tasks "$status_tree"
status_write_index "$status_tree"
status_expect_pass "$status_tree"

# Every task file must be indexed.
status_unindexed=$status_scratch/unindexed
status_seed_tasks "$status_unindexed"
status_write_index "$status_unindexed"
status_write "$status_unindexed/docs/roadmaps/g05/006-zeta.md" '# 006 zeta'
status_expect_failure "task 006-zeta.md is not indexed" "$status_unindexed"

# A task may be indexed only once.
status_duplicate=$status_scratch/duplicate
status_seed_tasks "$status_duplicate"
status_write_index "$status_duplicate"
printf '%s\n' '- [003-gamma.md](./003-gamma.md) — gamma again' \
  >>"$status_duplicate/docs/roadmaps/g05/README.md"
status_expect_failure "task 003-gamma.md is indexed more than once" "$status_duplicate"

# Every indexed link must resolve.
status_missing=$status_scratch/missing
status_seed_tasks "$status_missing"
status_write_index "$status_missing"
rm "$status_missing/docs/roadmaps/g05/004-delta.md"
status_expect_failure "task index links missing file 004-delta.md" "$status_missing"

# The index must carry a `## Tasks` section.
status_no_section=$status_scratch/no-section
status_seed_tasks "$status_no_section"
status_write "$status_no_section/docs/roadmaps/g05/README.md" '# g05'
status_expect_failure "task index has no \`## Tasks\` section" "$status_no_section"

# Retired status buckets under `## Tasks` are a migration defect.
status_bucket=$status_scratch/bucket
status_seed_tasks "$status_bucket"
status_write_index "$status_bucket"
printf '%s\n' '' '### Completed' >>"$status_bucket/docs/roadmaps/g05/README.md"
status_expect_failure "retired status bucket under \`## Tasks\`" "$status_bucket"

# Nested dispatch level is rejected.
status_legacy=$status_scratch/legacy
status_seed_tasks "$status_legacy"
status_write_index "$status_legacy"
mkdir -p "$status_legacy/docs/roadmaps/g05/batch-cards"
status_write "$status_legacy/docs/roadmaps/g05/batch-cards/009-legacy.md" '# 009'
status_expect_failure "legacy nested dispatch level remains" "$status_legacy"

status_link=$status_scratch/legacy-link
status_seed_tasks "$status_link"
status_write_index "$status_link"
printf '%s\n' '' '- [legacy](batch-cards/009-legacy.md)' \
  >>"$status_link/docs/roadmaps/g05/001-alpha.md"
status_expect_failure "nested batch-cards/ link remains" "$status_link"

status_section=$status_scratch/legacy-section
status_seed_tasks "$status_section"
status_write_index "$status_section"
printf '\n## Batch Cards\n' >>"$status_section/docs/roadmaps/g05/001-alpha.md"
status_expect_failure "legacy dispatch structure remains" "$status_section"

status_verb=$status_scratch/legacy-verb
status_seed_tasks "$status_verb"
status_write_index "$status_verb"
printf '\nExecute card 001 when ready.\n' >>"$status_verb/docs/roadmaps/g05/001-alpha.md"
status_expect_failure "legacy dispatch structure remains" "$status_verb"

# An ambient root must never override --root or production argv.
status_passing=$status_scratch/passing
status_failing=$status_scratch/failing
mkdir -p "$status_passing" "$status_failing"
status_seed_tasks "$status_passing"
status_write_index "$status_passing"
status_seed_tasks "$status_failing"
status_write_index "$status_failing"
printf '%s\n' '- [003-gamma.md](./003-gamma.md) — gamma again' \
  >>"$status_failing/docs/roadmaps/g05/README.md"

status_env_output=$(
  SWALLOWTAIL_STATUS_CHECK_ROOT=$status_passing \
    "${status_checker[@]}" --root "$status_failing" 2>&1
) && {
  printf 'ambient status-check root hid fixture drift\n%s\n' "$status_env_output" >&2
  exit 1
}
if [[ "$status_env_output" != *"task 003-gamma.md is indexed more than once"* ]]; then
  printf 'status checker failure changed under ambient root:\n%s\n' "$status_env_output" >&2
  exit 1
fi

status_env_pass=$(
  SWALLOWTAIL_STATUS_CHECK_ROOT=$status_failing \
    "${status_checker[@]}" --root "$status_passing" 2>&1
) || {
  printf 'ambient status-check root overrode --root\n%s\n' "$status_env_pass" >&2
  exit 1
}

if ! SWALLOWTAIL_STATUS_CHECK_ROOT=$status_failing "${status_checker[@]}" >/dev/null; then
  printf 'production argv honored ambient status-check root\n' >&2
  exit 1
fi

if grep -q SWALLOWTAIL_STATUS_CHECK_ROOT "$status_checker_root/scripts/check-roadmap-status-drift.py"; then
  printf 'status checker must not read SWALLOWTAIL_STATUS_CHECK_ROOT\n' >&2
  exit 1
fi
if ! grep -q SWALLOWTAIL_STATUS_CHECK_ROOT "$status_checker_root/scripts/git-hooks/pre-push"; then
  printf 'pre-push must drop SWALLOWTAIL_STATUS_CHECK_ROOT\n' >&2
  exit 1
fi

printf 'roadmap status drift tests passed\n'
