# g04.037 Anthropic Messages Effort

Status: ready
Owner: Tom
Created: 2026-08-22
Depends on: per-route feature completion programme
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 011, 020, 029, 037, 040, 052
Research: 004, 067, 169; 185 to be produced by card 101

## Problem

`anthropic.messages` binds an exact model and output-token maximum, but it does
not expose the Messages API `output_config.effort` control. Consumers cannot
request an exact qualified effort through the prepared facade even when the
selected model supports it.

Messages effort is not Claude Code `--effort`, Ultracode, Fast mode, or the
separate `thinking` object. Model support and accepted values require current
official evidence. This milestone must add only an exact portable reasoning
mapping and must not infer effective effort from response content.

## Generation Runway Goal

Deliver the third route-local control family from the per-route feature
programme: exact Anthropic Messages effort selection with immutable
request-to-plan-to-wire agreement on qualified models and operation profiles.

## Goals

- [ ] freeze current official request, value, model, and compatibility evidence
      for `output_config.effort`
- [ ] classify one-attempt inference and direct-continuation sessions separately
- [ ] map only exact supported values to portable `ReasoningSelection`
- [ ] retain the requested mode in the immutable request, plan, and prepared
      evidence used for dispatch
- [ ] encode one exact `output_config.effort` object without changing the absent
      request body
- [ ] reject unsupported models, values, profile combinations, and mismatches
      before network work
- [ ] publish qualified dispatch without claiming provider acceptance or an
      effective effort level
- [ ] leave the exact shared architecture, matrix, changelog, and programme
      delta for orchestrator closeout after merge

## Non-Goals

- Claude Code effort, Ultracode, Fast mode, or Managed Agents `model.effort`
- the Messages `thinking` object, thinking budgets, or adaptive thinking
- a provider-neutral raw effort string or generation-parameter map
- model discovery, automatic model substitution, or catalogue-derived support
- changing web-search tool versions, output-token limits, attachments, or tools
- a live provider request, account inspection, or compatibility-ceiling change

## Named Scope

The milestone is restricted to the existing `anthropic.messages` direct facade,
official Anthropic endpoint, API-key access profile, and exact prepared model
route. Card 101 must recheck the current official Messages documentation and
freeze the model/value/profile subset into Research 185 before implementation.

Card 102 may bind only Research 185 deliver-now rows. An admitted effort maps to
the portable `ReasoningSelection` capability and exact constraint because the
provider field is an effort selection. The provider-owned field name and model
allowlist remain adapter authority. `thinking` stays independent and is not
silently added, removed, or synthesized.

The claim stops at exact dispatch. A successful response may establish request
acceptance, but it does not prove the effective reasoning allocation unless an
exact provider confirmation surface is qualified.

## Execution Plan

### Batch 37.1 — Exact Effort Evidence

- [ ] Execute card 101.
- [ ] freeze current official request, model, and value evidence
- [ ] promote Research 185 with profile and compatibility dispositions

### Batch 37.2 — Portable Binding

- [ ] Execute card 102 after card 101.
- [ ] add exact effort input to only the admitted prepared profiles
- [ ] bind request, capability constraint, prepared evidence, and driver
- [ ] preserve the absent path and fail before network work

### Batch 37.3 — Dispatch And Acceptance

- [ ] Execute card 103 after card 102.
- [ ] prove exact request bodies and mismatch failures
- [ ] update the route guide and report the shared architecture, matrix,
      programme, and changelog delta for orchestrator closeout after merge

## Acceptance Criteria

- [ ] only Research 185 model/value/profile combinations prepare
- [ ] request, plan constraint, prepared evidence, and wire value agree exactly
- [ ] `output_config.effort` remains distinct from `thinking` and Claude Code
      controls
- [ ] absent effort preserves current request JSON and public behavior
- [ ] unsupported combinations fail before endpoint or credential use
- [ ] deterministic QA makes no provider call or account inspection
- [ ] docs separate requested, planned, dispatched, accepted, and effective
      states
- [ ] the worker branch touches no shared parallel-closeout surface

## Lane Runway

- parallel predecessor: g04.036 Ollama attached `num_ctx`
- this milestone: Anthropic Messages `output_config.effort`
- fixed integration position: after g04.036
- next integration position: g04.038 DeepSeek continuation reasoning controls

## Decision Gates

- Stop if current official evidence cannot identify an exact model/value subset.
- Stop if effort requires the separate `thinking` object for correctness and the
  interaction cannot be expressed under Contract 040 without widening scope.
- Stop if direct-continuation replay would change the selected value between
  attempts or restoration.
- Stop if implementation needs raw provider strings or a generic options map.
- Stop before claiming effective effort without provider confirmation.

## Batch Cards

- [101-anthropic-messages-effort-evidence.md](batch-cards/101-anthropic-messages-effort-evidence.md) — ready
- [102-anthropic-messages-effort-binding.md](batch-cards/102-anthropic-messages-effort-binding.md) — ready after 101
- [103-anthropic-messages-effort-acceptance.md](batch-cards/103-anthropic-messages-effort-acceptance.md) — ready after 102

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation-Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [Anthropic Direct Prepared Integration](../../guides/anthropic-direct-prepared-integration.md)
