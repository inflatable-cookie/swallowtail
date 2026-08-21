# Pi SDK Sidecar Wire Corpus `pi-sdk-sidecar-v1`

Frozen corpus for the private sidecar wire `swallowtail-pi-sdk-jsonl-v1`
between the `swallowtail.pi.sdk-sidecar` driver and the source-tagged Node
sidecar asset `sidecar/pi-sdk-sidecar.mjs` in `swallowtail-adapter-pi`.

Identity, separate per Contract 019 and Contract 029:

- sidecar source tag: `swallowtail-pi-sdk-sidecar@<crate version>` (the crate
  packages this asset; the consuming application provisions the entry point)
- behavior revision: `pi.sdk-sidecar-v1`
- SDK package: exact `@earendil-works/pi-coding-agent@0.84.2`, public exports
  only, qualified-only one-point claim posture
- Node runtime: exact approved `22.23.2`, satisfying the upstream `>=22.19.0`
  requirement; observed through the bootstrap response, never through ambient
  discovery

Contents:

- `protocol.json` — wire identity, commands, events, replay item kinds,
  bounds, and suppressed ambient features
- `commands.jsonl` — outbound command corpus, one per qualified command
- `responses.jsonl` — success and failure response corpus
- `events.jsonl` — qualified turn event stream, including usage and a
  provider-failure `message_end`
- `replay.jsonl` — typed `replay_item` stream for session load
- `terminal.jsonl` — terminal failure record
- `diagnostics.jsonl` — redacted diagnostic records
- `unknown.jsonl` — unqualified event name; must fail closed
- `malformed.jsonl` — invalid JSON; must fail closed
- `disconnect.jsonl` — final record without LF terminator; must fail closed

Every record is synthetic. No credential, transcript, host path, or provider
payload appears in this corpus; `/app/...` paths are placeholders. The corpus
is executed by deterministic Rust tests and never resolves a package,
performs an install, or reaches a provider.
