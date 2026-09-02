#!/usr/bin/env bash
# Hermetic insteadOf isolation, advertised-main snapshot, and SHA-fetch denial.

collision_rewrite_subject=$collision_scratch/rewrite-subject
collision_git "$collision_scratch" clone -q --no-tags \
  "$collision_fork" "$collision_rewrite_subject"
collision_write_card \
  "$collision_rewrite_subject" \
  docs/roadmaps/g04/batch-cards/076-claude-code-2-1-238-identity.md \
  '# 076 Claude Code identity'
collision_git "$collision_rewrite_subject" add docs
collision_git "$collision_rewrite_subject" commit -q -m '076 against malicious 075-only fork'
collision_install_isolation_traps "$collision_rewrite_subject"
collision_write_fetch_head_sentinel "$collision_rewrite_subject"

collision_assert_isolated_canonical() {
  local collision_label=$1
  shift
  collision_snapshot_graph \
    "$collision_rewrite_subject" "$collision_scratch/rewrite-before"
  collision_expect_failure \
    'stale-base number collision' \
    env "$@" "${collision_checker[@]}" \
    --root "$collision_rewrite_subject" \
    --authority "$collision_canonical"
  collision_snapshot_graph \
    "$collision_rewrite_subject" "$collision_scratch/rewrite-after"
  collision_assert_graph_unchanged \
    "$collision_scratch/rewrite-before" \
    "$collision_scratch/rewrite-after" \
    "$collision_label"
  collision_assert_no_imported_release_tags \
    "$collision_rewrite_subject" \
    "$collision_label"
}

collision_local_config=$(collision_git_dir_file "$collision_rewrite_subject" config)
collision_write_instead_of "$collision_local_config" "$collision_fork"
collision_assert_rewrite_honored \
  'repo-local insteadOf' \
  git -C "$collision_rewrite_subject"
collision_assert_isolated_canonical 'repo-local insteadOf'
git config --file "$collision_local_config" --unset-all \
  "url.file://${collision_fork}.insteadOf"

collision_global_home=$collision_scratch/hostile-home
mkdir -p "$collision_global_home"
collision_write_instead_of "$collision_global_home/.gitconfig" "$collision_fork"
collision_assert_rewrite_honored \
  'global insteadOf' \
  env HOME="$collision_global_home" GIT_CONFIG_GLOBAL="$collision_global_home/.gitconfig" git
collision_assert_isolated_canonical \
  'global insteadOf' \
  HOME="$collision_global_home" \
  GIT_CONFIG_GLOBAL="$collision_global_home/.gitconfig"

collision_system_config=$collision_scratch/system.gitconfig
collision_write_instead_of "$collision_system_config" "$collision_fork"
collision_assert_rewrite_honored \
  'system insteadOf' \
  env GIT_CONFIG_NOSYSTEM=0 GIT_CONFIG_SYSTEM="$collision_system_config" git
collision_assert_isolated_canonical \
  'system insteadOf' \
  GIT_CONFIG_NOSYSTEM=0 \
  GIT_CONFIG_SYSTEM="$collision_system_config"

collision_include_config=$collision_scratch/included.gitconfig
collision_write_instead_of "$collision_include_config" "$collision_fork"
git config --file "$collision_local_config" include.path "$collision_include_config"
collision_assert_rewrite_honored \
  'included insteadOf' \
  git -C "$collision_rewrite_subject"
collision_assert_isolated_canonical 'included insteadOf'
git config --file "$collision_local_config" --unset-all include.path

collision_assert_rewrite_honored \
  'env GIT_CONFIG_COUNT insteadOf' \
  env \
    GIT_CONFIG_COUNT=1 \
    GIT_CONFIG_KEY_0="url.file://${collision_fork}.insteadOf" \
    GIT_CONFIG_VALUE_0="$collision_canonical" \
    git
collision_assert_isolated_canonical \
  'env GIT_CONFIG_COUNT insteadOf' \
  GIT_CONFIG_COUNT=1 \
  GIT_CONFIG_KEY_0="url.file://${collision_fork}.insteadOf" \
  GIT_CONFIG_VALUE_0="$collision_canonical"

collision_moved_before=$(git -C "$collision_canonical" rev-parse HEAD)
collision_git "$collision_updater" commit --allow-empty -q -m 'canonical main moved'
collision_git "$collision_updater" push -q origin HEAD:main
collision_moved_after=$(git -C "$collision_canonical" rev-parse HEAD)
if [[ "$collision_moved_before" == "$collision_moved_after" ]]; then
  printf 'canonical main did not move\n' >&2
  exit 1
fi
collision_moved_output=$(
  "${collision_checker[@]}" \
    --root "$collision_updater" \
    --authority "$collision_canonical"
)
if [[ "$collision_moved_output" != *"${collision_moved_after:0:12}"* ]]; then
  printf 'checker did not use moved advertised SHA\n%s\n' "$collision_moved_output" >&2
  exit 1
fi
if [[ "$collision_moved_output" == *"${collision_moved_before:0:12}"* ]]; then
  printf 'checker used stale advertised SHA after main moved\n%s\n' \
    "$collision_moved_output" >&2
  exit 1
fi

collision_policy_sha=$(git -C "$collision_canonical" rev-parse HEAD)
python3 "$collision_cases/deny-sha-fetch-server.py" \
  --sha "$collision_policy_sha" \
  --port-file "$collision_scratch/policy.port" \
  >/dev/null 2>&1 &
collision_policy_pid=$!
disown "$collision_policy_pid" 2>/dev/null || true
collision_exit_hooks+=('kill "$collision_policy_pid" 2>/dev/null || true')
collision_policy_port=
collision_policy_wait=0
while [[ -z "$collision_policy_port" && "$collision_policy_wait" -lt 50 ]]; do
  if [[ -f "$collision_scratch/policy.port" ]]; then
    collision_policy_port=$(cat "$collision_scratch/policy.port")
  else
    sleep 0.05
    collision_policy_wait=$((collision_policy_wait + 1))
  fi
done
if [[ -z "$collision_policy_port" ]]; then
  printf 'deny-sha-fetch server did not publish a port\n' >&2
  exit 1
fi
collision_policy_url="http://127.0.0.1:${collision_policy_port}/canonical.git"
collision_expect_failure \
  'cannot refresh canonical main' \
  "${collision_checker[@]}" \
  --root "$collision_rewrite_subject" \
  --authority "$collision_policy_url"
collision_expect_failure \
  'fetch by object id denied' \
  "${collision_checker[@]}" \
  --root "$collision_rewrite_subject" \
  --authority "$collision_policy_url"
