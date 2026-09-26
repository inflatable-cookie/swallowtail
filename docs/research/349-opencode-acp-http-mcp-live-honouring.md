# 349 OpenCode ACP HTTP MCP Live Honouring

Status: accepted honouring evidence; exact `1.18.18` point only; host default
agent model unverified
Owner: Swallowtail worker
Created: 2026-09-24
Task: g06.028; planning `9709bc23f17e7ae38318c15c00fcec77f9edb2ce`
Disposition: `docs/logs/2026-09-24-g06-028-live-gate-disposition.md` at
`78cc35db0c9e0780c9492adbc9394dc3d777b887`
Contracts: 063

## Question

Does installed exact `opencode.acp` `1.18.18` honour a consumer-supplied
streamable-HTTP MCP entry: connect, list one tool, and complete one tool call?

## Answer

Yes. The planning ruling at
the retired logs (Git history)
accepts the one spent attempt as HTTP honouring evidence on installed exact
`1.18.18`, the `http` form, a loopback URL, and one bearer header. OpenCode
connected, listed the tool, completed one tool call, and the turn ended
`Completed` with `Clean` cleanup.

The card's exact-model requirement was provenance, not a property of the
claim. Honouring is OpenCode's client behaviour: it opens the declared server
and routes the call whichever model drives the turn. The model is recorded as
the host default agent model, unverified (configured `kimi-for-coding/k3`
present, not proven effective). The claim does not depend on it.

The live attempt was not rerun. The gate now resolves the selected default
agent (`default_agent`, else `build`) model's override, else the root
`model`, requires that definition plus existing host auth, records that
string only, and stops with `no_usable_model` when those are missing. If a
future ACP session advertises a current model, it must match that resolved
value before the prompt.

The committed harness proof
(`http_mcp_live_harness` against a fake ACP agent and the disposable
loopback MCP server) passed first. Host `opencode --version` was exact
`1.18.18`. No install, update, login, rerun, or model swap.

Number 349 avoids collision with Research 348 (Claude Code `2.1.281` narrowed
response-only claim) and Research 347 (Command Code `1.65.0` live acceptance).

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

ACP on this route does not negotiate a model. The host default agent model is
unverified; the honouring claim does not depend on it. Future live records
use the agent-override-aware host default, not an ACP `current_value`.
Honouring of the HTTP MCP entry on exact `1.18.18` stands.

## Disposition

`opencode.acp` `client_mcp_servers` is `Yes` on exact `1.18.18` only. Later
points in the qualified window stay unqualified for honouring. stdio MCP live
honouring and `sse` emission are not this evidence. Research 337 identity and
g06.019 emission stand.

## Next move

Chatterbox reconciles the cell against Longhorn `direct-http`. Any version
other than exact `1.18.18` needs its own live gate.
