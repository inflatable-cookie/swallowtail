# Claude Agent ACP 0.96.0 currentness corpus

This corpus qualifies the `claude-agent.acp` adapter claim from its previous
ceiling, `0.87.0`, through official stable `0.96.0`. It preserves the existing
baseline, seven behavior milestones, exclusions, and unverified newer posture.

The selected sources were npm `latest`, the matching GitHub release tag and
commit, and the ACP registry entry `claude-acp`. All ten stable hops are listed
in `identity.json`; preview-only patch lines remain holes. Each npm tarball
digest was checked against npm integrity metadata before its full file tree
was hashed in `dist-inventory.json`. The trees include every extracted file
and SHA-256. Each added or changed file is classified at its exact hop. No
file was removed.

Inspection was static. The packages were not installed or executed, and there
were no provider prompts, credentials, live ACP sessions, or host changes.
The source of truth for the route's mapped behavior remains the ACP v1 wire
Swallowtail sends and decodes. A v1 initialize still selects the v1 agent when
`CLAUDE_AGENT_ACP_PROTOCOL` is unset. ACP SDK 2.0.0 is an import move, not a
v1 wire reset. Missing close returns an empty success from `0.89.0`; live
close and stored delete success shapes stay empty objects. Unstored delete
throws via Agent SDK `deleteSession` through `0.90.0`. From `0.91.0`,
`deleteSessionIfStored` returns empty success for this non-AIR client.
Swallowtail does not gain a consumer operation or a stronger deletion claim.

This qualification does not extend the separate HTTP MCP live gate. Its
honouring evidence stays on exact `0.79.0` and `0.81.2` only. The repo-local
development dependency also remains pinned to `0.81.2`. Claude Code and
`claude-agent.sdk` claims are untouched.

Official identity sources:

- [npm package 0.96.0](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.96.0)
- [GitHub tag v0.96.0](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.96.0)
- [ACP registry latest entry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
