#!/usr/bin/env bash
set -euo pipefail

backport_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd "$backport_dir/../../../.." && pwd)
manifest="$backport_dir/manifest.json"
patch_file="$backport_dir/0001-sdk-correction.patch"

manifest_value() {
  python3 - "$manifest" "$1" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as stream:
    document = json.load(stream)
value = document
for part in sys.argv[2].split("."):
    value = value[part]
if isinstance(value, list):
    print("\n".join(value))
else:
    print(value)
PY
}

base_commit=$(manifest_value base_commit)
base_tree=$(manifest_value base_tree)
base_tag_object=$(manifest_value base_tag_object)
expected_tree=$(manifest_value expected_tree)
api_tool_version=$(manifest_value toolchain.cargo_public_api)
api_toolchain=$(manifest_value toolchain.rust)

die() {
  printf 'sdk-v0.5.2 backport: %s\n' "$1" >&2
  exit 2
}

check_recipe_identity() {
  python3 - "$manifest" "$patch_file" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

with open(sys.argv[1], encoding="utf-8") as stream:
    manifest = json.load(stream)
actual = hashlib.sha256(open(sys.argv[2], "rb").read()).hexdigest()
expected = manifest["patch_sha256"]
if actual != expected:
    raise SystemExit(f"patch SHA-256 mismatch: expected {expected}, found {actual}")
recipe = Path(sys.argv[1]).parent
inventory = (recipe / "files.txt").read_text(encoding="utf-8").splitlines()
if inventory != manifest["files"]:
    raise SystemExit("files.txt and manifest file inventory differ")
consumer = recipe / "consumer"
for filename, field in (
    ("Cargo.lock.template", "cargo_lock_template_sha256"),
    ("Cargo.toml.template", "cargo_manifest_template_sha256"),
    ("src/main.rs", "main_rs_sha256"),
):
    actual = hashlib.sha256((consumer / filename).read_bytes()).hexdigest()
    expected = manifest["consumer_fixture"][field]
    if actual != expected:
        raise SystemExit(f"consumer fixture SHA-256 mismatch: {filename}")
PY
  [[ -s $patch_file ]] || die "patch file is missing or empty"
}

check_root_tag() {
  local actual_tag actual_commit actual_tree
  actual_tag=$(git -C "$repo_root" rev-parse 'refs/tags/v0.5.1^{tag}') ||
    die "v0.5.1 annotated tag is unavailable in the source repository"
  actual_commit=$(git -C "$repo_root" rev-parse 'refs/tags/v0.5.1^{commit}') ||
    die "v0.5.1 peeled commit is unavailable in the source repository"
  actual_tree=$(git -C "$repo_root" rev-parse 'refs/tags/v0.5.1^{tree}') ||
    die "v0.5.1 source tree is unavailable in the source repository"
  [[ $actual_tag == "$base_tag_object" ]] || die "v0.5.1 tag object changed"
  [[ $actual_commit == "$base_commit" ]] || die "v0.5.1 peeled commit changed"
  [[ $actual_tree == "$base_tree" ]] || die "v0.5.1 source tree changed"
}

checkout_base() {
  local destination=$1
  mkdir -p "$destination"
  git -C "$destination" init -q
  git -C "$destination" remote add task-source "$repo_root"
  git -C "$destination" fetch --quiet --no-tags --depth=1 task-source \
    refs/tags/v0.5.1:refs/tags/v0.5.1
  git -C "$destination" checkout --quiet --detach "$base_commit"
}

