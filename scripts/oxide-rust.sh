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
# them as a Cargo argument list, or after `oxide exec --` when a tool runs
# Cargo itself. The helper replaces itself with the tool, so argv, exit status,
# signals, stdout and stderr are the tool's own.
#
# This is the only place that spells a plain Cargo command. Escapes, both plain
# Cargo with no Oxide on the route:
#   EFFIGY_OXIDE_ESCAPE=cargo   Effigy's escape: one run on plain Cargo
#   GITHUB_ACTIONS=true         hosted CI has no Oxide installed
# There is no project escape variable and no per-selector plain-Cargo sibling.
#
# A missing Oxide fails loudly and names the escape; it never falls back to
# Cargo silently. Toolchain pins travel as `RUSTUP_TOOLCHAIN=<name>`, the
# supported mapping for `cargo +<name>`; the plain branch passes it to Cargo
# unchanged. Cargo-running tools and private-target or temporary-workspace
# drivers enter once through `--host-tool`, which is one `oxide exec --` lease;
# docs/guides/validation-tiers.md owns their custody rules.

oxide_host_tool=false
if [[ ${1:-} == "--host-tool" ]]; then
  oxide_host_tool=true
  shift
fi

if (($# == 0)); then
  printf 'usage: %s [--host-tool] <cargo command|host tool> [args...]\n' "$0" >&2
  exit 2
fi

tool=${EFFIGY_OXIDE_ESCAPE:-oxide}
case $tool in
  oxide | cargo) ;;
  *)
    printf 'oxide-rust: EFFIGY_OXIDE_ESCAPE must be oxide or cargo, not %q\n' "$tool" >&2
    exit 2
    ;;
esac

# Hosted runners have no Oxide, so CI keeps its plain-Cargo route. Workflow
# files stay unchanged; this branch is the only CI-specific step.
if [[ ${GITHUB_ACTIONS:-} == "true" ]]; then
  tool=cargo
fi

if [[ $tool == "cargo" ]]; then
  if [[ $oxide_host_tool == true ]]; then
    exec "$@"
  fi
  exec cargo "$@"
fi

if ! command -v oxide >/dev/null 2>&1; then
  cat >&2 <<'MESSAGE'
oxide-rust: oxide is not on PATH, and this route does not fall back to Cargo.
Install Oxide, or set EFFIGY_OXIDE_ESCAPE=cargo for one plain-Cargo run. See
docs/guides/validation-tiers.md, Oxide Adoption Boundary.
MESSAGE
  exit 127
fi

if [[ $oxide_host_tool == true ]]; then
  exec oxide exec -- "$@"
fi
exec oxide "$@"
