#!/usr/bin/env bash
set -euo pipefail

identity_repo_root=$(cd "$(dirname "$0")/../.." && pwd)
identity_checker="$identity_repo_root/scripts/release/sdk-patch-candidate/check_identity.py"
identity_fixture_root=$(mktemp -d "${TMPDIR:-/tmp}/swallowtail-sdk-candidate-identity.XXXXXXXX")
trap 'rm -rf "$identity_fixture_root"' EXIT

write_fixture() {
  local fixture_root=$1
  mkdir -p "$fixture_root/crates/swallowtail-adapter-claude-agent/src/sdk" \
    "$fixture_root/crates/swallowtail-adapter-claude-agent/sidecar"
  cat > "$fixture_root/Cargo.toml" <<'EOF'
[workspace.package]
version = "0.5.3"

[workspace.dependencies]
swallowtail-core = { path = "crates/swallowtail-core", version = "0.5.3" }
EOF
  cat > "$fixture_root/crates/swallowtail-adapter-claude-agent/Cargo.toml" <<'EOF'
[package]
name = "swallowtail-adapter-claude-agent"
version.workspace = true
EOF
  cat > "$fixture_root/crates/swallowtail-adapter-claude-agent/src/sdk.rs" <<'EOF'
pub const CLAUDE_AGENT_SDK_VERSION: &str = "0.3.259";
pub const CLAUDE_AGENT_SDK_NATIVE_VERSION: &str = "2.1.259";
pub const CLAUDE_AGENT_SDK_NODE_RUNTIME: &str = "22.23.2";
EOF
  cat > "$fixture_root/crates/swallowtail-adapter-claude-agent/src/sdk/asset.rs" <<'EOF'
include_str!("../../sidecar/claude-agent-sdk-sidecar.mjs");
concat!("swallowtail-claude-agent-sdk-sidecar@", env!("CARGO_PKG_VERSION"));
EOF
  cp "$identity_repo_root/crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs" \
    "$fixture_root/crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
}

replace_fixture_text() {
  local target_file=$1
  local before=$2
  local after=$3
  python3 - "$target_file" "$before" "$after" <<'PY'
from pathlib import Path
import sys

target = Path(sys.argv[1])
before, after = sys.argv[2:4]
before = before.replace("\\n", "\n")
after = after.replace("\\n", "\n")
contents = target.read_text(encoding="utf-8")
if contents.count(before) != 1:
    raise SystemExit(f"fixture replacement target is not unique: {target}")
target.write_text(contents.replace(before, after), encoding="utf-8")
PY
}

expect_refusal() {
  local fixture_root=$1
  local expected=$2
  local output="$identity_fixture_root/refusal.log"
  if python3 "$identity_checker" "$fixture_root" >"$output" 2>&1; then
    cat "$output" >&2
    printf 'identity fixture unexpectedly accepted: %s\n' "$expected" >&2
    exit 1
  fi
  if ! rg -Fq "$expected" "$output"; then
    cat "$output" >&2
    printf 'identity fixture refused for the wrong reason: %s\n' "$expected" >&2
    exit 1
  fi
}

write_fixture "$identity_fixture_root/valid"
python3 "$identity_checker" "$identity_fixture_root/valid"

write_fixture "$identity_fixture_root/wrong-version"
replace_fixture_text "$identity_fixture_root/wrong-version/Cargo.toml" \
  'version = "0.5.3"\n\n[workspace.dependencies]' \
  'version = "0.5.2"\n\n[workspace.dependencies]'
expect_refusal "$identity_fixture_root/wrong-version" 'workspace package version must be 0.5.3'

write_fixture "$identity_fixture_root/wrong-package-version"
replace_fixture_text \
  "$identity_fixture_root/wrong-package-version/crates/swallowtail-adapter-claude-agent/Cargo.toml" \
  'version.workspace = true' 'version = "0.5.3"'
expect_refusal "$identity_fixture_root/wrong-package-version" 'must inherit the workspace version'

write_fixture "$identity_fixture_root/wrong-asset-path"
replace_fixture_text \
  "$identity_fixture_root/wrong-asset-path/crates/swallowtail-adapter-claude-agent/src/sdk/asset.rs" \
  '../../sidecar/claude-agent-sdk-sidecar.mjs' '../../sidecar/other.mjs'
expect_refusal "$identity_fixture_root/wrong-asset-path" 'embedded sidecar source path changed'

write_fixture "$identity_fixture_root/wrong-source-tag"
replace_fixture_text \
  "$identity_fixture_root/wrong-source-tag/crates/swallowtail-adapter-claude-agent/src/sdk/asset.rs" \
  'env!("CARGO_PKG_VERSION")' '"0.5.2"'
expect_refusal "$identity_fixture_root/wrong-source-tag" 'sidecar source tag must derive'

write_fixture "$identity_fixture_root/wrong-sdk-point"
replace_fixture_text \
  "$identity_fixture_root/wrong-sdk-point/crates/swallowtail-adapter-claude-agent/src/sdk.rs" \
  'CLAUDE_AGENT_SDK_VERSION: &str = "0.3.259"' \
  'CLAUDE_AGENT_SDK_VERSION: &str = "0.3.284"'
expect_refusal "$identity_fixture_root/wrong-sdk-point" 'released SDK identity changed'

write_fixture "$identity_fixture_root/wrong-asset-bytes"
printf '\n' >> "$identity_fixture_root/wrong-asset-bytes/crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
expect_refusal "$identity_fixture_root/wrong-asset-bytes" 'sidecar source bytes differ'

printf 'candidate identity version and asset refusal fixtures passed\n'
