# Kimi Code 0.38.0 currentness corpus

This secret-free identity corpus freezes official npm/GitHub `0.38.0`
before Swallowtail widened the three `kimi-code.executable` route claims.
Host `kimi` was not installed.

Exact source, npm, linux-x64, and darwin-arm64 identities live in
`identity.json`. Selected ACP event-map, ACP server, ACP auth-methods,
stream-JSON renderer, CLI options, run-prompt v1-branch blobs, bearer
middleware, model catalogue protocol, and WebSocket control blobs stay
byte-identical to `0.37.2`. Those v1 headless blobs are frozen as historical
and legacy-comparison corpus only.

At `0.38.0`, naked npm `kimi -p` defaults to agent-core-v2 `runV2Print`,
not the legacy v1 print body, unless `KIMI_CODE_LEGACY_FLAG` is truthy.
Swallowtail headless `0.38.0` is not qualified: the qualified ceiling stays
`0.37.2` under `kimi.headless.stream-json.v1` until agent-core-v2
stream-json is independently qualified under an exact revision. ACP and
local-server `0.38.0` qualifications stand.

Changelog extras stay unmapped: `acp --region` / `login --region` on the
already-unmapped login flow, the WaitFor agent tool, advertised ACP
close/delete, `acp --login`, terminal-auth metadata, and watch-fs
`runtime_id`. Local-server `modelCatalog.ts` only refactors existing
collection actions. Application `ping`/`pong` remains. Swallowtail maps
no new public operation.

Prompt-free ACP initialize on the extracted `0.38.0` linux-x64 binary
returned protocol v1, auth method `login`, the same capability keys as
`0.37.2`, and no stderr. The initialize `terminal-auth` command path was
discarded. The host executable was not installed or replaced. The frozen
`0.37.2` corpus stays. Python `kimi-cli` `1.49.0` and Kimi Platform Chat
stay separate.

No fixture contains a credential, bearer token, host path, account
identity, provider payload, real session id, or model observation.
