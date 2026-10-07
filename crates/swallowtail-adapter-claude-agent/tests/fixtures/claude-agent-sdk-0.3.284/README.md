# claude-agent-sdk 0.3.284 identity

Frozen evidence for rebinding the exact `claude-agent.sdk` package and native
axes from `0.3.270` to official npm `0.3.284`. Secret-free: no credential, host
path, account id, payload, or conversation id appears here.

All fourteen wrapper tarballs (`0.3.270` plus the thirteen published stables
through `0.3.284`) were downloaded to `/tmp`, hashed, and extracted. Nothing
was executed, no platform package was fetched, no host was changed, and no
provider session, login, or token read occurred. The native binaries are
identified from the shipped `manifest.json` digests rather than by downloading
200 MB artifacts. Unpublished `0.3.279` stays a gap. npm `next` `0.3.285` is
recorded and ignored. Research 367 is the identity record.

- `identity.json` — official and previous-ceiling artifact identity, every
  published hop with its coupled native, the unpublished gap, and the rebind
  decision. The `0.3.270` digests corroborate Research 315 exactly.
- `dist-inventory.json` — deterministic package-tree inventory across all
  fourteen points: 4 files identical through every hop, 15 files through
  `0.3.281`, 19 files from `0.3.282` after the `./core` split. Per-hop
  added/removed/changed sets and per-file digests. Three hops carry zero
  declaration changes: `0.3.271→0.3.272`, `0.3.275→0.3.276`,
  `0.3.277→0.3.278`.
- `sdk-declarations.d.ts` — reproducible excerpts of the pinned `sdk.d.ts`
  for `query()`, `AccountInfo`, `SpawnedProcess`, `SpawnOptions`,
  `CanUseTool`, `PermissionResult`, stdio MCP config, `McpServerStatus`,
  `mcpServers`, `strictMcpConfig`, `mcpServerStatus()`, and `close()`, plus
  `SDKResultSuccess`/`SDKResultError` usage fields, `ModelUsage`,
  `Query.getContextUsage()`, its context response, model identifiers, and the
  separate assistant-message `usage_report` field. The added excerpts cite the
  exact declaration locations and the pinned `sdk.d.ts` digest.
- `mcp-protocol.json` — the MCP protocol-version constants the pinned bundle
  carries, recovered from `package/sdk.mjs` by literal string search. The
  version list is identical to `0.3.270`; that equality is not live evidence.
- `protocol.json` — the selected mapped subset, every classified declaration
  delta with why it stays unmapped, the unchanged implementation invariants,
  the Card 146 allowlist growth for `source`, and the registered-tool
  disposition: Research 301 stays bound to `0.3.259`/`2.1.259`.
