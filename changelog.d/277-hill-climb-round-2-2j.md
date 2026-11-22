- Fix `swallowtail-adapter-pi` `usage_missing` ordering on both `pi.rpc` and
  `pi.sdk-sidecar` so missing token usage does not increment `completed_prompts`
  or close open activities as `Completed` (closing them as `Failed` instead).
- Poll the worker future in `swallowtail-transport-http`
  `Subscription::poll_next` so lazy `BlockingWorkService` futures cannot
  deadlock and completed worker futures are driven cleanly.
- Serialize `swallowtail-acp-client` `ActiveTurn` update projection, output
  accumulation, sequence allocation, and terminal publication without holding
  the turn lock across user closures or channel wakes, add
  `AcpTurnSink::observe_permission_with_tool_call`, and remove the duplicated
  permission rejection hook and shadow turn slot from
  `swallowtail-adapter-copilot-cli`.
