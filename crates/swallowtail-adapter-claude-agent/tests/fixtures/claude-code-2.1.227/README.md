# Claude Code 2.1.227 response-only fixtures

`response-complete.jsonl` records the qualified stream shape for one
tool-free assistant text response. The fixture is synthetic and contains no
prompt, credential, filesystem path, or provider session value from a live
run.

The route parser requires an exact init envelope with empty `tools` and
`mcp_servers`, one text-only assistant message with the observed null stop
reason, and one matching success
result with `num_turns: 1` and no structured output.
