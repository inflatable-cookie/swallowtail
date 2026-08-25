# g04.062 Anthropic Messages Adaptive Thinking

Status: complete pending review
Owner: Tom
Created: 2026-08-25
Depends on: g04.037; per-route feature completion programme
Vision tags: explicit selection, provider truth, private continuation
Contract refs: 011, 030, 037, 040, 041, 044, 052
Research: 004, 067, 169, 185, 209

## Problem

`anthropic.messages` supports exact effort dispatch but never sends a Messages
`thinking` object. Exact current official documentation identifies adaptive
thinking as a separate control. The selected direct-continuation profile also
cannot safely enable it today: its stream decoder rejects thinking blocks and
its private history reconstructs only the tool-use block, while Anthropic
requires the complete signed thinking sequence to be returned unmodified with
tool results.

This is not another reasoning-effort mapping. Effort remains the portable
`ReasoningSelection`; adaptive thinking is an adapter-local mode whose
provider-private blocks need bounded, zeroized continuation treatment.

## Generation Runway Goal

Qualify and bind one exact adapter-local adaptive-thinking mode on the existing
`anthropic.messages` structured-attempt and direct-continuation profiles. Use
omitted thinking display so no thought text becomes portable output or
activity. Preserve every provider-required signed private block exactly across
the authorized tool loop.

## Goals

- [x] freeze current official model, request, display, stream, tool-loop,
      signature, redacted-block, output-token, effort, and compatibility truth
- [x] promote Research 209 with an exact deliver-now profile/model table or an
      honest empty set
- [x] distinguish adapter-local adaptive thinking from portable effort and
      manual token-budget thinking
- [x] expose only an exact typed `AnthropicThinkingMode::adaptive()` selection
      admitted by Research 209
- [x] bind the selected mode through immutable request and prepared evidence
- [x] emit exact `thinking.type=adaptive` plus the qualified omitted-display
      field without changing omission bytes
- [x] accept, bound, and privately retain exact streamed thinking and
      redacted-thinking blocks without exposing their content
- [x] replay the complete unmodified private block sequence before the
      correlated tool-use block on the authorized continuation attempt
- [x] prove independent composition with omission and every admitted effort
      value
- [x] preserve cancellation, deadline, terminal, restoration, zeroization,
      diagnostics, access, and facade truth

## Non-Goals

- a provider-neutral thinking-mode capability or raw generation-options map
- manual `thinking.type=enabled`, `budget_tokens`, or token-budget selection
- readable thinking summaries, hidden reasoning, chain-of-thought, or a new
  `ReasoningSummary` activity claim
- mapping `adaptive` to `ReasoningMode`, changing effort values, or inferring
  reasoning depth from output
- durable thinking persistence, export, import, resume, reconstruction, or
  consumer transcript authority
- another Anthropic model, facade, endpoint, access profile, Managed Agents,
  Claude Code, UltraCode, Fast mode, or newer web-search tool
- live account inspection, credential use, paid inference, currentness,
  release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `anthropic.messages`, driver
`swallowtail.anthropic.direct`, facade `anthropic-2023-06-01`, public API-key
access, and exact model `claude-opus-4-7` as the first evidence candidate.
Official current documentation is a lead; card 173 owns the exact supported
model/profile result.

The only candidate public value is an adapter-local
`AnthropicThinkingMode::adaptive()`. Research 209 must decide whether the exact
qualified wire is `thinking: {"type":"adaptive","display":"omitted"}` and
which response block shapes that request can produce. Omission must retain the
current request bytes and behavior. The existing optional
`output_config.effort` selection stays independent and must compose without
defaulting, clamping, or shared confirmation.

For one-attempt structured inference, the driver may consume qualified private
thinking blocks only to validate stream order and bounds; it exposes no thought
content and retains no private continuation after terminal. For the
direct-continuation profile, every consecutive provider thinking or
redacted-thinking block belonging to the assistant tool-use response must be
captured in exact order, held in bounded zeroizing memory, and replayed
complete and unmodified before the tool-use block. It may cross only the
already-authorized result continuation inside the same configured instance,
facade, access profile, model route, model, and runtime session.

