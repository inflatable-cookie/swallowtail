# 291 Codex App-Server Client MCP Servers Evidence

Status: complete; provider limitation; no live/provider claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 / 131

## Question

Does the frozen Codex app-server protocol on the qualified CLI range expose a
request or configuration surface that lets a client declare MCP servers per
session or per thread, distinct from Codex configuration-owned MCP
(`config.toml`)?

## Decision

No. `codex.app-server` has no client-declared MCP server surface on the
qualified range. The `client_mcp_servers` matrix cell is a provider
limitation. No fake transcript. No binding card.

Card 117 native host-mediated registered tools remain the qualified
consumer-tool path. Provider-direct MCP stays withheld. Observed
`mcpToolCall` activity is provider-owned tool observation, not consumer
registration or result authority.

## Method And Boundary

Read only the already-frozen Codex app-server corpus in this repository.
No installed Codex. No live session. No credentials. No `config.toml`
mutation. No Desktop, tag, or release action.

Route: `codex.app-server`. Combined matrix row: `codex.app-server; codex.exec`.
Qualified CLI range: `0.80.0-0.81.0`; `0.84.0-0.107.0`; `0.110.0-0.152.1`.

Deterministic fixture:

`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-client-mcp-servers-range.json`

## Per-Segment Anchors

| Segment | Frozen corpus | Client-MCP request or per-thread declaration field | Finding |
| --- | --- | --- | --- |
| `0.80.0..=0.81.0` and `0.84.0..=0.107.0` | `legacy-app-server-releases.json` `selected_methods` | none; selected methods are `initialize`, `model/list`, `thread/start`, `thread/resume`, `turn/start`, `turn/interrupt` | no client MCP method |
| `0.110.0..=0.152.1` | `app-server-releases.json` `experimental_thread_fields` | `dynamicTools` only, later plus `runtimeWorkspaceRoots` and `allowProviderModelFallback` | no `mcpServers` / `mcp_servers` field |
| `0.148.0..=0.152.1` identity protocol | `codex-cli-0.148.0` through `codex-cli-0.152.1` `protocol.json` `schema.methods_present` | same closed method set; no MCP request method | no client MCP method |
| `0.152.0` / `0.152.1` resume params | `protocol.json` `thread_resume_properties` | typed resume keys include untyped `config`; no MCP server key | `config` is the ambient overlay from Research 229, not a client MCP declaration |
| Activity observation | `activity/app-server.jsonl` case `mcp-tool` | `item` type `mcpToolCall` | provider-owned tool observation; expected `kind` is `provider_owned_tool` |

Ceiling protocol methods at `0.152.1`:

- `initialize`
- `model/list`
- `thread/list`, `thread/read`, `thread/start`, `thread/resume`, `thread/archive`, `thread/delete`
- `turn/start`, `turn/interrupt`
- `item/started`, `item/completed`, `item/plan/delta`
- `subAgentActivity`
- `collabAgentToolCall`

Unused mapped MCP deltas at `0.152.0`/`0.152.1` (`optional MCP grace`, `MCP package-style names`, `MCP output_token_limit`) are configuration-owned unused features. They are not a per-session client declaration RPC.

`thread/start` / `thread/resume` / `thread/fork` `config` remains the untyped overlay onto Codex's ambient config stack (Research 229). That is configuration-owned MCP, which this card excludes.

## Support And Withheld Boundaries

| Surface | Disposition |
| --- | --- |
| Card 117 host-mediated dynamic native tools (`dynamicTools`) | supported; unchanged |
| Observed provider `mcpToolCall` activity | observation only; not `client_mcp_servers` |
| Configuration-owned MCP (`config.toml` and the untyped `config` overlay) | not a client-declared route surface |
| Provider-direct MCP registration (`RegisteredToolExecutionKind::Mcp`) | withheld; existing `swallowtail.codex.app_server.registered_mcp_withheld` refusal stands |
| Consumer-declared MCP servers per session or thread | unavailable; provider limitation |

## Matrix

`codex.app-server; codex.exec` `client_mcp_servers` is `No` /
`provider_limitation`. The checker row is
`docs/research/290-feature-matrix-cross-evidence.tsv#L672`, basis this
corpus TSV. Card 114 is complete and is not the reference.

## Non-Claims

No runtime change. No fake stdio MCP transcript. No new binding card.
`codex.exec` has no app-server session RPC; it does not gain a client MCP
surface from this finding.
