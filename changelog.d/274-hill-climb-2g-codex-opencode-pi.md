# 274: Fix remaining Codex, OpenCode, and Pi adapter defects

### Fixes

- **Codex (`swallowtail-adapter-codex`):**
  - Accept all workspace-roots behavior revisions (`CodexAppServerBehavior::supports_workspace_roots()`, covering `0.156.0`–`0.164.0` and newer) when preparing bounded-workspace sessions and provider-session imports.
  - Populate `extension_namespaces` from `access_policy.provider_requests()` during provider-session import preparation.
  - Route the remaining four app-server cancellation/abort teardown sites (`thread_catalogue`, `session_open`, `app_server::continuity`, and `session_management` init error) through the shared abort-safe `cancel_connection` path so a refused `force_stop()` with open stdout cannot hang `task.join()`.
  - Bound `ExecActivityProjection` tracked item state with FIFO completed-item eviction and minted-ID fallback for oversized provider item IDs.
- **OpenCode (`swallowtail-adapter-opencode`):**
  - Advance the pagination cursor in `project_opencode_page` whenever a full page (`raw_len == limit`) is returned, even when all entries are filtered out by `directory`.
  - Preserve the authentic `TerminalStatus` (`Completed`, `Cancelled`, `TimedOut`, `ProviderFailed`) when final `Activity` or `Usage` `events.send(...)` fails after the consumer closes or drops the event stream early.
- **Pi (`swallowtail-adapter-pi`):**
  - Resolve rejected/failed turn `abort` on both the RPC and SDK sidecar routes with the `cancel_rejected` `RuntimeFailed` diagnostic instead of misclassifying it as `TerminalStatus::Cancelled`.
  - Close open activities in `fail_connection` before `events.mark_terminal()` on both the RPC and SDK sidecar routes so mid-tool or mid-reasoning disconnects do not leave open activities unclosed.
  - Bound `prompt` command dispatch with `run_before_deadline` and escalate `spawn_deadline` abort failures via `close_on_rejected_abort` and `force_stop` on both the RPC and SDK sidecar routes.
