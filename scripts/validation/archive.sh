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

# A packaged manifest must carry no path or git dependency. The dependency
# tables are [dependencies], [dev-dependencies], [build-dependencies], their
# target-specific forms such as [target.'cfg(unix)'.dependencies], and the
# dotted table form [dependencies.name]. Inside them a `path` or `git` key is
# caught in either spelling: a line of its own, or an inline table such as
# `name = { path = "../name" }`, or the dotted key `name.path = "../name"`.
validation_manifest_has_no_path_or_git() {
  local validation_manifest=$1
  ! awk '
    /^[[:space:]]*\[/ {
      header = $0
      sub(/^[[:space:]]*/, "", header)
      dependency = (header ~ /^\[((target\.[^]]+\.)?(dev-|build-)?dependencies)(\.[^]]+)?\]/)
      next
    }
    dependency && /(^|[{,.[:space:]])(path|git)[[:space:]]*=/ { found = 1 }
    END { exit found ? 0 : 1 }
  ' "$validation_manifest"
}

# Prints every file under the tree that matches a forbidden pattern, then fails.
# Exit status 1 from rg means no match; any other failure is an audit error and
# is never reported as a clean tree.
validation_extracted_tree_is_safe() {
  local validation_tree=$1
  local validation_repo_root=$2
  local validation_hits
  local validation_status=0
  validation_hits=$(rg -l \
    "$validation_repo_root|/home/tom/|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|\\bsk-[A-Za-z0-9_-]{20,}" \
    "$validation_tree") || validation_status=$?
  if [[ $validation_status -eq 1 ]]; then
    return 0
  fi
  if [[ $validation_status -ne 0 ]]; then
    printf 'content audit could not scan %s (rg exit %s)\n' \
      "$validation_tree" "$validation_status" >&2
    return 1
  fi
  printf 'content audit found forbidden content in:\n%s\n' "$validation_hits" >&2
  return 1
}
