#!/usr/bin/env bash
set -euo pipefail

# Desktop-only runner for the Grok ACP client-MCP probe. Swallowtail crate
# tests never invoke this script. Live spawn requires
# SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1.
#
# Cargo keeps the host HOME. If GROK_HOME is unset, an empty isolated directory
# is created and exported so the Grok child cannot mutate host Grok state.
# Desktop may export an already-authorized isolated GROK_HOME instead.
#
# The copied echo helper is not assumed live. The Rust live peer proves helper
# liveness before scoring; a dead or unspawnable copy is echo_liveness_unproven,
# not ignores_client_mcp, and must not freeze provider_limitation.
#
# Usage:
#   SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1 \
#     scripts/run-grok-acp-client-mcp-probe.sh \
#       --version 1.0.5 \
#       --grok-executable /path/to/grok \
#       --output /path/to/capsule.json
#
# Routing: this script runs Cargo itself, so it re-executes through the shared
# helper's host-tool mode as one supported lease, and its example build and run
# inherit Oxide's storage placement. `GITHUB_ACTIONS=true` or
# `EFFIGY_OXIDE_ESCAPE=cargo` keeps the plain-Cargo branch, and a missing
# Oxide fails with the escape named. Live spawn still needs the authority guard
# above; routing changes no probe behaviour.

probe_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$probe_repo_root"

probe_original_args=("$@")
probe_version=""
probe_grok_executable=""
probe_output_path=""
probe_echo_mcp=""
probe_isolated_home=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      shift
      probe_version=${1:?missing --version}
      shift
      ;;
    --grok-executable)
      shift
      probe_grok_executable=${1:?missing --grok-executable}
      shift
      ;;
    --output)
      shift
      probe_output_path=${1:?missing --output}
      shift
      ;;
    --echo-mcp)
      shift
      probe_echo_mcp=${1:?missing --echo-mcp}
      shift
      ;;
    *)
      printf 'usage: %s --version <1.0.4|1.0.5> --grok-executable <path> --output <capsule.json> [--echo-mcp <path>]\n' "$0" >&2
      exit 2
      ;;
  esac
done

if [[ -z $probe_version || -z $probe_grok_executable || -z $probe_output_path ]]; then
  printf 'usage: %s --version <1.0.4|1.0.5> --grok-executable <path> --output <capsule.json> [--echo-mcp <path>]\n' "$0" >&2
  exit 2
fi

if [[ "${SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE:-}" != "1" ]]; then
  printf '%s: refusing live Grok path; Desktop sets SWALLOWTAIL_DESKTOP_GROK_ACP_CLIENT_MCP_PROBE=1\n' "$0" >&2
  exit 2
fi

# One supported lease for this Cargo-running tool. The guard stops a re-exec
# loop; the child inherits the lease's storage environment. The shared helper
# owns the CI branch, Effigy's escape and the missing-Oxide refusal.
if [[ ${SWALLOWTAIL_OXIDE_EXEC_CHILD:-} != "1" ]]; then
  SWALLOWTAIL_OXIDE_EXEC_CHILD=1 exec bash "$probe_repo_root/scripts/oxide-rust.sh" \
    --host-tool "$probe_repo_root/scripts/run-grok-acp-client-mcp-probe.sh" \
    "${probe_original_args[@]}"
fi

probe_cleanup() {
  if [[ -n $probe_isolated_home ]]; then
    rm -rf "$probe_isolated_home"
  fi
  if [[ -n ${probe_echo_copied:-} ]]; then
    rm -f "$probe_echo_mcp"
  fi
}
trap probe_cleanup EXIT

if [[ -z $probe_echo_mcp ]]; then
  cargo build --offline --locked -p swallowtail-testkit --example grok-acp-echo-mcp
  probe_echo_source="$probe_repo_root/target/debug/examples/grok-acp-echo-mcp"
  probe_output_dir=$(dirname "$probe_output_path")
  mkdir -p "$probe_output_dir"
  probe_echo_mcp=$(cd "$probe_output_dir" && pwd)/grok-acp-echo-mcp
  cp "$probe_echo_source" "$probe_echo_mcp"
  chmod 700 "$probe_echo_mcp"
  probe_echo_copied=1
fi

if [[ -z ${GROK_HOME:-} ]]; then
  probe_isolated_home=$(mktemp -d)
  export GROK_HOME="$probe_isolated_home"
fi

cargo run --offline --locked -p swallowtail-testkit --example grok-acp-client-mcp-probe -- \
  --live \
  --version "$probe_version" \
  --grok-executable "$probe_grok_executable" \
  --echo-mcp "$probe_echo_mcp" \
  --output "$probe_output_path"
