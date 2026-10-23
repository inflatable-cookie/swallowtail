#!/usr/bin/env bash
set -euo pipefail

# Hermetic routing proof for `scripts/oxide-rust.sh`.
#
# Every tool the helper can start is a fake recorded in this run's own scratch
# directory, so no real Oxide build, Cargo build, pool or slot is touched. The
# checks cover both routing branches, argv fidelity (spaces, metacharacters,
# empty arguments, `--`, filter expressions, `--profile`, `-D warnings`), exit
# and signal status, stdout/stderr ownership, and the fact that the helper
# replaces itself with the tool instead of adding a wrapper process.

routing_root=$(cd "$(dirname "$0")/../.." && pwd)
routing_helper="$routing_root/scripts/oxide-rust.sh"
routing_scratch=$(mktemp -d "${TMPDIR:-/tmp}/swallowtail-oxide-routing.XXXXXXXX")
cleanup() {
  rm -rf -- "$routing_scratch"
}
trap cleanup EXIT

routing_fake_bin="$routing_scratch/bin"
routing_cargo_only_bin="$routing_scratch/cargo-only-bin"
routing_signal_bin="$routing_scratch/signal-bin"
mkdir -p "$routing_fake_bin" "$routing_cargo_only_bin" "$routing_signal_bin"

routing_failure() {
  printf 'oxide-rust routing failure: %s\n' "$1" >&2
  exit 1
}

# A recording fake: one bracketed line per invocation, the caller's process
# identity, one stdout line, one stderr line, and a settable exit status.
routing_write_fake() {
  local routing_path=$1
  local routing_name=$2
  cat >"$routing_path" <<FAKE
#!/usr/bin/env bash
{
  printf '%s' '$routing_name'
  for routing_arg in "\$@"; do
    printf ' [%s]' "\$routing_arg"
  done
  printf '\n'
} >>"$routing_scratch/$routing_name.log"
printf 'ppid=%s\n' "\$PPID" >>"$routing_scratch/$routing_name.log"
printf '%s stdout\n' '$routing_name'
printf '%s stderr\n' '$routing_name' >&2
exit "\${FAKE_EXIT_CODE:-0}"
FAKE
  chmod +x "$routing_path"
}

routing_write_fake "$routing_fake_bin/oxide" oxide
routing_write_fake "$routing_fake_bin/cargo" cargo
routing_write_fake "$routing_cargo_only_bin/cargo" cargo
cat >"$routing_signal_bin/oxide" <<'FAKE'
#!/usr/bin/env bash
kill -TERM "$$"
sleep 5
FAKE
chmod +x "$routing_signal_bin/oxide"

routing_reset() {
  rm -f "$routing_scratch/oxide.log" "$routing_scratch/cargo.log"
}

routing_log() {
  local routing_name=$1
  if [[ -f "$routing_scratch/$routing_name.log" ]]; then
    cat "$routing_scratch/$routing_name.log"
  fi
}

routing_expect_line() {
  local routing_name=$1
  local routing_expected=$2
  local routing_actual
  routing_actual=$(routing_log "$routing_name" | head -1)
  if [[ "$routing_actual" != "$routing_expected" ]]; then
    routing_failure "$routing_name was called as
  actual:   $routing_actual
  expected: $routing_expected"
  fi
}

routing_expect_absent() {
  local routing_name=$1
  if [[ -e "$routing_scratch/$routing_name.log" ]]; then
    routing_failure "$routing_name ran on a route that must not reach it:
$(routing_log "$routing_name")"
  fi
}

# 1. Local default: Oxide receives the exact Cargo arguments. The invocation
#    is not wrapped in a command substitution, so the recorded parent identity
#    below is this script's own process.
routing_reset
routing_status=0
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" \
  check --locked --all-targets -p swallowtail-core \
  >"$routing_scratch/stdout" 2>"$routing_scratch/stderr" || routing_status=$?
[[ $routing_status -eq 0 ]] || routing_failure "default route exited $routing_status"
routing_output=$(cat "$routing_scratch/stdout")
routing_expect_line oxide \
  'oxide [check] [--locked] [--all-targets] [-p] [swallowtail-core]'
routing_expect_absent cargo
[[ "$routing_output" == "oxide stdout" ]] ||
  routing_failure "helper changed stdout: $routing_output"
[[ "$(cat "$routing_scratch/stderr")" == "oxide stderr" ]] ||
  routing_failure "helper changed stderr: $(cat "$routing_scratch/stderr")"

# 2. The helper replaces itself with the tool, adding no wrapper process.
routing_recorded_ppid=$(routing_log oxide | sed -n 's/^ppid=//p')
[[ "$routing_recorded_ppid" == "$$" ]] ||
  routing_failure "helper is not exec-replaced: tool parent $routing_recorded_ppid is not $$"

# 3. CI routing: GITHUB_ACTIONS=true selects plain Cargo, even with Oxide present.
routing_reset
PATH="$routing_fake_bin:$PATH" GITHUB_ACTIONS=true bash "$routing_helper" \
  check --locked -p swallowtail-core >/dev/null 2>&1
