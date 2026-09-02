#!/usr/bin/env bash
# Non-ASCII numbered path occupancy must participate in collision checks.

collision_cafe_file=$(printf 'docs/roadmaps/g04/batch-cards/081-caf\303\251-identity.md')
collision_write_card \
  "$collision_updater" \
  "$collision_cafe_file" \
  '# 081 cafe identity'
collision_git "$collision_updater" add docs
collision_git "$collision_updater" commit -q -m '081 non-ascii occupancy'
collision_git "$collision_updater" push -q origin HEAD:main

collision_cafe_stale=$collision_scratch/cafe-stale
collision_git "$collision_scratch" clone -q "$collision_fork" "$collision_cafe_stale"
collision_write_card \
  "$collision_cafe_stale" \
  docs/roadmaps/g04/batch-cards/081-plain-identity.md \
  '# 081 plain reuse'
collision_git "$collision_cafe_stale" add docs
collision_git "$collision_cafe_stale" commit -q -m 'reuse 081 on an ascii path'
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_cafe_stale" \
  --authority "$collision_canonical"
collision_expect_failure \
  '081-plain-identity.md' \
  "${collision_checker[@]}" \
  --root "$collision_cafe_stale" \
  --authority "$collision_canonical"
collision_expect_failure \
  "$(printf '081-caf\303\251-identity.md')" \
  "${collision_checker[@]}" \
  --root "$collision_cafe_stale" \
  --authority "$collision_canonical"
