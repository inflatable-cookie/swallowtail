#!/usr/bin/env bash
set -euo pipefail

# Hermetic routing proof for `scripts/oxide-rust.sh`.
#
# Every tool the helper can start is a fake recorded in this run's own scratch
# directory, so no real Oxide build, Cargo build, pool or slot is touched. The
# checks cover both routing branches, Effigy's one-run escape, argv fidelity
# (spaces, metacharacters, empty arguments, `--`, filter expressions,
# `--profile`, `-D warnings`), exit and signal status, stdout/stderr ownership,
# and the fact that the helper replaces itself with the tool instead of adding a
# wrapper process. Two self-routing Cargo-running tools are covered too: the
# Grok MCP probe and the private-target courier driver, each of which must take
# exactly one `oxide exec --` lease and keep its own scratch custody. The leased
# fake models the installed `oxide exec` storage step: it overwrites an
# inherited CARGO_TARGET_DIR with `<workspace>/target` and sets its own build
# directory before starting the tool, so a driver that assigned its private
# target before the lease is caught rather than accidentally passing.

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
routing_exec_bin="$routing_scratch/exec-bin"
mkdir -p "$routing_fake_bin" "$routing_cargo_only_bin" "$routing_signal_bin" \
  "$routing_exec_bin"

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
printf 'env:RUSTUP_TOOLCHAIN=%s\n' "\${RUSTUP_TOOLCHAIN:-unset}" >>"$routing_scratch/$routing_name.log"
printf '%s stdout\n' '$routing_name'
printf '%s stderr\n' '$routing_name' >&2
exit "\${FAKE_EXIT_CODE:-0}"
FAKE
  chmod +x "$routing_path"
}

routing_write_fake "$routing_fake_bin/oxide" oxide
routing_write_fake "$routing_fake_bin/cargo" cargo
routing_write_fake "$routing_cargo_only_bin/cargo" cargo
cat >"$routing_scratch/host-tool" <<FAKE
#!/usr/bin/env bash
{
  printf 'host-tool'
  for routing_arg in "\$@"; do
    printf ' [%s]' "\$routing_arg"
  done
  printf '\n'
} >>"$routing_scratch/tool.log"
exit "\${FAKE_EXIT_CODE:-0}"
FAKE
chmod +x "$routing_scratch/host-tool"
cat >"$routing_signal_bin/oxide" <<'FAKE'
#!/usr/bin/env bash
kill -TERM "$$"
sleep 5
FAKE
chmod +x "$routing_signal_bin/oxide"

# The values the leased fake injects, and the caller's sentinels that must die.
# `<workspace>/target` is the destination the installed `oxide exec` applies; the
# slot build directory stands in for the held slot's build directory.
routing_exec_workspace_target="$routing_root/target"
routing_exec_slot_build="$routing_scratch/slot-build"
routing_sentinel_target="$routing_scratch/SENTINEL-TARGET-SHOULD-NOT-SURVIVE"
routing_sentinel_build="$routing_scratch/SENTINEL-BUILD-SHOULD-NOT-SURVIVE"

# A leased fake: it records itself like the plain fake, records the caller's
# inherited storage variables, then behaves like `oxide exec`: it overwrites
# them (`CARGO_TARGET_DIR` becomes `<workspace>/target`, the build directory
# becomes the held slot's) and only then hands the recorded argv to the tool, so
# a self-routing driver's own behaviour runs inside this proof. A driver that
# assigned its private target before the lease therefore cannot pass.
cat >"$routing_exec_bin/oxide" <<FAKE
#!/usr/bin/env bash
{
  printf 'oxide'
  for routing_arg in "\$@"; do
    printf ' [%s]' "\$routing_arg"
  done
  printf '\n'
  printf 'inherited:CARGO_TARGET_DIR=%s\n' "\${CARGO_TARGET_DIR:-unset}"
  printf 'inherited:CARGO_BUILD_BUILD_DIR=%s\n' "\${CARGO_BUILD_BUILD_DIR:-unset}"
} >>"$routing_scratch/oxide.log"
if [[ \${1:-} != exec || \${2:-} != -- ]]; then
  printf 'fake oxide: unexpected argv\n' >&2
  exit 90
