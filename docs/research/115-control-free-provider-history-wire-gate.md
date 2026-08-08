# 115 Control-Free Provider History Wire Gate

Status: accepted
Owner: Tom
Date: 2026-08-08

## Question

After g03.058 wired Contract 054 history pages on Codex, OpenCode, and Alibaba
retained conversations, do current Claude Agent ACP, Kimi Code ACP, or Kimi
local-server artifacts expose a control-free history or transcript wire that
Swallowtail can map without wrapping `session/load` or inventing endpoints?

## Method

Local evidence only on 2026-08-08. No live authentication, prompt, paid call,
or provider mutation.

Inspected:

- Contract 054 unsupported inventory and Research 099 ACP load-is-continuation
  gate
- Claude Agent ACP continuity corpus `0.53.0..=0.61.0`, compatibility
  `0.62.0..=0.64.0`, and adapter attachment (`session/load`, `session/resume`)
- Kimi ACP baseline `0.28.1` and list/import range `0.28.1..=0.31.1`
- Stable ACP session-list fixture `2026-03-09` / schema pins through `v1.20.0`
- Kimi local-server OpenAPI/AsyncAPI selected fixtures
  (`0.28.1`–`0.29.0` surface; REST lifecycle unchanged through `0.31.1` per
  Research 069) and reconciliation observation code

## Findings

### Claude Agent ACP — blocked

| Surface | History role |
| --- | --- |
| `session/load` | Only history path; restores resumable context and returns a live handle after replay notifications |
| `session/resume` | Continuation without replay; still attaches |
| `session/list` | Not selected for Claude |
| Separate history RPC | None in qualified corpora |

Underlying SDK history (`getSessionMessages`) still sits behind
`getOrCreateSession` on the checked continuity evidence. That is not an ACP
read-only method. Closing a loaded handle immediately does not make load
observe-only (Research 099).

### Kimi Code ACP — blocked

| Surface | History role |
| --- | --- |
| `session/load` | Only history path; ordered replay then ready session |
| `session/resume` | Continuation without replay |
| `session/list` | Catalogue metadata (`sessionId`, cwd, title, updatedAt); not message history |

No `session/history` / `session/messages` method appears in the Kimi ACP
fixtures or adapter attachment surface.

### Kimi local-server — blocked

Qualified REST lifecycle paths: health, meta, session list/create/get,
archive, restore. Interactive paths add prompts, approvals, questions.
WebSocket v2 delivers turn/activity events with cursor catch-up for
exact-turn reconciliation.

Missing for Contract 054:

- no REST messages / transcript / history list
- `GET /api/v1/sessions/{id}` returns metadata (`last_seq`, busy, archive),
  not message content
- reconciliation returns empty `SessionReplayItem` replay by design
- descriptor advertises no `ProviderSessionHistory` role

WS activity catch-up is not a qualified full-session message transcript API.

## Recommendation

Keep all three routes unsupported for Contract 054. Do not open adapter
mapping cards. Do not wrap ACP `session/load` or invent a local-server
messages endpoint from activity events.

## Promotion Gate

Reopen only when exact artifact evidence shows one of:

1. **ACP:** a separately advertised bounded history read that creates no
   resumable context, session handle, MCP connection, callback, provider
   request, or control authority — with request/response corpus across the
   maintained version range; or
2. **Kimi local-server:** a documented control-free transcript/messages API
   (REST list or separately qualified event-to-message projection with
   pagination or bounded complete snapshot), frozen in OpenAPI/corpus and
   proven free of prompt/abort/approval side effects.

Then amend Contract 054’s route table, add fixtures, and wire
`ProviderSessionHistoryDriver` using the existing portable page vocabulary.

## Tradeoffs

| Choice | Cost | Benefit |
| --- | --- | --- |
| Stay unsupported vs load→replay→close | ACP consumers wait for upstream | Contract 054 / 048 authority stays honest |
| Stay unsupported vs projecting WS events as history | no local-server history UX yet | avoids inventing a messages API Swallowtail does not own |
| Park at evidence gate vs speculative research cards | no active runway | reopens only on artifact change |

## Promotion Targets

- No contract change — Contract 054 already states the ACP stop
- Research index entry for this negative closeout
- Guide already lists these routes unsupported; keep that inventory

## Validation Needs

None for implementation. Re-run this inventory when Claude Agent, Kimi ACP,
stable ACP, or Kimi local-server OpenAPI corpora move.
