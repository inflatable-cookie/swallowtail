# Claude Code 2.1.228 response-only fixtures

`response-complete.jsonl` records the latest live-evidenced qualified stream shape for one
tool-free assistant text response. The fixture is synthetic and contains no
prompt, credential, filesystem path, or provider session value from a live
run.

The route parser requires an init envelope whose version equals the executable
version frozen by preparation, with empty `tools` and `mcp_servers`, one
text-only assistant message with the observed null stop reason, and one matching success
result with `num_turns: 1` and no structured output.

`response-thinking-progress.jsonl` records the separately observed
medium-effort sequence: exact cumulative integer progress, one empty private
thinking block carrying an opaque signature, one text record with the same
message id, and the matching terminal result. The synthetic signature and
session contain no provider data. The private block is validation evidence,
not disclosed reasoning.
