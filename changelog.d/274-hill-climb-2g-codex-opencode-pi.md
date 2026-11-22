# 274: Fix remaining Codex, OpenCode, and Pi adapter defects

### Fixes

- **Codex (`swallowtail-adapter-codex`):**
  - Accept all workspace-roots behavior revisions (`CodexAppServerBehavior::supports_workspace_roots()`, covering `0.156.0`–`0.164.0` and newer) when preparing bounded-workspace sessions and provider-session imports.
  - Populate `extension_namespaces` from `access_policy.provider_requests()` during provider-session import preparation.
  - Route the remaining four app-server cancellation/abort teardown sites (`thread_catalogue`, `session_open`, `app_server::continuity`, and `session_management` init error) through the shared abort-safe `cancel_connection` path so a refused `force_stop()` with open stdout cannot hang `task.join()`.
  - Bound `ExecActivityProjection` tracked item state with FIFO completed-item eviction, minted-ID fallback for oversized provider item IDs, and optional `ProviderActivityRef` attachment when the provider ID exceeds 512 bytes.
- **OpenCode (`swallowtail-adapter-opencode`):**
  - Advance the pagination cursor in `project_opencode_page` whenever a full page (`raw_len == limit`) is returned and scan past fully directory-filtered pages in `execute_provider_session_catalogue` under an explicit page-scan bound so `ProviderSessionCatalogueOutcome` never receives an empty page with a continuation cursor.
  - Track receiver-stream abandonment locally so final `Activity` or `Usage` send failures preserve the authentic `TerminalStatus` (`Completed`, `Cancelled`, `TimedOut`, `ProviderFailed`) when the consumer dropped the event stream, while continuing to surface real buffer overflow failures on a still-attached stream.
- **Pi (`swallowtail-adapter-pi`):**
  - Track in-flight turn `abort` separately from acknowledged cancellation so a failed, rejected, or EOF-interrupted `abort` on both the RPC and SDK sidecar routes resolves with the `cancel_rejected` `RuntimeFailed` diagnostic instead of `TerminalStatus::Cancelled`.
  - Close open activities in `fail_connection` before `events.mark_terminal()` on both the RPC and SDK sidecar routes so mid-tool or mid-reasoning disconnects do not leave open activities unclosed.
  - Keep `start_turn` as the sole deadline owner during `prompt` dispatch (retiring dropped pending commands, closing and force-stopping on prompt timeout) and bound `spawn_deadline` abort acknowledgement before escalating via `close_on_rejected_abort` and `force_stop` on both the RPC and SDK sidecar routes.
