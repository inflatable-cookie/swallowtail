### Fixed

- `swallowtail-runtime`:
  - `poll_delegated_async` now reuses a retained `PendingSignInPoll::Delegated` future matching `poll_interactive_async` and `poll_device_async`.
  - Bounded `cleanup_session_async` teardown, preserved `LoopbackCallbackLease` when `services.loopback_callback()` is absent, and retained pending process stop futures in `cleanup_session_now`.
  - `installed_discovery` now races `services.process().start(...)` against the probe deadline and cancellation signals.
- `swallowtail-host-local`:
  - `LocalProcessHandle::write_stdin` no longer holds the `stdin` mutex across blocking writes, preventing executor stalls and deadlocks with concurrent `close_stdin()` / `force_stop()`.
- `swallowtail-transport-acp-remote`:
  - Preserved terminal `WorkerEvent::Failed` errors when the bounded events channel is full so `next_event()` observes `Err(error)` instead of clean EOF.
  - Configured tungstenite `WebSocketConfig` (`max_message_size`, `max_frame_size`) from `maximum_frame_bytes` and forwarded `validate_connection_id` / cookie storage failures on the `ready` channel.
  - Added a bounded close grace period on `WorkerCommand::Deadline` in HTTP and WebSocket transports and validated `maximum_frame_bytes` at config time.
- `swallowtail-transport-http`:
  - Removed `line.clear()` in `append_secret_header` so `Zeroizing::drop` zeroizes the formatted secret header bytes.
  - Validated non-secret HTTP header names (RFC 7230 tokens) and values (no `\r`/`\n`) before passing them to libcurl.
  - Cleared captured response headers on each `HTTP/` status line (for `100 Continue`) and emitted `StreamItem::Headers` for empty-body `2xx` streaming responses.
  - Checked `SseFramer` `event_limit` before growing `pending` when no frame boundary is present and stripped at most one leading space in `data:` fields.
- `swallowtail-acp-client`:
  - Stored `AcpRemoteMcpPlacement` URL in `Zeroizing<String>` and zeroized secret URL and header values on `Drop`.
