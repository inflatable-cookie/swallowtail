# 357 Claude Agent ACP HTTP MCP Live Honouring, Second Attempt

Status: accepted honouring evidence; exact `0.79.0` point only
Owner: Swallowtail worker
Created: 2026-09-26
Task: swallowtail#073; planning `e754fe5312c2ccf2e05fa1d2f18a73ca9d43bf46`
Contracts: 063
Ruling: Tom approved a second live attempt on 2026-09-26 in the Swallowtail
planning thread, after the Research 355 diagnosis.

## Question

Does `claude-agent.acp` on exact `claude-agent-acp` `0.79.0` honour a
consumer-supplied streamable-HTTP MCP entry end to end: connect, list one
tool, complete one tool call, and end the turn `Completed` with `Clean`
cleanup, and if not, which typed cleanup code stopped it?

## Answer

Yes. The one authorized second attempt accepted the full honouring tuple
with `Clean` cleanup. Read against Research 355's code-to-cause table, the
observed diagnostic is the `Clean` row: cleanup class `clean`, no diagnostic
code, no stage. No other code fired, so no table row applies.

The committed harness proof (`http_mcp_live_harness`, 9 tests against a fake
ACP agent and the disposable loopback MCP server) passed first on the
current head. Repo-local `@agentclientprotocol/claude-agent-acp` was exact
`0.79.0` (installed from the lockfile; the pin is unchanged). The host
Homebrew install was not touched. No login, auth change, gate-deadline
change, or production cleanup-path change. Model recorded:
`claude-sonnet-4-6`. Honouring is the provider's client behaviour on the
declared HTTP entry; the model string is provenance.

## Observed gate

Live record stderr (no raw stream retained):

- accepted: true
- typed stop: none
- cleanup class: clean
- cleanup code: none
- cleanup stage: none
- model: `claude-sonnet-4-6`

The disposable listener is test-only, behind `live-probes` for the live
binary and default-feature for the harness proof. It is not a production
path.

No raw provider stream, bearer, account identifier, session id, or private
path is retained.

## Relation to the first attempt

Research 352's spent stop (`cleanup_failed` with no retained code) is not
reconstructed and stays spent. This attempt proves the sidecar answered
`session/close` inside the unchanged thirty-second cleanup boundary on the
same pinned `0.79.0` with the same gate. Whether the Research 355 listener
fix (idle SSE streams, persistent HTTP/1.1 connections) contributed to the
different outcome is not proven; the first attempt's exact code remains
unknown by construction.

## Limits

The record covers exact `0.79.0` only. Other window points stay unqualified
for honouring. This attempt is spent; do not rerun it.

## Disposition

`claude-agent.acp` `client_mcp_servers` is `Yes` on exact `0.79.0` only.
Research 351 emission stands. Research 349 OpenCode honouring is unchanged.

## Next move

Any version other than exact `0.79.0` needs its own live gate. stdio MCP
live honouring and `sse` emission are not this evidence.
