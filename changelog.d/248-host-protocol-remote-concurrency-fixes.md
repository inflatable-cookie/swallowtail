Fix concurrency, timer, deadlock, leak, and decoder bugs across
`swallowtail-host-local`, `swallowtail-protocol-acp`, and
`swallowtail-transport-acp-remote`:

- `swallowtail-host-local`: replace thread-per-wait deadlines with
  `async_io::Timer`, prune finished reaper handles and settle reservations via a
  `Drop` guard, retry `EINTR` in `process_reader`, and close on HTTP `401`,
  reject duplicate sensitive headers with `400`, and preserve pipelined bytes in
  the operation bridge listener.
- `swallowtail-transport-acp-remote`: use non-blocking worker failure reporting
  and close `events` during `close()`, join `connection_task` on `deadline_task`
  spawn failure and signal close on connect timeout, honor configured
  `maximum_frame_bytes` with `CapacityExceeded`, and deliver valid JSON-RPC
  `Err` responses to `session/new`.
- `swallowtail-protocol-acp`: classify invalid `AcpSessionListRequest::new`
  cursors as `RequestInvalid`, treat `"line": null` in `tool_locations` as
  `None`, measure encoded JSON byte lengths with a zero-allocation `Write` sink,
  and add `encode_message_with_limits` and `decode_message_with_limits`.
