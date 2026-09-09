#!/usr/bin/env bash
# Hermetic fixtures for task Stopped section mapping and nested-dispatch rejection.
# Injects via --root; does not read the live checkout's roadmap indexes.
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

status_add_index() {
  local status_tree=$1
  local status_line=$2
  local status_anchor=$3
  python3 - "$status_tree/docs/roadmaps/g05/README.md" "$status_line" "$status_anchor" <<'PY'
import sys
index, line, anchor = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(index).read()
assert anchor in text, f"anchor missing: {anchor}"
open(index, "w").write(text.replace(anchor, anchor + "\n" + line, 1))
PY
}

status_seed_tasks() {
  local status_tree=$1
  status_write "$status_tree/docs/roadmaps/generation-index.md" \
'# Roadmap Generation Index

| Generation | Status | Focus |
| --- | --- | --- |
| `g05` | active | fixture |

g05 now has 1 completed task, honest evidence stops at 004, one ready task at 002, and one planned task at 001.'
  status_write "$status_tree/docs/roadmaps/g05/README.md" '# g05'
  status_write "$status_tree/docs/roadmaps/g05/001-planned.md" $'# 001\nStatus: planned'
  status_write "$status_tree/docs/roadmaps/g05/002-ready.md" $'# 002\nStatus: ready'
  status_write "$status_tree/docs/roadmaps/g05/003-blocked.md" $'# 003\nStatus: blocked'
  status_write "$status_tree/docs/roadmaps/g05/004-stopped.md" $'# 004\nStatus: stopped'
  status_write "$status_tree/docs/roadmaps/g05/005-complete.md" $'# 005\nStatus: complete'
}

status_write_index() {
  local status_tree=$1
  local status_stopped_section=$2
  local status_extra_in_stopped=${3-}
  local status_planned=$'## Tasks\n\n### Planned\n- [001-planned.md](./001-planned.md) — planned\n'
  local status_ready=$'### Ready\n- [002-ready.md](./002-ready.md) — ready\n'
  local status_blocked=$'### Blocked\n- [003-blocked.md](./003-blocked.md) — blocked\n'
  local status_completed=$'### Completed\n- [005-complete.md](./005-complete.md) — complete\n'
  local status_stopped=$'### Stopped\n'
  local status_stopped_task='- [004-stopped.md](./004-stopped.md) — stopped'

  case "$status_stopped_section" in
    Planned) status_planned+=$status_stopped_task$'\n' ;;
    Ready) status_ready+=$status_stopped_task$'\n' ;;
    Blocked) status_blocked+=$status_stopped_task$'\n' ;;
    Stopped) status_stopped+=$status_stopped_task$'\n' ;;
    Completed) status_completed+=$status_stopped_task$'\n' ;;
    *)
      printf 'unknown stopped section %s\n' "$status_stopped_section" >&2
      exit 1
      ;;
  esac

  case "$status_extra_in_stopped" in
    '') ;;
    planned)
      status_planned=$'## Tasks\n\n### Planned\n'
      status_stopped+='- [001-planned.md](./001-planned.md) — planned'$'\n'
      ;;
    ready)
      status_ready=$'### Ready\n'
      status_stopped+='- [002-ready.md](./002-ready.md) — ready'$'\n'
      ;;
    blocked)
      status_blocked=$'### Blocked\n'
      status_stopped+='- [003-blocked.md](./003-blocked.md) — blocked'$'\n'
      ;;
    complete)
      status_completed=$'### Completed\n'
      status_stopped+='- [005-complete.md](./005-complete.md) — complete'$'\n'
      ;;
    *)
      printf 'unknown extra task %s\n' "$status_extra_in_stopped" >&2
      exit 1
      ;;
  esac

  status_write "$status_tree/docs/roadmaps/g05/README.md" \
"# g05 Tasks

${status_planned}
${status_ready}
${status_blocked}
${status_stopped}
${status_completed}"
}

status_tree=$status_scratch/tree
status_seed_tasks "$status_tree"

status_write_index "$status_tree" Stopped
status_expect_pass "$status_tree"

status_write_index "$status_tree" Planned
status_expect_failure \
  "Status bucket is 'stopped' but index lists it under 'planned'" \
  "$status_tree"
status_write_index "$status_tree" Ready
status_expect_failure \
  "Status bucket is 'stopped' but index lists it under 'ready'" \
  "$status_tree"
status_write_index "$status_tree" Blocked
status_expect_failure \
  "Status bucket is 'stopped' but index lists it under 'blocked'" \
  "$status_tree"
