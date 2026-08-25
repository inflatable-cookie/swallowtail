# 173 Anthropic Messages Adaptive-Thinking Evidence

Status: complete
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Milestone: [g04.062 Anthropic Messages Adaptive Thinking](../062-anthropic-messages-adaptive-thinking.md)
Depends on: Research 004, 067, 169, 185; Contracts 030, 040, 044

## Goal

Determine the exact Anthropic Messages model and operation profiles on which
adapter-local adaptive thinking with omitted display can be dispatched and,
for consumer-tool continuation, replayed safely as bounded provider-private
state. Promote an honest empty set if any required fact remains unproved.

## Work

1. Retrieve and digest current official Anthropic thinking, adaptive-thinking,
   Messages API, streaming, tool-use, model, token, caching, and error sources.
   Record retrieval dates and decisive body hashes in Research 209.
2. Freeze exact `thinking` request forms and supported model rows. Distinguish
   adaptive, disabled, manual enabled/budget, summarized display, omitted
   display, and omission; do not translate them into effort values.
3. Freeze exact composition with `output_config.effort`, `max_tokens`, tools,
   `tool_choice:auto`, streaming, cache accounting, and every selected
   operation profile. Identify any minimum or incompatibility that changes the
   existing prepared limits.
4. Freeze every possible private response form for the candidate request:
   `thinking`, `redacted_thinking`, content-block start/delta/stop,
   `thinking_delta`, `signature_delta`, opaque signature/data, ordering,
   multiplicity, and absent-thinking behavior.
5. Prove the exact tool-result continuation rule: which assistant blocks must
   be replayed, in what order, with what unchanged fields, across immediate
   continuation and later user turns. Classify provider 400 behavior for
   missing, altered, reordered, or incomplete private blocks.
6. Audit the current Anthropic request builder, SSE decoder, attempt parser,
   activity projection, private history, bounds, zeroization, restoration,
   diagnostics, and fixtures. Name the smallest safe production delta.
7. Decide whether omitted display keeps all thought text private while yielding
   exact replay material. Do not qualify summarized-display activity in this
   lane.
8. Classify one-attempt structured inference and fixed direct continuation
   separately. Freeze model/facade/profile rows, omission, effort composition,
   access, privacy, failure, and `UnverifiedNewer` dispositions.
9. Promote Research 209 with one exact deliver-now table or an explicit empty
   set. Update the milestone/card state and reserved closeout honestly.

## Acceptance Criteria

- [x] exact official source bodies, dates, and hashes are recorded
- [x] model/profile/request/display support is explicit and exact
- [x] effort, adaptive thinking, manual budget thinking, and omission remain
      distinct
- [x] stream and private-block grammar is complete enough for fail-closed
      parsing and bounded replay
- [x] tool continuation preservation and provider rejection truth are explicit
- [x] one-attempt disposal and session replay/zeroization boundaries are exact
- [x] no readable or hidden thinking disclosure is admitted
- [x] Research 209 contains a non-empty exact table or honest empty set
- [x] no production code, public API, shared contract/runtime, currentness,
      release, merge, generation rollover, or g04 closure changes
- [x] `effigy qa:northstar`, relevant indexes, and `git diff --check` pass

## Stop Conditions

- exact model/display/profile support or private block grammar remains ambiguous
- omitted display cannot provide exact tool-loop replay material
- delivery needs live provider work, hidden-reasoning exposure, durable private
  state, generic JSON, or a new shared contract/capability

## Out Of Scope

- production binding, summarized thought activity, manual budgets, other
  Anthropic products/models/facades, live access, currentness, release, merge,
  generation rollover, or g04 closure
