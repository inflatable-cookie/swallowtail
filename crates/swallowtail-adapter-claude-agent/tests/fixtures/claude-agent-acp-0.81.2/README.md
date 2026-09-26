# Claude Agent ACP 0.81.2 currentness corpus

Secret-free identity for official npm `@agentclientprotocol/claude-agent-acp`
`0.81.2` before Swallowtail raises the `claude-agent.acp-adapter` ceiling from
`0.79.0`. Host `claude-agent-acp` was on `PATH` at Homebrew `0.63.0` and was
observed only through `--version`. It was not installed, updated, or executed
beyond that flag.

Published stable hops above the previous `0.79.0` ceiling are exactly
`0.80.0`, `0.81.0`, `0.81.1`, and `0.81.2`. `0.58.0` stays unpublished and
incompatible; `0.73.1`, `0.74.1`, `0.75.2`, `0.76.1`, `0.77.1`, `0.78.1`,
`0.79.1`, `0.80.1`, and `0.81.3` are unpublished stables (`0.79.1`, `0.80.1`,
and `0.81.3` have preview builds only).

Selected mapped ACP routes stay the same as the frozen `0.79.0` corpus.
`dist/elicitation.js`, `dist/settings.js`, `dist/utils.js`, `dist/lib.js`, and
`dist/session-config-ids.js` are byte-identical across every hop.
`session/close` → `teardownSession` → `closeQueryStream` are byte-identical.
The `mcpServers` HTTP mapping is byte-identical. `cancel()` gains an optional
`contextCompaction?.interrupt()` at `0.80.0`; on the selected route that await
is a no-op because Swallowtail does not advertise `session.compaction`. The
unbounded `query.interrupt()` await remains. HTTP MCP honouring stays bound to
exact `0.79.0`.

The ACP SDK pin moves `1.4.0` → `1.5.0` at `0.80.0`. Schema
`PROTOCOL_VERSION` stays `1`; the sidecar still returns `protocolVersion: 1`.
The Agent SDK pin moves `0.3.274` → `0.3.278` → `0.3.280` and stays unmapped.
Capability-gated session notices, managed-policy env apply, native-subagent
resume announcement, and clear-context plan follow-up stay unmapped with
reasons. `dist-inventory.json` freezes the complete
`0.79.0`→`0.80.0`→`0.81.0`→`0.81.1`→`0.81.2` package file inventory. It is
not a complete semantic changelog of every internal line.

Claude Code stays a separate axis. No provider prompt. No live ACP
initialize. Official artifacts stayed in `/tmp` and were not executed.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
