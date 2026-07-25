#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source "$release_repo_root/scripts/release-package-set.sh"

release_candidate=${1:-.effigy/release-candidates/$release_version}
if [[ "$release_candidate" != /* ]]; then
  release_candidate="$release_repo_root/$release_candidate"
fi

release_nucleus_root=${SWALLOWTAIL_NUCLEUS_ROOT:-/Users/tom/Dev/projects/nucleus}
release_soundcheck_root=${SWALLOWTAIL_SOUNDCHECK_ROOT:-/Users/tom/Dev/projects/soundcheck}
release_soundcheck_library_root=$(cd "$release_soundcheck_root/../soundcheck-library" && pwd)
release_signal_root=$(cd "$release_soundcheck_root/../signal" && pwd)

test -f "$release_candidate/packages.sha256"
test -f "$release_nucleus_root/Cargo.toml"
test -f "$release_soundcheck_root/Cargo.toml"

release_tmp=$(mktemp -d)
trap 'rm -rf "$release_tmp"' EXIT

release_copy_source() {
  local source_root=$1
  local destination=$2
  local file_list=$3

  mkdir -p "$destination"
  (
    cd "$source_root"
    while IFS= read -r -d '' source_path; do
      if [[ -e "$source_path" || -L "$source_path" ]]; then
        printf '%s\0' "$source_path"
      fi
    done < <(git ls-files --cached --others --exclude-standard -z)
  ) > "$file_list"
  (
    cd "$source_root"
    tar --null -T "$file_list" -cf -
  ) | tar -xf - -C "$destination"
}

release_snapshot_commit() {
  local source_root=$1
  local message=$2

  (
    cd "$source_root"
    git init -q
    git add -A
    GIT_AUTHOR_DATE=2000-01-01T00:00:00Z \
      GIT_COMMITTER_DATE=2000-01-01T00:00:00Z \
      git \
      -c user.name=Swallowtail \
      -c user.email=release-gate@invalid \
      commit -q -m "$message"
    test -z "$(git status --porcelain)"
    git rev-parse HEAD
  )
}

release_packages_root="$release_tmp/packages"
mkdir -p "$release_packages_root"
for release_package in "${release_packages[@]}"; do
  tar -xzf \
    "$release_candidate/packages/$release_package-$release_version.crate" \
    -C "$release_packages_root"
  mv \
    "$release_packages_root/$release_package-$release_version" \
    "$release_packages_root/$release_package"
done

release_candidate_source="$release_tmp/candidate-source"
git clone -q \
  "$release_candidate/swallowtail-$release_version-source.bundle" \
  "$release_candidate_source"

release_consumer_parent="$release_tmp/consumers"
release_nucleus_copy="$release_consumer_parent/nucleus"
release_soundcheck_copy="$release_consumer_parent/soundcheck"
release_soundcheck_library_copy="$release_consumer_parent/soundcheck-library"
release_signal_copy="$release_consumer_parent/signal"
mkdir -p "$release_consumer_parent"
release_copy_source \
  "$release_nucleus_root" \
  "$release_nucleus_copy" \
  "$release_tmp/nucleus-files.txt"
release_copy_source \
  "$release_soundcheck_root" \
  "$release_soundcheck_copy" \
  "$release_tmp/soundcheck-files.txt"
release_copy_source \
  "$release_soundcheck_library_root" \
  "$release_soundcheck_library_copy" \
  "$release_tmp/soundcheck-library-files.txt"
release_copy_source \
  "$release_signal_root" \
  "$release_signal_copy" \
  "$release_tmp/signal-files.txt"

release_nucleus_source_commit=$(
  release_snapshot_commit "$release_nucleus_copy" \
    'Nucleus candidate-consumer source snapshot'
)
release_soundcheck_source_commit=$(
  release_snapshot_commit "$release_soundcheck_copy" \
    'Soundcheck candidate-consumer source snapshot'
)
release_soundcheck_library_source_commit=$(
  release_snapshot_commit "$release_soundcheck_library_copy" \
    'Soundcheck Library candidate-consumer source snapshot'
)
release_signal_source_commit=$(
  release_snapshot_commit "$release_signal_copy" \
    'Signal candidate-consumer source snapshot'
)
mkdir -p "$release_soundcheck_copy/node_modules"
cp -R \
  "$release_soundcheck_root/node_modules/lucide-static" \
  "$release_soundcheck_copy/node_modules/lucide-static"

release_lucide_version=$(
  node -p \
    "require('$release_soundcheck_root/node_modules/lucide-static/package.json').version"
)
release_soundcheck_lock_sha256=$(
  shasum -a 256 "$release_soundcheck_root/package-lock.json" | awk '{ print $1 }'
)
release_candidate_packages_sha256=$(
  shasum -a 256 "$release_candidate/packages.sha256" | awk '{ print $1 }'
)

for release_package in "${release_consumer_packages[@]}"; do
  perl -0pi -e \
    "s#\\Q$release_package = { path = \"../../../swallowtail/crates/$release_package\" }\\E#$release_package = { version = \"=$release_version\" }#g" \
    "$release_nucleus_copy/crates/nucleus-agent-adapters/Cargo.toml"
  perl -0pi -e \
    "s#\\Q$release_package = { path = \"../swallowtail/crates/$release_package\" }\\E#$release_package = { version = \"=$release_version\" }#g" \
    "$release_soundcheck_copy/Cargo.toml"
done

{
  printf '\n[patch.crates-io]\n'
  for release_package in "${release_consumer_packages[@]}"; do
    printf '%s = { path = "%s/%s" }\n' \
      "$release_package" "$release_packages_root" "$release_package"
  done
} >> "$release_nucleus_copy/Cargo.toml"

{
  printf '\n[patch.crates-io]\n'
  for release_package in "${release_consumer_packages[@]}"; do
    printf '%s = { path = "%s/%s" }\n' \
      "$release_package" "$release_packages_root" "$release_package"
  done
} >> "$release_soundcheck_copy/Cargo.toml"

rg -q 'swallowtail-core = \{ version = "=0.1.0" \}' \
  "$release_nucleus_copy/crates/nucleus-agent-adapters/Cargo.toml"
rg -q 'swallowtail-core = \{ version = "=0.1.0" \}' \
  "$release_soundcheck_copy/Cargo.toml"

CARGO_TARGET_DIR="$release_tmp/target/nucleus" \
  cargo test \
    --manifest-path "$release_nucleus_copy/Cargo.toml" \
    --package nucleus-agent-adapters \
    swallowtail_codex:: \
    --locked

CARGO_TARGET_DIR="$release_tmp/target/soundcheck" \
  cargo test \
    --manifest-path "$release_soundcheck_copy/Cargo.toml" \
    --package soundcheck-app \
    swallowtail_codex:: \
    --locked

{
  printf '[workspace]\nresolver = "3"\nmembers = [\n'
  for release_package in "${release_packages[@]}"; do
    printf '  "%s",\n' "$release_package"
  done
  printf ']\n\n[patch.crates-io]\n'
  for release_package in \
    swallowtail-core \
    swallowtail-host-local \
    swallowtail-protocol-acp \
    swallowtail-protocol-openai-chat \
    swallowtail-runtime \
    swallowtail-testkit
  do
    printf '%s = { path = "%s" }\n' \
      "$release_package" "$release_package"
  done
} > "$release_packages_root/Cargo.toml"

cp "$release_candidate_source/Cargo.lock" "$release_packages_root/Cargo.lock"
CARGO_TARGET_DIR="$release_tmp/target/packaged-codex" \
  cargo test \
    --manifest-path "$release_packages_root/Cargo.toml" \
    --package swallowtail-adapter-codex \
    --tests \
    --locked

release_consumer_evidence="$release_tmp/consumer-validation.env"
{
  printf 'format=swallowtail.consumer-candidate-validation.v2\n'
  printf 'version=%s\n' "$release_version"
  printf 'candidate_source_commit=%s\n' \
    "$(sed -n 's/^source_commit=//p' "$release_candidate/candidate.env")"
  printf 'candidate_packages_sha256=%s\n' \
    "$release_candidate_packages_sha256"
  printf 'nucleus_base_commit=%s\n' "$(git -C "$release_nucleus_root" rev-parse HEAD)"
  printf 'nucleus_source_commit=%s\n' "$release_nucleus_source_commit"
  printf 'nucleus_source_scope=tracked-plus-untracked-nonignored\n'
  printf 'nucleus_package=nucleus-agent-adapters\n'
  printf 'nucleus_validation=cargo-test-swallowtail-codex-locked\n'
  printf 'soundcheck_base_commit=%s\n' \
    "$(git -C "$release_soundcheck_root" rev-parse HEAD)"
  printf 'soundcheck_source_commit=%s\n' "$release_soundcheck_source_commit"
  printf 'soundcheck_source_scope=tracked-plus-untracked-nonignored\n'
  printf 'soundcheck_library_base_commit=%s\n' \
    "$(git -C "$release_soundcheck_library_root" rev-parse HEAD)"
  printf 'soundcheck_library_source_commit=%s\n' \
    "$release_soundcheck_library_source_commit"
  printf 'signal_base_commit=%s\n' \
    "$(git -C "$release_signal_root" rev-parse HEAD)"
  printf 'signal_source_commit=%s\n' "$release_signal_source_commit"
  printf 'soundcheck_package_lock_sha256=%s\n' \
    "$release_soundcheck_lock_sha256"
  printf 'soundcheck_lucide_static_version=%s\n' "$release_lucide_version"
  printf 'soundcheck_lucide_static_source=read-only-consumer-worktree\n'
  printf 'soundcheck_package=soundcheck-app\n'
  printf 'soundcheck_validation=cargo-test-swallowtail-codex-locked\n'
  printf 'packaged_codex_validation=cargo-test-tests-locked\n'
  printf 'live_credentials=absent\n'
  printf 'provider_calls=none\n'
} > "$release_consumer_evidence"

if [[ -f "$release_candidate/consumer-validation.env" ]]; then
  diff -u \
    "$release_candidate/consumer-validation.env" \
    "$release_consumer_evidence"
else
  cp "$release_consumer_evidence" \
    "$release_candidate/consumer-validation.env"
  (
    cd "$release_candidate"
    shasum -a 256 consumer-validation.env > consumer-validation.sha256
  )
fi

printf 'isolated Nucleus and Soundcheck candidate checks passed\n'
