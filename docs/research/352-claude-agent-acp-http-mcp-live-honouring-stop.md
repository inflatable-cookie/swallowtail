# 352 Claude Agent ACP HTTP MCP Live Honouring Stop

Status: typed stop; honouring not accepted; exact `0.79.0` point only
Owner: Swallowtail worker
Created: 2026-09-26
Task: swallowtail#064; planning `fa7caf45f712cd65d45d99850cfa795b47038651`
Contracts: 063

## Question

Does `claude-agent.acp` on exact `claude-agent-acp` `0.79.0` honour a
consumer-supplied streamable-HTTP MCP entry: connect, list one tool, complete
one tool call, and end the turn `Completed` with `Clean` cleanup?

## Answer

No. The honouring tuple was not accepted. The one authorized live attempt
stopped with typed `cleanup_failed`. That stop is assigned only after
authenticated MCP `initialize`, `tools/list`, `tools/call` returning the
deterministic result, and a `Completed` turn. Session cleanup was not
`Clean`.

The committed harness proof against a fake ACP agent and the disposable
loopback HTTP MCP server passed first. Repo-local
`@agentclientprotocol/claude-agent-acp` was pinned to exact `0.79.0`. Host
Homebrew `claude-agent-acp` was not updated. No login, auth change, or second
live attempt.

Model recorded: `claude-sonnet-4-6`. Honouring is the provider's client
behaviour on the declared HTTP entry; the model string is provenance.

## Observed gate

Live record stderr (no raw stream retained):

- accepted: false
- typed stop: `cleanup_failed`
- model: `claude-sonnet-4-6`

The disposable listener is test-only, behind `live-probes` for the live
binary and default-feature for the harness proof. It is not a production
path.

No raw provider stream, bearer, account identifier, session id, or private
path is retained.

## Limits

The record covers exact `0.79.0` only. Other window points stay unqualified
for honouring. The attempt is spent; do not rerun it.

## Disposition

`claude-agent.acp` `client_mcp_servers` stays No as a producer gap. Research
351 emission stands. Research 349 OpenCode honouring is unchanged.

## Next move

A separately authorized live gate would be required to re-test honouring,
including Clean cleanup. This task performs no second attempt.
