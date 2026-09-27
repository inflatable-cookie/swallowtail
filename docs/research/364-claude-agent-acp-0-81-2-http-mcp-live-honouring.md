# 364 Claude Agent ACP 0.81.2 HTTP MCP Live Honouring

Status: accepted honouring evidence; exact `0.81.2` point
Owner: Swallowtail worker
Created: 2026-09-27
Task: swallowtail#076; planning `af782cfaccd0692fe44d9807a8cb82b4408bd745`
Contracts: 063
Ruling: Tom authorized exactly one additional live attempt on 2026-09-27
under the amended Queue brief, after the first attempt record was lost.

## Question

Does `claude-agent.acp` on exact `claude-agent-acp` `0.81.2` honour a
consumer-supplied streamable-HTTP MCP entry end to end: connect, list one
tool, complete one tool call, and end the turn `Completed` with `Clean`
cleanup?

## Answer

Yes. The single additional authorized attempt accepted the full honouring
tuple with `Clean` cleanup. The saved gate record reports connection, tool
listing, a completed tool call with result, terminal status `completed` and
no terminal code, and cleanup class `clean` with no cleanup code or stage.
The reported model was `claude-sonnet-4-6`; model is provenance.

Before the live attempt, all ten `http_mcp_live_harness` tests passed against
the fake agent on the committed gate implementation. They prove that the
typed record is persisted before it is printed, persistence failure is
fail-closed, and the record includes the terminal and cleanup diagnostics.
The live gate required the record file and ran with `--nocapture`. The repo-
local package pin was exact `0.81.2`. No host update, login, auth or pin
change occurred. The existing thirty-second cleanup deadline and production
cleanup path were unchanged.

## Observed gate

The persisted typed record (also printed by the gate) contains:

- accepted: true
- typed stop: none
- model: `claude-sonnet-4-6`
- MCP connected: true
- tool listed: true
- tool called: true
- tool result: true
- terminal status: `completed`; terminal code: none
- cleanup class: `clean`; cleanup code: none; cleanup stage: none

No raw provider stream, bearer, account identifier, session id, or private
path is retained.

## Relation to the first 0.81.2 attempt

The first `0.81.2` attempt on 2026-09-27 is spent. Its output record was
lost because cargo captured the gate's `eprintln`; its typed result is not
reconstructed here. This record reflects only the one additional attempt
authorized by the amended brief. No further attempt is permitted by this
authorization.

Research 361 remains accepted prior evidence on exact `0.79.0`; Research
355's cleanup diagnosis and Research 363's identity comparison stand.

## Disposition

`claude-agent.acp` `client_mcp_servers` is `Yes` on exact `0.79.0` and
`0.81.2`. Other points in the window stay unqualified. Research 351
emission stands; `sse` remains modelled and unemitted.

## Next move

A different exact version point needs its own authorization and live gate.
