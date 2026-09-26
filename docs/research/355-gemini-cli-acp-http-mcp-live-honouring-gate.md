# 355 Gemini CLI ACP HTTP MCP Live Honouring Gate

Status: gate ready; waiting on usage; honouring not accepted
Owner: Swallowtail worker
Created: 2026-09-26
Task: swallowtail#068; planning `41989748fc2ab5f00e2dafac9ca4efe1279c2722`
Contracts: 063
Ruling: Tom 2026-09-26 — do not run a live Gemini attempt; no Gemini usage
available

## Question

Does `gemini-cli.acp` on exact Gemini CLI `0.59.0` honour a consumer-supplied
streamable-HTTP MCP entry: connect, list one tool, complete one tool call, and
end the turn `Completed` with `Clean` cleanup?

## Answer

Unproven. The opt-in gate is ready and waiting on usage. Honouring is not
accepted.

The committed harness proof against a fake ACP agent and a disposable loopback
HTTP MCP server passed first. The live binary stays ignored, behind
`live-probes` and `SWALLOWTAIL_LIVE_GEMINI_ACP_HTTP_MCP=1`. Isolated npm
`@google/gemini-cli@0.59.0` is the only allowed executable. Host Homebrew
`gemini` `0.53.0` is not updated. No login or auth change.

Tom 2026-09-26: do not run the live attempt; no Gemini usage is available. The
two cargo runs below already happened under the earlier brief. No third run.

## Observed cargo runs

First cargo invocation, before any ACP session:

- accepted: false
- typed stop: `host_version`
- model: none
- no terminal or cleanup diagnostic (pre-amendment record)

Cause: `LocalProcessHost` refuses a `#!/usr/bin/env` shebang unless launch is
`LocalExecutableLaunch::interpreted_script`. Isolated `0.59.0` was installed
and `--version` printed exact `0.59.0`. The gate then switched to interpreted
Node plus the canonicalized `gemini` script. This run never reached
`session/new` or the MCP listener.

Second cargo invocation, after that launch fix:

- accepted: false
- typed stop: `mcp_not_connected`
- model: none
- terminal diagnostic: `swallowtail.gemini.acp.mode_rejected` /
  `Unknown/Unknown/Unknown`
- cleanup diagnostic: `swallowtail.gemini.acp.mode_rejected` /
  `Unknown/Unknown/Unknown`

`session/new` reached the isolated `0.59.0` binary. Open failed because Gemini
did not return the preflight-bound access mode (`autoEdit` from
`GeminiSessionProfileInput::bounded_write`). The disposable MCP listener never
saw authenticated `initialize`. Host `~/.gemini` was present. No
`GEMINI_API_KEY` or `GOOGLE_API_KEY` in the environment. No raw stream,
bearer, account, session id, or private path is retained.

Whether that open billed Gemini usage is unknown. Do not treat the stop as a
finished honouring verdict.

## Limits

The record covers exact `0.59.0` only. Other window points stay unqualified
for honouring. The gate remains ready.

## Next move

A later authorized live attempt needs Gemini usage and a separate ruling. The
first open step on exact `0.59.0` is the observed
`swallowtail.gemini.acp.mode_rejected` failure: `bounded_write` expected
`autoEdit` and Gemini did not return that mode. Honouring cannot be proven
until that open succeeds.

## Disposition

`gemini-cli.acp` `client_mcp_servers` stays No as a producer gap. Research 351
emission stands. Research 349 OpenCode honouring and Research 352 Claude stop
are unchanged.
