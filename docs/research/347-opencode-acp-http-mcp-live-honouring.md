# 347 OpenCode ACP HTTP MCP Live Honouring

Status: accepted live evidence; exact `1.18.18` point only
Owner: Swallowtail worker
Created: 2026-09-24
Task: g06.028; planning `9709bc23f17e7ae38318c15c00fcec77f9edb2ce`
Contracts: 063

## Question

Does installed exact `opencode.acp` `1.18.18` honour a consumer-supplied
streamable-HTTP MCP entry: connect, list one tool, and complete one tool call?

## Answer

Yes. One authorized attempt on the `http` form, a loopback URL, and one
bearer header accepted. Honouring (connect, list, call, completed turn) is
not in dispute.

The spent run printed model `kimi-for-coding/k3` because that gate treated a
provider definition as the tuple. This route does not send a model on
`session/new` or `session/prompt`, and `negotiated_model_options` is none, so
the host default is the ACP-effective model. A definition-only K3 cell does
not prove that default or that it was usable without a login.

The live attempt was not rerun. The gate now resolves `config.model` (or
`agent.model`), requires that exact definition plus existing host auth for
its provider, records that string only, and stops with `no_usable_model`
when those are missing. If a future ACP session advertises a current model,
it must match that resolved default before the prompt.

The committed harness proof
(`http_mcp_live_harness` against a fake ACP agent and the disposable
loopback MCP server) passed first. Host `opencode --version` was exact
`1.18.18`. No install, update, login, rerun, or model swap.

## Observed gate

Server-side transcript plus runtime terminal:

- production `session/new` declared the route-owned HTTP entry
- authenticated MCP `initialize` reached the disposable server
- `tools/list` listed the deterministic tool
- `tools/call` ran and returned the deterministic result
- the turn ended `Completed`
- session cleanup was `Clean`

Typed stop was none. The disposable listener is test-only, behind
`live-probes` for the live binary and default-feature for the harness proof.
It is not a production path.

No raw provider stream, bearer, account identifier, session id, or private
path is retained.

## Limits

ACP on this route does not negotiate a model. The recorded model is the
resolved host default, not an ACP `current_value`. The spent attempt's printed
`kimi-for-coding/k3` is not a verified ACP-effective default. Honouring of the
HTTP MCP entry on exact `1.18.18` still stands.

## Disposition

`opencode.acp` `client_mcp_servers` is `Yes` on exact `1.18.18` only.
Later points in the qualified window stay unqualified for honouring.
stdio MCP live honouring and `sse` emission are not this evidence.
Research 337 identity and g06.019 emission stand.

## Next move

Chatterbox reconciles the cell against Longhorn `direct-http`. Any version
other than exact `1.18.18` needs its own live gate.