fi
shift 2
export CARGO_TARGET_DIR="$routing_exec_workspace_target"
export CARGO_BUILD_BUILD_DIR="$routing_exec_slot_build"
exec "\$@"
FAKE
chmod +x "$routing_exec_bin/oxide"

# A tool that only reports the storage environment it was started with.
cat >"$routing_scratch/env-tool" <<FAKE
#!/usr/bin/env bash
{
  printf 'env-tool'
  for routing_arg in "\$@"; do
    printf ' [%s]' "\$routing_arg"
  done
  printf '\n'
  printf 'CARGO_TARGET_DIR=%s\n' "\${CARGO_TARGET_DIR:-unset}"
  printf 'CARGO_BUILD_BUILD_DIR=%s\n' "\${CARGO_BUILD_BUILD_DIR:-unset}"
} >>"$routing_scratch/tool.log"
exit 0
FAKE
chmod +x "$routing_scratch/env-tool"
cat >"$routing_exec_bin/cargo" <<FAKE
#!/usr/bin/env bash
{
  printf 'cargo'
  for routing_arg in "\$@"; do
    printf ' [%s]' "\$routing_arg"
  done
  printf '\n'
} >>"$routing_scratch/cargo.log"
printf 'env:CARGO_TARGET_DIR=%s\n' "\${CARGO_TARGET_DIR:-unset}" >>"$routing_scratch/cargo.log"
printf 'env:CARGO_BUILD_BUILD_DIR=%s\n' "\${CARGO_BUILD_BUILD_DIR:-unset}" >>"$routing_scratch/cargo.log"
if [[ -n \${CARGO_TARGET_DIR:-} && -d \$CARGO_TARGET_DIR ]]; then
  if [[ -f \$CARGO_TARGET_DIR/.swallowtail-run-owned ]]; then
    printf 'marker=present\n' >>"$routing_scratch/cargo.log"
  else
    printf 'marker=absent\n' >>"$routing_scratch/cargo.log"
  fi
fi
FAKE
chmod +x "$routing_exec_bin/cargo"

