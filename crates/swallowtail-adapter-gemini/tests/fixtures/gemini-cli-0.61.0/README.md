# Gemini CLI 0.61.0 currentness corpus

Secret-free identity corpus for official npm `@google/gemini-cli@0.59.0`,
`0.60.0` and `0.61.0`, their GitHub tags `v0.59.0` through `v0.61.0`, and each
darwin-arm64 unsigned release asset. Research 357 owns the reading.

The host auto-updated from `0.53.0` to `0.61.0`. Its npm `bundle/gemini.js` is
byte-identical to the official `0.61.0` npm bin entry. The host was read for
`--version` and its digest only, not changed.

Published stables after the `0.59.0` ceiling are exactly `0.60.0` and
`0.61.0`. The first unpublished later stable is `0.61.1`. `0.62.0-preview.0`
is a preview and is ignored.

Every npm tarball reproduces its registry `integrity` and `shasum`. Every
GitHub asset reproduces the release digest. The `0.59.0` values reproduce the
Research 324 corpus byte-for-byte. No downloaded artifact was executed.

`surface-ledger.json` holds one deterministic tagged-source file inventory per
point and the changed-path set per hop: 93 paths `0.59.0..0.60.0`, 89 paths
`0.60.0..0.61.0`. The method reproduces Research 324's 24-path
`0.58.0..0.59.0` ledger.

Both axes are compatible extensions:

- ACP keeps `gemini-cli.acp.v0.51.0` and raises its ceiling to `0.61.0`. The
  six selected ACP sources are byte-identical. The only ACP-directory change is
  `acpUtils.ts` model advertisement, which stays a provider-private negotiated
  observation.
- Headless keeps `gemini-cli.headless.stream-json.v1` and raises its ceiling
  to `0.61.0`. Every selected option, stream-json, terminal and retention
  source is byte-identical.
- The ACP `mcpServers` HTTP mapping (Research 351 shape) is unchanged:
  `acpRpcDispatcher.ts`, `acpSessionManager.ts` and `MCPServerConfig` are
  byte-identical. `mcp-client.ts` changes only stdio env filtering.

Unmapped: Flash rollout model routing on the API key path, build-file
protection and out-of-workspace shell ASK_USER (surfacing as the existing
rejected permission request), stdio MCP env filtering, MCP OAuth issuer check,
sandbox and storage-under-sandbox changes.

No fixture holds a credential, host path, account identity, provider payload,
prompt or session id.
