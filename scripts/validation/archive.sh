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

# Host paths are any absolute /home/<user>/ or /Users/<user>/ location. Packaged
# files that already carry one are listed here, relative to the package
# directory, until the crate fixture or source string that holds it is
# replaced. Each entry is pre-existing debt; a new host path in any other file
# fails the audit, and so does a new host path in a listed file on a line the
# list does not name. Remove an entry when its file is cleaned.
validation_host_path_debt=(
  swallowtail-adapter-antigravity/src/discovery/probe.rs
  swallowtail-adapter-antigravity/tests/foundation.rs
  swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.270/identity.json
  swallowtail-adapter-codex/src/discovery/outcome.rs
  swallowtail-adapter-codex/tests/prepared.rs
  swallowtail-adapter-cursor/src/discovery/outcome.rs
  swallowtail-adapter-cursor/tests/foundation.rs
  swallowtail-adapter-kimi/tests/fixtures/kimi-code-0.43.0/identity.json
  swallowtail-adapter-opencode/tests/fixtures/opencode-1.18.18/identity.json
  swallowtail-adapter-opencode/tests/fixtures/opencode-acp-1.18.32/identity.json
)

# Prints the files under a tree that match one pattern. Status 0 means matches
# were printed, 1 means none, and anything else is an audit error.
validation_scan_tree() {
  local validation_pattern=$1
  local validation_tree=$2
  rg -l -- "$validation_pattern" "$validation_tree"
}

# Prints the host-path hits that are not listed debt. Returns 0 when the scan
# ran, whether or not it found hits, and the scan's own status otherwise, so an
# audit error is never mistaken for a clean tree.
validation_unlisted_host_path_hits() {
  local validation_tree=$1
  local validation_all
  local validation_status=0
  local validation_hit
  local validation_debt
  local validation_listed
  validation_all=$(validation_scan_tree '/(home|Users)/[^/[:space:]]+/' "$validation_tree") ||
    validation_status=$?
  if [[ $validation_status -eq 1 ]]; then
    return 0
  fi
  if [[ $validation_status -ne 0 ]]; then
    return "$validation_status"
  fi
  while IFS= read -r validation_hit; do
    [[ -z $validation_hit ]] && continue
    validation_listed=false
    for validation_debt in "${validation_host_path_debt[@]}"; do
      case $validation_hit in
        */"$validation_debt") validation_listed=true ;;
      esac
    done
    if [[ $validation_listed == false ]]; then
      printf '%s\n' "$validation_hit"
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

  validation_hits=$(validation_scan_tree \
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
      printf 'host path outside the listed debt in:\n%s\n' "$validation_host_hits"
    fi
    printf 'content audit found forbidden content in %s\n' "$validation_tree" >&2
    return 1
  fi
  return 0
}