verify_source_base() {
  local source=$1 actual_head actual_tree actual_tag
  [[ -d $source/.git ]] || die "source is not an isolated Git checkout: $source"
  actual_head=$(git -C "$source" rev-parse HEAD 2>/dev/null) ||
    die "source HEAD cannot be read"
  [[ $actual_head == "$base_commit" ]] ||
    die "source is not the exact v0.5.1 peeled commit $base_commit"
  actual_tree=$(git -C "$source" rev-parse 'HEAD^{tree}') ||
    die "source base tree cannot be read"
  [[ $actual_tree == "$base_tree" ]] || die "source tree does not match immutable v0.5.1"
  actual_tag=$(git -C "$source" rev-parse 'refs/tags/v0.5.1^{tag}' 2>/dev/null) ||
    die "source checkout does not contain the annotated v0.5.1 tag"
  [[ $actual_tag == "$base_tag_object" ]] || die "source v0.5.1 tag object is unexpected"
}

apply_to_source() {
  local source=$1 status actual_tree actual_files expected_files
  verify_source_base "$source"
  actual_tree=$(git -C "$source" write-tree)
  if [[ $actual_tree == "$expected_tree" ]] &&
    ! git -C "$source" diff --cached --quiet HEAD; then
    die "refusing already-applied v0.5.2 patch"
  fi
  status=$(git -C "$source" status --porcelain=v1 --untracked-files=all)
  [[ -z $status ]] || die "refusing dirty source checkout"
  if ! git -C "$source" apply --check --index "$patch_file"; then
    die "patch does not apply cleanly to the exact base; source may already be patched"
  fi
  git -C "$source" apply --index "$patch_file"
  actual_tree=$(git -C "$source" write-tree)
  [[ $actual_tree == "$expected_tree" ]] ||
    die "resulting tree mismatch: expected $expected_tree, found $actual_tree"
  actual_files=$(git -C "$source" diff --cached --name-only HEAD | LC_ALL=C sort)
  expected_files=$(manifest_value files | LC_ALL=C sort)
  [[ $actual_files == "$expected_files" ]] ||
    die "changed-file inventory differs from manifest"
  git -C "$source" diff --quiet || die "patch unexpectedly left unstaged changes"
  python3 - "$source" <<'PY'
from pathlib import Path
import sys

sdk = Path(sys.argv[1], "crates/swallowtail-adapter-claude-agent/src/sdk.rs")
contents = sdk.read_text(encoding="utf-8")
for expected in (
    'CLAUDE_AGENT_SDK_VERSION: &str = "0.3.259"',
    'CLAUDE_AGENT_SDK_NATIVE_VERSION: &str = "2.1.259"',
    'CLAUDE_AGENT_SDK_NODE_RUNTIME: &str = "22.23.2"',
):
    if expected not in contents:
        raise SystemExit(f"released SDK identity changed or disappeared: {expected}")
PY
  printf 'applied patch to %s; resulting tree %s\n' "$source" "$actual_tree"
}

expect_refusal() {
  local expected=$1 output=$2
  shift 2
  if "$@" >"$output" 2>&1; then
    cat "$output" >&2
    die "expected refusal containing: $expected"
  fi
  if ! grep -Fq "$expected" "$output"; then
    cat "$output" >&2
    die "refusal did not identify the expected condition: $expected"
  fi
}

new_scratch_root() {
  mktemp -d "${TMPDIR:-/tmp}/swallowtail-sdk-backport.XXXXXXXX"
}

