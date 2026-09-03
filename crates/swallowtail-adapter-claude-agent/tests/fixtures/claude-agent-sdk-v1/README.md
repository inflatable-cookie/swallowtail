# claude-agent-sdk-v1 corpus

Provider-free capture of the private `swallowtail-claude-agent-sdk-jsonl-v1`
wire at behavior revision `claude-agent.sdk-v1`. Every record was written by
hand against the shipped sidecar asset in `sidecar/claude-agent-sdk-sidecar.mjs`;
no provider session, login, or downloaded binary was ever run.

- `protocol.json` — frozen identity, bounds, command set, close states, and the
  mechanically checked forbidden-specifier list for credential non-custody.
- `commands.jsonl` — every outbound command plus both callback decisions.
- `responses.jsonl` — open readiness, query acceptance, interrupt receipt, both
  sidecar-observable native joins, and one rejected open.
- `events.jsonl`, `callbacks.jsonl`, `terminal.jsonl`, `diagnostics.jsonl` —
  qualified inbound records.
- `unknown.jsonl`, `malformed.jsonl`, `disconnect.jsonl` — fail-closed
  negatives: an unqualified event name, invalid JSON, and a truncated stream
  whose final record has no LF delimiter.
