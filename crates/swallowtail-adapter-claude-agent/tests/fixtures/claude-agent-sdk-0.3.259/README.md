# claude-agent-sdk 0.3.259 identity

Frozen evidence for rebinding the exact `claude-agent.sdk` package and native
axes from `0.3.258` to official npm `0.3.259`. Secret-free: no credential, host
path, account id, payload, or conversation id appears here.

Both tarballs were downloaded to `/tmp`, hashed, and extracted. Nothing was
executed, no platform package was fetched, no host was changed, and no provider
session, login, or token read occurred. The native binaries are identified from
the shipped `manifest.json` digests rather than by downloading 200 MB artifacts.

- `identity.json` — official and previous-ceiling artifact identity, coupled
  native identity, and the rebind decision. The `0.3.258` digests corroborate
  Research 278 exactly.
- `dist-inventory.json` — deterministic 15-file package-tree inventory across
  the hop: 7 identical, 8 changed, 0 added, 0 removed, with per-file digests.
- `sdk-declarations.d.ts` — reproducible excerpts of the pinned `sdk.d.ts`
  declarations for `AccountInfo`, the initialize-control methods on `Query`,
  `SpawnedProcess`, and `SpawnOptions`.
- `protocol.json` — the selected mapped subset, every classified declaration
  delta with why it stays unmapped, the unchanged implementation invariants,
  and the credential non-custody re-verification.
