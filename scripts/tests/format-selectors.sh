#!/usr/bin/env bash
set -euo pipefail

format_repo_root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$format_repo_root"

format_real_bash=$(command -v bash)
format_real_cargo=$(command -v cargo)
format_work=$(mktemp -d)
trap 'rm -rf "$format_work"' EXIT

format_shim="$format_work/shim"
format_scratch="$format_work/scratch"
mkdir -p "$format_shim" "$format_scratch"

format_expect_failure() {
  local format_expected=$1
  shift
  local format_output
  local format_status=0
  format_output=$("$@" 2>&1) || format_status=$?
  if (( format_status == 0 )); then
    printf 'expected format selector failure: %s\n' "$*" >&2
    exit 1
  fi
  if [[ "$format_output" != *"$format_expected"* ]]; then
    printf 'format selector failure changed: %s\n' "$format_output" >&2
    exit 1
  fi
}

# Plan mode runs against the real workspace metadata and never formats.
format_plan=$(
  bash scripts/format-packages.sh --plan swallowtail-core swallowtail-runtime
)
[[ "$format_plan" == *"selector=format:package"* ]]
[[ "$format_plan" == *"package=swallowtail-core"* ]]
[[ "$format_plan" == *"package=swallowtail-runtime"* ]]
[[ "$format_plan" == *"proof=cargo-fmt-check"* ]]

format_expect_failure \
  "requires one to four package names" \
  bash scripts/format-packages.sh
format_expect_failure \
  "requires one to four package names" \
  bash scripts/format-packages.sh \
    swallowtail-core swallowtail-runtime swallowtail-testkit \
    swallowtail-host-local swallowtail-adapter-pi
format_expect_failure \
  "is not an exact workspace package" \
  bash scripts/format-packages.sh swallowtail-adapter-not-real
format_expect_failure \
  "is not an exact workspace package" \
  bash scripts/format-packages.sh ""
format_expect_failure \
  "is not an exact workspace package" \
  bash scripts/format-packages.sh "swallowtail-core;touch $format_work/pwned"
format_expect_failure \
  "is not an exact workspace package" \
  bash scripts/format-packages.sh 'swallowtail-core $(touch pwned)'
format_expect_failure \
  "is not an exact workspace package" \
  bash scripts/format-packages.sh "swallowtail-core*"
format_expect_failure \
  "contains a duplicate" \
  bash scripts/format-packages.sh swallowtail-core swallowtail-core
format_expect_failure \
  "unsupported option" \
  bash scripts/format-packages.sh --all
format_expect_failure \
  "unsupported option" \
  bash scripts/format-packages.sh --manifest-path Cargo.toml

# This test's subject is the exact `cargo fmt` argument list, so it forces the
# plain-Cargo branch of the routed formatter; the routed branch is proved by
# scripts/tests/oxide-rust-routing.sh.
export SWALLOWTAIL_RUST_RUNNER=cargo

# A fake cargo records every fmt invocation with explicit argument brackets.
# Metadata still comes from the real cargo so package names are real.
cat >"$format_shim/cargo" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == metadata ]]; then
  exec "$format_real_cargo" "\$@"
fi
printf '[%s]' "\$@" >>"$format_work/cargo.log"
printf '\n' >>"$format_work/cargo.log"
EOF
chmod +x "$format_shim/cargo"

PATH="$format_shim:$PATH" bash scripts/format-packages.sh \
  swallowtail-core swallowtail-runtime
format_expected_cargo=$(cat <<'EOF'
[fmt][--package][swallowtail-core][--package][swallowtail-runtime]
[fmt][--package][swallowtail-core][--package][swallowtail-runtime][--][--check]
EOF
)
if [[ "$(cat "$format_work/cargo.log")" != "$format_expected_cargo" ]]; then
  printf 'format:package cargo calls changed:\n%s\n' \
    "$(cat "$format_work/cargo.log")" >&2
  exit 1
fi

rm -f "$format_work/cargo.log"
format_expect_failure \
  "is not an exact workspace package" \
  env PATH="$format_shim:$PATH" bash scripts/format-packages.sh \
    "swallowtail-core;touch $format_work/pwned"