Thinking text, signatures, redacted data, and raw blocks never enter public
events, terminal output, activity content, callbacks, evidence, serialization,
`Debug`, `Display`, or stable diagnostics. Fresh working-state restoration
continues to replace the session and loses private continuation. A missing,
duplicate, reordered, oversized, malformed, altered, or post-terminal private
block fails closed without fallback.

## Execution Plan

### Batch 62.1 — Exact Adaptive-Thinking Evidence

- [x] Execute card 173.
- [x] freeze exact official and route-local request, response, continuation,
      privacy, effort-composition, and compatibility truth
- [x] promote Research 209 with a non-empty exact table or honest empty set

### Batch 62.2 — Conditional Private Binding

- [x] Execute card 174 only when Research 209 admits a non-empty set.
- [x] bind the adapter-local mode and exact private block capture/replay path

### Batch 62.3 — Route-Local Acceptance

- [x] Execute card 175 only after card 174.
- [x] prove wire, stream, replay, bounds, redaction, effort composition,
      lifecycle, API, guide, matrix, and closeout truth

## Acceptance Criteria

- [x] only Research 209 exact model/profile rows prepare
- [x] request, prepared evidence, driver, stream behavior, and continuation
      replay agree on the same adaptive-thinking selection
- [x] omission preserves existing request bytes and response behavior
- [x] adaptive thinking remains distinct from portable effort; every admitted
      effort value composes independently
- [x] one-attempt inference retains no thinking continuation after terminal
- [x] direct continuation replays every required private block complete,
      unmodified, ordered, bounded, route-bound, and zeroized
- [x] no thinking text, signature, redacted payload, or raw block reaches a
      public event, activity, output, callback, evidence, or diagnostic
- [x] malformed, missing, duplicate, reordered, oversized, or contradictory
      blocks fail closed without retry or fallback
- [x] fresh restoration remains `SessionReplaced` and grants no private-state
      recovery
- [x] default QA performs no account, credential, provider, or paid work
- [x] g04.062 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.061 Kimi Code ACP plan mode
- this milestone: Anthropic Messages adaptive thinking with private replay
- execution topology: one serial worker lane, cards 173-175
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if official evidence cannot freeze one exact model/profile/display row.
- Stop if omitted display does not return a replayable signed private block
  shape or if exact replay requires exposing hidden reasoning.
- Stop if safe delivery needs a new shared capability/contract, durable private
  state, generic provider JSON, a breaking public API, or live provider work.
- Stop if thinking and effort cannot compose independently on every admitted
  profile, or if the tool loop cannot retain and zeroize the complete private
  sequence within Contract 030 bounds.

## Batch Cards

- [173-anthropic-messages-adaptive-thinking-evidence.md](batch-cards/173-anthropic-messages-adaptive-thinking-evidence.md)
- [174-anthropic-messages-adaptive-thinking-binding.md](batch-cards/174-anthropic-messages-adaptive-thinking-binding.md)
- [175-anthropic-messages-adaptive-thinking-acceptance.md](batch-cards/175-anthropic-messages-adaptive-thinking-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 185 Anthropic Messages Effort](../../research/185-anthropic-messages-effort-evidence.md)
- [Research 209 Anthropic Messages Adaptive Thinking](../../research/209-anthropic-messages-adaptive-thinking-evidence.md)
- [Contract 030 Consumer-Owned Direct Tool Continuation](../../contracts/030-consumer-owned-direct-tool-continuation.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 044 Observable Agent Activity](../../contracts/044-observable-agent-activity-and-disclosure.md)
- [Anthropic Direct Prepared Integration](../../guides/anthropic-direct-prepared-integration.md)
- [Anthropic thinking](https://platform.claude.com/docs/en/about-claude/models/extended-thinking-models)
- [Anthropic Messages create API](https://platform.claude.com/docs/en/api/messages/create)
