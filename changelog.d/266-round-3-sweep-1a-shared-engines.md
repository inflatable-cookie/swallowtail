# 266: Round 3 sweep 1A: shared HTTP/ACP engine fixes and 41-crate docs

- `swallowtail-transport-http` now fuses `Subscription::poll_next`: after a
  terminal outcome every later poll returns `None` instead of repeating
  `TransportError::Transport`.
- `Method::Put` and `Method::Patch` requests attach their body, so non-POST
  write verbs no longer silently send an empty payload.
- `SseFramer::push` drains every complete frame before applying the per-event
  bound, so one chunk carrying several bounded events no longer fails with
  `LimitExceeded`.
- `SseFramer` and `SseFrame` zeroize retained and decoded bytes with
  `zeroize::Zeroize` instead of a dead-store-eliminable `Vec::fill(0)`.
- `swallowtail-adapter-openai` migrates its Responses SSE decoder onto the
  shared `swallowtail_transport_http::SseDecoder`, inheriting one delimiter
  authority (mixed `\r\n\r\n` and `\n\n` boundaries) and redacted, zeroized
  frames while keeping its own `sse_limit`, `sse_disconnected`, and malformed
  failure codes.
- `swallowtail-acp-client` reaps the spawned stdio process or remote
  connection on early open-session validation failures instead of only closing
  the write side.
- `swallowtail-acp-client` gates `swallowtail-transport-acp-remote` behind an
  off-by-default `remote` feature, so stdio-only ACP adapters no longer
  compile Tokio, `reqwest`, and `tungstenite`.
- `docs/knowledge/architecture/system-architecture.md` and
  `release-and-package-topology.md` describe the exact 41-package workspace,
  including `swallowtail-acp-client`, `swallowtail-transport-http`, and the
  current `swallowtail-runtime` and `swallowtail-protocol-acp` dependencies.
