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

# Pre-existing host paths in packaged files are listed by exact line in this
# file (package-relative path, then a tab, then the line text). A host path is
# exempt only when its whole line matches a row, so any other host path, and any
# changed or added line in a listed file, fails the audit.
validation_host_path_debt_file="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/host-path-debt.tsv"

# Prints each host-path line under the tree that no debt row names, as
# <relative path>:<line>: <text>. Returns 0 when the scan ran, whether or not it
# found anything, and rg's own status otherwise, so an audit error is never read
# as a clean tree.
validation_unlisted_host_path_hits() {
  local validation_tree=${1%/}
  local validation_all
  local validation_status=0
  local validation_hit
  local validation_path
  local validation_rest
  local validation_line
  local validation_text
  local validation_relative
  validation_all=$(rg -n -H --no-heading --color never \
    -e '/(home|Users)/[^/[:space:]]+/' -- "$validation_tree") || validation_status=$?
  if [[ $validation_status -eq 1 ]]; then
    return 0
  fi
  if [[ $validation_status -ne 0 ]]; then
    return "$validation_status"
  fi
  while IFS= read -r validation_hit; do
    [[ -z $validation_hit ]] && continue
    validation_path=${validation_hit%%:*}
    validation_rest=${validation_hit#*:}
    validation_line=${validation_rest%%:*}
    validation_text=${validation_rest#*:}
    validation_relative=${validation_path#"$validation_tree"/}
    if ! grep -Fxq -- "$(printf '%s\t%s' "$validation_relative" "$validation_text")" \
      "$validation_host_path_debt_file"
    then
      printf '%s:%s: %s\n' "$validation_relative" "$validation_line" "$validation_text"
    fi
  done <<< "$validation_all"
  return 0
}

# Fails, naming every offending file, when the extracted tree holds the repo
# root, a private key, a secret-shaped token, or an unlisted host path. Exit
# status 1 from rg means no match; any other failure is an audit error and is
# never reported as a clean tree.
validation_extracted_tree_is_safe() {
  local validation_tree=$1
  local validation_repo_root=$2
  local validation_hits=""
  local validation_status=0
  local validation_host_hits=""
  local validation_host_status=0

  validation_hits=$(rg -l -- \
    "$validation_repo_root|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|\\bsk-[A-Za-z0-9_-]{20,}" \
    "$validation_tree") || validation_status=$?
  if [[ $validation_status -ne 0 && $validation_status -ne 1 ]]; then
    printf 'content audit could not scan %s (rg exit %s)\n' \
      "$validation_tree" "$validation_status" >&2
    return 1
  fi

  validation_host_hits=$(validation_unlisted_host_path_hits "$validation_tree") ||
    validation_host_status=$?
  if [[ $validation_host_status -ne 0 ]]; then
    printf 'content audit could not check host paths in %s (status %s)\n' \
      "$validation_tree" "$validation_host_status" >&2
    return 1
  fi

  if [[ -n $validation_hits || -n $validation_host_hits ]]; then
    if [[ -n $validation_hits ]]; then
      printf 'forbidden content in:\n%s\n' "$validation_hits"
    fi
    if [[ -n $validation_host_hits ]]; then
      printf 'host path outside the listed debt rows:\n%s\n' "$validation_host_hits"
    fi
    printf 'content audit found forbidden content in %s\n' "$validation_tree" >&2
    return 1
  fi
  return 0
}
