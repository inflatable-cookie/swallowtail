#!/usr/bin/env bash
set -euo pipefail

validation_repo_root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$validation_repo_root"

source "$validation_repo_root/scripts/validation/archive.sh"
source "$validation_repo_root/scripts/validation/path.sh"

validation_scratch_base=${TMPDIR:-/tmp}
while [[ "$validation_scratch_base" == */ && "$validation_scratch_base" != / ]]; do
  validation_scratch_base=${validation_scratch_base%/}
done
validation_real_mktemp=$(command -v mktemp)
validation_real_tar=$(command -v tar)
validation_cleanup_paths=()
validation_cleanup() {
  if ((${#validation_cleanup_paths[@]})); then
    rm -rf "${validation_cleanup_paths[@]}"
  fi
}
trap validation_cleanup EXIT

validation_expect_failure() {
  local validation_expected=$1
  shift
  local validation_output
  if validation_output=$("$@" 2>&1); then
    printf 'expected validation selector failure\n' >&2
    exit 1
  fi
  if [[ "$validation_output" != *"$validation_expected"* ]]; then
    printf 'validation selector failure changed: %s\n' "$validation_output" >&2
    exit 1
  fi
}

validation_focused_plan=$(
  bash scripts/validate-focused-packages.sh \
    --plan \
    swallowtail-adapter-pi \
    swallowtail-adapter-xai
)
[[ "$validation_focused_plan" == *"selector=validate:focused"* ]]
[[ "$validation_focused_plan" == *"package=swallowtail-adapter-pi"* ]]
[[ "$validation_focused_plan" == *"proof=warnings-denied-all-target-clippy"* ]]

validation_affected_plan=$(
  bash scripts/verify-affected-packages.sh \
    --plan \
    swallowtail-adapter-pi \
    swallowtail-adapter-xai
)
[[ "$validation_affected_plan" == *"selector=package:verify-affected"* ]]
[[ "$validation_affected_plan" == *"archive_isolation=independent"* ]]
[[ "$validation_affected_plan" == *"compile_target=shared"* ]]

validation_expect_failure \
  "requires one to four package names" \
  bash scripts/validate-focused-packages.sh --plan
validation_expect_failure \
  "contains a duplicate" \
  bash scripts/validate-focused-packages.sh \
    --plan \
    swallowtail-adapter-pi \
    swallowtail-adapter-pi
validation_expect_failure \
  "is not an exact workspace package" \
  bash scripts/verify-affected-packages.sh \
    --plan \
    swallowtail-adapter-not-real
validation_expect_failure \
  "requires one to four package names" \
  bash scripts/verify-affected-packages.sh \
    --plan \
    swallowtail-core \
    swallowtail-runtime \
    swallowtail-testkit \
    swallowtail-host-local \
    swallowtail-adapter-pi

printf '%s\n' "package-0.1.0/src/lib.rs" |
  validation_archive_member_list_is_safe
if printf '%s\n' "package-0.1.0/.env" |
  validation_archive_member_list_is_safe
then
  printf 'unsafe archive member was accepted\n' >&2
  exit 1
fi

validation_audit_tree=$(mktemp -d "${validation_scratch_base}/validation-audit.XXXXXXXX")
validation_cleanup_paths+=("$validation_audit_tree")
validation_audit_repo_root="/nonexistent-repo-root-for-archive-audit"
printf '%s\n' \
  "headless-background-task-waiting-notice-occasionally-skipped" \
  >"$validation_audit_tree/hyphenated-english.txt"
if ! validation_extracted_tree_is_safe \
  "$validation_audit_tree" \
  "$validation_audit_repo_root"
then
  printf 'hyphenated English tripped the secret pattern\n' >&2
  exit 1
fi
printf '%s\n' "/repoxwithmeta1" >"$validation_audit_tree/regex-like-path.txt"
if ! validation_extracted_tree_is_safe \
  "$validation_audit_tree" \
  "/repo.with+meta[1]"
then
  printf 'regex metacharacters in repo root were not escaped\n' >&2
  exit 1
fi
printf '%s\n' "/repo.with+meta[1]/file" >"$validation_audit_tree/literal-meta-path.txt"
if validation_extracted_tree_is_safe \
  "$validation_audit_tree" \
  "/repo.with+meta[1]"
then
  printf 'literal repo root with regex metacharacters was not flagged\n' >&2
  exit 1
fi
rm -f "$validation_audit_tree/literal-meta-path.txt"
printf '%s\n' "sk-abcdefghijklmnopqrstuvwxyz123456" \
  >"$validation_audit_tree/synthetic-key.txt"
if validation_extracted_tree_is_safe \
  "$validation_audit_tree" \
  "$validation_audit_repo_root"
then
  printf 'synthetic sk- key was not flagged\n' >&2
  exit 1
fi
rm -rf "$validation_audit_tree"

validation_rg_fail_bin=$(mktemp -d "${validation_scratch_base}/validation-rg-fail.XXXXXXXX")
validation_cleanup_paths+=("$validation_rg_fail_bin")
cat >"$validation_rg_fail_bin/rg" <<'EOF'
#!/usr/bin/env bash
exit 2
EOF
chmod +x "$validation_rg_fail_bin/rg"
if PATH="$validation_rg_fail_bin:$PATH" \
  validation_archive_member_list_is_safe <<< "package-0.1.0/src/lib.rs" 2>/dev/null
then
  printf 'archive member audit false-passed on rg exit 2\n' >&2
  exit 1
fi
rm -rf "$validation_rg_fail_bin"

validation_real_root=$(mktemp -d "${validation_scratch_base}/validation-real.XXXXXXXX")
validation_alias_root="${validation_real_root}-alias"
validation_cleanup_paths+=("$validation_real_root" "$validation_alias_root")
ln -s "$validation_real_root" "$validation_alias_root"
validation_canonical_real=$(validation_canonical_path "$validation_real_root")
validation_canonical_alias=$(validation_canonical_path "$validation_alias_root")
if [[ "$validation_canonical_real" != "$validation_canonical_alias" ]]; then
  printf 'path alias canonicalization failed: %s != %s\n' \
    "$validation_canonical_real" "$validation_canonical_alias" >&2
  exit 1
fi
rm -f "$validation_alias_root"
rm -rf "$validation_real_root"

validation_host_home=$HOME
if ! bash scripts/run-with-isolated-home.sh --home-var GROK_HOME -- \
  env | rg -q '^GROK_HOME='
then
  printf 'isolated-home wrapper did not export GROK_HOME during probe\n' >&2
  exit 1
fi
if [[ "$HOME" != "$validation_host_home" ]]; then
  printf 'isolated-home wrapper left HOME=%s instead of %s\n' \
    "$HOME" "$validation_host_home" >&2
  exit 1
fi
if env | rg -q '^GROK_HOME='; then
  printf 'isolated-home wrapper left GROK_HOME exported after probe\n' >&2
  exit 1
fi

validation_shim_root=$(mktemp -d "${validation_scratch_base}/validation-shim.XXXXXXXX")
validation_cleanup_paths+=("$validation_shim_root")
validation_mktemp_log="$validation_shim_root/mktemp.log"
{
  printf '#!/usr/bin/env bash\n'
  printf 'validation_real_mktemp=%q\n' "$validation_real_mktemp"
  cat <<'EOF'
printf '%s\n' "$*" >>"${VALIDATION_MKTEMP_LOG:?}"
exec "$validation_real_mktemp" "$@"
EOF
} >"$validation_shim_root/mktemp"
chmod +x "$validation_shim_root/mktemp"
validation_shim_path="$validation_shim_root:$PATH"
if VALIDATION_MKTEMP_LOG="$validation_mktemp_log" PATH="$validation_shim_path" \
  bash scripts/run-with-isolated-home.sh --home-var 2>/dev/null
then
  printf 'isolated-home wrapper accepted a malformed --home-var invocation\n' >&2
  exit 1
fi
if VALIDATION_MKTEMP_LOG="$validation_mktemp_log" PATH="$validation_shim_path" \
  bash scripts/run-with-isolated-home.sh 2>/dev/null
then
  printf 'isolated-home wrapper accepted a missing-command invocation\n' >&2
  exit 1
fi
if [[ -s "$validation_mktemp_log" ]]; then
  printf 'isolated-home wrapper called mktemp before argument validation:\n' >&2
  cat "$validation_mktemp_log" >&2
  exit 1
fi
rm -rf "$validation_shim_root"

# Exercise the affected-package driver through a fake Oxide lease. The mktemp
# shim models BSD's refusal to honor TMPDIR for an unqualified template, while
# keeping that simulated host temp inside this test's own fixture.
validation_driver_fixture=$(mktemp -d "${validation_scratch_base}/affected-driver-fixture.XXXXXXXX")
validation_cleanup_paths+=("$validation_driver_fixture")
validation_driver_bin="$validation_driver_fixture/bin"
validation_host_temp="$validation_driver_fixture/host temp"
mkdir -p "$validation_driver_bin" "$validation_host_temp"
cat >"$validation_driver_bin/oxide" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ ${1:-} == metadata ]]; then
  printf '%s\n' '{"packages":[{"name":"swallowtail-adapter-pi","version":"9.9.9"}]}'
  exit 0
fi
if [[ ${1:-} == exec && ${2:-} == -- ]]; then
  printf 'lease\n' >>"${VALIDATION_DRIVER_LOG:?}/oxide.log"
  shift 2
  exec "$@"
fi
printf 'unexpected fake oxide arguments: %s\n' "$*" >&2
exit 90
EOF
cat >"$validation_driver_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case ${1:-} in
  package)
    target=
    while (($#)); do
      if [[ $1 == --target-dir ]]; then target=$2; shift 2; else shift; fi
    done
    stage="$target/fixture-stage/swallowtail-adapter-pi-9.9.9"
    mkdir -p "$stage"
    printf '[package]\nname = "swallowtail-adapter-pi"\nversion = "9.9.9"\n' >"$stage/Cargo.toml"
    mkdir -p "$target/package"
    tar -czf "$target/package/swallowtail-adapter-pi-9.9.9.crate" -C "$target/fixture-stage" swallowtail-adapter-pi-9.9.9
    ;;
  generate-lockfile) ;;
  check)
    printf '%s\n' "$*" >>"${VALIDATION_DRIVER_LOG:?}/cargo-check.log"
    [[ ${VALIDATION_DRIVER_FAIL_CHECK:-0} != 1 ]] || exit 42
    ;;
  *) printf 'unexpected fake cargo arguments: %s\n' "$*" >&2; exit 91 ;;
