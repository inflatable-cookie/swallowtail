# 2026-08-18 Claude Agent ACP 0.69.0 Identity

## Result

Card 252 froze host Claude Agent ACP `0.63.0` and official npm
`@agentclientprotocol/claude-agent-acp` `0.69.0` against qualified
`0.64.0`. `dist/elicitation.js` stays byte-identical through `0.69.0`.
Initialize selected capabilities and permission kinds stay on the mapped
v1 subset. Goal, Air, and file-change initialize `_meta` stay unmapped.
No provider prompt. Live ACP initialize was not run. The host install was
not changed.

## Next

Card 253 raises `claude-agent.acp-adapter` through `0.69.0`.
