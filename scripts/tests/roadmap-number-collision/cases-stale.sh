#!/usr/bin/env bash
# Stale origin, fork authority, detached HEAD, and fail-closed discovery.

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

collision_git "$collision_worker" checkout -q --detach HEAD
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_worker" \
  --authority "$collision_canonical"
collision_git "$collision_worker" checkout -q stale-currentness

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
