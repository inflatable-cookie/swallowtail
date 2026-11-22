# 275: Close final Codex, OpenCode, and Pi asymmetries

### Fixes

- **Codex (`swallowtail-adapter-codex`):**
  - Hash provider IDs into fixed 32-byte SHA-256 keys (`ItemKey`) across `AppServerActivityProjection` (`TrackedItemTable` and `BoundedMap`) and omit `provider_activity_ref` when `ProviderActivityRef::new` rejects IDs over 512 bytes instead of failing with `malformed_notification()`.
  - Guard late non-tool server requests (`item/tool/requestUserInput`, approvals) on stopping turns in `rpc/pump.rs` so a late request racing `turn/interrupt` is rejected on the wire without tearing down the session.
  - Bound `CodexExecRunHandle::close` and `exec_pump::force_cleanup` with a pump abort signal and teardown budget on refused `force_stop()` so `close()` cannot hang indefinitely on open stdout.
- **OpenCode (`swallowtail-adapter-opencode`):**
  - Use abandon semantics on deadline/cancellation (`ProbeInterrupt::Abandon`) for `network.authorize(...)` and `credential_service.acquire(...)` during `prepare_opencode_attached` so stalled host calls do not hang past the probe deadline or cancellation.
  - Use `cancellation.request_bounded()` in `OpenCodeTurnHandle::close` and `close_active` so a stalled `/abort` cannot hang cleanup.
- **Pi (`swallowtail-adapter-pi`):**
  - Align `PiRunCancellation` with `TurnCancellation` (`mark_abort_in_flight()`, `connection.abort_command(...)`, `mark_abort_acknowledged()`, and `reconcile_abort_response(false)`) so rejected or failed aborts do not misreport `TerminalStatus::Cancelled`.
  - Escalate `PiRunHandle::close` to `connection.force_stop()` on any abort error (including transport/write errors) as well as `abort_rejected`.
