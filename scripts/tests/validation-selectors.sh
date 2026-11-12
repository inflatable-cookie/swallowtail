#!/usr/bin/env bash
set -euo pipefail

validation_repo_root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$validation_repo_root"

source "$validation_repo_root/scripts/validation/archive.sh"
source "$validation_repo_root/scripts/validation/path.sh"

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

validation_audit_tree=$(mktemp -d)
validation_audit_repo_root="/nonexistent-repo-root-for-archive-audit"
printf '%s\n' \
  "headless-background-task-waiting-notice-occasionally-skipped" \
  >"$validation_audit_tree/hyphenated-english.txt"
if ! validation_extracted_tree_is_safe \
  "$validation_audit_tree" \
  "$validation_audit_repo_root"
then
  printf 'hyphenated English tripped the secret pattern\n' >&2
  rm -rf "$validation_audit_tree"
  exit 1
fi
printf '%s\n' "sk-abcdefghijklmnopqrstuvwxyz123456" \
  >"$validation_audit_tree/synthetic-key.txt"
if validation_extracted_tree_is_safe \
  "$validation_audit_tree" \
  "$validation_audit_repo_root"
then
  printf 'synthetic sk- key was not flagged\n' >&2
  rm -rf "$validation_audit_tree"
  exit 1
fi
rm -rf "$validation_audit_tree"

validation_real_root=$(mktemp -d)
validation_alias_root="${validation_real_root}-alias"
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

validation_shim_root=$(mktemp -d)
validation_mktemp_log="$validation_shim_root/mktemp.log"
cat >"$validation_shim_root/mktemp" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"${VALIDATION_MKTEMP_LOG:?}"
exec /usr/bin/mktemp "$@"
EOF
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
validation_driver_fixture=$(mktemp -d "$TMPDIR/affected-driver-fixture.XXXXXXXX")
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
cat >"$validation_driver_bin/mktemp" <<'EOF'
#!/usr/bin/env bash
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
  exec /usr/bin/mktemp "$@" "${VALIDATION_HOST_TEMP:?}/bsd-default.XXXXXXXX"
fi
printf '%s\n' "$*" >>"${VALIDATION_DRIVER_LOG:?}/mktemp.log"
exec /usr/bin/mktemp "$@"
EOF
cat >"$validation_driver_bin/tar" <<'EOF'
#!/usr/bin/env bash
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
exec /usr/bin/tar "$@"
EOF
chmod +x "$validation_driver_bin/oxide" "$validation_driver_bin/cargo" \
  "$validation_driver_bin/mktemp" "$validation_driver_bin/tar"

# An invalid TMPDIR must fail closed even when TMP and TEMP point at usable
# locations. The fake BSD mktemp would keep any accidental fallback confined
# to validation_host_temp, where this check also proves none was allocated.
validation_invalid_log="$validation_driver_fixture/log-invalid-root"
mkdir -p "$validation_invalid_log"
validation_invalid_root="$validation_driver_fixture/missing scratch root"
if env VALIDATION_DRIVER_FAIL_CHECK=0 \
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
  if env "${validation_driver_failure_env[@]}" \
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

printf 'validation selector argument and archive-scope tests passed\n'
