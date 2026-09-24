# 336 Consumer-Supplied HTTP MCP Acceptance Per Route

Status: complete; evidence and matrix truth; no live/provider claim
Owner: Tom
Date: 2026-09-22
Lane: g06.014

## Question

For every route that documents an MCP client configuration seam, does the
route accept a **consumer-supplied streamable-HTTP MCP server entry** —
loopback URL, per-instance bearer header, origin-constrained — honoured
rather than ignored or rejected? The answer decides which routes connect
directly to the consumer's production MCP (Longhorn Contract 022
`agent-control`) and which need Longhorn's stdio carrier.

## Route List (decision for Longhorn)

**`direct-http`: none.** No route is proven to accept a consumer-supplied
streamable-HTTP MCP entry.

**`carrier-required`** — the route honours a consumer-supplied **stdio**
entry and therefore can reach the production MCP only through Longhorn's
stdio carrier. These two rows are the carrier's named dependents:

- `claude-agent.sdk` — consumer-declared stdio servers only; SSE/HTTP not
  representable (evidence §3).
- `grok-build.catalogue + grok-build.acp` — ACP `mcpServers` entry proven
  only for the Swallowtail-owned stdio courier on exact `1.0.4`/`1.0.5`;
  no URL/header shape exists on that wire (evidence §4).

**`provider-limitation`** — 33 rows: no consumer seam is exposed, or the
seam is pinned empty/closed, so HTTP acceptance is unproven and unsettleable
provider-free (evidence §5–§7).

**`producer-gap (carrier candidacy open, not carrier dependents)`** — six ACP
rows whose `client_mcp_servers` cell is already a producer gap owned by
[g06.005](../roadmaps/g06/005-registered-tool-adoption-remaining-acp-routes.md)
(planned; no dispatch authorization): `cline.acp`, `copilot-cli.acp`,
`gemini-cli.acp + gemini-cli.headless`, `goose.acp`, `kiro.acp`,
`deepagents.acp`. The shared kernel exists but no route-local adoption is
built and no live capsule admits any entry on those providers, so HTTP
acceptance is equally unproven there. Longhorn must not size the carrier
for them until a promoted g06.005 route plus its live gate moves one to
`carrier-required`.

The two Swallowtail-owned stdio couriers (Card 084 consumer-declared path on
`claude-agent.sdk`; Card 116/128 mediated-stdio proxy on both rows above)
are non-production development surfaces. They prove stdio admission, never
HTTP acceptance, and are classified separately from the consumer seam.

## Method And Boundary

Frozen provider artifacts, published documentation, CLI help, and committed
Swallowtail source only. No provider prompt, login, install, host update, or
live session; provider artifacts hashed, never executed. No connector,
carrier, client, listener, registry, or lease implemented. No route admission
changed; Contracts 060/063 semantics unchanged; no Longhorn code touched.

The production boundary is settled input, not a finding of this lane:
Contract 063 (Production MCP Boundary, 2026-09-22) names Longhorn's Contract
022 `agent-control` server — loopback, per-instance bearer, stateless
streamable HTTP — as the sole production MCP, and narrows Swallowtail's
production role to harness-side MCP client configuration. Spec 014 records
the same withdrawal.

**Scope of every classification below.** Each route is classified by what
*Swallowtail's admitted seam* can carry, established from Swallowtail's frozen
artifacts. None of these classifications describes what a harness can do when a
consumer drives it directly, outside Swallowtail: a consumer that configures the
harness itself is not bound by an adapter's pin. `claude-code` is the concrete
case — the CLI flag shape exists and Swallowtail pins it empty, and this record
states plainly that what the CLI would honour is unsettleable from our
artifacts.

So `carrier-required` means "this *Swallowtail route* cannot reach the
production MCP without Longhorn's stdio carrier". It never means "this harness
cannot", and it is not a dependency map for a consumer that drops Swallowtail
and drives a harness directly. Such a consumer must establish its own harness's
acceptance independently; its harness may accept a streamable-HTTP entry that
this record cannot settle.

## Seam Inventory

Sources: Contracts 012, 017, 041, 063; every prepared guide; each route's
frozen artifacts and adapter source. Contract 012 carries no MCP seam.
Contract 017 carries no MCP seam. Contract 041 names MCP only to exclude it
as a portable callback path (closed Contract 060 watcher bridge excepted).
Contract 063 admits exactly three MCP placements: the closed
`swallowtail-watchers` watcher family, the `claude-agent.sdk` mediated-stdio
carrier, and the `grok-build.acp` ACP client-MCP courier — all stdio-shaped,
all Swallowtail-owned.