if [[ -e "$format_work/cargo.log" || -e "$format_work/pwned" ]]; then
  printf 'rejected package selection still reached cargo or the shell\n' >&2
  exit 1
fi

# Fake ruff detects a scratch fixture by marker; the proof runner must never run.
cat >"$format_shim/ruff" <<EOF
#!/usr/bin/env bash
if [[ "\$1" == --version ]]; then
  printf 'ruff 0.14.1\n'
  exit 0
fi
printf '[%s]' "\$@" >>"$format_work/ruff.log"
printf '\n' >>"$format_work/ruff.log"
for format_arg in "\$@"; do
  if [[ -f "\$format_arg" ]] && grep -q UNFORMATTED-FIXTURE "\$format_arg"; then
    exit 1
  fi
done
EOF
chmod +x "$format_shim/ruff"
cat >"$format_shim/python3" <<EOF
#!/usr/bin/env bash
printf 'python3 invoked\n' >>"$format_work/python.log"
exit 99
EOF
chmod +x "$format_shim/python3"

printf 'UNFORMATTED-FIXTURE\nx = {  "a":1 }\n' >"$format_scratch/unformatted.py"
printf 'x = {"a": 1}\n' >"$format_scratch/formatted.py"

format_unformatted_status=0
PATH="$format_shim:$PATH" bash scripts/check-python-format.sh \
  "$format_scratch/unformatted.py" > /dev/null 2>&1 || format_unformatted_status=$?
if (( format_unformatted_status != 1 )); then
  printf 'unformatted scratch fixture was not detected (status %s)\n' \
    "$format_unformatted_status" >&2
  exit 1
fi
PATH="$format_shim:$PATH" bash scripts/check-python-format.sh \
  "$format_scratch/formatted.py"
format_expected_ruff=$(cat <<EOF
[format][--check][--no-cache][--][$format_scratch/unformatted.py]
[format][--check][--no-cache][--][$format_scratch/formatted.py]
EOF
)
if [[ "$(cat "$format_work/ruff.log")" != "$format_expected_ruff" ]]; then
  printf 'python format check ruff calls changed:\n%s\n' \
    "$(cat "$format_work/ruff.log")" >&2
  exit 1
fi
if [[ -e "$format_work/python.log" ]]; then
  printf 'python format check executed python3 or the proof runner\n' >&2
  exit 1
fi

# With no ruff on PATH the check fails with the install message, not a pass.
if PATH=/usr/bin:/bin command -v ruff > /dev/null 2>&1; then
  printf 'ruff is installed in /usr/bin or /bin; missing-tool case cannot run\n' >&2
  exit 1
fi
format_expect_failure \
  "needs ruff 0.14.1 on PATH; found: none" \
  env PATH=/usr/bin:/bin "$format_real_bash" scripts/check-python-format.sh \
    "$format_scratch/formatted.py"

# A ruff of the wrong version is refused, so the verdict stays pinned.
mkdir -p "$format_work/wrong-ruff"
cat >"$format_work/wrong-ruff/ruff" <<'EOF'
#!/usr/bin/env bash
printf 'ruff 0.9.0\n'
EOF
chmod +x "$format_work/wrong-ruff/ruff"
format_expect_failure \
  "found: ruff 0.9.0" \
  env PATH="$format_work/wrong-ruff:/usr/bin:/bin" "$format_real_bash" \
    scripts/check-python-format.sh "$format_scratch/formatted.py"
format_expect_failure \
  "unsupported option" \
  env PATH="$format_shim:$PATH" bash scripts/check-python-format.sh --fix \
    "$format_scratch/formatted.py"
format_expect_failure \
  "not a regular file" \
  env PATH="$format_shim:$PATH" bash scripts/check-python-format.sh \
    "$format_scratch/missing.py"
format_expect_failure \
  "requires one or more file paths" \
  env PATH="$format_shim:$PATH" bash scripts/check-python-format.sh

printf 'format selector argument and python-check tests passed\n'
