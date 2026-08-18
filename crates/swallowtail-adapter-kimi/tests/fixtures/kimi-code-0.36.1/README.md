# Kimi Code 0.36.1 currentness corpus

This secret-free identity corpus freezes host Kimi Code `0.34.0` and official
npm/GitHub `0.36.1` before Swallowtail widens the three `kimi-code.executable`
route claims.

Exact source, npm, and darwin-arm64 identities live in `identity.json`.
Selected ACP event-map, ACP server, stream-JSON renderer, CLI options, bearer
middleware, and model-catalogue protocol blobs stay byte-identical to
`0.31.1`. The ACP adapter package only bumps `0.3.6` through `0.3.9`. Default
headless stays off the experimental v2 runner.

Local-server selected REST/WebSocket source adds optional meta flags and extra
unknown events from `0.32.0`, then application `ping`/`pong` from `0.35.0`.
Swallowtail maps no new public operation. Initialize may advertise session
close and delete; those stay unmapped.

Prompt-free ACP initialize on host `0.34.0` and the extracted `0.36.1`
darwin-arm64 binary both returned protocol v1, auth method `login`, and no
stderr. The installed host executable was not replaced.

No fixture contains a credential, bearer token, host path, account identity,
provider payload, real session id, or model observation.
