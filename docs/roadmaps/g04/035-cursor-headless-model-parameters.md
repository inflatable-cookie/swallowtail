# g04.035 Cursor Headless Model Parameters

Status: complete
Owner: Tom
Created: 2026-08-22
Depends on: completed g04.034; per-route feature completion programme
Vision tags: explicit selection, provider truth, route-local controls
Contract refs: 005, 020, 029, 037, 040, 052
Research: 075, 077, 087, 135, 183

## Problem

`cursor-agent.headless` accepts model parameters inside the exact `--model`
value, but Swallowtail currently binds only a plain model id. Consumers cannot
select Cursor Fast, context, or effort without constructing provider grammar
themselves and bypassing typed preparation.

The selected CLI surface does not expose model-parameter descriptors through
its catalogue. Official documentation also says parameter availability varies
by model. Swallowtail therefore needs an exact evidence-backed allowlist, not a
generic parameter map or an account-wide support claim.

## Generation Runway Goal

Deliver the first route-local control family from the per-route feature
programme: typed Cursor headless model parameters with immutable
selection-to-plan-to-argv agreement.

## Goals

- [x] freeze exact qualified CLI and official-documentation evidence for
      `fast`, `context`, and `effort`
- [x] classify each exact model, parameter, and value tuple as deliver-now or
      evidence-gated
- [x] add adapter-local typed inputs for only the deliver-now tuples
- [x] preserve the existing plain-model path and its exact argv
- [x] reject raw parameter grammar and unqualified model/value combinations
      before provider work
- [x] bind Cursor `effort` to portable reasoning selection only where the
      exact tuple is qualified
- [x] publish qualified-dispatch truth without claiming provider acceptance or
      effective-value confirmation

## Non-Goals

- a provider-neutral generation-parameter map
- inferring parameter support from a catalogue model id
- accepting arbitrary strings, keys, values, or model combinations
- claiming that dispatch proves provider acceptance or effective application
- changing Cursor ACP or catalogue behavior
- sandbox, force, ask mode, session management, or another route feature
- a provider prompt, authenticated catalogue, login, install, or host update
- changing the Cursor Contract 029 ceiling

## Named Scope

The milestone binds only `cursor-agent.headless` on the existing
`cursor-agent.release-date` behavior segment. All four exact qualified builds
already publish the same bracket-parameter syntax:

- `2026.07.01-41b2de7`
- `2026.07.23-e383d2b`
- `2026.08.04-aaa8809`
- `2026.08.11-e8db854`

Card 095 froze the exact model/parameter/value allowlist. Fast and context
remain Cursor-local selected-model parameters. Qualified effort additionally
binds the exact `ReasoningMode` request, plan capability constraint, rendered
model id, and dispatched argv. The route claims qualified dispatch only; model
or account rejection remains provider truth.

## Execution Plan

### Batch 35.1 — Exact Parameter Evidence

- [x] Execute card 095.
- [x] freeze official help and documentation specimens without provider work
- [x] promote Research 183 with the exact allowlist and evidence-gated rows

### Batch 35.2 — Typed Binding

- [x] Execute card 096 after card 095.
- [x] add typed adapter-local inputs and canonical rendering
- [x] bind selected parameters into immutable plan, request, and evidence
- [x] fail closed on raw grammar, unknown values, and unsupported combinations

### Batch 35.3 — Dispatch And Acceptance

- [x] Execute card 097 after card 096.
- [x] prove exact single-argument dispatch and unchanged plain-model behavior
- [x] update realized architecture, guides, matrices, programme, and changelog

## Acceptance Criteria

- [x] consumers never construct Cursor bracket grammar themselves
- [x] only exact evidence-backed model/parameter/value tuples prepare
- [x] typed parameters render in one canonical order into one `--model` argv
- [x] the immutable plan carries the exact rendered model id
- [x] effort request, capability constraint, model id, and argv agree
- [x] fast and context remain route-local and do not become portable aliases
- [x] plain-model preparation and dispatch remain unchanged
- [x] documentation separates dispatched, accepted, and effective states
- [x] deterministic QA performs no provider call or account inspection

## Lane Runway

- previous: g04.034 Gemini CLI 0.56.0 useful-newer qualification
- this milestone: Cursor headless model parameters — complete
- next: Ollama attached `num_ctx`

## Decision Gates

- Stop if exact parameter tuples cannot be qualified without authentication or
  a provider prompt.
- Stop if implementation needs a generic string map or provider-neutral
  Cursor parameter names.
- Stop if raw bracket grammar cannot be rejected before provider work while
  preserving the plain-model public path.
- Stop if effort cannot bind request, plan, rendered model id, and dispatch
  exactly under Contract 040.
- Stop before claiming provider acceptance or effective values without an
  exact confirming surface.

## Batch Cards

- [095-cursor-headless-model-parameter-evidence.md](batch-cards/095-cursor-headless-model-parameter-evidence.md) — complete
- [096-cursor-headless-model-parameter-binding.md](batch-cards/096-cursor-headless-model-parameter-binding.md) — complete
- [097-cursor-headless-model-parameter-acceptance.md](batch-cards/097-cursor-headless-model-parameter-acceptance.md) — complete

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 075 Cursor And Antigravity Installed-Harness Refocus](../../research/075-cursor-and-antigravity-installed-harness-refocus.md)
- [Research 077 Cursor Headless Installed-Source Qualification](../../research/077-cursor-headless-installed-source-qualification.md)
- [Research 087 Cursor Agent 2026.07.23 Range Checkpoint](../../research/087-cursor-agent-2026-07-23-range-checkpoint.md)
- [Research 135 Cursor Agent 2026.08.04 And 2026.08.11 Identity](../../research/135-cursor-agent-2026-08-04-2026-08-11-identity.md)
- [Research 183 Cursor Headless Model Parameter Evidence](../../research/183-cursor-headless-model-parameter-evidence.md)
- [Contract 040 Generation-Control Application And Enforcement](../../contracts/040-generation-control-application-and-enforcement.md)
- [Cursor Prepared Integration](../../guides/cursor-prepared-integration.md)
