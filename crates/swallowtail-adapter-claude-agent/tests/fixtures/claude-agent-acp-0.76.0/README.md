# Claude Agent ACP 0.76.0 currentness corpus

Secret-free identity for host Claude Agent ACP `0.63.0` and official npm
`@agentclientprotocol/claude-agent-acp` `0.76.0` before Swallowtail raises
the `claude-agent.acp-adapter` ceiling from `0.73.0`.

Published stable hops above the previous `0.73.0` ceiling are exactly
`0.74.0`, `0.75.0`, `0.75.1`, and `0.76.0`. `0.58.0` stays unpublished and
incompatible; `0.73.1`, `0.74.1`, `0.75.2`, `0.76.1`, and `0.77.0` are
unpublished stables (`0.75.2` and `0.76.1` have preview builds only).

Selected mapped ACP routes stay the same as the frozen `0.73.0` corpus.
`dist/index.js`, `dist/elicitation.js`, `dist/settings.js`, `dist/utils.js`,
`dist/tools.js`, `dist/session-mode.js`, `dist/session-config-ids.js`, and
the whole `dist/permissions/**` tree are byte-identical across every hop, so
mode ids/categories, `plan`/`acceptEdits`, permission option kinds, the
effort config id, and the elicitation surface are untouched.
`dist/acp-agent.js` changes on every hop; each change is classified in
`protocol.json` as mapped-adjacent, provider-internal, or unmapped with a
reason. `dist-inventory.json` freezes the complete
`0.73.0`→`0.74.0`→`0.75.0`→`0.75.1`→`0.76.0` package file inventory. It is
not a complete semantic changelog of every internal line.

The ACP SDK pin stays `1.4.0` and the Agent SDK pin stays `0.3.257` at every
hop, so the ACP v1 wire schema is unchanged. The `--hide-claude-auth`
`argv` guard, the `authStatus` `_auth/status_update` extension, the
synthetic context-compaction tool call, usage Markdown, the AIR fork
metadata, clear-context coordination, and the capability-gated
recommended-config-value presentation stay unmapped with reasons. Claude
Code stays a separate axis. No provider prompt. No live ACP initialize. The
host install was not replaced. Official artifacts stayed in `/tmp` and were
not executed.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