esac
EOF
{
  printf '#!/usr/bin/env bash\n'
  printf 'validation_real_mktemp=%q\n' "$validation_real_mktemp"
  cat <<'EOF'
set -euo pipefail
validation_mktemp_has_template=false
for validation_mktemp_arg in "$@"; do
  if [[ "$validation_mktemp_arg" != -* ]]; then
    validation_mktemp_has_template=true
    break
  fi
done
if [[ "$validation_mktemp_has_template" == false ]]; then
  # Simulate BSD mktemp's system-temp default without touching host temp.
  printf 'options-only: %s\n' "$*" >>"${VALIDATION_DRIVER_LOG:?}/mktemp.log"
  exec "$validation_real_mktemp" "$@" "${VALIDATION_HOST_TEMP:?}/bsd-default.XXXXXXXX"
fi
printf '%s\n' "$*" >>"${VALIDATION_DRIVER_LOG:?}/mktemp.log"
exec "$validation_real_mktemp" "$@"
EOF
} >"$validation_driver_bin/mktemp"
{
  printf '#!/usr/bin/env bash\n'
  printf 'validation_real_tar=%q\n' "$validation_real_tar"
  cat <<'EOF'
set -euo pipefail
validation_tar_args=("$@")
if [[ ${validation_tar_args[0]:-} == *x* ]]; then
  for ((validation_tar_index = 0; validation_tar_index < ${#validation_tar_args[@]}; validation_tar_index++)); do
    if [[ ${validation_tar_args[$validation_tar_index]} == -C ]]; then
      printf '%s\n' "${validation_tar_args[$((validation_tar_index + 1))]}" >>"${VALIDATION_DRIVER_LOG:?}/extract.log"
      break
    fi
  done
fi
exec "$validation_real_tar" "$@"
EOF
} >"$validation_driver_bin/tar"
chmod +x "$validation_driver_bin/oxide" "$validation_driver_bin/cargo" \
  "$validation_driver_bin/mktemp" "$validation_driver_bin/tar"

# An invalid TMPDIR must fail closed even when TMP and TEMP point at usable
# locations. The fake BSD mktemp would keep any accidental fallback confined
# to validation_host_temp, where this check also proves none was allocated.
validation_invalid_log="$validation_driver_fixture/log-invalid-root"
mkdir -p "$validation_invalid_log"
validation_invalid_root="$validation_driver_fixture/missing scratch root"
if env GITHUB_ACTIONS=false EFFIGY_OXIDE_ESCAPE=oxide \
  VALIDATION_DRIVER_FAIL_CHECK=0 \
  VALIDATION_DRIVER_LOG="$validation_invalid_log" \
  VALIDATION_HOST_TEMP="$validation_host_temp" \
  TMPDIR="$validation_invalid_root" TMP="$validation_driver_fixture" \
  TEMP="$validation_driver_fixture" PATH="$validation_driver_bin:$PATH" \
  bash scripts/verify-affected-packages.sh swallowtail-adapter-pi \
    >"$validation_invalid_log/stdout" 2>"$validation_invalid_log/stderr"
then
  printf 'affected-package driver accepted a missing supplied TMPDIR\n' >&2
  exit 1
fi
if ! rg -q 'scratch root is not an available writable directory' \
  "$validation_invalid_log/stderr"; then
  printf 'affected-package driver did not explain the invalid scratch root\n' >&2
  cat "$validation_invalid_log/stderr" >&2
  exit 1
fi
[[ $(wc -l <"$validation_invalid_log/oxide.log" | tr -d ' ') == 1 ]]
[[ ! -e "$validation_invalid_log/mktemp.log" ]]
[[ ! -e "$validation_invalid_log/extract.log" ]]
if find "$validation_host_temp" -mindepth 1 -print -quit | rg -q .; then
  printf 'invalid TMPDIR fell back to the simulated host temp\n' >&2
  exit 1
fi

for validation_driver_outcome in success failure; do
  validation_driver_root="$validation_driver_fixture/run scratch $validation_driver_outcome"
  mkdir -p "$validation_driver_root"
  printf 'parent sentinel\n' >"$validation_driver_root/PARENT-SENTINEL"
  printf 'sibling sentinel\n' >"$validation_driver_root/SIBLING-SENTINEL"
  validation_driver_log="$validation_driver_fixture/log-$validation_driver_outcome"
  mkdir -p "$validation_driver_log"
  if [[ $validation_driver_outcome == failure ]]; then
    validation_driver_failure_env=(VALIDATION_DRIVER_FAIL_CHECK=1)
  else
    validation_driver_failure_env=(VALIDATION_DRIVER_FAIL_CHECK=0)
  fi
  if env GITHUB_ACTIONS=false EFFIGY_OXIDE_ESCAPE=oxide \
    "${validation_driver_failure_env[@]}" \
    VALIDATION_DRIVER_LOG="$validation_driver_log" \
    VALIDATION_HOST_TEMP="$validation_host_temp" \
    TMPDIR="$validation_driver_root/" TMP="$validation_driver_root/" \
    TEMP="$validation_driver_root/" PATH="$validation_driver_bin:$PATH" \
    bash scripts/verify-affected-packages.sh swallowtail-adapter-pi \
      >"$validation_driver_log/stdout" 2>"$validation_driver_log/stderr"
  then
    [[ $validation_driver_outcome == success ]] || {
      printf 'affected-package driver unexpectedly passed its failure fixture\n' >&2
      exit 1
    }
  else
    [[ $validation_driver_outcome == failure ]] || {
      cat "$validation_driver_log/stderr" >&2
      exit 1
    }
  fi
  [[ $(wc -l <"$validation_driver_log/oxide.log" | tr -d ' ') == 1 ]]
  [[ -s "$validation_driver_log/extract.log" ]]
  validation_extract_root=$(cat "$validation_driver_log/extract.log")
  [[ "$validation_extract_root" == "$validation_driver_root"/swallowtail-affected.*/extracted ]]
  [[ ! -e "${validation_extract_root%/extracted}" ]]
  [[ ! -e "$validation_driver_root/$(basename "${validation_extract_root%/extracted}")" ]]
  [[ -f "$validation_driver_root/PARENT-SENTINEL" ]]
  [[ -f "$validation_driver_root/SIBLING-SENTINEL" ]]
  if rg -q '^options-only:' "$validation_driver_log/mktemp.log"; then
    printf 'affected-package driver used BSD options-only mktemp\n' >&2
    exit 1
  fi
  if ! rg -q 'swallowtail-affected\.XXXXXXXX' "$validation_driver_log/mktemp.log"; then
    printf 'affected-package driver did not pass an explicit scratch template\n' >&2
    exit 1
  fi
done
rm -rf "$validation_driver_fixture"

# Host-path debt is exempt by exact line, never by file. A listed line passes;
# the same file with a new or altered host path fails, as does any host path in
# an unlisted file.
validation_debt_root=$(mktemp -d "${validation_scratch_base}/validation-debt.XXXXXXXX")
validation_cleanup_paths+=("$validation_debt_root")
validation_debt_file=scripts/validation/host-path-debt.tsv
validation_debt_path=swallowtail-adapter-antigravity/src/discovery/probe.rs
validation_debt_line=$(grep -F -- "$validation_debt_path"$'\t' "$validation_debt_file" | head -1 | cut -f2-)
if [[ -z $validation_debt_line ]]; then
  printf 'host-path debt file lacks a row for %s\n' "$validation_debt_path" >&2
  exit 1
fi
validation_debt_write() {
  local validation_tree=$1
  local validation_relative=$2
  local validation_text=$3
  mkdir -p "$validation_tree/$(dirname "$validation_relative")"
  printf '%s\n' "$validation_text" >>"$validation_tree/$validation_relative"
}
validation_debt_tree=$validation_debt_root/listed
validation_debt_write "$validation_debt_tree" "$validation_debt_path" "$validation_debt_line"
if ! validation_extracted_tree_is_safe "$validation_debt_tree" /nonexistent-repo-root-for-archive-audit; then
  printf 'listed host-path debt row was rejected\n' >&2
  exit 1
fi
validation_debt_tree=$validation_debt_root/listed-extended
validation_debt_write "$validation_debt_tree" "$validation_debt_path" "$validation_debt_line"
validation_debt_write "$validation_debt_tree" "$validation_debt_path" "  wrapper at /Users/private/bin/newtool"
if validation_extracted_tree_is_safe "$validation_debt_tree" /nonexistent-repo-root-for-archive-audit >/dev/null 2>&1; then
  printf 'new host path in a listed debt file was accepted\n' >&2
  exit 1
fi
validation_debt_tree=$validation_debt_root/listed-altered
validation_debt_write "$validation_debt_tree" "$validation_debt_path" "$validation_debt_line /Users/private/extra"
if validation_extracted_tree_is_safe "$validation_debt_tree" /nonexistent-repo-root-for-archive-audit >/dev/null 2>&1; then
  printf 'altered host-path debt line was accepted\n' >&2
  exit 1
fi
validation_debt_tree=$validation_debt_root/unlisted
validation_debt_write "$validation_debt_tree" private/file "path /home/runner/private/file"
if validation_extracted_tree_is_safe "$validation_debt_tree" /nonexistent-repo-root-for-archive-audit >/dev/null 2>&1; then
  printf 'host path in an unlisted file was accepted\n' >&2
  exit 1
fi
rm -rf "$validation_debt_root"

# Nextest profile.ci / profile.ci-process complement sync and archive package
# ownership against .github/workflows/ci.yml.
validation_nextest_sync_check() {
  local validation_nextest_toml=$1
  local validation_ci_yml=$2
  local validation_metadata_json=$3
  python3 - "$validation_nextest_toml" "$validation_ci_yml" "$validation_metadata_json" <<'PY'
import json
import re
import shlex
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    tomllib = None

nextest_path = Path(sys.argv[1])
ci_path = Path(sys.argv[2])
metadata = json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))


def fail(message: str) -> None:
    print(f"nextest sync check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def parse_nextest_filters(path: Path) -> dict[str, str]:
    text = path.read_text(encoding="utf-8")
    if tomllib is not None:
        data = tomllib.loads(text)
        profiles = data.get("profile", {})
        return {
            name: prof["default-filter"]
            for name, prof in profiles.items()
            if isinstance(prof, dict) and isinstance(prof.get("default-filter"), str)
        }
    filters: dict[str, str] = {}
    current_profile: str | None = None
    for raw_line in text.splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        section = re.match(r"^\[profile\.([A-Za-z0-9_-]+)\]$", line)
        if section:
            current_profile = section.group(1)
            continue
        if line.startswith("["):
            current_profile = None
            continue
        if current_profile is not None:
            m = re.match(r'^default-filter\s*=\s*"([^"]*)"$', line)
            if m:
                filters[current_profile] = m.group(1)
    return filters


filters = parse_nextest_filters(nextest_path)
if "ci" not in filters or "ci-process" not in filters:
    fail("missing default-filter on profile.ci or profile.ci-process")

ci_filter = filters["ci"]
ci_process_filter = filters["ci-process"]

ci_match = re.fullmatch(
    r"not \((binary\([A-Za-z0-9_]+\)(?: \| binary\([A-Za-z0-9_]+\))*)\)",
    ci_filter,
)
if ci_match is None:
    fail(f"profile.ci default-filter is not an exact negated binary list: {ci_filter!r}")

ci_process_match = re.fullmatch(
    r"(binary\([A-Za-z0-9_]+\)(?: \| binary\([A-Za-z0-9_]+\))*)",
    ci_process_filter,
)
if ci_process_match is None:
    fail(f"profile.ci-process default-filter is not a binary disjunction: {ci_process_filter!r}")

ci_bins = re.findall(r"binary\(([A-Za-z0-9_]+)\)", ci_match.group(1))
ci_process_bins = re.findall(r"binary\(([A-Za-z0-9_]+)\)", ci_process_match.group(1))

if len(ci_bins) != len(set(ci_bins)):
    fail(f"profile.ci lists duplicate binaries: {ci_bins}")
if len(ci_process_bins) != len(set(ci_process_bins)):
    fail(f"profile.ci-process lists duplicate binaries: {ci_process_bins}")
if set(ci_bins) != set(ci_process_bins):
    fail(
        f"profile.ci and profile.ci-process binary sets differ: "
        f"ci_only={sorted(set(ci_bins) - set(ci_process_bins))}, "
        f"ci_process_only={sorted(set(ci_process_bins) - set(ci_bins))}"
    )

ci_lines = ci_path.read_text(encoding="utf-8").splitlines()
in_job = False
job_lines: list[str] = []
for line in ci_lines:
    if re.match(r"^  stable-process-nextest:\s*$", line):
        in_job = True
        continue
    if in_job:
        if re.match(r"^  [A-Za-z0-9_-]+:\s*$", line):
            break
        job_lines.append(line)
if not job_lines:
    fail("missing stable-process-nextest job in ci.yml")

job_text = "\n".join(job_lines)
archive_match = re.search(
    r"cargo nextest archive\b((?:[^\n]*\\\n)*[^\n]*)",
    job_text,
)
if archive_match is None:
    fail("missing cargo nextest archive invocation in stable-process-nextest")

archive_cmd = archive_match.group(0).replace("\\\n", " ")
tokens = shlex.split(archive_cmd)
archive_packages: list[str] = []
idx = 0
while idx < len(tokens):
    if tokens[idx] in ("-p", "--package"):
        if idx + 1 >= len(tokens):
            fail("trailing -p/--package in cargo nextest archive")
        archive_packages.append(tokens[idx + 1])
        idx += 2
    else:
        idx += 1

if not archive_packages:
    fail("stable-process-nextest archive step lists no -p packages")
if len(archive_packages) != len(set(archive_packages)):
    fail(f"stable-process-nextest archive step lists duplicate -p packages: {archive_packages}")
archive_pkg_set = set(archive_packages)

test_owners: dict[str, set[str]] = {}
workspace_packages: set[str] = set()
for pkg in metadata.get("packages", []):
    pkg_name = pkg["name"]
    workspace_packages.add(pkg_name)
    for target in pkg.get("targets", []):
        if "test" in target.get("kind", []):
            test_owners.setdefault(target["name"], set()).add(pkg_name)

unknown_archive_pkgs = sorted(archive_pkg_set - workspace_packages)
if unknown_archive_pkgs:
    fail(f"stable-process-nextest lists unknown workspace packages: {unknown_archive_pkgs}")

owned_by_ci_process: set[str] = set()
for binary in ci_process_bins:
    owners = test_owners.get(binary)
    if not owners:
        fail(f"ci-process binary {binary!r} is not a workspace test target")
    missing_pkgs = sorted(owners - archive_pkg_set)
    if missing_pkgs:
        fail(
            f"ci-process binary {binary!r} belongs to package(s) {missing_pkgs} "
            f"omitted from stable-process-nextest archive (-p {sorted(archive_pkg_set)})"
        )
    owned_by_ci_process |= owners

unused_archive_pkgs = sorted(archive_pkg_set - owned_by_ci_process)
if unused_archive_pkgs:
    fail(
        f"stable-process-nextest archive lists package(s) with no ci-process binary: "
        f"{unused_archive_pkgs}"
    )
PY
}

validation_nextest_meta="$validation_scratch_base/validation-nextest-meta.$$.json"
validation_cleanup_paths+=("$validation_nextest_meta")
bash scripts/oxide-rust.sh metadata --no-deps --format-version 1 > "$validation_nextest_meta"
validation_nextest_sync_check \
  "$validation_repo_root/.config/nextest.toml" \
  "$validation_repo_root/.github/workflows/ci.yml" \
  "$validation_nextest_meta"
rm -f "$validation_nextest_meta"

validation_nextest_fixture=$(mktemp -d "${validation_scratch_base}/validation-nextest-sync.XXXXXXXX")
validation_cleanup_paths+=("$validation_nextest_fixture")
cat >"$validation_nextest_fixture/metadata.json" <<'EOF'
{
  "packages": [
    {"name": "pkg-a", "targets": [{"name": "bin_one", "kind": ["test"]}]},
    {"name": "pkg-b", "targets": [{"name": "bin_two", "kind": ["test"]}]}
  ]
}
EOF
cat >"$validation_nextest_fixture/nextest.toml" <<'EOF'
[profile.ci]
default-filter = "not (binary(bin_one) | binary(bin_two))"

[profile.ci-process]
default-filter = "binary(bin_one) | binary(bin_two)"
EOF
printf '%s\n' \
  'jobs:' \
  '  stable-process-nextest:' \
  '    steps:' \
  '      - run: |' \
  '          cargo nextest archive \' \
  '            -p pkg-a \' \
  '            -p pkg-b \' \
  '            --all-features' >"$validation_nextest_fixture/ci.yml"
validation_nextest_sync_check \
  "$validation_nextest_fixture/nextest.toml" \
  "$validation_nextest_fixture/ci.yml" \
  "$validation_nextest_fixture/metadata.json"

cat >"$validation_nextest_fixture/nextest-mismatch.toml" <<'EOF'
[profile.ci]
default-filter = "not (binary(bin_one))"

[profile.ci-process]
default-filter = "binary(bin_one) | binary(bin_two)"
EOF
validation_expect_failure \
  "binary sets differ" \
  validation_nextest_sync_check \
    "$validation_nextest_fixture/nextest-mismatch.toml" \
    "$validation_nextest_fixture/ci.yml" \
    "$validation_nextest_fixture/metadata.json"

cat >"$validation_nextest_fixture/nextest-non-complement.toml" <<'EOF'
[profile.ci]
default-filter = "binary(bin_one) | binary(bin_two)"

[profile.ci-process]
default-filter = "binary(bin_one) | binary(bin_two)"
EOF
validation_expect_failure \
  "not an exact negated binary list" \
  validation_nextest_sync_check \
    "$validation_nextest_fixture/nextest-non-complement.toml" \
    "$validation_nextest_fixture/ci.yml" \
    "$validation_nextest_fixture/metadata.json"

printf '%s\n' \
  'jobs:' \
  '  stable-process-nextest:' \
  '    steps:' \
  '      - run: |' \
  '          cargo nextest archive \' \
  '            -p pkg-a \' \
  '            --all-features' >"$validation_nextest_fixture/ci-missing-pkg.yml"
validation_expect_failure \
  "omitted from stable-process-nextest archive" \
  validation_nextest_sync_check \
    "$validation_nextest_fixture/nextest.toml" \
    "$validation_nextest_fixture/ci-missing-pkg.yml" \
    "$validation_nextest_fixture/metadata.json"
rm -rf "$validation_nextest_fixture"

printf 'validation selector argument and archive-scope tests passed\n'
