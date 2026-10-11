#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd "$script_dir/../.." && pwd)
owned_target_runner="$repo_root/scripts/run-with-owned-cargo-target.sh"

die() {
  printf 'sdk-courier-timeout: %s\n' "$1" >&2
  exit 2
}

# Every courier group runs through the run-owned target runner so the outer
# test build and the nested courier build share one warm, isolated target that
# belongs to this selector invocation alone.
run_tests() {
  local host_filter runtime_filter adapter_filter sidecar_filter

  host_filter='test(/real_courier_and_sdk_shaped_process_reach_the_kernel_dispatcher/) | test(/mounted_proxy_denies_through_kernel_admission_before_dispatch/) | test(/mounted_proxy_revocation_between_dispatch_and_delivery_is_reported_without_replay/) | test(/mounted_proxy_expired_deadline_rejects_the_callable_frame_before_dispatch/) | test(/mounted_proxy_cancellation_closes_the_shared_listener_before_late_transport/) | test(/close_joins_an_idle_keep_alive_connection_without_paying_the_read_timeout/) | test(/mounted_proxy_rejects_stale_foreign_and_late_generations_without_reauth/) | test(/mounted_proxy_second_real_courier_cannot_claim_one_rendezvous_or_connection/) | test(/mounted_proxy_concurrency_rejects_a_second_connection_while_dispatch_is_in_flight/) | test(/mounted_proxy_uncooperative_teardown_reports_failure_and_retains_resources/) | test(/mounted_proxy_drops_oversized_and_partial_transport_without_dispatch/)'
  runtime_filter='test(/a_session_lease_ignores_its_elapsed_open_deadline/) | test(/an_explicit_session_lease_deadline_rejects_late_calls/)'
  adapter_filter='test(/registered_session_lease_survives_open_deadline_and_pending_permission/) | test(/expired_call_returns_one_correlated_error_then_the_courier_accepts_a_new_call/) | test(/a_revoked_before_dispatch_never_reaches_the_dispatcher/) | test(/cancellation_during_a_registered_call_joins_the_lease/) | test(/close_joins_the_registered_listener/) | test(/ready_follows_kernel_authenticated_connect/) | test(/a_second_courier_cannot_reread_an_expired_rendezvous/)'
  sidecar_filter='test(/a_declared_stdio_mcp_server_connects_and_its_tool_is_mediated/)'

  cd "$repo_root"
  bash "$owned_target_runner" \
    cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-host-local --features mediated-stdio-proxy \
    --bin swallowtail-registered-tool-courier \
    --then cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-host-local --features mediated-stdio-proxy \
    --test registered_tool_proxy -E "$host_filter" \
    --then cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-runtime --lib -E "$runtime_filter" \
    --then cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-adapter-claude-agent --test claude_agent_sdk_driver \
    -E "$adapter_filter" \
    --then cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-adapter-claude-agent --test claude_agent_sdk_sidecar_asset \
    -E "$sidecar_filter"
  printf 'courier deadline, correlation, cleanup, lease expiry, and prepared SDK regressions passed\n'
}

# The authenticated-idle closure only: listener state transitions, real-courier
# idle/expiry/cancellation, watcher idle on the shared listener, and the
# prepared-facade SDK idle regression.
run_idle_tests() {
  local listener_filter host_filter adapter_filter

  listener_filter='test(/operation_bridge::listener::tests::/)'
  host_filter='test(/real_courier_survives_idle_gaps_longer_than_the_io_timeout/) | test(/an_explicit_lease_deadline_expiring_during_idle_rejects_late_calls_without_dispatch/) | test(/cancelling_a_registered_lease_during_idle_joins_the_listener_and_refuses_later_frames/) | test(/watcher_unauthenticated_idle_is_bounded_on_the_shared_listener/) | test(/close_joins_an_idle_keep_alive_connection_without_paying_the_read_timeout/) | test(/mounted_both_profiles_demux_through_one_listener_and_one_resource_owner/) | test(/mounted_proxy_drops_oversized_and_partial_transport_without_dispatch/)'
  adapter_filter='test(/registered_session_survives_idle_before_can_use_tool_approval/) | test(/a_lease_revoked_while_idle_never_reaches_the_dispatcher/)'

  cd "$repo_root"
  bash "$owned_target_runner" \
    cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-host-local --features mediated-stdio-proxy \
    --lib -E "$listener_filter" \
    --then cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-host-local --features mediated-stdio-proxy \
    --test registered_tool_proxy -E "$host_filter" \
    --then cargo nextest run --locked --profile sdk-patch-candidate \
    -p swallowtail-adapter-claude-agent --test claude_agent_sdk_driver \
    -E "$adapter_filter"
  printf 'authenticated courier idle, revocation, watcher, and SDK idle regressions passed\n'
}

main() {
  (($# == 1)) || die "usage: sdk-courier-timeout.sh format | check | validate | run-tests | idle"
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
    idle)
      run_idle_tests
      ;;
    *)
      die "unknown command: $1"
      ;;
  esac
}

main "$@"
