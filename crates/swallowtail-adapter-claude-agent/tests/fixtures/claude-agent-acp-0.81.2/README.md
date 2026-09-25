# Claude Agent ACP 0.81.2 currentness corpus

Secret-free identity for official npm `@agentclientprotocol/claude-agent-acp`
`0.81.2` before Swallowtail raises the `claude-agent.acp-adapter` ceiling from
`0.79.0`. Host `claude-agent-acp` was not on `PATH` and was not installed.

Published stable hops above the previous `0.79.0` ceiling are exactly
`0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2`. `0.58.0` stays unpublished and
incompatible. `0.79.1` and `0.80.1` are unpublished stables with preview
builds only. `0.81.3` is the first unpublished later stable.

Selected mapped ACP routes stay the same as the frozen `0.79.0` corpus.
`dist/elicitation.js`, `dist/lib.js`, `dist/settings.js`, `dist/utils.js`,
`dist/session-config-ids.js`, `dist/permissions/options.js`, and
`dist/permissions/presentation.js` are byte-identical across every hop.
`dist/index.js` is byte-identical through `0.81.0` and changes only at
`0.81.1` for a pre-wire managed-policy env read. Mode option id/category,
`plan` / `acceptEdits`, permission option kinds, the effort config id, prompt
usage fields, and `stopReason` stay. `protocolVersion` stays `1`.

The ACP SDK pin moves `1.4.0` → `1.5.0` at `0.80.0`. Published
`@agentclientprotocol/sdk` `dist/acp.js` is byte-identical across that pin,
and `PROTOCOL_VERSION` stays `1`. The schema adds optional `notices` and a
capability-gated `notice` session update. Swallowtail does not advertise
`clientCapabilities.session.notices`, so the selected route keeps the
transcript `agent_message_chunk`. The Agent SDK pin moves `0.3.274` →
`0.3.278` → `0.3.280` and stays unmapped. This family is not flattened onto
Claude Agent SDK or Claude Code stream-JSON.

`dist-inventory.json` freezes the complete `0.79.0`→`0.81.2` package file
inventory. It is not a complete semantic changelog of every internal line.

Claude Code stays a separate axis. No provider prompt. No live ACP
initialize. Official artifacts stayed in `/tmp` and were not executed.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