routing_reset() {
  rm -f "$routing_scratch/oxide.log" "$routing_scratch/cargo.log" \
    "$routing_scratch/tool.log"
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

# 5. Effigy's documented one-run escape is plain Cargo.
routing_reset
PATH="$routing_fake_bin:$PATH" EFFIGY_OXIDE_ESCAPE=cargo bash "$routing_helper" \
  clippy --workspace --all-targets -- -D warnings >/dev/null 2>&1
routing_expect_line cargo 'cargo [clippy] [--workspace] [--all-targets] [--] [-D] [warnings]'
routing_expect_absent oxide

# 6. Naming Oxide explicitly keeps the routed branch.
routing_reset
PATH="$routing_fake_bin:$PATH" EFFIGY_OXIDE_ESCAPE=oxide bash "$routing_helper" \
  check -p swallowtail-core >/dev/null 2>&1
routing_expect_line oxide 'oxide [check] [-p] [swallowtail-core]'
routing_expect_absent cargo

# 7. An unknown escape value refuses before any tool runs.
routing_reset
routing_status=0
routing_message=$(
  PATH="$routing_fake_bin:$PATH" EFFIGY_OXIDE_ESCAPE=latest bash "$routing_helper" \
    check 2>&1 >/dev/null
) || routing_status=$?
[[ $routing_status -eq 2 ]] || routing_failure "unknown escape exited $routing_status"
[[ "$routing_message" == *"EFFIGY_OXIDE_ESCAPE must be oxide or cargo"* ]] ||
  routing_failure "unknown escape message: $routing_message"
routing_expect_absent oxide
routing_expect_absent cargo

# 8. Missing Oxide fails loudly, names the escape, and never falls back.
routing_reset
routing_status=0
routing_message=$(
  PATH="$routing_cargo_only_bin:/usr/bin:/bin" bash "$routing_helper" \
    check -p swallowtail-core 2>&1 >/dev/null
) || routing_status=$?
[[ $routing_status -eq 127 ]] || routing_failure "missing Oxide exited $routing_status"
[[ "$routing_message" == *"oxide is not on PATH"* ]] ||
  routing_failure "missing Oxide message: $routing_message"
[[ "$routing_message" == *"EFFIGY_OXIDE_ESCAPE=cargo"* ]] ||
  routing_failure "missing Oxide does not name the escape: $routing_message"
routing_expect_absent cargo

# 9. Exit status is the tool's own.
routing_reset
routing_status=0
PATH="$routing_fake_bin:$PATH" FAKE_EXIT_CODE=42 bash "$routing_helper" \
  check >/dev/null 2>&1 || routing_status=$?
[[ $routing_status -eq 42 ]] || routing_failure "exit status $routing_status is not 42"

# 10. A signal ending the tool survives as a signal status. The nested shell
#     owns and silences the job-status notice bash prints for a signalled child.
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

# 11. No arguments is a usage refusal, not a Cargo or Oxide run.
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

# 12. Nextest-shaped argv survives intact: profile, filters with spaces and
#     metacharacters, an empty argument, `--`, and warnings-denied flags.
routing_reset
routing_filter='test(a b|c) & d$e`f"\g'
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" \
  nextest run --locked --profile ci -p swallowtail-protocol-acp \
  -E "$routing_filter" '' -- --test-threads=1 >/dev/null 2>&1
routing_expect_line oxide \
  "oxide [nextest] [run] [--locked] [--profile] [ci] [-p] [swallowtail-protocol-acp] [-E] [$routing_filter] [] [--] [--test-threads=1]"

# 13. A toolchain- or profile-shaped argument that Oxide refuses is never
#     rewritten here. The helper passes it unchanged.
routing_reset
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" \
  clippy --locked --all-features -p swallowtail-adapter-pi --test integration \
  -- -D warnings >/dev/null 2>&1
routing_expect_line oxide \
  'oxide [clippy] [--locked] [--all-features] [-p] [swallowtail-adapter-pi] [--test] [integration] [--] [-D] [warnings]'

# 14. Non-building and formatting routes keep their arguments too.
routing_reset
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" \
  fmt -p swallowtail-core -- --check >/dev/null 2>&1
routing_expect_line oxide 'oxide [fmt] [-p] [swallowtail-core] [--] [--check]'
routing_reset
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" \
  deny check advisories licenses sources >/dev/null 2>&1
routing_expect_line oxide 'oxide [deny] [check] [advisories] [licenses] [sources]'
routing_reset
PATH="$routing_fake_bin:$PATH" RUSTDOCFLAGS='-D missing_docs' bash "$routing_helper" \
  doc --workspace --all-features --no-deps --locked >/dev/null 2>&1
routing_expect_line oxide \
  'oxide [doc] [--workspace] [--all-features] [--no-deps] [--locked]'

# 15. The toolchain pin travels as RUSTUP_TOOLCHAIN, never as `+name`.
routing_reset
PATH="$routing_fake_bin:$PATH" RUSTUP_TOOLCHAIN=1.95.0 bash "$routing_helper" \
  clippy --locked -p swallowtail-adapter-pi -- -D warnings >/dev/null 2>&1
routing_expect_line oxide \
  'oxide [clippy] [--locked] [-p] [swallowtail-adapter-pi] [--] [-D] [warnings]'
routing_toolchain=$(sed -n 's/^env:RUSTUP_TOOLCHAIN=//p' "$routing_scratch/oxide.log")
[[ "$routing_toolchain" == "1.95.0" ]] ||
  routing_failure "toolchain pin did not reach Oxide: $routing_toolchain"
if grep -q '\[+1.95.0\]' "$routing_scratch/oxide.log"; then
  routing_failure "toolchain pin was rewritten into a leading +toolchain"
fi

# 16. Host-tool mode takes one lease: `oxide exec -- <tool> <argv>`.
routing_reset
PATH="$routing_fake_bin:$PATH" bash "$routing_helper" --host-tool \
  "$routing_scratch/host-tool" 'a b' '' --flag >/dev/null 2>&1
routing_expect_line oxide \
  "oxide [exec] [--] [$routing_scratch/host-tool] [a b] [] [--flag]"
routing_expect_absent tool

# 17. The host tool's own exit status survives its lease.
routing_reset
routing_status=0
PATH="$routing_fake_bin:$PATH" FAKE_EXIT_CODE=9 bash "$routing_helper" --host-tool \
  "$routing_scratch/host-tool" run >/dev/null 2>&1 || routing_status=$?
[[ $routing_status -eq 9 ]] || routing_failure "host-tool status $routing_status is not 9"

# 18. Host-tool CI and escape branches run the tool directly, never Oxide.
routing_reset
PATH="$routing_fake_bin:$PATH" GITHUB_ACTIONS=true bash "$routing_helper" --host-tool \
  "$routing_scratch/host-tool" check >/dev/null 2>&1
routing_expect_line tool 'host-tool [check]'
routing_expect_absent oxide
routing_reset
PATH="$routing_fake_bin:$PATH" EFFIGY_OXIDE_ESCAPE=cargo bash "$routing_helper" \
  --host-tool "$routing_scratch/host-tool" check >/dev/null 2>&1
routing_expect_line tool 'host-tool [check]'
routing_expect_absent oxide

# 19. A missing Oxide refuses a host-tool route before the tool runs.
routing_reset
routing_status=0
routing_message=$(
  PATH="$routing_cargo_only_bin:/usr/bin:/bin" bash "$routing_helper" --host-tool \
    "$routing_scratch/host-tool" check 2>&1 >/dev/null
) || routing_status=$?
[[ $routing_status -eq 127 ]] || routing_failure "host-tool missing Oxide exited $routing_status"
[[ "$routing_message" == *"EFFIGY_OXIDE_ESCAPE=cargo"* ]] ||
  routing_failure "host-tool missing Oxide message: $routing_message"
routing_expect_absent tool

# 20. The Grok MCP probe (a script that runs Cargo itself) takes one host-tool
#     lease and keeps its exact argv; the fake Oxide ends the route there.
routing_reset
PATH="$routing_fake_bin:$PATH" SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1 \
  bash "$routing_root/scripts/run-grok-acp-client-mcp-probe.sh" \
  --version 1.0.5 --grok-executable /bin/false \
  --output "$routing_scratch/capsule.json" >/dev/null 2>&1
routing_expect_line oxide \
  "oxide [exec] [--] [bash] [$routing_root/scripts/run-grok-acp-client-mcp-probe.sh] [--version] [1.0.5] [--grok-executable] [/bin/false] [--output] [$routing_scratch/capsule.json]"
routing_expect_absent cargo

# 21. Under GITHUB_ACTIONS the same probe runs its Cargo calls directly.
routing_reset
PATH="$routing_fake_bin:$PATH" GITHUB_ACTIONS=true \
  SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1 \
  bash "$routing_root/scripts/run-grok-acp-client-mcp-probe.sh" \
  --version 1.0.5 --grok-executable /bin/false \
  --output "$routing_scratch/capsule.json" >/dev/null 2>&1 || true
routing_expect_absent oxide
routing_expect_line cargo \
  'cargo [build] [--offline] [--locked] [-p] [swallowtail-testkit] [--example] [grok-acp-echo-mcp]'

# 22. The leased fake really overwrites an inherited target, as `oxide exec`
#     does: a tool started through the helper's host-tool route sees
#     `<workspace>/target` and the injected build directory, never the caller's
#     sentinels. This is the mutation-sensitive half of test 23: moving a
#     driver's assignment before its lease cannot pass there.
routing_reset
PATH="$routing_exec_bin:$PATH" \
  CARGO_TARGET_DIR="$routing_sentinel_target" \
  CARGO_BUILD_BUILD_DIR="$routing_sentinel_build" \
  bash "$routing_helper" --host-tool "$routing_scratch/env-tool" check \
  >/dev/null 2>&1 || routing_failure "leased env-tool route failed"
grep -qx "CARGO_TARGET_DIR=$routing_exec_workspace_target" \
  "$routing_scratch/tool.log" ||
  routing_failure "the leased fake did not overwrite the inherited target:
$(cat "$routing_scratch/tool.log")"
grep -qx "CARGO_BUILD_BUILD_DIR=$routing_exec_slot_build" \
  "$routing_scratch/tool.log" ||
  routing_failure "the leased fake did not overwrite the inherited build directory:
$(cat "$routing_scratch/tool.log")"
if grep -q 'SENTINEL' "$routing_scratch/tool.log"; then
  routing_failure "the caller's storage sentinels survived the lease:
$(cat "$routing_scratch/tool.log")"
fi

# 23. The private-target courier driver takes exactly one lease for the whole
#     run: the fake Oxide leases it and hands its argv to the driver, whose own
#     `cargo` calls run inside a freshly created, marked run-owned target that
#     is gone afterwards. The caller's inherited sentinels are overwritten by the
#     lease, so only the driver's own in-route assignment can produce the
#     observed target and build directory.
routing_reset
routing_courier_log="$routing_scratch/courier.log"
routing_courier_args=(cargo check -p swallowtail-core --then cargo metadata --no-deps)
PATH="$routing_exec_bin:$PATH" \
  CARGO_TARGET_DIR="$routing_sentinel_target" \
  CARGO_BUILD_BUILD_DIR="$routing_sentinel_build" \
  bash "$routing_root/scripts/run-with-owned-cargo-target.sh" \
  "${routing_courier_args[@]}" \
  >"$routing_courier_log" 2>&1 || routing_failure "courier driver failed: $(cat "$routing_courier_log")"
sed -n 's/^inherited:CARGO_TARGET_DIR=//p' "$routing_scratch/oxide.log" |
  grep -qx "$routing_sentinel_target" ||
  routing_failure "the leased fake did not see the caller's target sentinel:
$(cat "$routing_scratch/oxide.log")"
sed -n 's/^inherited:CARGO_BUILD_BUILD_DIR=//p' "$routing_scratch/oxide.log" |
  grep -qx "$routing_sentinel_build" ||
  routing_failure "the leased fake did not see the caller's build sentinel:
$(cat "$routing_scratch/oxide.log")"
if grep -Eq 'SENTINEL' "$routing_scratch/cargo.log"; then
  routing_failure "the driver's cargo calls inherited a caller sentinel:
$(cat "$routing_scratch/cargo.log")"
fi
if grep -qx "env:CARGO_TARGET_DIR=$routing_exec_workspace_target" \
  "$routing_scratch/cargo.log"; then
  routing_failure "the driver's cargo calls used the lease's checkout target:
$(cat "$routing_scratch/cargo.log")"
fi
if grep -qx "env:CARGO_BUILD_BUILD_DIR=$routing_exec_slot_build" \
  "$routing_scratch/cargo.log"; then
  routing_failure "the driver's cargo calls used the lease's slot build directory:
$(cat "$routing_scratch/cargo.log")"
fi
routing_expect_line oxide \
  "oxide [exec] [--] [bash] [$routing_root/scripts/run-with-owned-cargo-target.sh] [cargo] [check] [-p] [swallowtail-core] [--then] [cargo] [metadata] [--no-deps]"
[[ $(grep -c '^oxide ' "$routing_scratch/oxide.log") -eq 1 ]] ||
  routing_failure "courier driver took more than one lease:
$(cat "$routing_scratch/oxide.log")"
routing_courier_targets=$(sed -n 's/^env:CARGO_TARGET_DIR=//p' "$routing_scratch/cargo.log")
[[ $(printf '%s\n' "$routing_courier_targets" | wc -l | tr -d ' ') -eq 2 ]] ||
  routing_failure "courier driver ran an unexpected number of cargo groups:
$(cat "$routing_scratch/cargo.log")"
routing_first_target=$(printf '%s\n' "$routing_courier_targets" | head -1)
routing_temp_root=$(cd "${TMPDIR:-/tmp}" && pwd)
routing_normalised_target=$(printf '%s' "$routing_first_target" | sed 's|//*|/|g')
case "$routing_normalised_target" in
  "$routing_temp_root"/swallowtail-sdk-courier.*) ;;
  *)
    routing_failure \
      "run-owned target is not under the system temp root: $routing_first_target"
    ;;
