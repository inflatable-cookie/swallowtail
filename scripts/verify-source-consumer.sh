#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

release_tmp=$(mktemp -d)
trap 'rm -rf "$release_tmp"' EXIT

release_consumer_root="$release_tmp/consumer"
mkdir -p "$release_consumer_root/src"

if [[ -z $(git status --porcelain) ]]; then
  release_source_root="$release_repo_root"
  release_source_commit=$(git rev-parse HEAD)
  release_source_kind=commit
else
  release_source_root="$release_tmp/swallowtail-source"
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
      -c user.email=source-gate@invalid \
      commit -q -m 'Source consumer verification snapshot'
    test -z "$(git status --porcelain)"
  )

  release_source_commit=$(git -C "$release_source_root" rev-parse HEAD)
  release_source_kind=snapshot
fi

release_source_url="file://$release_source_root"

cat > "$release_consumer_root/Cargo.toml" <<EOF
[package]
name = "swallowtail-source-consumer"
version = "0.0.0"
edition = "2024"
publish = false
rust-version = "1.95"

[dependencies]
swallowtail-core = { git = "$release_source_url", rev = "$release_source_commit" }
swallowtail-runtime = { git = "$release_source_url", rev = "$release_source_commit" }
swallowtail-host-local = { git = "$release_source_url", rev = "$release_source_commit" }
swallowtail-adapter-codex = { git = "$release_source_url", rev = "$release_source_commit" }
EOF

cat > "$release_consumer_root/src/main.rs" <<'EOF'
fn main() {
    let _ = swallowtail_adapter_codex::codex_exec_descriptor();
    let _ = core::mem::size_of::<swallowtail_core::AdapterId>();
    let _ = core::mem::size_of::<swallowtail_runtime::RuntimeFailure>();
    use swallowtail_host_local as _;
}
EOF

cargo generate-lockfile --manifest-path "$release_consumer_root/Cargo.toml"
cargo check --manifest-path "$release_consumer_root/Cargo.toml" --locked

release_metadata="$release_tmp/metadata.json"
cargo metadata \
  --manifest-path "$release_consumer_root/Cargo.toml" \
  --format-version 1 \
  --locked \
  > "$release_metadata"
jq -e --arg commit "$release_source_commit" '
  [
    .packages[] |
    select((.name | startswith("swallowtail-")) and .source != null)
  ] as $packages |
  ($packages | length) >= 4 and
  all($packages[];
    (.source // "") | startswith("git+file://") and endswith("#" + $commit)
  )
' "$release_metadata" > /dev/null

printf 'external source consumer passed at exact %s %s\n' \
  "$release_source_kind" \
  "$release_source_commit"
