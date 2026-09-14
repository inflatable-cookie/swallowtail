# Kiro ACP 2.21.4 identity corpus

Secret-free artifact and executable identity for `kiro.acp` at official
stable-manifest `2.21.4` (g05.072 useful-newer qualification).

The exact `2.18.1` baseline plus all eleven published stable successors
through `2.21.4` were retrieved as exact official archives into `/tmp`,
verified against manifest digests, and statically inspected (extracted or
read-only mounted) without executing a downloaded artifact, sending an ACP
message, logging in, installing, or touching host Kiro state.

- `identity.json` — official manifest/installer/docs/changelog identity, the
  complete successor ledger, and the compatible-extension decision.
- `artifact-ledger.json` — per-release archive digests, BUILD-INFO build
  identities, per-executable SHA-256, and the read-only-mounted DMG facts.
- `surface-ledger.json` — the mapped selected-surface closure: ACP library
  pins, selected-surface literal counts per hop, per-hop classifications,
  and bounded unmapped deltas with non-effect reasons.
- `protocol.json` — the selected-surface classification at `2.21.4`: the
  unchanged selected method set, unmapped additions, permission kinds, stop
  reasons, and the unchanged decoder corpus.

The mutation-sensitive assertions live in
`tests/kiro_2_21_4_delta_ledger.rs`. The production claim moves from exact
`2.18.1` to exact `2.21.4` under the unchanged `kiro.acp.stdio-v1` behavior
revision and `QualifiedOnly` posture.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
