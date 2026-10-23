#!/usr/bin/env bash
set -euo pipefail

# Route one maintained Rust command through Oxide.
#
#   scripts/oxide-rust.sh check --locked --all-targets -p swallowtail-core
#   scripts/oxide-rust.sh fmt -p swallowtail-core -- --check
#   scripts/oxide-rust.sh deny check advisories licenses sources
#   scripts/oxide-rust.sh --host-tool <tool> [args...]
#
# The arguments after the mode are passed to Oxide unchanged: Oxide receives
# them as a Cargo argument list, or after `oxide exec --` when a host tool runs
# Cargo itself. The helper replaces itself with the tool, so argv, exit status,
# signals, stdout and stderr are the tool's own.
#
# Escapes, all plain Cargo with no Oxide on the route:
#   effigy <selector>:cargo                 the selector's sibling in effigy.toml
#   SWALLOWTAIL_RUST_RUNNER=cargo effigy    plain Cargo for one selector run
#   GITHUB_ACTIONS=true                     hosted CI has no Oxide installed
#
# A missing Oxide fails loudly and names the escapes; it never falls back to
# Cargo silently. Toolchain pins travel as `RUSTUP_TOOLCHAIN=<name>`, the
# supported mapping for `cargo +<name>`; the plain branch passes it to Cargo
# unchanged. Private-target, one-use-workspace, packaging-clone and CI routes
# do not use this helper; they are the documented exceptions in
# docs/guides/validation-tiers.md.

oxide_host_tool=false
if [[ ${1:-} == "--host-tool" ]]; then
  oxide_host_tool=true
  shift
fi

if (($# == 0)); then
  printf 'usage: %s [--host-tool] <cargo command|host tool> [args...]\n' "$0" >&2
  exit 2
fi

runner=${SWALLOWTAIL_RUST_RUNNER:-oxide}
case $runner in
  oxide | cargo) ;;
  *)
    printf 'oxide-rust: SWALLOWTAIL_RUST_RUNNER must be oxide or cargo, not %q\n' "$runner" >&2
    exit 2
    ;;
esac

# Hosted runners have no Oxide, so CI keeps its plain-Cargo route. Workflow
# files stay unchanged; this branch is the only CI-specific step.
if [[ ${GITHUB_ACTIONS:-} == "true" ]]; then
  runner=cargo
fi

if [[ $runner == "cargo" ]]; then
  if [[ $oxide_host_tool == true ]]; then
    exec "$@"
  fi
  exec cargo "$@"
fi

if ! command -v oxide >/dev/null 2>&1; then
  cat >&2 <<'MESSAGE'
oxide-rust: oxide is not on PATH, and this route does not fall back to Cargo.
Run the matching plain-Cargo selector instead (effigy <selector>:cargo, for
example `effigy test:rust:cargo`), or set SWALLOWTAIL_RUST_RUNNER=cargo for one
run. See docs/guides/validation-tiers.md, Oxide Adoption Boundary.
MESSAGE
  exit 127
fi

if [[ $oxide_host_tool == true ]]; then
  exec oxide exec -- "$@"
fi
exec oxide "$@"
