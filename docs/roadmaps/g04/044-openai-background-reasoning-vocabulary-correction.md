# g04.044 OpenAI Background Reasoning Vocabulary Correction

Status: complete
Owner: Tom
Created: 2026-08-23
Depends on: per-route feature completion programme; g04.043; Research 191
Vision tags: explicit selection, provider truth, fail closed
Contract refs: 011, 029, 036, 037, 040, 052
Research: 191

## Problem

Research 191 proves that the exact GPT-5.6 model page supports reasoning
`none|low|medium|high|xhigh|max`. The current `openai.background` guide and
preparation validator additionally admit `minimal`. That value has no exact
model evidence, so the route's stated and executable vocabulary is wider than
its qualified production claim.

This is named follow-up `g04.043-R1`. It is a correction, not another evidence
lane and not an opportunity to widen OpenAI features.

## Generation Runway Goal

Make `openai.background` admit exactly the six Research 191 GPT-5.6 reasoning
values, reject `minimal` before provider work, and version the corrected opaque
facade truth without aliasing or fallback.

## Goals

- [x] replace the inherited seven-value claim with exact
      `none|low|medium|high|xhigh|max` truth
- [x] reject `minimal` at preparation with the existing safe unsupported-value
      diagnostic and no endpoint, credential, request, or provider effect
- [x] preserve the provider-neutral `ReasoningMode` vocabulary for routes that
      qualify `minimal`
- [x] preserve absent reasoning and every admitted value across input, plan,
      prepared evidence, request policy, driver validation, and exact wire
- [x] publish a new exact opaque background-facade point and private behavior
      revision rather than rewriting the July point
- [x] classify the guarantee shrink as a Contract 036 next-minor change
- [x] close named follow-up `g04.043-R1` with deterministic route-local proof

## Non-Goals

- web search, Responses tools, service tier, Fast mode, model expansion, or
  another OpenAI route
- changing global `ReasoningMode` syntax or another adapter's reasoning values
- mapping `minimal` to `none`, clamping, default substitution, retry, or
  fallback
- live credentials, account inspection, provider requests, or paid work
- selecting, tagging, or publishing a release

## Named Scope

The lane is restricted to production route `openai.background`, driver
`swallowtail.openai.background`, exact model route
`openai.public.gpt-5.6.background`, axis
`openai.responses-background-facade`, and the public API-key pay-as-you-go
boundary.

The current exact facade point
`openai-responses-background-2026-07-21` carries the over-wide reasoning
mapping. Contract 029 requires a new exact opaque point and adapter-private
behavior revision for the corrected mapping. The old point is not silently
rewritten or retained as a supported way to request an unqualified value.

Removing a previously guaranteed route value is a guaranteed-behavior shrink
under Contract 036. The implementation may land on unreleased source, but its
closeout must request an explicit next-minor changelog and release disposition.
It must not change workspace versions or create a release.

## Execution Plan

### Batch 44.1 — Exact Vocabulary And Facade Correction

- [x] Execute card 122.
- [x] remove only `minimal` from the exact GPT-5.6 preparation mapping
- [x] bind a new exact opaque facade point and private behavior revision
- [x] preserve the six admitted values and absent path without substitution

### Batch 44.2 — Route-Local Acceptance

- [x] Execute card 123 after card 122.
- [x] prove every admitted value, explicit `minimal` rejection, facade drift,
      plan/evidence/driver agreement, exact request bytes, and zero effects
- [x] update route-local guidance and follow-up records; report the shared
      next-minor closeout delta

## Acceptance Criteria

- [x] exactly `none|low|medium|high|xhigh|max` prepare for exact GPT-5.6
- [x] `minimal`, foreign values, and old/new facade drift fail before effects
- [x] no unsupported value is aliased, clamped, defaulted, retried, or routed
- [x] the global `ReasoningMode` type and other production routes are unchanged
- [x] absent reasoning preserves current request behavior
- [x] plan, evidence, policy, driver, and wire carry the same admitted value
- [x] the new opaque facade point and behavior revision are exact and qualified
- [x] default QA uses no credential, account, external request, or paid work
- [x] route-local docs and tests state only dispatch truth proved by the route
- [x] closeout records the breaking next-minor requirement without releasing

## Completion

Cards 122-123 are complete and merged through PR 43 at `bdb7ea88`. The exact route now admits
only `none|low|medium|high|xhigh|max`; `minimal` and foreign values fail in
preparation before effects. The corrected mapping is bound to
`openai-responses-background-2026-08-23` with private behavior revision
`openai.responses-background-v2`. The July point remains historical and is not
retained as a supported way to request the removed value.

## Lane Runway

- predecessor: g04.043 OpenAI background search evidence stop
- this milestone: exact GPT-5.6 reasoning-vocabulary correction
- execution topology: one serial worker lane, cards 122-123
- next route family: selected while compiling g04.045 from the remaining
  promoted per-route feature inventory; no implementation card is ready yet

## Decision Gates

- Stop if Research 191 no longer identifies the exact model evidence used by
  this correction.
- Stop if the route cannot reject `minimal` before endpoint, credential, or
  provider work.
- Stop if correctness would require changing provider-neutral vocabulary or
  another route's qualified values.
- Stop if a new opaque facade point and behavior revision cannot keep the
  current and corrected claims distinct.
- Stop if release selection, a contract amendment, or wider OpenAI behavior is
  required to implement the route-local correction.

## Batch Cards

- [122-openai-background-reasoning-vocabulary-correction.md](batch-cards/122-openai-background-reasoning-vocabulary-correction.md) — complete
- [123-openai-background-reasoning-vocabulary-acceptance.md](batch-cards/123-openai-background-reasoning-vocabulary-acceptance.md) — complete

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [g04.043 OpenAI Background Hosted Search](./043-openai-background-hosted-search.md)
- [Research 191 OpenAI Background Web Search Evidence](../../research/191-openai-background-web-search-evidence.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 036 Source Release And Compatibility Boundary](../../contracts/036-crate-release-and-compatibility-boundary.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [OpenAI Background Prepared Integration](../../guides/openai-background-prepared-integration.md)
