# g04.084 OpenAI Realtime Reasoning Effort

Status: done
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Depends on: g04.083 closeout; promoted Research 236
Vision tags: explicit selection, provider truth, realtime continuity
Contract refs: 011, 024, 026, 029, 037, 040, 047, 052
Research: 236

## Problem

Research 236 closes five exact session-scoped `reasoning.effort` rows for
`openai.realtime` on fixed model `gpt-realtime-2.1`. Current production still
rejects every caller reasoning selection on facade
`openai-realtime-2026-07-22`, even though the shared realtime request already
carries optional portable reasoning.

The binding must mint a new exact facade point, preserve the historical point
and omission bytes, serialize one immutable session selection, and require a
matching `session.updated` acknowledgement before the prepared session becomes
usable.

## Goal

Deliver only Research 236's exact `minimal|low|medium|high|xhigh` rows as
session-scoped OpenAI Realtime reasoning selection. Keep per-response override,
provider effectiveness, reasoning-token inference, and every Responses-only
value out.

## Named Scope

- route `openai.realtime`
- driver `swallowtail.openai.realtime`
- model `gpt-realtime-2.1`
- public API-key access at `api.openai.com`
- manual mono PCM16 24 kHz, two serial responses, no planned rollover
- new facade `openai-realtime-reasoning-2026-08-27`
- new private behavior `openai.realtime-manual-pcm-reasoning-v2`
- historical facade `openai-realtime-2026-07-22` retained as superseded proof

| Portable value | Session wire value |
| --- | --- |
| `minimal` | `session.reasoning.effort = "minimal"` |
| `low` | `session.reasoning.effort = "low"` |
| `medium` | `session.reasoning.effort = "medium"` |
| `high` | `session.reasoning.effort = "high"` |
| `xhigh` | `session.reasoning.effort = "xhigh"` |

`none`, `max`, `off`, `default`, `on`, `auto`, aliases, casing variants, and
numeric budgets reject before endpoint, credential, socket, or media work.

## Goals

- [x] bind one optional `ReasoningMode` on the prepared Realtime session input
- [x] carry exact capability, constraint, plan, evidence, request, driver, and
      `session.update` agreement
- [x] require matching explicit `session.updated.session.reasoning.effort`
      before returning a selected session
- [x] preserve exact omission bytes and current omission acknowledgement
      behavior without inferring a provider default
- [x] preserve the selected value through fresh working-state restoration
- [x] compose every admitted value with the existing output-token maximum
- [x] preserve media, cancellation, deadline, failure, disconnect, and joined
      cleanup truth
- [x] publish dispatch and acknowledgement truth only

## Non-Goals

- per-response `response.create` reasoning override
- live provider verification or effective reasoning-depth claims
- reasoning-token usage, thought summaries, transcript inference, or billing
- Responses/Background vocabulary or sibling OpenAI route promotion
- model choice, aliases, rollover, text turns, tools, WebRTC, SIP, or browser use
- generic provider settings, fallback, clamping, substitution, or retry
- contract changes, currentness widening, release, generation rollover, or g04
  closure

## Execution Plan

### Batch 84.1 — Prepared Binding

- [x] execute card 236
- [x] mint the new exact facade and private behavior point
- [x] bind Research 236's five values through preparation and session setup
- [x] preserve the historical point and exact omission behavior

### Batch 84.2 — Route-Local Acceptance

- [x] execute card 237 after card 236
- [x] prove positive, omission, mismatch, restoration, composition, and
      unchanged lifecycle behavior with deterministic fixtures
- [x] update route guidance, matrices, API baseline, and route-local closeout

## Acceptance Criteria

- [x] only Research 236's five exact values prepare
- [x] selected value agrees across input, capability constraint, plan, evidence,
      request, setup bytes, and matching acknowledgement
- [x] mismatch or missing acknowledgement for explicit selection fails the open
      before returning a usable session
- [x] omission retains the existing no-`reasoning` setup bytes and claims no
      selected or default effort
- [x] fresh restoration reuses the exact immutable selected request
- [x] output maximum and reasoning compose without changing either bound
- [x] no provider acceptance beyond the matching configuration event,
      effectiveness, reasoning-token, or observed-reasoning claim appears
- [x] credential-free focused validation passes

## Decision Gates

- Stop if the new facade cannot retain the old exact proof as superseded.
- Stop if explicit selection cannot be confirmed from the matching
  `session.updated` event before the session becomes usable.
- Stop if omission would gain a default claim or different setup bytes.
- Stop if binding needs a shared runtime change, contract amendment, live
  provider operation, or per-response control.
- Stop if selected state can drift across fresh restoration.

## Batch Cards

- [236 OpenAI Realtime Reasoning-Effort Binding](batch-cards/236-openai-realtime-reasoning-effort-binding.md)
- [237 OpenAI Realtime Reasoning-Effort Acceptance](batch-cards/237-openai-realtime-reasoning-effort-acceptance.md)

## Shared Closeout Deltas (orchestrator only)

After merge, update:

- per-route feature inventory / programme rows for `openai.realtime` reasoning
- research/log/card indexes and sole Next Task pointer
- do not edit those shared surfaces on the worker branch

## References

- [Research 236 OpenAI Realtime Reasoning-Effort Evidence](../../research/236-openai-realtime-reasoning-effort-evidence.md)
- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Per-Route Feature Inventory](./per-route-feature-inventory.md)
- [Contract 026 Realtime Media Direct Session Boundary](../../contracts/026-realtime-media-direct-session-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Realtime Prepared Integration](../../guides/realtime-prepared-integration.md)
