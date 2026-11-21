# 269: Bound runtime and transport state, and share Codex description handling

### Fixes

- Activity lifecycle tracking is bounded at 1024 activities. Completed
  activities are evicted to admit new ones; when every tracked activity is
  still in flight, a new activity fails with the existing semantic-overflow
  failure instead of growing the tracker.
- Installed-executable discovery and the local host now share one
  `EXECUTABLE_NOT_FOUND_CODE` constant rather than repeating the diagnostic
  string.
- Route resolution rejects a relative route that climbs above the endpoint
  base path, for example `../../other`.
- ACP remote error responses are sent as a constructed response instead of a
  re-encode and decode round trip.
- HTTP MCP header values containing a carriage return or line feed are
  rejected, as the HTTP transport's secret headers already are.
- Codex callback call ids are released when their callback is answered or
  abandoned, so the set is bounded by pending callbacks rather than the
  session length.
- Codex lifecycle notification retention keeps the most recent 256 entries.
- Codex lifecycle cancellation and timeout paths return the cleanup outcome
  from closing the connection instead of discarding it.
- OpenCode SSE framing finds the first blank line whether the framing is LF,
  CRLF, or mixed, so a CRLF line before an LF blank line no longer leaves a
  carriage return in event data.
- OpenCode attachment input and Pi attachment input share one runtime
  materialization path (`materialize_bounded_attachment`), so the lease,
  bounded read, and release-on-refusal rules exist once. Each driver still
  reports its own diagnostic namespace.
- OpenCode's cancellation and deadline waits share one race helper. A
  deadline-only wait abandons the work, and a drained wait keeps the result
  while reporting the interrupt.
- OpenCode session history returns the runtime's provider-neutral
  `swallowtail.provider_session_history.cleanup_incomplete` code, instead of an
  adapter-namespaced copy of it.
- Pi's SDK-sidecar and RPC drivers both fail a non-zero process exit at
  cleanup. The RPC driver previously reported it clean.
- Pi close reports a failed native abort as a cleanup failure, instead of
  discarding it.
- Codex thread catalogue, history, import, and reconciliation share one
  controlled-RPC lifecycle. Each keeps its own diagnostic codes; a pre-dispatch
  cancellation or timeout in import, history, or reconciliation now reports
  that operation's codes rather than the catalogue's.

### Compatibility

- Additive public API: `swallowtail_runtime::EXECUTABLE_NOT_FOUND_CODE`,
  `swallowtail_runtime::{AttachmentMaterializationFailure,
  SharedAttachmentMaterialization, materialize_bounded_attachment}`, and
  `swallowtail_protocol_acp::RpcError::new`. No public item is removed or
  changed.
- An event buffer whose activity tracker is full of in-flight activities now
  reports the `SemanticOverflow` failure kind where the tracker previously grew
  without bound.
