# 362 Gemini CLI ACP HTTP MCP Live Honouring

Status: accepted honouring evidence; exact host `0.61.0` point only
Owner: Swallowtail worker
Created: 2026-09-26
Task: swallowtail#075; planning `44ce84257372165def7f2e0c8b26ff5d233645f4`
Contracts: 063
Ruling: Tom 2026-09-26 — provider-free mode diagnosis, then one live
attempt on the free Gemini key if the fake proof holds

## Question

Does `gemini-cli.acp` on host exact Gemini CLI `0.61.0` complete one
consumer-supplied HTTP MCP tool call, and is Research 360's
`tool_not_called` stop Plan-mode policy or provider behaviour that no
route-selectable mode can avoid?

## Answer

Yes on the first; Plan-mode policy on the second. Frozen `0.61.0`
`packages/core/src/policy/policies/plan.toml` catch-all DENY at priority
40 excludes unannotated MCP tools from the registry
(`getExcludedTools`) and denies them at the policy engine. MCP tools
with `readOnlyHint: true` stay visible as `ASK_USER`. The Research 360
disposable `ping` tool advertised no annotations, so Plan hid it. The
one turn never called it.

`--approval-mode default` (and `auto_edit`) keep unannotated MCP tools
visible. The policy default is `ASK_USER`. ACP
`session/request_permission` offers `optionId` `proceed_once` (`kind`
`allow_once`). `auto_edit` auto-allows `write_file` / `replace` /
`web_fetch` only; MCP still asks. `yolo` auto-allows MCP and is not a
route-selected mode.

The narrowest route-selectable path that can complete the call is
`--approval-mode default` with the gate answering `proceed_once`. The
read-only Plan profile and the default reject-and-cancel permission
path stay. `permission_exchange` stays No: this is gate-owned
auto-allow, not a consumer callback.

The committed harness proof passed first: default plus `proceed_once`
honours the tuple against the fake; Plan records typed `tool_not_called`;
default without allow-once records typed `permission_observed`. Host
`gemini --version` was exact `0.61.0`. No host update, login, auth
change, or second live attempt.

The one authorized live attempt accepted the honouring tuple. Model
recorded: `auto`. Honouring is the provider's client behaviour on the
declared HTTP entry; the model string is provenance.

## Observed gate

Live record stderr (no raw stream retained):

- accepted: true
- typed stop: none
- model: `auto`
- terminal diagnostic: none
- cleanup diagnostic: none

Server-side transcript plus runtime terminal:

- production `session/new` declared the route-owned HTTP entry
- authenticated MCP `initialize` reached the disposable server
- `tools/list` listed the deterministic tool
- `tools/call` ran and returned the deterministic result
- the turn ended `Completed`
- session cleanup was `Clean`

The disposable listener is test-only, behind `live-probes` for the live
binary and default-feature for the harness proof. It is not a production
path.

No raw provider stream, bearer, account identifier, session id, or
private path is retained.

## Limits

The record covers host exact `0.61.0` only. Other window points stay
unqualified for honouring. The attempt is spent; do not rerun it. Plan
mode still excludes unannotated MCP tools. Headless keeps MCP disabled.

## Disposition

`gemini-cli.acp` `client_mcp_servers` is `Yes` on exact `0.61.0` only.
Research 351 emission, Research 358 identity, and Research 360's spent
Plan-mode stop stand. Research 349 OpenCode honouring and Research 352
Claude stop are unchanged.

## Next move

Any version other than exact `0.61.0` needs its own live gate.