esac
[[ "$(printf '%s\n' "$routing_courier_targets" | sort -u | wc -l | tr -d ' ')" -eq 1 ]] ||
  routing_failure "the run reused more than one target directory: $routing_courier_targets"
grep -qx 'marker=present' "$routing_scratch/cargo.log" ||
  routing_failure "run-owned target carried no ownership marker:
$(cat "$routing_scratch/cargo.log")"
routing_courier_builds=$(sed -n 's/^env:CARGO_BUILD_BUILD_DIR=//p' "$routing_scratch/cargo.log")
while IFS= read -r routing_build_dir; do
  case "$routing_build_dir" in
    "$routing_first_target"/*) ;;
    *)
      routing_failure \
        "the courier driver built outside its run-owned target: $routing_build_dir"
      ;;
  esac
done <<<"$routing_courier_builds"
if [[ -e $routing_first_target ]]; then
  routing_failure "courier driver left its run-owned target behind: $routing_first_target"
fi
case "$routing_first_target" in
  "$routing_root"/*) routing_failure "run-owned target is inside the worktree: $routing_first_target" ;;
esac
[[ "$(grep -c '^cargo ' "$routing_scratch/cargo.log")" -eq 2 ]] ||
  routing_failure "the driver ran an unexpected number of cargo groups:
$(cat "$routing_scratch/cargo.log")"
grep -qx 'cargo \[check\] \[-p\] \[swallowtail-core\]' "$routing_scratch/cargo.log" ||
  routing_failure "the driver's first cargo group changed:
$(cat "$routing_scratch/cargo.log")"
grep -qx 'cargo \[metadata\] \[--no-deps\]' "$routing_scratch/cargo.log" ||
  routing_failure "the driver's --then group changed:
$(cat "$routing_scratch/cargo.log")"

# 24. Under GITHUB_ACTIONS the same driver runs its groups directly, with no
#     Oxide anywhere on the route, and its in-route assignment still replaces an
#     inherited target.
routing_reset
routing_courier_ci_args=(cargo check -p swallowtail-core)
PATH="$routing_exec_bin:$PATH" GITHUB_ACTIONS=true \
  CARGO_TARGET_DIR="$routing_sentinel_target" \
  bash "$routing_root/scripts/run-with-owned-cargo-target.sh" \
  "${routing_courier_ci_args[@]}" >/dev/null 2>&1 ||
  routing_failure "courier driver CI route failed"
routing_expect_absent oxide
routing_expect_line cargo 'cargo [check] [-p] [swallowtail-core]'
routing_ci_target=$(sed -n 's/^env:CARGO_TARGET_DIR=//p' "$routing_scratch/cargo.log")
routing_ci_normalised=$(printf '%s' "$routing_ci_target" | sed 's|//*|/|g')
case "$routing_ci_normalised" in
  "$routing_temp_root"/swallowtail-sdk-courier.*) ;;
  *)
    routing_failure \
      "the CI route did not use its own run-owned target: $routing_ci_target"
    ;;
esac
if grep -q 'SENTINEL' "$routing_scratch/cargo.log"; then
  routing_failure "the CI route inherited the caller's target sentinel:
$(cat "$routing_scratch/cargo.log")"
fi

printf '%s\n' "oxide-rust routing passed"
