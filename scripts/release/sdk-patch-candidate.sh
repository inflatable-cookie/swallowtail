#!/usr/bin/env bash
set -euo pipefail

candidate_script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")/sdk-patch-candidate" && pwd)
candidate_repo_root=$(cd "$candidate_script_dir/../../.." && pwd)
candidate_manifest="$candidate_script_dir/manifest.json"
candidate_identity="$candidate_script_dir/check_identity.py"
candidate_consumer="$candidate_script_dir/consumer"
candidate_version=0.5.2
candidate_base=e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27
candidate_active_scratch=

candidate_cleanup() {
  [[ -z $candidate_active_scratch ]] || rm -rf "$candidate_active_scratch"
}

trap candidate_cleanup EXIT

candidate_die() {
  printf 'sdk-patch-candidate: %s\n' "$1" >&2
  exit 2
}

candidate_manifest_value() {
  python3 - "$candidate_manifest" "$1" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as stream:
    value = json.load(stream)
for part in sys.argv[2].split("."):
    value = value[part]
if isinstance(value, list):
    print("\n".join(value))
else:
    print(value)
PY
}

candidate_scratch() {
  mktemp -d "${TMPDIR:-/tmp}/swallowtail-sdk-candidate.XXXXXXXX"
}

candidate_identity_check() {
  python3 "$candidate_identity" "$1"
}

candidate_check_manifest() {
  python3 - "$candidate_manifest" <<'PY'
import json
import re
import sys

with open(sys.argv[1], encoding="utf-8") as stream:
    manifest = json.load(stream)
if manifest.get("schema_version") != 1 or manifest.get("candidate_version") != "0.5.2":
    raise SystemExit("candidate manifest version or schema is unsupported")
if manifest.get("base_commit") != "e9140b4634ee8ccd7cb0b08979cafe7d9e1e9e27":
    raise SystemExit("candidate manifest does not name the exact released v0.5.1 base")
if manifest.get("intermediate_tree") != "35252ecf3dcb4254f66a9ed16caf813d2397f5f7":
    raise SystemExit("candidate manifest does not name the reviewed intermediate tree")
files = manifest.get("patch_files", [])
if len(files) != 29 or len(set(files)) != 29:
    raise SystemExit("candidate manifest must preserve the exact reviewed 29-path patch inventory")
for key in ("patch_sha256", "sidecar_source_sha256"):
    if not re.fullmatch(r"[0-9a-f]{64}", manifest.get(key, "")):
        raise SystemExit(f"candidate manifest has an invalid {key}")
PY
}

candidate_check_clean_head() {
  local head merge_base merges
  [[ -z $(git -C "$candidate_repo_root" status --porcelain=v1 --untracked-files=all) ]] ||
    candidate_die "candidate checkout must be clean before its release proof"
  head=$(git -C "$candidate_repo_root" rev-parse HEAD)
  merge_base=$(git -C "$candidate_repo_root" merge-base "$head" "$candidate_base") ||
    candidate_die "candidate does not descend from released v0.5.1"
  [[ $merge_base == "$candidate_base" ]] ||
    candidate_die "candidate base is not the exact released v0.5.1 commit"
  merges=$(git -C "$candidate_repo_root" rev-list --merges "$candidate_base..$head")
  [[ -z $merges ]] || candidate_die "candidate contains a merge commit after v0.5.1"
  printf '%s\n' "$head"
}

candidate_check_fixture_hashes() {
  python3 - "$candidate_manifest" "$candidate_consumer" <<'PY'
import hashlib
import json
import sys
from pathlib import Path

manifest_path, consumer_root = Path(sys.argv[1]), Path(sys.argv[2])
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
fixtures = manifest["consumer_fixture"]
for filename, field in (
    ("Cargo.lock.template", "cargo_lock_template_sha256"),
    ("Cargo.toml.template", "cargo_manifest_template_sha256"),
    ("src/main.rs", "main_rs_sha256"),
):
    actual = hashlib.sha256((consumer_root / filename).read_bytes()).hexdigest()
    expected = fixtures[field]
    if actual != expected:
        raise SystemExit(f"consumer fixture SHA-256 mismatch: {filename}")
PY
}

