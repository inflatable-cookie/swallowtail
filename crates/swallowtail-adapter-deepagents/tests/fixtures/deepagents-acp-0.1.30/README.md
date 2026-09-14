# Deep Agents ACP 0.1.30 identity and surface ledger

Secret-free source identity for the g05.074 exact-pin qualification of
`deepagents.acp` (Research 308 family fourteen, Research 322).

Official npm `deepagents-acp@0.1.25..=0.1.30` tarballs plus the exact-pinned
`deepagents@1.12.4..=1.13.4` runtime dependencies were retrieved into `/tmp`,
hashed against registry digests, and statically inspected. The `0.1.25`
baseline reproduces Research 157 and Research 206 byte-for-byte. The selected
no-extra-argv stdio wire is byte-stable across the whole window: the only CLI
body change (`0.1.27..0.1.28`) rewrites `ACPFilesystemBackend.read`, which the
selected route never constructs because the CLI passes an explicit
`FilesystemBackend` config backend before any client-capability branch.

The selected runtime dependency `deepagents` moves 1.12.4 through 1.13.4; every
hop is classified in `surface-ledger.json`. Additive selected deltas are
decoder-invisible: a builtin `delete` tool flowing through the unchanged
generic tool/permission paths, and read_file pagination footer text inside
opaque tool-result content.

No live ACP initialize. No provider prompt. No install. No host
`deepagents-acp`. Library embed, custom `tsx`, `npx`, and advertised
`session/load` are not this corpus. The stale ACP registry entry (`0.1.7`)
and the CLI constructor-default `agentInfo.version` (`0.0.1`) are never
compared to the npm package axis.

No fixture contains a credential, host path, account identity, provider
payload, or real session id. The wire fixtures under
`../deepagents-acp-0.1.25/` remain the decoder corpus: their surfaces are
byte-stable through `0.1.30`.
