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

printf 'validation selector argument and archive-scope tests passed\n'
