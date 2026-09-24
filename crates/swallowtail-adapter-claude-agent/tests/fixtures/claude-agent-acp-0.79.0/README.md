# Claude Agent ACP 0.79.0 currentness corpus

Secret-free identity for official npm `@agentclientprotocol/claude-agent-acp`
`0.79.0` before Swallowtail raises the `claude-agent.acp-adapter` ceiling from
`0.76.0`. Host `claude-agent-acp` was not on `PATH` and was not installed.

Published stable hops above the previous `0.76.0` ceiling are exactly
`0.77.0`, `0.78.0`, and `0.79.0`. `0.58.0` stays unpublished and
incompatible; `0.73.1`, `0.74.1`, `0.75.2`, `0.76.1`, `0.77.1`, `0.78.1`,
and `0.80.0` are unpublished stables (`0.75.2`, `0.76.1`, `0.77.1`, and
`0.78.1` have preview builds only).

Selected mapped ACP routes stay the same as the frozen `0.76.0` corpus.
`dist/index.js`, `dist/settings.js`, `dist/utils.js`, and `dist/lib.js` are
byte-identical across every hop. Mode option id/category, `plan` /
`acceptEdits`, permission option kinds, and the effort config id stay by
classified source. `dist/acp-agent.js` changes on every hop; each change is
classified in `protocol.json` as mapped-adjacent, provider-internal, or
unmapped with a reason. `dist-inventory.json` freezes the complete
`0.76.0`→`0.77.0`→`0.78.0`→`0.79.0` package file inventory. It is not a
complete semantic changelog of every internal line.

The ACP SDK pin stays `1.4.0` at every hop, so the ACP v1 wire schema is
unchanged. The Agent SDK pin moves `0.3.257`→`0.3.270`→`0.3.274` and stays
unmapped; this family is not flattened onto Claude Agent SDK or Claude Code
stream-JSON. The removed `agent` config option, capability-gated
`compaction_update`, AIR file-change / diffStats, defaultToNo option order,
and shell permission titles stay unmapped with reasons. Already-mapped form
elicitation keeps the same field keys and `Other` title; the adapter accepts
the two new Other description strings so those forms still map.

Claude Code stays a separate axis. No provider prompt. No live ACP
initialize. Official artifacts stayed in `/tmp` and were not executed.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