run_self_check() {
  local scratch first second dirty wrong first_tree second_tree first_files second_files
  check_root_tag
  scratch=$(new_scratch_root)
  export TMPDIR="$scratch"
  first="$scratch/first"
  second="$scratch/second"
  dirty="$scratch/dirty"
  wrong="$scratch/wrong"
  checkout_base "$first"
  apply_to_source "$first"
  first_tree=$(git -C "$first" write-tree)
  first_files=$(git -C "$first" diff --cached --name-only HEAD | LC_ALL=C sort)
  checkout_base "$second"
  apply_to_source "$second"
  second_tree=$(git -C "$second" write-tree)
  second_files=$(git -C "$second" diff --cached --name-only HEAD | LC_ALL=C sort)
  [[ $first_tree == "$second_tree" ]] || die "fresh reapplications produced different trees"
  [[ $first_files == "$second_files" ]] || die "fresh reapplications produced different file inventories"
  expect_refusal "already-applied" "$scratch/already-applied.log" \
    bash "$backport_dir/runner.sh" apply --source "$first"
  checkout_base "$dirty"
  printf 'task-owned dirty sentinel\n' > "$dirty/task-owned-dirty-sentinel"
  expect_refusal "dirty source" "$scratch/dirty.log" \
    bash "$backport_dir/runner.sh" apply --source "$dirty"
  mkdir -p "$wrong"
  git -C "$wrong" init -q
  git -C "$wrong" remote add task-source "$repo_root"
  git -C "$wrong" fetch --quiet --no-tags task-source HEAD
  git -C "$wrong" checkout --quiet --detach FETCH_HEAD
  expect_refusal "not the exact v0.5.1 peeled commit" "$scratch/wrong-base.log" \
    bash "$backport_dir/runner.sh" apply --source "$wrong"
  printf 'runner refusal/application proof passed; independent trees: %s\n' "$first_tree"
  printf 'task-owned fixture root: %s\n' "$scratch"
}