| Route(s) | Documented seam shape | Frozen shape evidence |
| --- | --- | --- |
| `claude-agent.sdk` | consumer-declared stdio servers on `ClaudeAgentSdkMcpBinding` (`with_mcp_servers`); registered-tool courier as reserved internal stdio entry | `crates/swallowtail-adapter-claude-agent/src/sdk/mcp.rs:1-11` (SSE/HTTP/in-process not representable); sole constructor `ClaudeAgentSdkMcpServer::stdio`; guide Client MCP Servers section |
| `grok-build.acp` | ACP `session/new` `mcpServers` list; one reserved courier entry on exact `1.0.4`/`1.0.5` | `crates/swallowtail-adapter-grok/src/registered_tool/declaration.rs:1-10` plus `to_acp_value` (`name`/`command`/`args`/`env` only); `driver.rs:282`; guide Consumer Registered Tools section |
| `codex.app-server` | none per session/thread; untyped ambient `config` overlay only | Research 291 frozen corpus; Contract 063 Card 117 path |
| `claude-code.headless; claude-code.response-only` | provider flag pinned empty: `--mcp-config {"mcpServers":{}}` + `--strict-mcp-config`; closed watcher profile separate | guide argv blocks; `ProviderSuppressed` posture; Contract 063 watcher-only row |
| `claude-agent.acp` | `session/new` with `mcpServers: []`; no MCP input facade | `crates/swallowtail-adapter-claude-agent/src/connection.rs:132-135`; Contract 063 host-mediated-only row |
| `cline.acp`, `cursor-agent` (ACP part), `copilot-cli.acp`, `deepagents.acp`, `gemini-cli.acp`, `goose.acp`, `kimi-code.acp`, `kiro.acp` | `session/new` with fixed `mcpServers: []`; no consumer builder | adapter drivers send the empty literal (`cursor/driver.rs:200`, `gemini/driver.rs:238`, `goose/driver.rs:228`, `copilot-cli/driver.rs:228`, `kiro/driver.rs:228`, `kimi/connection/attachment.rs:43`); guides document the fixed empty value |
| all remaining rows | no MCP client surface in guide, driver, or frozen artifacts | Research 281 ledger via the 290 TSV (one `client_mcp_servers` `provider_limitation` row per route) |

## Per-Route Classification

`direct-http`: (none).

`carrier-required`:

| Route | Accepted today | HTTP finding |
| --- | --- | --- |
| `claude-agent.sdk` | consumer-declared **stdio** servers (Card 084 merged); registered-tool courier qualified on the exact Card 318 live tuple (Research 301; rebound tuple unqualified per Research 315/329) | the committed type has no URL/header constructor; SSE/HTTP configs are documented not-representable; omission sends `mcpServers: []` |
| `grok-build.catalogue + grok-build.acp` | client-declared server admitted, listed, and called on exact `1.0.4` and `1.0.5` (Card 128 accepted return, Research 295; Card 143 qualification) | the ACP declaration object carries only `name`/`command`/`args`/`env`; no endpoint, bearer, or generation travels that wire; other versions refuse with `version_not_admitted` |

`provider-limitation` (HTTP acceptance unproven; existing cell and basis
stand): `qwen.headless`, `alibaba.conversations`,
`bedrock.catalogue; bedrock.runtime`, `claude-agent.acp`,
`claude-code.headless; claude-code.response-only`, `anthropic.managed-agent`,
`anthropic.messages`, `pi.rpc`, `pi.sdk-sidecar`,
`cline.headless`, `command-code.headless`,
`cursor-agent.catalogue + cursor-agent.acp + cursor-agent.headless`,
`deepseek-harness.jsonrpc`, `deepseek-harness.local-server`,
`deepseek.continuation`,
`antigravity.catalogue + antigravity.headless`,
`gemini.live`,
`llama-cpp.attached`, `llama-cpp.owned`,
`muse-code.headless`, `mistral-vibe.headless`,
`kimi-code.acp + kimi-code.headless`, `kimi-code.local-server`,
`kimi-platform.chat`, `oh-my-pi.rpc`, `ollama.attached`,
`codex.app-server; codex.exec`, `openai.realtime`, `openai.background`,
`opencode.http`, `qoder.headless`, `xai.responses-websocket`,
`zcode.app-server` — 33 rows.

`producer-gap` (HTTP equally unproven; stdio-carrier candidacy open but
unbuilt; cells already name g06.005 and are unchanged): `cline.acp`,
`copilot-cli.acp`, `gemini-cli.acp + gemini-cli.headless`, `goose.acp`,
`kiro.acp`, `deepagents.acp` — 6 rows. Per g06.005, promotion is per route
and requires a consumer requirement plus operator direction; neither exists,
so none is a carrier dependent in this pass.

