# OpenCode ACP 1.19.5 identity

Secret-free identity evidence for `opencode.acp` on the same official
`opencode-ai` stable as the HTTP record, observed 2026-10-28. No downloaded
artifact was executed. `opencode` was not on `PATH`.

The ACP behavior sources, including `packages/opencode/src/acp/` and
`packages/opencode/src/cli/cmd/acp.ts`, are byte-identical from `1.19.4` to
`1.19.5`. `@agentclientprotocol/sdk` stays `0.21.0`. The manifest changes its
version and the `gitlab-ai-provider` pin. Bundled Bun stays `1.4.4`.

The ACP registry `opencode` `2.2.0` entry is the OpenCode 2 release line.
npm `opencode-ai` `latest` is `1.19.5`. This record does not qualify `2.2.0`.

HTTP MCP live honouring stays exact `1.18.18` and does not transfer.
`opencode.http` stays a separate axis.

Decision: compatible extension of `opencode.acp-v1.client-mcp-servers-v2`.
Keep `1.18.31..=1.18.35`. Extend `1.19.0..=1.19.4` to `1.19.0..=1.19.5`.
Leave unpublished `1.18.36` incompatible and synthetic `1.19.6` unverified.
