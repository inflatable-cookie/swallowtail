#!/usr/bin/env bash
# Ref-graph snapshots and containment traps.

collision_snapshot_graph() {
  local collision_cwd=$1
  local collision_out=$2
  local collision_fetch_head
  mkdir -p "$collision_out"
  git -C "$collision_cwd" for-each-ref \
    --format='%(objectname) %(objecttype) %(refname)%(if)%(symref)%(then) -> %(symref)%(end)' \
    --sort=refname >"$collision_out/refs"
  git -C "$collision_cwd" rev-parse HEAD >"$collision_out/HEAD"
  if ! git -C "$collision_cwd" symbolic-ref -q HEAD >"$collision_out/HEAD_SYM"; then
    : >"$collision_out/HEAD_SYM"
  fi
  git -C "$collision_cwd" count-objects -v >"$collision_out/objects"
  collision_fetch_head=$(collision_git_dir_file "$collision_cwd" FETCH_HEAD)
  if [[ -e "$collision_fetch_head" ]]; then
    cp "$collision_fetch_head" "$collision_out/FETCH_HEAD"
    printf 'present\n' >"$collision_out/FETCH_HEAD_STATE"
  else
    printf 'absent\n' >"$collision_out/FETCH_HEAD_STATE"
  fi
}

collision_assert_graph_unchanged() {
  local collision_before=$1
  local collision_after=$2
  local collision_label=$3
  local collision_name
  for collision_name in refs HEAD HEAD_SYM objects FETCH_HEAD_STATE; do
    if ! cmp -s "$collision_before/$collision_name" "$collision_after/$collision_name"; then
      printf '%s %s changed\n' "$collision_label" "$collision_name" >&2
      diff -u "$collision_before/$collision_name" "$collision_after/$collision_name" >&2 || true
      exit 1
    fi
  done
  if [[ "$(<"$collision_before/FETCH_HEAD_STATE")" == present ]]; then
    if ! cmp -s "$collision_before/FETCH_HEAD" "$collision_after/FETCH_HEAD"; then
      printf '%s FETCH_HEAD bytes changed\n' "$collision_label" >&2
      diff -u "$collision_before/FETCH_HEAD" "$collision_after/FETCH_HEAD" >&2 || true
      exit 1
    fi
  fi
}

collision_install_isolation_traps() {
  local collision_cwd=$1
  collision_git "$collision_cwd" commit --allow-empty -q -m 'isolation sentinel'
  collision_git "$collision_cwd" update-ref refs/heads/review-sentinel HEAD
  collision_git "$collision_cwd" tag roadmap-authority HEAD
  collision_git "$collision_cwd" tag swallowtail-roadmap-authority HEAD
  collision_git "$collision_cwd" tag v0.1.0 HEAD
  collision_git "$collision_cwd" symbolic-ref \
    refs/swallowtail/roadmap-authority refs/heads/review-sentinel
}

collision_write_fetch_head_sentinel() {
  local collision_cwd=$1
  printf 'sentinel-fetch-head-not-from-git\n' \
    >"$(collision_git_dir_file "$collision_cwd" FETCH_HEAD)"
}

collision_assert_no_imported_release_tags() {
  local collision_cwd=$1
  local collision_label=$2
  local collision_tag_oid
  local collision_sentinel_oid
  if git -C "$collision_cwd" show-ref --verify --quiet refs/tags/v0.3.3; then
    printf '%s imported remote release tag v0.3.3\n' "$collision_label" >&2
    exit 1
  fi
  collision_tag_oid=$(git -C "$collision_cwd" rev-parse refs/tags/v0.1.0)
  collision_sentinel_oid=$(git -C "$collision_cwd" rev-parse refs/heads/review-sentinel)
  if [[ "$collision_tag_oid" != "$collision_sentinel_oid" ]]; then
    printf '%s local tag v0.1.0 was rewritten\n' "$collision_label" >&2
    exit 1
  fi
  if [[ "$(git -C "$collision_cwd" symbolic-ref refs/swallowtail/roadmap-authority)" != \
    refs/heads/review-sentinel ]]; then
    printf '%s authority symbolic ref was rewritten\n' "$collision_label" >&2
    exit 1
  fi
}

collision_write_instead_of() {
  local collision_file=$1
  local collision_malicious=$2
  git config --file "$collision_file" \
    "url.file://${collision_malicious}.insteadOf" \
    "$collision_canonical"
}

collision_assert_rewrite_honored() {
  local collision_label=$1
  shift
  local collision_resolved
  collision_resolved=$("$@" ls-remote --get-url "$collision_canonical")
  if [[ "$collision_resolved" != *"$collision_fork"* ]]; then
    printf '%s rewrite not honored outside isolation: %s\n' \
      "$collision_label" "$collision_resolved" >&2
    exit 1
  fi
}
