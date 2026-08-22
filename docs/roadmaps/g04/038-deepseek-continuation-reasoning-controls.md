# g04.038 DeepSeek Continuation Reasoning Controls

Status: ready
Owner: Tom
Created: 2026-08-22
Depends on: per-route feature completion programme
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 011, 020, 024, 029, 030, 037, 040, 041, 052
Research: 023, 067, 169; 186 to be produced by card 104

## Problem

`deepseek.continuation` currently requires portable reasoning mode `high` and
hard-codes `reasoning_effort=high` plus `thinking.type=enabled` on every
request. Official DeepSeek V4 documentation exposes a wider effort ladder and
a thinking-mode switch, but Swallowtail cannot express any qualified choice.

The two fields are not interchangeable. Disabling thinking may also remove the
private `reasoning_content` lifecycle on which the current continuation proof
depends. The milestone must revalidate structured runs and continuation
sessions separately, preserve private replay truth, and implement only the
exact Research 186 deliver-now controls.

## Generation Runway Goal

Deliver the fourth route-local control family from the per-route feature
programme: exact DeepSeek reasoning selection without weakening the direct
continuation contract or mapping unsupported labels to `high`.

## Goals

- [ ] freeze current official `reasoning_effort`, `thinking`, model, tool, and
      continuation evidence
- [ ] classify `low`, `high`, and `max` independently from documented aliases
      or provider mappings
- [ ] classify thinking disable independently for structured runs and
      continuation sessions
- [ ] bind only exact deliver-now portable reasoning modes through input, plan,
      evidence, driver, and every request attempt
- [ ] preserve private `reasoning_content` replay and fixed-session selection
      wherever continuation remains admitted
- [ ] reject unqualified modes, aliases, field combinations, and mismatches
      before network work
- [ ] publish qualified dispatch without claiming effective reasoning depth
- [ ] leave the exact shared architecture, matrix, changelog, and programme
      delta for orchestrator closeout after merge

## Non-Goals

- accepting `medium` or `xhigh` merely because the provider maps them to `high`
- a generic reasoning options map or provider-neutral thinking boolean
- consumer-visible private reasoning or serialized continuation material
- V4 Flash, retired aliases, Anthropic facade, `/v1`, beta strict tools, or
  streamed tool-call assembly
- changing tool-loop bounds, output limits, cache posture, model selection, or
  the exact facade revision without a separate compatibility decision
- live provider, account, balance, or model work

## Named Scope

The milestone is restricted to exact `deepseek-v4-pro` on the qualified
`deepseek-openai-chat-2026-07-22` facade. Card 104 must recheck current official
documentation against the frozen corpus and decide whether the current facade
revision remains sufficient. A required new revision or behavior segment is a
stop for orchestrator review, not an implementation detail.

Research 186 must classify structured-run and interactive continuation
profiles separately. Exact effort values may extend portable
`ReasoningSelection`; provider aliasing does not. `thinking.type=disabled` may
be delivered only if it has an exact typed representation and does not falsify
the route's capabilities or continuation contract. It may remain withheld for
sessions while an effort ladder is delivered.

For admitted sessions, one prepared reasoning selection applies to the initial
attempt, every tool-result continuation, every later user turn, and fresh local
restoration. The provider-private continuation remains adapter-held and never
becomes consumer output.

## Execution Plan

### Batch 38.1 — Exact Reasoning Evidence

- [ ] Execute card 104.
- [ ] freeze current official field, value, model, and continuation evidence
- [ ] promote Research 186 with per-profile and per-control dispositions

### Batch 38.2 — Exact Binding

- [ ] Execute card 105 after card 104.
- [ ] bind only deliver-now reasoning controls through prepared operations
- [ ] keep one fixed selection across every continuation request
- [ ] preserve current high/enabled behavior when no new selection is supplied

### Batch 38.3 — Dispatch And Acceptance

- [ ] Execute card 106 after card 105.
- [ ] prove exact request bodies, replay invariants, and failure boundaries
- [ ] update the route guide and report the shared architecture, matrix,
      programme, and changelog delta for orchestrator closeout after merge

## Acceptance Criteria

- [ ] only Research 186 model/value/profile combinations prepare
- [ ] no alias or unsupported mode is silently mapped to `high`
- [ ] request, plan, evidence, driver, and every wire attempt agree exactly
- [ ] continuation-private reasoning stays private, bounded, and invalidated on
      the existing lifecycle
- [ ] current fixed high/enabled request behavior remains unchanged when the
      additive control path is absent
- [ ] deterministic QA makes no provider call or account inspection
- [ ] docs separate dispatch, acceptance, effective reasoning, and observed
      private continuation
- [ ] the worker branch touches no shared parallel-closeout surface

## Lane Runway

- parallel predecessor: g04.037 Anthropic Messages effort
- this milestone: DeepSeek continuation reasoning controls
- fixed integration position: after g04.037
- next: xAI Responses reasoning and output bounds, to be compiled after this
  parallel wave is reviewed

## Decision Gates

- Stop if current evidence requires a new facade revision or compatibility
  segment.
- Stop if a selected value cannot remain fixed across all continuation
  attempts and restoration.
- Stop if thinking disable contradicts Contract 030 private-continuation
  requirements or would require a false portable capability.
- Stop if implementation would accept provider aliases or raw strings.
- Stop before claiming effective reasoning depth from output or acceptance.

## Batch Cards

- [104-deepseek-reasoning-evidence.md](batch-cards/104-deepseek-reasoning-evidence.md) — ready
- [105-deepseek-reasoning-binding.md](batch-cards/105-deepseek-reasoning-binding.md) — ready after 104
- [106-deepseek-reasoning-acceptance.md](batch-cards/106-deepseek-reasoning-acceptance.md) — ready after 105

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 023 DeepSeek V4 Direct Continuation Boundary](../../research/023-deepseek-v4-direct-continuation-boundary.md)
- [Contract 030 Consumer-Owned Direct Tool Continuation](../../contracts/030-consumer-owned-direct-tool-continuation.md)
- [Contract 040 Generation-Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [DeepSeek Prepared Integration](../../guides/deepseek-prepared-integration.md)
