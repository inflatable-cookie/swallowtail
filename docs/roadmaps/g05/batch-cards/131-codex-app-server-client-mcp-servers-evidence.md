# 131 Codex App-Server Client MCP Servers Evidence

Status: ready
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 129 backlog rank 5 and 12; Contract 063 Codex row ("provider-direct MCP needs a separate app-server surface and corpus"); card 117 native registered tools

## Goal

Settle the `client_mcp_servers` cross on `codex.app-server` under the Feature
Matrix Rule: either frozen evidence that the app-server protocol offers no
client-declared MCP server surface on the qualified range (provider
limitation, cited), or the exact surface and a follow-up binding card
(producer gap). Today the cross points at card 114, which is complete, so the
matrix check would reject it; this card is the correct reference.

## Scope

1. From the frozen Codex app-server corpus and the qualified CLI range: does
   any request or configuration surface let a client declare MCP servers per
   session or per thread, distinct from Codex's configuration-owned MCP
   (`config.toml`)? Record with anchors per version segment.
2. If a surface exists: freeze one bounded fake transcript (declare one stdio
   server, observe discovery and one call) and write the binding card.
3. If none exists: record the provider limitation with citations, and note
   that Codex native host-mediated registered tools (card 117) are the
   qualified consumer-tool path.
4. Update the matrix cell's `cross_kind`/`cross_ref` accordingly.

## Out Of Scope

Runtime changes; live Codex; configuration-owned MCP as a route.

## Acceptance Criteria

- [ ] the cross is classified with anchored evidence or a real binding card
- [ ] the matrix check passes with the new reference
- [ ] card 117's native path is named as the qualified alternative

## Validation

- `effigy qa:routes`; `effigy qa:docs`; `git diff --check`

## Review Oracle

Invariant: the cross names who owes the work, with evidence. Smallest
counterexample: a citation to a guide instead of the corpus.

## Auto-Continuation

No. Stop for exact-head review.
