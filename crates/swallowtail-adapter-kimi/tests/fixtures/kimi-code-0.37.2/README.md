# Kimi Code 0.37.2 currentness corpus

This secret-free identity corpus freezes host Kimi Code `0.34.0` and official
npm/GitHub `0.37.2` before Swallowtail widens the three `kimi-code.executable`
route claims.

Exact source, npm, and darwin-arm64 identities live in `identity.json`.
Selected ACP event-map, ACP server, stream-JSON renderer, CLI options, and
run-prompt blobs stay byte-identical to `0.36.1`.

Local-server selected REST/WebSocket source at `0.37.0` strips comments and
adds unused optional `runtime_id` on watch-fs payloads. Application
`ping`/`pong` remains. Swallowtail maps no new public operation. Initialize
may advertise session close and delete; those stay unmapped.

Prompt-free ACP initialize on the extracted `0.37.2` darwin-arm64 binary
returned protocol v1, auth method `login`, and no stderr. The initialize
`terminal-auth` command path was discarded. The installed host executable
was not replaced. The frozen `0.36.1` corpus stays.

## Errata (2026-09-01, Research 270)

This corpus originally read `experimental_v2_selected: false` as meaning the
default `kimi -p` path at `0.37.2` is agent-core v1. That is wrong.
`KIMI_CODE_EXPERIMENTAL_FLAG` stopped selecting the engine at `0.33.0`; from
`0.33.0` the default `-p` path is agent-core-v2 `runV2Print` unless
`KIMI_CODE_LEGACY_FLAG` is truthy, and Swallowtail never sets that flag. The
default engine at `0.37.2` is therefore v2, and `0.37.2` is no longer a
qualified `kimi.headless.stream-json.v1` point. See
`kimi-code-0.33.0-headless-routing`.

The ACP statements in this corpus stand, but the named implementation does
not: from `0.33.0` a naked `kimi acp` runs `packages/acp-server`, not
`packages/acp-adapter`. The prompt-free initialize observation was taken on
that native path and remains valid.

No fixture contains a credential, bearer token, host path, account identity,
provider payload, real session id, or model observation.
