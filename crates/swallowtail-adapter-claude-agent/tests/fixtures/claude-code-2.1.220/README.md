# Claude Code headless corpus

Frozen against Claude Code `2.1.220` and the provider-published Agent SDK
message declarations shipped with `@anthropic-ai/claude-agent-sdk 0.3.220`.
The completion ordering also matches a local subscription live probe, including
the `hook_started` and `hook_response` metadata observed before `system/init`.

The selected invocation uses `claude -p`, text stdin, `stream-json` stdout,
`plan` permission mode, the `Read`, `Glob`, and `Grep` tool subset, ambient
user/project/local settings, an empty strict MCP configuration, explicit model
and optional effort, and `--no-session-persistence`.

Fixtures retain only structural provider evidence. Prompts, tool results,
provider errors, credentials, and local paths are synthetic.

The selected command does not pass `--include-partial-messages`.
`headless-complete.jsonl` and `headless-tools.jsonl` are therefore
completion-only activity evidence. The unknown and malformed fixtures freeze
safe post-init namespace handling versus fail-closed framing without changing
production decoding in card 128.

`headless-max-turns.jsonl` freezes the native limit-reached terminal shape
Research 226 extracted from exact `2.1.220..=2.1.241` artifacts: one
`error_max_turns` result with `is_error`, `num_turns`, `stop_reason`, `usage`,
and a `Reached maximum number of turns (N)` message, and no `result` field.
The native process exits `1` alongside it. No provider prompt was sent; the
stream is synthetic and structural.
