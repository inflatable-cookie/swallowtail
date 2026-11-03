# Claude Agent ACP 0.100.0 currentness corpus

This corpus qualifies the `claude-agent.acp` adapter claim from its previous
ceiling, `0.98.0`, through official stable `0.100.0`, walking every published
stable hop: `0.99.0`, `0.99.1`, and `0.100.0`. The dispatch named `0.99.0` and
`0.100.0`; npm also published stable `0.99.1` between them (2026-10-29), so the
run walks all three. It preserves the existing baseline, seven behavior
milestones, exclusions, and unverified newer posture.

The selected sources were npm `latest`, the matching GitHub release tags and
commits, and the ACP registry entry `claude-acp`. The canonical repository is
`agentclientprotocol/claude-agent-acp` — the same repository Research 460
used; the `zed-industries/claude-agent-acp` name resolves to it as a redirect.
Each npm tarball digest was checked against npm integrity metadata before its
full file tree was hashed in `dist-inventory.json`. The `0.98.0` tarball
SHA-256 reproduces Research 460, and `0.96.0` reproduces Research 446. No file
was removed across the three hops.

`dist/index.js`, `dist/serve.js`, and `dist/tools.js` are byte-identical from
`0.96.0` through `0.100.0`, so an unset `CLAUDE_AGENT_ACP_PROTOCOL` still
serves a protocolVersion 1 client the v1 agent. `deleteSessionIfStored` is
byte-identical through `0.100.0`; only the AIR archive-instead-of-delete path
changed in `session-index/service.js`. `dist/session-mode.js` leaves the
byte-identical set at `0.99.0` because its failure codes moved to protocol
error codes; its selected success path stays.

Selected-surface deltas, bounded in `protocol.json`: failure codes on
`session/set_config_option`, `session/set_mode`, and session lookups become
protocol error codes the adapter already maps to one rejected-request failure
class; the model config option now always lists `currentValue`; prompt
content-type validation rejects types Swallowtail never sends; config changes
serialize per session; client-provided MCP server startup failures add
`agent_message_chunk` fallback text; `usage_update` gains a clear-context send
site; `available_commands_update` drops whitespace and duplicate command
names. AIR, plan, notices, sessionIndex, async tasks, native subagents,
boolean config options, and background work stay behind capabilities or
options Swallowtail does not advertise. `dist/session-ultracode.js` reaches
the `configOptions` payload of selected sessions and is classified
mapped-adjacent here, correcting the provider-internal classification the
`0.98.0` fixture gave the same files. The host PATH observation belongs to the
environment that took it and is not a claim of universal absence.

Inspection was static. The packages were not installed or executed, and there
were no provider prompts, credentials, live ACP sessions, or host changes.

This qualification does not extend the separate HTTP MCP live gate. Its
honouring evidence stays on exact `0.79.0` and `0.81.2` only. The repo-local
development dependency also remains pinned to `0.81.2`. Claude Code and
`claude-agent.sdk` claims are untouched.

Official identity sources:

- [npm package 0.100.0](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.100.0)
- [GitHub tag v0.100.0](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.100.0)
- [ACP registry latest entry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
