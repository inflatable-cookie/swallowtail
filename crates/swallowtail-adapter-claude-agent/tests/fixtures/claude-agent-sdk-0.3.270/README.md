# claude-agent-sdk 0.3.270 identity

Frozen evidence for rebinding the exact `claude-agent.sdk` package and native
axes from `0.3.259` to official npm `0.3.270`. Secret-free: no credential, host
path, account id, payload, or conversation id appears here.

All ten wrapper tarballs (`0.3.259` plus the nine published stables through
`0.3.270`) were downloaded to `/tmp`, hashed, and extracted. Nothing was
executed, no platform package was fetched, no host was changed, and no provider
session, login, or token read occurred. The native binaries are identified from
the shipped `manifest.json` digests rather than by downloading 200 MB artifacts.
Unpublished `0.3.262` and `0.3.264` stay gaps. Research 315 is the identity
record.

- `identity.json` — official and previous-ceiling artifact identity, every
  published hop with its coupled native, the two unpublished gaps, and the
  rebind decision. The `0.3.259` digests corroborate Research 280 exactly.
- `dist-inventory.json` — deterministic 15-file package-tree inventory across
  all ten points: 6 files identical through every hop, 0 added, 0 removed,
  per-hop changed sets, with per-file digests. Three hops (`0.3.261→0.3.263`,
  `0.3.265→0.3.266`, `0.3.269→0.3.270`) carry zero declaration changes: pure
  native rotation and metadata.
- `sdk-declarations.d.ts` — reproducible excerpts of the pinned `sdk.d.ts`
  declarations for the streaming `query()` input, `SDKUserMessage`,
  `AccountInfo`, the initialize-control methods on `Query`, `SpawnedProcess`,
  and `SpawnOptions`, followed by the Card 116 extension with exact 0.3.270
  line ranges. The CanUseTool region gains only the two optional ask-dialog
  hints; every MCP shape is byte-identical to the 0.3.259 excerpt.
- `mcp-protocol.json` — the MCP protocol-version constants the pinned bundle
  carries, recovered from `package/sdk.mjs` by literal string search. The
  version list is identical to 0.3.259; that equality is not live evidence.
- `protocol.json` — the selected mapped subset, every classified declaration
  delta with why it stays unmapped, the unchanged implementation invariants,
  the credential non-custody re-verification, and the registered-tool
  disposition: Research 301 stays bound to `0.3.259`/`2.1.259`.
