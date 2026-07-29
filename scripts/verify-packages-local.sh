#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source "$release_repo_root/scripts/release-package-set.sh"

release_tmp=$(mktemp -d)
release_candidate_output=${SWALLOWTAIL_CANDIDATE_OUTPUT:-}
release_candidate_staging=
trap 'rm -rf "$release_tmp" "$release_candidate_staging"' EXIT

if [[ -n "$release_candidate_output" ]]; then
  if [[ "$release_candidate_output" != /* ]]; then
    release_candidate_output="$release_repo_root/$release_candidate_output"
  fi
  if [[ -e "$release_candidate_output" ]]; then
    printf 'candidate output already exists: %s\n' "$release_candidate_output" >&2
    exit 1
  fi
  mkdir -p "$(dirname "$release_candidate_output")"
  release_candidate_staging=$(mktemp -d "$(dirname "$release_candidate_output")/.staging.XXXXXX")
fi

release_source_root="$release_tmp/source"
release_source_commit=
release_source_parent_commit=
release_source_scope=

if [[ -n "$release_candidate_output" ]]; then
  release_dirty_source=$(
    git status \
      --porcelain=v1 \
      --untracked-files=all \
      -- \
      . \
      ':(exclude)release-candidates'
  )
  if [[ -n "$release_dirty_source" ]]; then
    printf 'final candidate requires clean source state:\n%s\n' \
      "$(printf '%s\n' "$release_dirty_source" | sed -n '1,40p')" >&2
    exit 1
  fi

  release_source_commit=$(git rev-parse HEAD)
  if ! release_source_parent_commit=$(git rev-parse HEAD^ 2>/dev/null); then
    printf 'final candidate source commit must not be a root commit\n' >&2
    exit 1
  fi
  release_source_scope=clean-head-excluding-generated-candidate-evidence

  git clone -q --no-hardlinks "$release_repo_root" "$release_source_root"
  test "$(git -C "$release_source_root" rev-parse HEAD)" = \
    "$release_source_commit"
  test -z "$(git -C "$release_source_root" status --porcelain)"
else
  mkdir -p "$release_source_root"

  release_source_list="$release_tmp/source-files.txt"
  while IFS= read -r -d '' release_source_path; do
    if [[ "$release_source_path" == release-candidates/* ]]; then
      continue
    fi
    if [[ -e "$release_source_path" || -L "$release_source_path" ]]; then
      printf '%s\0' "$release_source_path"
    fi
  done < <(git ls-files --cached --others --exclude-standard -z) \
    > "$release_source_list"

  tar --null -T "$release_source_list" -cf - |
    tar -xf - -C "$release_source_root"

  (
    cd "$release_source_root"
    git init -q
    git add -A
    GIT_AUTHOR_DATE=2000-01-01T00:00:00Z \
      GIT_COMMITTER_DATE=2000-01-01T00:00:00Z \
      git \
      -c user.name=Swallowtail \
      -c user.email=release-gate@invalid \
      commit -q -m 'Local package verification snapshot'
    test -z "$(git status --porcelain)"
  )
fi

(
  cd "$release_source_root"
  for release_package in "${release_packages[@]}"; do
    cargo package \
      --package "$release_package" \
      --no-verify \
      --target-dir "$release_tmp/target" \
      "${release_patch_args[@]}"
  done
)

mkdir -p "$release_tmp/extracted"

for release_package in "${release_packages[@]}"; do
  release_archive="$release_tmp/target/package/$release_package-$release_version.crate"
  test -f "$release_archive"

  release_archive_size=$(stat -f '%z' "$release_archive")
  if (( release_archive_size >= 10000000 )); then
    printf 'package exceeds crates.io 10 MB limit: %s\n' "$release_package" >&2
    exit 1
  fi

  if tar -tzf "$release_archive" |
    rg -q '/(\.git|\.effigy|target|\.env|credentials?)(/|$)|\.(pem|key)$'
  then
    printf 'forbidden package path: %s\n' "$release_package" >&2
    exit 1
  fi

  tar -xzf "$release_archive" -C "$release_tmp/extracted"
  mv \
    "$release_tmp/extracted/$release_package-$release_version" \
    "$release_tmp/extracted/$release_package"
  release_manifest="$release_tmp/extracted/$release_package/Cargo.toml"

  if awk '
    /^\[(dev-|build-)?dependencies(\.|])/{ dependency = 1; next }
    /^\[/{ dependency = 0 }
    dependency && /^(path|git)[[:space:]]*=/{ found = 1 }
    END { exit found ? 0 : 1 }
  ' "$release_manifest"
  then
    printf 'packaged dependency retained path or git source: %s\n' \
      "$release_package" >&2
    exit 1
  fi

  shasum -a 256 "$release_archive"
done

if rg -l \
  '/Users/tom/Dev/projects/swallowtail|/home/tom/|BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|sk-[A-Za-z0-9_-]{20,}' \
  "$release_tmp/extracted" > "$release_tmp/forbidden-content.txt"
then
  printf 'forbidden packaged content found in:\n' >&2
  sed -n '1,40p' "$release_tmp/forbidden-content.txt" >&2
  exit 1
fi

release_verify_root="$release_tmp/extracted"
{
  printf '[workspace]\nresolver = "3"\nmembers = [\n'
  for release_package in "${release_packages[@]}"; do
    printf '  "%s",\n' "$release_package"
  done
  printf ']\n\n[patch.crates-io]\n'
  for release_package in "${release_internal_patch_packages[@]}"; do
    printf '%s = { path = "%s" }\n' \
      "$release_package" "$release_package"
  done
} > "$release_verify_root/Cargo.toml"

cp "$release_source_root/Cargo.lock" "$release_verify_root/Cargo.lock"

(
  cd "$release_verify_root"
  cargo check --workspace --all-targets --locked
  cargo test --workspace --no-run --locked
  for release_structured_suite in \
    "swallowtail-adapter-alibaba-model-studio|prepared_facade" \
    "swallowtail-adapter-claude-agent|prepared_facade" \
    "swallowtail-adapter-claude-agent|structured_run" \
    "swallowtail-adapter-codex|prepared_profiles" \
    "swallowtail-adapter-deepseek|prepared_facade" \
    "swallowtail-adapter-gemini|prepared_facade" \
    "swallowtail-adapter-gemini|headless_structured_run" \
    "swallowtail-adapter-kimi|headless_structured_run" \
    "swallowtail-adapter-kimi|local_server_structured_run" \
    "swallowtail-adapter-opencode|prepared_facade" \
    "swallowtail-adapter-pi|structured_run" \
    "swallowtail-adapter-xai|prepared_facade"
  do
    IFS='|' read -r release_structured_package release_structured_test \
      <<< "$release_structured_suite"
    cargo test \
      --package "$release_structured_package" \
      --test "$release_structured_test" \
      --locked
  done
  for release_kimi_test in \
    local_server_corpus \
    local_server_lifecycle \
    local_server_binding_import \
    local_server_interactive
  do
    cargo test \
      --package swallowtail-adapter-kimi \
      --test "$release_kimi_test" \
      --locked
  done
)

if [[ -n "$release_candidate_output" ]]; then
  mkdir -p \
    "$release_candidate_staging/packages" \
    "$release_candidate_staging/package-files"

  git -C "$release_source_root" bundle create \
    "$release_candidate_staging/swallowtail-$release_version-source.bundle" \
    HEAD

  for release_package in "${release_packages[@]}"; do
    release_archive_name="$release_package-$release_version.crate"
    cp \
      "$release_tmp/target/package/$release_archive_name" \
      "$release_candidate_staging/packages/$release_archive_name"
    tar -tzf "$release_candidate_staging/packages/$release_archive_name" \
      > "$release_candidate_staging/package-files/$release_package.txt"
  done

  {
    printf 'format=swallowtail.release-candidate.v2\n'
    printf 'version=%s\n' "$release_version"
    printf 'registry=crates-io\n'
    printf 'source_commit=%s\n' "$release_source_commit"
    printf 'source_parent_commit=%s\n' "$release_source_parent_commit"
    printf 'source_scope=%s\n' "$release_source_scope"
    printf 'verified_target=aarch64-apple-darwin\n'
    printf 'general_rust_version=1.93.0\n'
    printf 'bedrock_rust_version=1.94.1\n'
    printf 'current_stable_rust_version=1.97.1\n'
    printf 'package_count=%s\n' "${#release_packages[@]}"
  } > "$release_candidate_staging/candidate.env"

  {
    for release_package in "${release_stage_1[@]}"; do
      printf '1\t%s\n' "$release_package"
    done
    for release_package in "${release_stage_2[@]}"; do
      printf '2\t%s\n' "$release_package"
    done
    for release_package in "${release_stage_3[@]}"; do
      printf '3\t%s\n' "$release_package"
    done
  } > "$release_candidate_staging/publication-order.tsv"

  (
    cd "$release_candidate_staging"
    for release_package in "${release_packages[@]}"; do
      shasum -a 256 "packages/$release_package-$release_version.crate"
    done > packages.sha256
    for release_package in "${release_packages[@]}"; do
      shasum -a 256 "package-files/$release_package.txt"
    done > package-files.sha256
    shasum -a 256 \
      "swallowtail-$release_version-source.bundle" \
      candidate.env \
      publication-order.tsv \
      packages.sha256 \
      package-files.sha256 \
      > evidence.sha256
  )

  mv "$release_candidate_staging" "$release_candidate_output"
  release_candidate_staging=
  printf 'candidate artifacts retained at %s\n' "$release_candidate_output"
fi

printf 'local package assembly and extracted-workspace verification passed\n'
