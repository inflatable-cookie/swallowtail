#!/usr/bin/env bash
# Throwaway canonical, fork, updater, and stale worker remotes.

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

collision_malicious_sha=$(git -C "$collision_fork" rev-parse HEAD)
