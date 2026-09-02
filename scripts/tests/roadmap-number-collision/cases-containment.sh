#!/usr/bin/env bash
# Symbolic dest, similarly named tags, and FETCH_HEAD non-mutation.

collision_git "$collision_canonical" tag v0.1.0 HEAD
collision_git "$collision_canonical" tag v0.3.3 HEAD

collision_iso_pass=$collision_scratch/iso-pass
collision_git "$collision_scratch" clone -q --no-tags \
  "$collision_canonical" "$collision_iso_pass"
collision_install_isolation_traps "$collision_iso_pass"
collision_write_fetch_head_sentinel "$collision_iso_pass"
collision_snapshot_graph "$collision_iso_pass" "$collision_scratch/iso-pass-before"
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_iso_pass" \
  --authority "$collision_canonical"
collision_snapshot_graph "$collision_iso_pass" "$collision_scratch/iso-pass-after"
collision_assert_graph_unchanged \
  "$collision_scratch/iso-pass-before" \
  "$collision_scratch/iso-pass-after" \
  'successful isolated check'
collision_assert_no_imported_release_tags "$collision_iso_pass" 'successful isolated check'

collision_iso_fail=$collision_scratch/iso-fail
collision_git "$collision_scratch" clone -q --no-tags \
  "$collision_fork" "$collision_iso_fail"
collision_write_card \
  "$collision_iso_fail" \
  docs/roadmaps/g04/batch-cards/076-claude-code-2-1-238-identity.md \
  '# 076 Claude Code identity'
collision_git "$collision_iso_fail" add docs
collision_git "$collision_iso_fail" commit -q -m 'stale currentness allocates 076'
collision_install_isolation_traps "$collision_iso_fail"
collision_write_fetch_head_sentinel "$collision_iso_fail"
collision_snapshot_graph "$collision_iso_fail" "$collision_scratch/iso-fail-before"
collision_expect_failure \
  'stale-base number collision' \
  "${collision_checker[@]}" \
  --root "$collision_iso_fail" \
  --authority "$collision_canonical"
collision_snapshot_graph "$collision_iso_fail" "$collision_scratch/iso-fail-after"
collision_assert_graph_unchanged \
  "$collision_scratch/iso-fail-before" \
  "$collision_scratch/iso-fail-after" \
  'failing isolated check'
collision_assert_no_imported_release_tags "$collision_iso_fail" 'failing isolated check'

collision_iso_absent=$collision_scratch/iso-absent
collision_git "$collision_scratch" clone -q --no-tags \
  "$collision_canonical" "$collision_iso_absent"
collision_install_isolation_traps "$collision_iso_absent"
rm -f "$(collision_git_dir_file "$collision_iso_absent" FETCH_HEAD)"
collision_snapshot_graph "$collision_iso_absent" "$collision_scratch/iso-absent-before"
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_iso_absent" \
  --authority "$collision_canonical"
collision_snapshot_graph "$collision_iso_absent" "$collision_scratch/iso-absent-after"
collision_assert_graph_unchanged \
  "$collision_scratch/iso-absent-before" \
  "$collision_scratch/iso-absent-after" \
  'isolated check with no FETCH_HEAD'
collision_assert_no_imported_release_tags \
  "$collision_iso_absent" \
  'isolated check with no FETCH_HEAD'
