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

### Compatibility

- Additive public API: `swallowtail_runtime::EXECUTABLE_NOT_FOUND_CODE` and
  `swallowtail_protocol_acp::RpcError::new`. No public item is removed or
  changed.
- An event buffer whose activity tracker is full of in-flight activities now
  reports the `SemanticOverflow` failure kind where the tracker previously grew
  without bound.
