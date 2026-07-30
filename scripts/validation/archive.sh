#!/usr/bin/env bash

validation_archive_member_list_is_safe() {
  if rg -q \
    '/(\.git|\.effigy|target|\.env|credentials?)(/|$)|\.(pem|key)$'
  then
    return 1
  fi
}

validation_archive_is_safe() {
  local validation_archive=$1
  tar -tzf "$validation_archive" |
    validation_archive_member_list_is_safe
}

validation_manifest_has_no_path_or_git() {
  local validation_manifest=$1
  ! awk '
    /^\[(dev-|build-)?dependencies(\.|])/{ dependency = 1; next }
    /^\[/{ dependency = 0 }
    dependency && /^(path|git)[[:space:]]*=/{ found = 1 }
    END { exit found ? 0 : 1 }
  ' "$validation_manifest"
}

validation_extracted_tree_is_safe() {
  local validation_tree=$1
  local validation_repo_root=$2
  if rg -l \
    "$validation_repo_root|/home/tom/|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|sk-[A-Za-z0-9_-]{20,}" \
    "$validation_tree" > /dev/null
  then
    return 1
  fi
}