candidate_check_api_baselines() {
  python3 - "$candidate_repo_root" "$candidate_manifest" <<'PY'
import json
import sys
from pathlib import Path

root, manifest_path = Path(sys.argv[1]), Path(sys.argv[2])
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
previous = root / "release-baselines/public-api-0.5.1"
candidate = root / f"release-baselines/public-api-{manifest['candidate_version']}"
packages = (previous / "packages.txt").read_text(encoding="utf-8").splitlines()
candidate_packages = (candidate / "packages.txt").read_text(encoding="utf-8").splitlines()
if candidate_packages != packages:
    raise SystemExit("candidate package inventory differs from immutable v0.5.1")
expected_files = {"packages.txt", *(f"{name}.txt" for name in packages)}
actual_files = {path.name for path in candidate.iterdir() if path.is_file()}
if actual_files != expected_files:
    raise SystemExit("candidate API baseline has a missing or unexpected file")
for package in packages:
    old = set((previous / f"{package}.txt").read_text(encoding="utf-8").splitlines())
    new = set((candidate / f"{package}.txt").read_text(encoding="utf-8").splitlines())
    removed = sorted(old - new)
    if removed:
        raise SystemExit(f"candidate API baseline removes {package} items: {removed}")
    additions = sorted(new - old)
    expected = manifest["api_additions"] if package == "swallowtail-runtime" else []
    missing = sorted(name for name in expected if not any(name in line for line in additions))
    unexpected = sorted(line for line in additions if not any(name in line for name in expected))
    if missing or unexpected or len(additions) != len(expected):
        raise SystemExit(
            f"unexpected {package} baseline additions: missing {missing}; unexpected {unexpected}"
        )

old_routes = (root / "release-baselines/production-routes-0.5.1.txt").read_bytes()
new_routes = (root / f"release-baselines/production-routes-{manifest['candidate_version']}.txt").read_bytes()
if old_routes != new_routes:
    raise SystemExit("candidate route inventory differs from immutable v0.5.1")
old_edges = (root / "release-baselines/internal-dependencies-0.5.1.tsv").read_text(encoding="utf-8")
new_edges = (root / f"release-baselines/internal-dependencies-{manifest['candidate_version']}.tsv").read_text(encoding="utf-8")
if old_edges.replace("^0.5.1", "^0.5.2") != new_edges:
    raise SystemExit("candidate internal dependency inventory differs beyond coordinated version")
print("candidate inventories passed: seven runtime additions, zero removals, unchanged routes and edges")
PY
}