status_write_index "$status_tree" Completed
status_expect_failure \
  "Status bucket is 'stopped' but index lists it under 'complete'" \
  "$status_tree"

status_write_index "$status_tree" Stopped planned
status_expect_failure \
  "Status bucket is 'planned' but index lists it under 'stopped'" \
  "$status_tree"
status_write_index "$status_tree" Stopped ready
status_expect_failure \
  "Status bucket is 'ready' but index lists it under 'stopped'" \
  "$status_tree"
status_write_index "$status_tree" Stopped blocked
status_expect_failure \
  "Status bucket is 'blocked' but index lists it under 'stopped'" \
  "$status_tree"
status_write_index "$status_tree" Stopped complete
status_expect_failure \
  "Status bucket is 'complete' but index lists it under 'stopped'" \
  "$status_tree"

# Nested dispatch level is rejected.
status_legacy=$status_scratch/legacy
status_seed_tasks "$status_legacy"
status_write_index "$status_legacy" Stopped
mkdir -p "$status_legacy/docs/roadmaps/g05/batch-cards"
status_write "$status_legacy/docs/roadmaps/g05/batch-cards/009-legacy.md" \
  $'# 009\nStatus: planned'
status_expect_failure "legacy nested dispatch level remains" "$status_legacy"

status_link=$status_scratch/legacy-link
status_seed_tasks "$status_link"
status_write_index "$status_link" Stopped
status_write "$status_link/docs/roadmaps/g05/006-extra.md" \
  $'# 006\nStatus: planned\n\n- [legacy](batch-cards/009-legacy.md)'
status_add_index "$status_link" \
  '- [006-extra.md](./006-extra.md) — planned' \
  '- [001-planned.md](./001-planned.md) — planned'
status_write "$status_link/docs/roadmaps/generation-index.md" \
'# Roadmap Generation Index

| Generation | Status | Focus |
| --- | --- | --- |
| `g05` | active | fixture |

g05 now has 1 completed task, honest evidence stops at 004, one ready task at 002, and planned tasks at 001 and 006.'
status_expect_failure "nested batch-cards/ link remains" "$status_link"
status_section=$status_scratch/legacy-section
status_seed_tasks "$status_section"
status_write_index "$status_section" Stopped
printf '\n## Batch Cards\n' >>"$status_section/docs/roadmaps/g05/001-planned.md"
status_expect_failure "legacy dispatch structure remains" "$status_section"

status_verb=$status_scratch/legacy-verb
status_seed_tasks "$status_verb"
status_write_index "$status_verb" Stopped
printf '\nExecute card 001 when ready.\n' >>"$status_verb/docs/roadmaps/g05/001-planned.md"
status_expect_failure "legacy dispatch structure remains" "$status_verb"

# Census parity is enforced per bucket.
status_census=$status_scratch/census
status_seed_tasks "$status_census"
status_write_index "$status_census" Stopped
status_write "$status_census/docs/roadmaps/generation-index.md" \
'# Roadmap Generation Index

| Generation | Status | Focus |
| --- | --- | --- |
| `g05` | active | fixture |

g05 now has 1 completed task, honest evidence stops at 004, one ready task at 003, and one planned task at 001.'
status_expect_failure "ready task set" "$status_census"

status_write "$status_census/docs/roadmaps/generation-index.md" \
'# Roadmap Generation Index

| Generation | Status | Focus |
| --- | --- | --- |
| `g05` | active | fixture |

g05 now has 1 completed task, honest evidence stops at 004, one ready task at 002, and one planned task at 003.'
status_expect_failure "planned task set" "$status_census"

status_write "$status_census/docs/roadmaps/generation-index.md" \
'# Roadmap Generation Index

| Generation | Status | Focus |
| --- | --- | --- |
| `g05` | active | fixture |

g05 now has 2 completed tasks, honest evidence stops at 004, one ready task at 002, and one planned task at 001.'
status_expect_failure "completed tasks but frontmatter has 1" "$status_census"

status_passing=$status_scratch/passing
status_failing=$status_scratch/failing
mkdir -p "$status_passing" "$status_failing"
status_seed_tasks "$status_passing"
status_write_index "$status_passing" Stopped
status_seed_tasks "$status_failing"
status_write_index "$status_failing" Planned

status_env_output=$(
  SWALLOWTAIL_STATUS_CHECK_ROOT=$status_passing \
    "${status_checker[@]}" --root "$status_failing" 2>&1
) && {
  printf 'ambient status-check root hid fixture drift\n%s\n' "$status_env_output" >&2
  exit 1
}
if [[ "$status_env_output" != *"Status bucket is 'stopped' but index lists it under 'planned'"* ]]; then
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
