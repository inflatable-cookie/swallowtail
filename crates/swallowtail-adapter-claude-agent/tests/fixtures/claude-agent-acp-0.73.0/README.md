# Claude Agent ACP 0.73.0 currentness corpus

Secret-free identity for host Claude Agent ACP `0.63.0` and official npm
`@agentclientprotocol/claude-agent-acp` `0.73.0` before Swallowtail raises
the `claude-agent.acp-adapter` ceiling. Operator restart of the unmerged
`0.72.0` family after official latest moved.

Selected mapped ACP routes stay the same as frozen `0.70.0`.
`dist/index.js`, `dist/elicitation.js`, `dist/lib.js`, `dist/settings.js`,
and `dist/utils.js` stay byte-identical from `0.70.0` through `0.73.0`.
`dist/elicitation.js` stays byte-identical from `0.64.0` through `0.73.0`.
Every `dist/**` file is byte-identical `0.72.0` to `0.73.0`. The only
`0.72.0`→`0.73.0` package change is `package.json`: version and Agent SDK
pin `0.3.252`→`0.3.257`. `0.71.0`/`0.72.0` mapped-adjacent deltas stay
classified in `protocol.json` as intermediate evidence, not a standalone
ceiling. `dist-inventory.json` freezes the complete
`0.70.0`→`0.71.0`→`0.72.0`→`0.73.0` package file inventory. Remaining
named files stay unmapped with reason. The inventory is not a complete
semantic changelog of every internal line.

Published intermediates after previous ceiling `0.70.0` are exactly
`0.71.0`, `0.72.0`, and `0.73.0`. `0.58.0` remains unpublished. Claude
Code stays a separate axis. No provider prompt. No live ACP initialize.
The host install was not replaced. Official artifacts stayed in `/tmp`
and were not executed.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
