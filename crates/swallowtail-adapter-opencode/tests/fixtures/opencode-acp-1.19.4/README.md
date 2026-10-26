# OpenCode ACP 1.19.4 identity

Secret-free identity evidence for `opencode.acp` on the same official
`opencode-ai` stables as the HTTP record, observed 2026-10-26. No downloaded
artifact was executed. `opencode` was not on `PATH`.

The ACP behavior sources, including `packages/opencode/src/acp/` and
`packages/opencode/src/cli/cmd/acp.ts`, are byte-identical from `1.18.35`
through `1.19.4`. `@agentclientprotocol/sdk` stays `0.21.0`. The manifest
changes version metadata and, at `1.19.4`, the bundled Bun pin.

HTTP MCP live honouring stays exact `1.18.18` and does not transfer.
`opencode.http` stays a separate axis.

Decision: compatible extension of `opencode.acp-v1.client-mcp-servers-v2`.
Keep `1.18.31..=1.18.35`. Add `1.19.0..=1.19.4`. Leave unpublished `1.18.36`
incompatible and synthetic `1.19.5` unverified.
