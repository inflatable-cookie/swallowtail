# 024 Kimi Structured Coverage And Matrix Closeout

Status: completed
Owner: Tom
Created: 2026-07-27
Depends on: g02.023
Vision tags: Kimi currentness, harness interaction, provider retention
Contract refs: 004-015, 017, 023, 029, 032-034, 037-039
Planning state: cards 077-079 completed

## Problem

Kimi ACP and local server can execute one bounded task, but their lifecycle and
retention differ. Swallowtail also trails maintained Kimi Code `0.29.2`.

The lane must qualify the latest maintained release before extending structured
coverage. Kimi local-server runs may retain threads; deletion remains
unsupported by operator decision.

## Goals

- [x] Qualify Kimi Code `0.29.2` across selected ACP, headless, local-server,
      lifecycle, and catalogue surfaces.
- [x] Add a separately registered Kimi headless structured route.
- [x] Add retained Kimi local-server structured execution.
- [x] Preserve ACP and local-server transport, access, and lifecycle identity.
- [x] Close the solution matrix at 18 `Yes`, two realtime `No`, and one
      serving `Not applicable`.

## Non-Goals

- Kimi thread deletion
- direct access to Kimi state files
- container or implicit sandbox requirements
- treating Kimi Code membership access as Kimi Platform access
- publishing packages or editing consumers

## Execution Plan

### Batch 24.1 — Latest Kimi Range

- [x] Execute card 077 after g02.023 closes.

### Batch 24.2 — Headless And Retained Server Runs

- [x] Execute card 078 after `0.29.2` qualification passes.

### Batch 24.3 — Provider-Wide Closeout

- [x] Execute card 079.

## Acceptance Criteria

- [x] `0.29.2` becomes guaranteed only through exact frozen evidence
- [x] headless and ACP remain separate route identities
- [x] local-server structured policy requires `DurableAllowed`
- [x] run close claims no archive or delete unless the exact operation performs
      it
- [x] owned local-server mode joins the foreground child without a container
- [x] llama.cpp owned becomes `Not applicable`
- [x] Gemini Live and OpenAI Realtime remain `No`
- [x] package, route, docs, and focused conformance pass without live access

## Decision Gates

- Stop if `0.29.2` changes a selected protocol without a new behavior segment.
- Stop local-server execution if retained-session identity cannot remain safe
  and opaque.
- Do not treat unverified-newer execution as guaranteed support.

## Next Planning Checkpoint

Return to the operator-held card 060 lifecycle adoption decision after card
079 unless a structured-run consumer handoff is separately authorized.
