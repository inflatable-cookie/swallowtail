# Claude Agent ACP 0.98.0 currentness corpus

This corpus qualifies the `claude-agent.acp` adapter claim from its previous
ceiling, `0.96.0`, through official stable `0.98.0`. It preserves the existing
baseline, seven behavior milestones, exclusions, and unverified newer posture.

The selected sources were npm `latest`, the matching GitHub release tag and
commit, and the ACP registry entry `claude-acp`. Both stable hops are listed
in `identity.json`. Preview-only patch lines `0.96.1` and `0.97.1` remain
holes. The `0.98.1-preview.1` tag is ignored. Each npm tarball digest was
checked against npm integrity metadata before its full file tree was hashed
in `dist-inventory.json`. The `0.96.0` tarball SHA-256 reproduces Research 446.
No file was removed.

`dist/index.js`, `dist/serve.js`, `dist/tools.js`, and `dist/session-mode.js`
are byte-identical from `0.96.0` through `0.98.0`. `deleteSessionIfStored` is
byte-identical. A protocolVersion 1 client still gets the v1 agent when
`CLAUDE_AGENT_ACP_PROTOCOL` is unset.

`0.97.0` adds config id `ultracode`. Swallowtail does not select it and does
not opt into boolean config options. Model, effort, and mode ids stay `model`,
`effort`, and `mode`. Duplicate model display names gain a parenthetical model
id; Swallowtail selects the model value. Advisor tool results emit an existing
tool result. A usage-limit message after compaction still settles `end_turn`.

`0.98.0` adds AIR `_session/batch_get`, advertised only with `sessionIndex`.
Swallowtail does not advertise `sessionIndex`. SandboxNetworkAccess permission
answers emit `tool_call_update` `completed` or `failed`. A v1 permission
subject still includes `status: pending`. Monitor async tasks publish only
when async tasks are enabled. Swallowtail does not advertise them.

Inspection was static. The packages were not installed or executed, and there
were no provider prompts, credentials, live ACP sessions, or host changes.

This qualification does not extend the separate HTTP MCP live gate. Its
honouring evidence stays on exact `0.79.0` and `0.81.2` only. The repo-local
development dependency also remains pinned to `0.81.2`. Claude Code and
`claude-agent.sdk` claims are untouched.

Official identity sources:

- [npm package 0.98.0](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.98.0)
- [GitHub tag v0.98.0](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.98.0)
- [ACP registry latest entry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
