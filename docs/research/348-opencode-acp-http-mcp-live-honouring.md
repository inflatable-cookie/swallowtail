# 348 OpenCode ACP HTTP MCP Live Honouring

Status: HTTP honouring observed; exact-model attribution unverified; card
accepted-run not met
Owner: Swallowtail worker
Created: 2026-09-24
Task: g06.028; planning `9709bc23f17e7ae38318c15c00fcec77f9edb2ce`
Contracts: 063

## Question

Does installed exact `opencode.acp` `1.18.18` honour a consumer-supplied
streamable-HTTP MCP entry: connect, list one tool, and complete one tool call?

## Answer

HTTP honouring was observed. One authorized attempt on the `http` form, a
loopback URL, and one bearer header connected, listed the tool, completed one
tool call, and the turn ended `Completed` with `Clean` cleanup.

That observation is not the card's accepted-run outcome. g06.028 requires the
cheapest already-usable host model and its exact model string. The spent run
printed `kimi-for-coding/k3` from a definition-exists check. That is not
verified exact-model attribution: this route does not send a model on
`session/new` or `session/prompt`, `negotiated_model_options` is none, and a
selected default agent can override the root `model`. The authorized attempt
has no verified exact model string.

The live attempt was not rerun. The gate now resolves the selected default
agent (`default_agent`, else `build`) model's override, else the root
`model`, requires that definition plus existing host auth, records that
string only, and stops with `no_usable_model` when those are missing. If a
future ACP session advertises a current model, it must match that resolved
value before the prompt. That resolver cannot establish which model the
spent attempt used.

The committed harness proof
(`http_mcp_live_harness` against a fake ACP agent and the disposable
loopback MCP server) passed first. Host `opencode --version` was exact
`1.18.18`. No install, update, login, rerun, or model swap.

Number 348 avoids collision with Research 347 (Command Code `1.65.0` live
acceptance on `main`).

## Observed gate

Server-side transcript plus runtime terminal:

- production `session/new` declared the route-owned HTTP entry
- authenticated MCP `initialize` reached the disposable server
- `tools/list` listed the deterministic tool
- `tools/call` ran and returned the deterministic result
- the turn ended `Completed`
- session cleanup was `Clean`

Typed stop was none on the MCP honouring path. The card's exact-model
condition is still unfulfilled, so this is not an accepted tuple. The
disposable listener is test-only, behind `live-probes` for the live binary
and default-feature for the harness proof. It is not a production path.

No raw provider stream, bearer, account identifier, session id, or private
path is retained.

## Limits

ACP on this route does not negotiate a model. The spent authorized attempt
has no verified exact model attribution. HTTP honouring on exact `1.18.18`
is an observation, not settlement of `client_mcp_servers`. Future live
records use the agent-override-aware host default, not an ACP
`current_value`.

## Disposition

`opencode.acp` `client_mcp_servers` stays `No`. The spent attempt does not
satisfy the card's accepted-run outcome. Planning authority owns disposition
for this exhausted attempt. Later points in the qualified window stay
unqualified for honouring. stdio MCP live honouring and `sse` emission are
not this evidence. Research 337 identity and g06.019 emission stand.

## Next move

Chatterbox or a new operator authority disposes the exhausted attempt.
Do not rerun the live gate.
