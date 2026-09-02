#!/usr/bin/env bash
# In-tree duplicate, same-path edit, delete/add, rename, and serial restack.

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

collision_edit=$collision_scratch/same-path-edit
collision_git "$collision_scratch" clone -q "$collision_canonical" "$collision_edit"
printf '\n# edited in place\n' >>"$collision_edit/docs/roadmaps/g04/batch-cards/076-kimi-platform-chat-addable-descriptor.md"
collision_git "$collision_edit" add docs
collision_git "$collision_edit" commit -q -m 'edit 076 in place'
collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_edit" \
  --authority "$collision_canonical"

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

collision_expect_pass \
  "${collision_checker[@]}" \
  --root "$collision_updater" \
  --authority "$collision_canonical"