candidate_run_external_consumer() {
  local scratch=$1 head=$2 source_url consumer target source_commit
  candidate_check_fixture_hashes
  consumer="$scratch/consumer"
  target="$scratch/target-consumer"
  source_url="file://$candidate_repo_root"
  source_commit=$head
  mkdir -p "$consumer/src"
  python3 - "$candidate_consumer" "$consumer" "$source_url" "$source_commit" <<'PY'
from pathlib import Path
import sys

templates, destination = Path(sys.argv[1]), Path(sys.argv[2])
source_url, source_commit = sys.argv[3:5]
manifest = (templates / "Cargo.toml.template").read_text(encoding="utf-8")
lock = (templates / "Cargo.lock.template").read_text(encoding="utf-8")
if manifest.count("__SWALLOWTAIL_CANDIDATE_SOURCE_URL__") != 4:
    raise SystemExit("consumer manifest has an unexpected Git-source inventory")
if manifest.count("__SWALLOWTAIL_CANDIDATE_SOURCE_COMMIT__") != 4:
    raise SystemExit("consumer manifest has an unexpected revision inventory")
manifest = manifest.replace("__SWALLOWTAIL_CANDIDATE_SOURCE_URL__", source_url)
manifest = manifest.replace("__SWALLOWTAIL_CANDIDATE_SOURCE_COMMIT__", source_commit)
lock_source = f"git+{source_url}?rev={source_commit}#{source_commit}"
if lock.count("__SWALLOWTAIL_CANDIDATE_GIT_SOURCE__") != 6:
    raise SystemExit("consumer lock template has an unexpected Git-source inventory")
lock = lock.replace("__SWALLOWTAIL_CANDIDATE_GIT_SOURCE__", lock_source)
if "__SWALLOWTAIL_CANDIDATE_" in manifest or "__SWALLOWTAIL_CANDIDATE_" in lock:
    raise SystemExit("consumer fixture contains an unresolved source token")
(destination / "Cargo.toml").write_text(manifest, encoding="utf-8")
(destination / "Cargo.lock").write_text(lock, encoding="utf-8")
(destination / "src/main.rs").write_bytes((templates / "src/main.rs").read_bytes())
PY
  CARGO_TARGET_DIR="$target" cargo run --locked --manifest-path "$consumer/Cargo.toml"
  CARGO_TARGET_DIR="$target" cargo metadata --manifest-path "$consumer/Cargo.toml" \
    --format-version 1 --locked > "$consumer/metadata.json"
  jq -e --arg commit "$source_commit" '
    [ .packages[] | select(.name | startswith("swallowtail-"))
      | select(.name != "swallowtail-sdk-candidate-consumer") ] as $packages |
    ($packages | length) >= 6 and
    all($packages[]; (.source // "") | startswith("git+file://") and endswith("#" + $commit))
  ' "$consumer/metadata.json" >/dev/null
  printf 'locked prepared-facade consumer passed against candidate commit %s\n' "$source_commit"
}

candidate_prepare_baselines() {
  local scratch output_root target_root previous current package tool_version toolchain
  candidate_check_manifest
  candidate_identity_check "$candidate_repo_root"
  tool_version=$(candidate_manifest_value api_tool_version)
  toolchain=$(candidate_manifest_value api_toolchain)
  [[ $(cargo-public-api --version 2>/dev/null) == "$tool_version" ]] ||
    candidate_die "required public API tool is $tool_version"
  rustup run "$toolchain" rustc --version >/dev/null 2>&1 ||
    candidate_die "required public API Rust toolchain is unavailable: $toolchain"
  previous="$candidate_repo_root/release-baselines/public-api-0.5.1"
  current="$candidate_repo_root/release-baselines/public-api-$candidate_version"
  [[ -d $previous ]] || candidate_die "immutable v0.5.1 API baseline is missing"
  [[ ! -e $current ]] || candidate_die "refusing to overwrite candidate API baselines"
  [[ ! -e "$candidate_repo_root/release-baselines/production-routes-$candidate_version.txt" ]] ||
    candidate_die "refusing to overwrite candidate route baseline"
  [[ ! -e "$candidate_repo_root/release-baselines/internal-dependencies-$candidate_version.tsv" ]] ||
    candidate_die "refusing to overwrite candidate dependency baseline"
  scratch=$(candidate_scratch)
  candidate_active_scratch=$scratch
  output_root="$scratch/api"
  target_root="$scratch/target-api"
  mkdir -p "$output_root"
  while IFS= read -r package; do
    [[ -n $package ]] || continue
    (
      cd "$candidate_repo_root"
      CARGO_TARGET_DIR="$target_root" cargo +"$toolchain" public-api \
        --package "$package" --all-features \
        --simplified --simplified --simplified --color never
    ) | LC_ALL=C sort > "$output_root/$package.txt"
    python3 - "$package" "$output_root/$package.txt" "$previous/$package.txt" "$candidate_manifest" <<'PY'
import json
import sys
from pathlib import Path

package = sys.argv[1]
actual = set(Path(sys.argv[2]).read_text(encoding="utf-8").splitlines())
previous = set(Path(sys.argv[3]).read_text(encoding="utf-8").splitlines())
manifest = json.loads(Path(sys.argv[4]).read_text(encoding="utf-8"))
removed = sorted(previous - actual)
if removed:
    raise SystemExit(f"public API removals in {package}:\n" + "\n".join(removed))
added = sorted(actual - previous)
expected = sorted(manifest["api_additions"] if package == "swallowtail-runtime" else [])
missing = sorted(name for name in expected if not any(name in line for line in added))
unexpected = sorted(line for line in added if not any(name in line for name in expected))
if missing or unexpected or len(added) != len(expected):
    raise SystemExit(
        f"unexpected public API additions in {package}: missing {missing}; unexpected {unexpected}"
    )
PY
  done < <(candidate_manifest_value api_packages)

  cp -R "$previous" "$current"
  while IFS= read -r package; do
    [[ -n $package ]] || continue
    cp "$output_root/$package.txt" "$current/$package.txt"
  done < <(candidate_manifest_value api_packages)
  cp "$candidate_repo_root/release-baselines/production-routes-0.5.1.txt" \
    "$candidate_repo_root/release-baselines/production-routes-$candidate_version.txt"
  python3 - "$candidate_repo_root/release-baselines/internal-dependencies-0.5.1.tsv" \
    "$candidate_repo_root/release-baselines/internal-dependencies-$candidate_version.tsv" <<'PY'
from pathlib import Path
import sys

source, destination = map(Path, sys.argv[1:3])
contents = source.read_text(encoding="utf-8")
if "^0.5.1" not in contents:
    raise SystemExit("v0.5.1 internal dependency inventory has no coordinated requirements")
destination.write_text(contents.replace("^0.5.1", "^0.5.2"), encoding="utf-8")
PY
  printf 'prepared v0.5.2 package, route, dependency, and API baselines from the actual candidate; v0.5.1 files were not changed\n'
  printf 'API baseline generation used only the three affected packages; temporary output was removed\n'
  rm -rf "$scratch"
  candidate_active_scratch=
}

candidate_format() {
  (($# == 0)) || candidate_die "format takes no arguments"
  local scratch consumer
  scratch=$(candidate_scratch)
  candidate_active_scratch=$scratch
  (cd "$candidate_repo_root" && cargo fmt -p swallowtail-runtime \
    -p swallowtail-host-local -p swallowtail-adapter-claude-agent -- --check)
  consumer="$scratch/consumer-format"
  mkdir -p "$consumer/src"
  cp "$candidate_consumer/src/main.rs" "$consumer/src/main.rs"
  cat > "$consumer/Cargo.toml" <<'EOF'
[package]
name = "swallowtail-sdk-candidate-consumer-format"
version = "0.0.0"
edition = "2024"
publish = false
EOF
  (cd "$consumer" && cargo fmt -- --check)
  printf 'scoped source and external consumer formatting passed\n'
}

candidate_check() {
  (($# == 0)) || candidate_die "check takes no arguments"
  local head scratch candidate_packages
  candidate_check_manifest
  head=$(candidate_check_clean_head)
  candidate_identity_check "$candidate_repo_root"
  bash "$candidate_repo_root/scripts/tests/sdk-patch-candidate-identity.sh"
  [[ -f "$candidate_repo_root/release-baselines/public-api-$candidate_version/packages.txt" ]] ||
    candidate_die "candidate package/API baselines are missing; run the baseline preparation selector first"
  candidate_check_api_baselines
  scratch=$(candidate_scratch)
  candidate_active_scratch=$scratch
  candidate_packages=(--package swallowtail-runtime --package swallowtail-host-local \
    --package swallowtail-adapter-claude-agent)
  CARGO_TARGET_DIR="$scratch/target-check" cargo check --locked \
    --manifest-path "$candidate_repo_root/Cargo.toml" --all-targets "${candidate_packages[@]}"
  candidate_run_external_consumer "$scratch" "$head"
  printf 'candidate check passed at commit %s; intermediate reviewed source tree %s\n' \
    "$head" "$(candidate_manifest_value intermediate_tree)"
  printf 'candidate check fixtures used a fresh temporary root that was removed\n'
}

candidate_validate() {
  (($# == 0)) || candidate_die "validate takes no arguments"
  local head scratch test_filter
  candidate_check_manifest
  head=$(candidate_check_clean_head)
  candidate_identity_check "$candidate_repo_root"
  test_filter='test(/a_session_lease_ignores_its_elapsed_open_deadline/) | test(/an_explicit_session_lease_deadline_rejects_late_calls/) | test(/the_effective_call_deadline_takes_the_earliest_bound/) | test(/an_expired_call_never_reaches_the_dispatcher/) | test(/the_kernel_is_the_only_lease_and_binding_source/) | test(/mounted_proxy_expired_deadline_rejects_the_callable_frame_before_dispatch/) | test(/a_pump_failure_between_command_check_and_registration_rejects_the_command/) | test(/registered_session_lease_survives_open_deadline_and_pending_permission/) | test(/cancellation_during_a_registered_call_joins_the_lease/) | test(/close_joins_the_registered_listener/) | test(/events_outside_an_active_turn_fail_closed/) | test(/closing_a_session_with_a_live_turn_resolves_it_instead_of_waiting_on_its_deadline/) | test(/usage_snapshots_keep_each_turn_independent_across_failure_and_reset/) | test(/usage_duplicate_result_after_turn_end_is_not_correlated_or_counted_twice/) | test(/usage_projection_keeps_only_bounded_per_turn_counters_and_rejects_invalid_snapshots/) | test(/usage_events_decode_qualified_payloads_and_reject_unknown_usage_report/) | test(/usage_turn_end_decodes_every_sanitized_result_observation_without_result_text/) | test(/turn_end_rejects_malformed_or_missing_usage_counts/) | test(/the_route_binds_five_independent_exact_identities/) | test(/the_lifecycle_and_credential_invariants_survived_the_hop/) | test(/identity_and_claim_qualify_0_69_0_as_compatible_extension/) | test(/prepared_route_returns_one_ordinary_text_result_without_authority/)'
  scratch=$(candidate_scratch)
  candidate_active_scratch=$scratch
  export CARGO_TARGET_DIR="$scratch"
  (
    cd "$candidate_repo_root"
    cargo nextest run --locked --profile sdk-patch-candidate \
      -p swallowtail-runtime --lib -E "$test_filter"
  )
  (
    cd "$candidate_repo_root"
    cargo nextest run --locked --profile sdk-patch-candidate \
      -p swallowtail-host-local --features mediated-stdio-proxy \
      --test registered_tool_proxy -E "$test_filter"
  )
  (
    cd "$candidate_repo_root"
    cargo nextest run --locked --profile sdk-patch-candidate \
      -p swallowtail-adapter-claude-agent --lib \
      --test claude_agent_sdk_driver --test claude_agent_sdk_sidecar_asset \
      --test integration -E "$test_filter"
  )
  printf '22 candidate regressions passed at commit %s\n' "$head"
}

candidate_main() {
  (($# >= 1)) || candidate_die "usage: candidate.sh prepare-baselines | format | check | validate"
  local command=$1
  shift
  case "$command" in
    prepare-baselines) candidate_prepare_baselines "$@" ;;
    format) candidate_format "$@" ;;
    check) candidate_check "$@" ;;
    validate) candidate_validate "$@" ;;
    *) candidate_die "unknown command: $command" ;;
  esac
}

candidate_main "$@"
