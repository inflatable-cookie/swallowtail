# 249: Harden the Codex adapter's panic, bound, and teardown paths

### Fixes

- Plan-mode sessions without a preflight-bound model now return
  `swallowtail.codex.app_server.model_missing` instead of panicking.
- Stdout lines and streamed turn output are bounded (16 MiB each). Exceeding
  a bound fails the stream with `swallowtail.codex.exec.line_too_large`,
  `swallowtail.codex.app_server.line_too_large`, or
  `swallowtail.codex.app_server.turn_output_too_large`.
- Closing an app-server session or connection escalates to a graceful stop at
  the midpoint of the caller's remaining cleanup budget and to a forced stop
  at three quarters, so a child that ignores stdin EOF is no longer left
  running.
- A Codex CLI version lookup now selects the Codex CLI axis rather than the
  first interface-version binding, which could be another axis.
- Registered-tool leases are returned when a turn fails before `turn/start`,
  when a dispatch never spawns, and at close, where every dispatch is joined
  before the lease is taken.
- Turn deadlines freeze registered-tool admission before the interrupt.
- A failed session open after `thread/start` aborts the started session.
- Cancellation claims are made inside the async body and released when the
  stop or interrupt fails, so a retry can run.
- RPC requests cannot register after the connection closes: the closed flag
  is set under the pending-response lock.
- Public protocol diagnostics no longer carry the raw inbound wire excerpt or
  the stderr tail. The public message keeps the bounded method name; the raw
  excerpt and stderr tail go only to the opt-in `DiagnosticObserver`.

### Compatibility

- Consumer-facing diagnostic message change: `swallowtail.codex.app_server.malformed_notification` and `malformed_message` messages no longer include `excerpt` or `; stderr:` text. Consumers that matched those fragments must read the observer.
- Behavior of Bovine Desktop (`AppServer`, `prepare_codex`, registered
  editing) and Soundcheck (`CodexExecProfileInput`, `CodexPreparedDriver`,
  `CODEX_CLI_AXIS`) is unchanged on the success path. No public shape changes.
- `merge_cleanup` and `wait_controlled` are single crate-internal helpers.
