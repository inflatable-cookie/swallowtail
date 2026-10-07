# Claude Agent ACP 0.87.0 currentness corpus

This corpus qualifies the `claude-agent.acp` adapter claim from its previous
ceiling, `0.81.2`, through official stable `0.87.0`. It preserves the existing
baseline, seven behavior milestones, exclusions, and unverified newer posture.

The selected sources were npm `latest`, the matching GitHub release tag and
commit, and the ACP registry entry. All seven stable hops are listed in
`identity.json`; preview only patch lines remain holes. Each npm tarball digest
was checked against npm integrity metadata before its full file tree was hashed
in `dist-inventory.json`. The trees include every extracted file and SHA-256.
Each added or changed file is classified at its exact hop. No file was removed.

Inspection was static. The packages were not installed or executed, and there
were no provider prompts, credentials, live ACP sessions, or host changes.
The source of truth for the route's mapped behavior remains the ACP v1 wire
Swallowtail sends and decodes. The Rust decoder accepts partial tool updates;
new AIR metadata is not requested or consumed; selected lifecycle, permission,
configuration, and usage behavior stays within the existing route.

This qualification does not extend the separate HTTP MCP live gate. Its
honouring evidence stays on exact `0.79.0` and `0.81.2` only. The repo local
development dependency also remains pinned to `0.81.2` by the task's no version
bump constraint.

Official identity sources:

- [npm package 0.87.0](https://www.npmjs.com/package/@agentclientprotocol/claude-agent-acp/v/0.87.0)
- [GitHub tag v0.87.0](https://github.com/agentclientprotocol/claude-agent-acp/tree/v0.87.0)
- [ACP registry latest entry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)

