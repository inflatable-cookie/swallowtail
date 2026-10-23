#!/usr/bin/env bash
set -euo pipefail

# Route one ordinary Rust build, check, clippy, test or nextest command through
# Oxide, per the supported switching pattern in Oxide's workflow contract. The
# caller passes Cargo's arguments unchanged; Oxide receives exactly those
# arguments after its own command word.
#
#   scripts/oxide-rust.sh check --locked --all-targets -p swallowtail-core
#   scripts/oxide-rust.sh nextest run --locked --profile ci -p swallowtail-core
#
# Escapes, all plain Cargo with no Oxide on the route:
#   effigy <selector>:cargo                 the selector's sibling in effigy.toml
#   SWALLOWTAIL_RUST_RUNNER=cargo effigy    plain Cargo for one selector run
#   GITHUB_ACTIONS=true                     hosted CI has no Oxide installed
#
# A missing Oxide fails loudly and names the escapes; it never falls back to
# Cargo silently. `cargo fmt`, probes, packaging, pinned-toolchain, private
# target and nested-Cargo routes do not use this helper at all; they are the
# documented Class2 exceptions in docs/guides/validation-tiers.md.

if (($# == 0)); then
  printf 'usage: %s <cargo command> [args...]\n' "$0" >&2
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
  exec cargo "$@"
fi

if ! command -v oxide >/dev/null 2>&1; then
  cat >&2 <<'MESSAGE'
oxide-rust: oxide is not on PATH, and this selector does not fall back to Cargo.
Run the matching plain-Cargo selector instead (effigy <selector>:cargo, for
example `effigy test:rust:cargo`), or set SWALLOWTAIL_RUST_RUNNER=cargo for one
run. See docs/guides/validation-tiers.md, Oxide Adoption Boundary.
MESSAGE
  exit 127
fi

exec oxide "$@"
