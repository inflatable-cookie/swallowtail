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
