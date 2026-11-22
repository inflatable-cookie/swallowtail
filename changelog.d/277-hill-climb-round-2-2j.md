- Fix `swallowtail-adapter-pi` sidecar outbound command framing so a valid 1 MiB
  PNG attachment (base64 ~1.33 MiB) encodes and decodes within a 2 MiB outbound
  command record bound while keeping inbound sidecar response/event records
  bounded at 1 MiB.
- Fix `swallowtail-adapter-pi` `usage_missing` ordering on both `pi.rpc` and
  `pi.sdk-sidecar` so missing token usage does not increment `completed_prompts`
  or close open activities as `Completed` (closing them as `Failed` instead).
- Poll the worker future in `swallowtail-transport-http`
  `Subscription::poll_next` so lazy `BlockingWorkService` futures cannot
  deadlock and completed worker futures are driven cleanly.
- Serialize `swallowtail-acp-client` `ActiveTurn` sequence allocation with event
  channel emission and `mark_terminal()`, add `AcpTurnSink::observe_permission_with_tool_call`,
  and remove the duplicated permission rejection hook and shadow turn slot from
  `swallowtail-adapter-copilot-cli`.