parse_scope() {
  local mode=$1
  shift
  packages=""
  test_filter=""
  while (($#)); do
    case $1 in
      --packages)
        (($# >= 2)) || die "--packages requires a comma-separated package list"
        packages=$2
        shift 2
        ;;
      --filter)
        (($# >= 2)) || die "--filter requires a nextest expression"
        test_filter=$2
        shift 2
        ;;
      *) die "unknown $mode argument: $1" ;;
    esac
  done
  [[ $packages == "swallowtail-runtime,swallowtail-host-local,swallowtail-adapter-claude-agent" ]] ||
    die "scope must name the three manifest packages in the declared order"
  IFS=, read -r -a package_names <<< "$packages"
  package_args=()
  for package in "${package_names[@]}"; do
    package_args+=(--package "$package")
  done
}

candidate_workspace() {
  local scratch=$1
  local candidate="$scratch/source"
  checkout_base "$candidate"
  apply_to_source "$candidate" >/dev/null
  printf '%s\n' "$candidate"
}

run_format() {
  (($# == 0)) || die "format takes no arguments"
  check_recipe_identity
  check_root_tag
  local scratch candidate consumer
  scratch=$(new_scratch_root)
  export TMPDIR="$scratch"
  candidate=$(candidate_workspace "$scratch")
  (cd "$candidate" && cargo fmt -p swallowtail-runtime -p swallowtail-host-local \
    -p swallowtail-adapter-claude-agent -- --check)
  consumer="$scratch/consumer-format"
  mkdir -p "$consumer/src"
  cp "$backport_dir/consumer/src/main.rs" "$consumer/src/main.rs"
  cat > "$consumer/Cargo.toml" <<'EOF'
[package]
name = "swallowtail-sdk-backport-consumer-format"
version = "0.0.0"
edition = "2024"
publish = false
EOF
  (cd "$consumer" && cargo fmt -- --check)
  printf 'format check passed for derived source tree %s\n' "$expected_tree"
  printf 'task-owned fixture root: %s\n' "$scratch"
}

run_check() {
  parse_scope check "$@"
  [[ -z $test_filter ]] || die "check accepts package scope only; tests use validate:sdk-patch-backport"
  check_recipe_identity
  check_root_tag
  run_self_check
  local scratch candidate baseline api_dir consumer
  scratch=$(new_scratch_root)
  export TMPDIR="$scratch"
  candidate=$(candidate_workspace "$scratch")
  baseline="$scratch/baseline"
  checkout_base "$baseline"
  export CARGO_TARGET_DIR="$scratch/target-check"
  cargo check --locked --manifest-path "$candidate/Cargo.toml" --all-targets "${package_args[@]}"
  run_public_api_inventory "$baseline" "$candidate" "$scratch/api"
  consumer="$scratch/consumer"
  run_external_consumer "$candidate" "$consumer" "$scratch/target-consumer"
  printf 'check passed for exact base %s and derived tree %s\n' "$base_commit" "$expected_tree"
  printf 'API comparison and external consumer evidence are under task-owned fixture root %s\n' "$scratch"
}

run_public_api_inventory() {
  local baseline=$1 candidate=$2 output_root=$3 package base_output candidate_output
  [[ $(cargo-public-api --version) == "$api_tool_version" ]] ||
    die "required public API tool is $api_tool_version"
  rustup run "$api_toolchain" rustc --version >/dev/null ||
    die "required public API Rust toolchain is unavailable: $api_toolchain"
  mkdir -p "$output_root/baseline" "$output_root/candidate" "$output_root/additions"
  while IFS= read -r package; do
    [[ -n $package ]] || continue
    base_output="$output_root/baseline/$package.txt"
    candidate_output="$output_root/candidate/$package.txt"
    (
      cd "$baseline"
      CARGO_TARGET_DIR="$output_root/target-baseline" \
        cargo +"$api_toolchain" public-api --package "$package" --all-features \
          --simplified --simplified --simplified --color never
    ) | LC_ALL=C sort > "$base_output"
    (
      cd "$candidate"
      CARGO_TARGET_DIR="$output_root/target-candidate" \
        cargo +"$api_toolchain" public-api --package "$package" --all-features \
          --simplified --simplified --simplified --color never
    ) | LC_ALL=C sort > "$candidate_output"
      python3 - "$base_output" "$candidate_output" "$output_root/additions/$package.txt" "$package" <<'PY'
from pathlib import Path
import sys

base_path, candidate_path, additions_path = map(Path, sys.argv[1:4])
package = sys.argv[4]
base = set(base_path.read_text(encoding="utf-8").splitlines())
candidate = set(candidate_path.read_text(encoding="utf-8").splitlines())
removed = sorted(base - candidate)
if removed:
    print(f"public API removals in {package}:", file=sys.stderr)
    print("\n".join(removed), file=sys.stderr)
    raise SystemExit(1)
additions = sorted(candidate - base)
additions_path.write_text("\n".join(additions) + ("\n" if additions else ""), encoding="utf-8")
print(f"{package}: {len(additions)} additive public API entries; zero removals")
PY
  done < <(manifest_value api_packages)
}

run_external_consumer() {
  local source=$1 consumer=$2 target=$3 source_commit source_url expected_commit
  mkdir -p "$consumer/src"
  git -C "$source" -c user.name='Swallowtail backport proof' \
    -c user.email='backport-proof@invalid' add -A
  GIT_AUTHOR_DATE=2000-01-01T00:00:00Z GIT_COMMITTER_DATE=2000-01-01T00:00:00Z \
    git -C "$source" -c user.name='Swallowtail backport proof' \
      -c user.email='backport-proof@invalid' -c commit.gpgsign=false \
      commit -q -m 'Deterministic v0.5.1 SDK backport source'
  source_commit=$(git -C "$source" rev-parse HEAD)
  expected_commit=$(manifest_value consumer_fixture.source_commit)
  [[ $source_commit == "$expected_commit" ]] ||
    die "external source commit mismatch: expected $expected_commit, found $source_commit"
  [[ $(git -C "$source" rev-parse 'HEAD^{tree}') == "$expected_tree" ]] ||
    die "external source snapshot does not match the expected patch tree"
  source_url="file://$source"
  python3 - "$backport_dir/consumer" "$consumer" "$source_url" "$source_commit" <<'PY'
from pathlib import Path
import sys

templates, destination = Path(sys.argv[1]), Path(sys.argv[2])
source_url, source_commit = sys.argv[3:5]
manifest = (templates / "Cargo.toml.template").read_text(encoding="utf-8")
manifest = manifest.replace("__SWALLOWTAIL_BACKPORT_SOURCE_URL__", source_url)
manifest = manifest.replace("__SWALLOWTAIL_BACKPORT_SOURCE_COMMIT__", source_commit)
lock = (templates / "Cargo.lock.template").read_text(encoding="utf-8")
lock_source = f"git+{source_url}?rev={source_commit}#{source_commit}"
if lock.count("__SWALLOWTAIL_BACKPORT_GIT_SOURCE__") != 6:
    raise SystemExit("consumer lock template has an unexpected Git source inventory")
lock = lock.replace("__SWALLOWTAIL_BACKPORT_GIT_SOURCE__", lock_source)
if "__SWALLOWTAIL_BACKPORT_" in manifest or "__SWALLOWTAIL_BACKPORT_" in lock:
    raise SystemExit("consumer template contains an unresolved source token")
(destination / "Cargo.toml").write_text(manifest, encoding="utf-8")
(destination / "Cargo.lock").write_text(lock, encoding="utf-8")
source_main = templates / "src/main.rs"
(destination / "src/main.rs").write_bytes(source_main.read_bytes())
PY
  CARGO_TARGET_DIR="$target" cargo run --locked --manifest-path "$consumer/Cargo.toml"
  CARGO_TARGET_DIR="$target" cargo metadata --manifest-path "$consumer/Cargo.toml" \
    --format-version 1 --locked > "$consumer/metadata.json"
  jq -e --arg commit "$source_commit" '
    [ .packages[] | select(.name | startswith("swallowtail-"))
      | select(.name != "swallowtail-sdk-backport-consumer") ] as $packages |
    ($packages | length) >= 4 and
    all($packages[]; (.source // "") | startswith("git+file://") and endswith("#" + $commit))
  ' "$consumer/metadata.json" >/dev/null
  printf 'external consumer prepared and ran against exact source commit %s\n' "$source_commit"
}

run_validate() {
  parse_scope validate "$@"
  [[ -n $test_filter ]] || die "validate requires an explicit nextest filter"
  check_recipe_identity
  check_root_tag
  local scratch candidate
  scratch=$(new_scratch_root)
  export TMPDIR="$scratch"
  candidate=$(candidate_workspace "$scratch")
  export CARGO_TARGET_DIR="$scratch/target-validate"
  (
    cd "$candidate"
    cargo nextest run --locked --profile sdk-patch-backport \
      --package swallowtail-runtime --lib -E "$test_filter"
  )
  (
    cd "$candidate"
    cargo nextest run --locked --profile sdk-patch-backport \
      --package swallowtail-host-local --features mediated-stdio-proxy \
      --test registered_tool_proxy -E "$test_filter"
  )
  (
    cd "$candidate"
    cargo nextest run --locked --profile sdk-patch-backport \
      --package swallowtail-adapter-claude-agent --lib \
      --test claude_agent_sdk_driver --test claude_agent_sdk_sidecar_asset \
      --test integration \
      -E "$test_filter"
  )
  printf 'targeted derived-code tests passed for %s\n' "$expected_tree"
  printf 'task-owned fixture root: %s\n' "$scratch"
}

main() {
  (($# >= 1)) || die "usage: runner.sh apply --source DIR | self-check | format | check | validate"
  local command=$1
  shift
  case $command in
    apply)
      local source=""
      [[ $# == 2 && $1 == --source ]] || die "usage: runner.sh apply --source DIR"
      source=$2
      check_recipe_identity
      apply_to_source "$source"
      ;;
    self-check)
      (($# == 0)) || die "self-check takes no arguments"
      check_recipe_identity
      run_self_check
      ;;
    format) run_format "$@" ;;
    check) run_check "$@" ;;
    validate) run_validate "$@" ;;
    *) die "unknown command: $command" ;;
  esac
}

main "$@"
