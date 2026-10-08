#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd "$script_dir/../.." && pwd)
courier_timeout_scratch=

cleanup() {
  [[ -z $courier_timeout_scratch ]] || rm -rf -- "$courier_timeout_scratch"
}

die() {
  printf 'sdk-courier-timeout: %s\n' "$1" >&2
  exit 2
}

run_tests() {
  local host_filter runtime_filter adapter_filter sidecar_filter
  courier_timeout_scratch=$(mktemp -d "${TMPDIR:-/tmp}/swallowtail-sdk-courier-timeout.XXXXXXXX")
  trap cleanup EXIT
  export CARGO_TARGET_DIR="$courier_timeout_scratch/target"

  host_filter='test(/real_courier_and_sdk_shaped_process_reach_the_kernel_dispatcher/) | test(/mounted_proxy_denies_through_kernel_admission_before_dispatch/) | test(/mounted_proxy_revocation_between_dispatch_and_delivery_is_reported_without_replay/) | test(/mounted_proxy_expired_deadline_rejects_the_callable_frame_before_dispatch/) | test(/mounted_proxy_cancellation_closes_the_shared_listener_before_late_transport/) | test(/close_joins_an_idle_keep_alive_connection_without_paying_the_read_timeout/) | test(/mounted_proxy_rejects_stale_foreign_and_late_generations_without_reauth/) | test(/mounted_proxy_second_real_courier_cannot_claim_one_rendezvous_or_connection/) | test(/mounted_proxy_concurrency_rejects_a_second_connection_while_dispatch_is_in_flight/) | test(/mounted_proxy_uncooperative_teardown_reports_failure_and_retains_resources/) | test(/mounted_proxy_drops_oversized_and_partial_transport_without_dispatch/)'
  runtime_filter='test(/a_session_lease_ignores_its_elapsed_open_deadline/) | test(/an_explicit_session_lease_deadline_rejects_late_calls/)'
  adapter_filter='test(/registered_session_lease_survives_open_deadline_and_pending_permission/) | test(/expired_call_returns_one_correlated_error_then_the_courier_accepts_a_new_call/) | test(/a_revoked_before_dispatch_never_reaches_the_dispatcher/) | test(/cancellation_during_a_registered_call_joins_the_lease/) | test(/close_joins_the_registered_listener/) | test(/ready_follows_kernel_authenticated_connect/) | test(/a_second_courier_cannot_reread_an_expired_rendezvous/)'
  sidecar_filter='test(/a_declared_stdio_mcp_server_connects_and_its_tool_is_mediated/)'

  cd "$repo_root"
  cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-host-local --features mediated-stdio-proxy \
    --bin swallowtail-registered-tool-courier
  cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-host-local --features mediated-stdio-proxy \
    --test registered_tool_proxy -E "$host_filter"
  cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-runtime --lib -E "$runtime_filter"
  cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-adapter-claude-agent --test claude_agent_sdk_driver \
    -E "$adapter_filter"
  cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-adapter-claude-agent --test claude_agent_sdk_sidecar_asset \
    -E "$sidecar_filter"
  printf 'courier deadline, correlation, cleanup, lease expiry, and prepared SDK regressions passed\n'
}

main() {
  (($# == 1)) || die "usage: sdk-courier-timeout.sh format | check | validate | run-tests"
  case "$1" in
    format)
      (cd "$repo_root" && cargo fmt -p swallowtail-host-local \
        -p swallowtail-adapter-claude-agent)
      bash "$script_dir/sdk-patch-candidate.sh" format
      ;;
    check)
      bash "$script_dir/sdk-patch-candidate.sh" check
      ;;
    validate)
      bash "$script_dir/sdk-patch-candidate.sh" validate
      ;;
    run-tests)
      run_tests
      ;;
    *)
      die "unknown command: $1"
      ;;
  esac
}

main "$@"
