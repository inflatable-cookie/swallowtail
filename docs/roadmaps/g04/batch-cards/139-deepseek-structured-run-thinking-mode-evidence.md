# 139 DeepSeek Structured-Run Thinking-Mode Evidence

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.050 DeepSeek Structured-Run Thinking Mode](../050-deepseek-structured-run-thinking-mode.md)
Depends on: Research 023 and 186; g04.038

## Goal

Freeze current exact-model and OpenAI Chat Completions thinking-mode behavior,
then define the smallest one-request structured-run subset that can satisfy
Contracts 024, 029, 037, 040, and the enabled-only Contract 030 boundary.

## Method

1. Fetch current official Chat Completions, Thinking Mode, Tool Calls, and
   Models/Pricing pages. Record retrieval dates, page dates when present,
   stable URLs, complete specimen digests, and authoritative schema excerpts.
2. Freeze current route source and deterministic fixtures for exact model and
   facade, structured-run/session inputs, plan/evidence, driver agreement,
   request encoding, response parsing, cache acceptance, cancellation,
   deadline, failure, and cleanup.
3. Enumerate explicit `thinking.type=enabled|disabled`, omission/default,
   unknown future values, and the exact relation to `reasoning_effort`.
   Determine whether disabled requires effort omission and reject provider
   mappings as portable values.
4. Classify one-request structured runs and direct tool continuation
   separately. Keep continuation enabled-only unless exact evidence and the
   existing contract prove otherwise; this card is not authorized to widen it.
5. Classify response `reasoning_content`, ordinary content, usage, finish
   reason, error, cache, and returned-model truth under disabled mode. Do not
   infer effective mode from response content.
6. Prove whether exact adapter-local selected mode can survive input,
   immutable plan/evidence, driver validation, and request encoding without a
   new shared capability or false `ReasoningSelection`.
7. Prove existing enabled `low|high|max` calls retain their exact public API,
   plan/evidence, request bytes, structured-run behavior, and every
   continuation path.
8. Decide the exact facade point, private behavior revision, claim id, and
   model-route revision for any admitted subset. Do not widen Contract 029
   currentness.
9. Replace Research 197's reservation with exact field/profile/response
   dispositions and a deliver-now table. Do not edit production code or shared
   closeout surfaces.

No credential, account/balance inspection, provider request, paid operation,
or live DeepSeek call is authorized. Current official public documentation and
secret-free deterministic repository evidence are sufficient for this gate.

## Acceptance Criteria

- [x] exact model, facade, request field, effort-omission, response, cache, and
      profile truth is source-backed or withheld
- [x] enabled, disabled, omission, aliases, and unknown values are distinct
- [x] structured runs and direct continuation have explicit dispositions
- [x] plan/evidence representation and compatibility revision are explicit
- [x] Research 197 is promoted with a deliver-now table or honest empty set
- [x] no production code, capability, matrix, contract, or currentness claim
      changes
- [x] `effigy validate:focused swallowtail-adapter-deepseek` passes
- [x] `effigy qa:northstar` and `effigy qa:docs:index:research` pass
- [x] `git diff --check` passes

Auto-continue to card 140 only when Research 197 admits a non-empty exact
structured-run value/profile set that preserves enabled calls and the
enabled-only continuation boundary.

## Stop Conditions

- exact field composition, response behavior, plan/evidence truth, or facade
  applicability cannot be closed without a live call or inference
- disabled mode requires a portable alias, shared capability, contract change,
  currentness work, or breaking existing API
- direct continuation or private replay would change

## Out Of Scope

- production binding, guide/matrix claims, another model/route/facade, live
  work, release, or shared closeout surfaces

## Closeout

Card 139 promoted Research 197 on 2026-08-23. The exact deliver-now set is
one adapter-local `DeepSeekThinkingMode::disabled()` selection for one-request
structured runs on exact `deepseek-v4-pro` and the existing
`deepseek-openai-chat-2026-07-22` facade. It dispatches explicit
`thinking.type=disabled` with `reasoning_effort` absent and carries no shared
`ReasoningSelection`. Existing enabled `low|high|max` structured and
direct-continuation paths remain unchanged.

The official pages were re-fetched today with complete-body digests and
same-day HTTP metadata. Current source establishes the independent toggle,
V4 Pro dual-mode support, thinking-only `reasoning_content`, and a
non-thinking tool-call example without an effort field. It does not establish
provider acceptance or effective mode, so those claims remain withheld.

Direct continuation, tools, restoration, private replay, V4 Flash, aliases,
`ReasoningMode("none")`, effort aliases, shared capabilities, facade and
currentness changes, and cache/quality/latency/price claims remain out of
scope. Cards 140-141 are admitted because the deliver-now set is non-empty.
