# Claude Agent ACP 0.70.0 currentness corpus

Secret-free identity for host Claude Agent ACP `0.63.0` and official npm
`@agentclientprotocol/claude-agent-acp` `0.70.0` before Swallowtail raises
the `claude-agent.acp-adapter` ceiling.

Selected initialize session capabilities stay the same. `dist/elicitation.js`
stays byte-identical from `0.64.0` through `0.70.0`. `dist/tools.js` matches
`0.69.0`. Changelog `0.70.0` recreates loaded SDK queries on
`providers/set` / `providers/disable`; that Providers API stays unmapped.
Goal, Air, and file-change initialize `_meta` stay unmapped.

`0.58.0` remains unpublished. Claude Code stays a separate axis. No provider
prompt. No live ACP initialize. The host install was not replaced.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