Notes on the non-obvious rows:

- `codex.app-server`: Research 291 proves the qualified range has no typed
  per-session client-declared MCP surface; the ambient `config` overlay
  carries no per-session identity, revision, digest, or lifecycle, so a
  stdio carrier entry is equally undeclareable per session. Provider
  limitation, not carrier-required.
- `claude-code.*`: the CLI flag shape exists but Swallowtail pins it empty
  and projects `ProviderSuppressed`; what the CLI would do with a
  consumer-supplied HTTP entry is unsettleable from frozen Swallowtail
  artifacts. Provider limitation.
- ACP routes sending `mcpServers: []`: the wire field exists in the
  protocol, but Swallowtail exposes no consumer builder and no frozen
  capsule admits any entry (stdio or HTTP) on those routes. The six rows
  named above stay `producer_gap` with g06.005 as the reference; the rest
  stay `provider_limitation`. No live packet exists, so `evidence_pending`
  is unavailable by the matrix rule.
- `opencode.http`: the attached-server route exposes no MCP configuration
  surface in the adapter or guide. Provider limitation.

## Deliberately Unclaimed: Subscriptions And Resources

`subscriptions`/`listen` and the `longhorn://agent-control/...` resources
are recorded as typed unsupported in this pass. No named route needs them:
no route accepts even the base HTTP entry, so no route reaches the question
of subscribing to it. Reopen condition: when a route is proven
`direct-http` (or a carrier-mediated equivalent is qualified) and its
consumer flow names a subscription or resource dependency, reclassify that
dependency with its own frozen evidence. Longhorn is so notified through the
route list above: nothing in this pass exercises those surfaces.

## Matrix Reconciliation

All 33 `No`/`provider_limitation` cells already project with anchored ledger
bases and agree with this classification; their cross-references are
unchanged. The six `producer_gap` cells already name g06.005 and agree with
the carrier-candidacy finding; unchanged. The two `Yes` cells carry no cross
entries by rule (available cells need none) and stay `Yes`: the stdio seams
are real. Their `notes`
each gain one appended sentence stating the HTTP boundary so no reader
infers HTTP capability from the `Yes`:

- `claude-agent.sdk`: consumer-declared servers are stdio-only; SSE/HTTP
  entries are not representable, so the production streamable-HTTP MCP
  needs Longhorn's stdio carrier on this route.
- `grok-build.catalogue + grok-build.acp`: the admitted ACP entry is a
  stdio command/args/env declaration on exact `1.0.4`/`1.0.5`; it carries
  no URL or bearer, so the production streamable-HTTP MCP needs Longhorn's
  stdio carrier on this route.

## Guide Reconciliation

Checked every prepared guide for MCP transport statements. No guide claims
HTTP acceptance: the SDK guide withholds SSE/HTTP explicitly; the Grok guide
bounds the courier to the reserved stdio entry with `mcpServers: []`
omission; ACP guides document the fixed empty list; the Claude Code guide
pins `--mcp-config {"mcpServers":{}}` with `ProviderSuppressed`; remaining
guides are silent. No guide edit required; none made.

## Typed Gaps

- Whether any non-g06.005 ACP route sending `mcpServers: []` would honour a
  stdio entry (carrier candidacy) or an HTTP entry is unsettleable
  provider-free.
  Reopen: separately authorized live gate per named route; until then the
  `provider_limitation` cells stand (no live packet exists, so
  `evidence_pending` is unavailable by the matrix rule). The six g06.005
  rows reopen through that task's own promotion gate instead.
- Whether the Claude Code CLI would honour a consumer-supplied HTTP entry
  in `--mcp-config` is unsettleable from Swallowtail's frozen artifacts,
  which pin the field empty. Same reopen path.
- No stop condition fired: no route required changing its admission or a
  contract's semantics to classify, and no two routes disagree on one
  mechanism (each negative rests on its own frozen shape).

## Non-Claims

No runtime, connector, carrier, client, listener, registry, or lease added
or modified. No claim that any route connects to the production MCP today.
No claim about `subscriptions`/`listen` or `longhorn://` resources beyond
typed-unsupported. Frozen artifacts hashed, never executed; no provider
session opened.

## Addendum 2026-09-24 (g06.031 / Research 351)

Research 336 classified Swallowtail's admitted seam. Research 351 classifies
the **provider** from hashed artifacts for the eight g06.031 routes and
supersedes this record's rows for those routes only: five
`direct-http-candidate`, two `provider-limitation` (`cline.acp`,
`deepagents.acp`), `grok-build.acp` still `carrier-required`. Other 336 rows
stand. No available cell.