routing_expect_line cargo 'cargo [check] [--locked] [-p] [swallowtail-core]'
routing_expect_absent oxide

# 4. Any other GITHUB_ACTIONS value keeps the Oxide route.
routing_reset
PATH="$routing_fake_bin:$PATH" GITHUB_ACTIONS=false bash "$routing_helper" \
  check -p swallowtail-core >/dev/null 2>&1
routing_expect_line oxide 'oxide [check] [-p] [swallowtail-core]'
routing_expect_absent cargo

# 5. The documented one-run escape is plain Cargo.
routing_reset
PATH="$routing_fake_bin:$PATH" SWALLOWTAIL_RUST_RUNNER=cargo bash "$routing_helper" \
  clippy --workspace --all-targets -- -D warnings >/dev/null 2>&1
routing_expect_line cargo 'cargo [clippy] [--workspace] [--all-targets] [--] [-D] [warnings]'
routing_expect_absent oxide

# 6. An unknown escape value refuses before any tool runs.
routing_reset
routing_status=0
routing_message=$(
  PATH="$routing_fake_bin:$PATH" SWALLOWTAIL_RUST_RUNNER=latest bash "$routing_helper" \
    check 2>&1 >/dev/null
) || routing_status=$?
[[ $routing_status -eq 2 ]] || routing_failure "unknown escape exited $routing_status"
[[ "$routing_message" == *"must be oxide or cargo"* ]] ||
  routing_failure "unknown escape message: $routing_message"
routing_expect_absent oxide
routing_expect_absent cargo

# 7. Missing Oxide fails loudly, names both escapes, and never falls back.
routing_reset
routing_status=0
routing_message=$(
  PATH="$routing_cargo_only_bin:/usr/bin:/bin" bash "$routing_helper" \
    check -p swallowtail-core 2>&1 >/dev/null
) || routing_status=$?
[[ $routing_status -eq 127 ]] || routing_failure "missing Oxide exited $routing_status"
[[ "$routing_message" == *"oxide is not on PATH"* ]] ||
  routing_failure "missing Oxide message: $routing_message"
[[ "$routing_message" == *":cargo"* && "$routing_message" == *"SWALLOWTAIL_RUST_RUNNER=cargo"* ]] ||
  routing_failure "missing Oxide does not name both escapes: $routing_message"
routing_expect_absent cargo

# 8. Exit status is the tool's own.
routing_reset
routing_status=0
PATH="$routing_fake_bin:$PATH" FAKE_EXIT_CODE=42 bash "$routing_helper" \
  check >/dev/null 2>&1 || routing_status=$?
[[ $routing_status -eq 42 ]] || routing_failure "exit status $routing_status is not 42"

# 9. A signal ending the tool survives as a signal status. The nested shell
#    owns and silences the job-status notice bash prints for a signalled child.
routing_status=0
# Bash reports a signalled foreground child on the waiting shell's stderr, not
# the child's; silence this script's own stderr for exactly that one call.
exec 9>&2
exec 2>/dev/null
PATH="$routing_signal_bin:$PATH" bash "$routing_helper" check >/dev/null 2>&1 ||
  routing_status=$?
exec 2>&9
exec 9>&-
[[ $routing_status -eq 143 ]] || routing_failure "signal status $routing_status is not 143"

# 10. No arguments is a usage refusal, not a Cargo or Oxide run.
routing_reset
routing_status=0
routing_message=$(
  PATH="$routing_fake_bin:$PATH" bash "$routing_helper" 2>&1 >/dev/null
) || routing_status=$?
[[ $routing_status -eq 2 ]] || routing_failure "empty invocation exited $routing_status"
[[ "$routing_message" == *"usage:"* ]] ||
  routing_failure "empty invocation message: $routing_message"
routing_expect_absent oxide
routing_expect_absent cargo

# 11. Nextest-shaped argv survives intact: profile, filters with spaces and
#     metacharacters, an empty argument, `--`, and warnings-denied flags.
routing_reset
routing_filter='test(a b|c) & d$e`f"\g'
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" \
  nextest run --locked --profile ci -p swallowtail-protocol-acp \
  -E "$routing_filter" '' -- --test-threads=1 >/dev/null 2>&1
routing_expect_line oxide \
  "oxide [nextest] [run] [--locked] [--profile] [ci] [-p] [swallowtail-protocol-acp] [-E] [$routing_filter] [] [--] [--test-threads=1]"

# 12. A toolchain- or profile-shaped argument that Oxide refuses is never
#     rewritten here. The helper passes it unchanged, and the selector-level
#     routing test keeps those commands on plain Cargo.
routing_reset
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" \
  clippy --locked --all-features -p swallowtail-adapter-pi --test integration \
  -- -D warnings >/dev/null 2>&1
routing_expect_line oxide \
  'oxide [clippy] [--locked] [--all-features] [-p] [swallowtail-adapter-pi] [--test] [integration] [--] [-D] [warnings]'

printf '%s\n' "oxide-rust routing passed"
