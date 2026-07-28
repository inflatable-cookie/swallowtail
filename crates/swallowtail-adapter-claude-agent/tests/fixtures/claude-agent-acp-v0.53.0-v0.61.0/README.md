# Claude Agent ACP Usage Corpus

Synthetic safe ACP records derived from every published qualified adapter
point in `0.53.0..=0.61.0`, excluding unpublished `0.58.0`.

The prompt response reports cumulative turn token usage. `usage_update`
reports context occupancy and optional cost. They are deliberately separate.

Sources accessed 2026-07-28:

- <https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.53.0/src/acp-agent.ts>
- <https://github.com/agentclientprotocol/claude-agent-acp/blob/v0.61.0/src/acp-agent.ts>

No fixture contains provider output, credentials, account data, or raw error
payloads.
