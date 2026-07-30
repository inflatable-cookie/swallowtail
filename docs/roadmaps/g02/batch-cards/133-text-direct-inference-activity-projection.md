# 133 Text Direct Inference Activity Projection

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../039-direct-inference-activity-truth.md`
Depends on: card 132

## Goal

Map exact assistant, reasoning-summary, and tool activity for selected text
direct-inference routes without fabricating harness work.

## Scope

1. Implement only the mapping gaps selected by card 132.
2. Correlate consumer direct-tool continuation with existing exchanges.
3. Preserve provider-owned tool identity where the selected API exposes it.
4. Publish exact route and operation activity profiles.
5. Keep usage, billed cost, rate, quota, request correlation, cache,
   retention, recovery, and cleanup as separate evidence.
6. Run focused direct and attached-runtime conformance.

## Out Of Scope

- commands, file changes, plans, tasks, hooks, or subagents without source
  evidence
- realtime media
- tool execution
- new API routes

## Acceptance Criteria

- [x] all selected direct mappings match frozen corpora
- [x] provider and consumer tool ownership remain distinct
- [x] reasoning summaries exclude private continuation
- [x] assistant activity and final output remain explicit
- [x] no direct route claims harness lifecycle
- [x] all route access, retention, cancellation, and cleanup tests remain green

## Validation

- selected hosted-direct and attached-runtime adapter tests
- direct continuation conformance
- `effigy check:rust`
- `effigy lint:rust`
- `effigy package:api`

## Stop Conditions

- Stop on ambiguous provider display intent.
- Keep identity-only or unavailable disclosure rather than exposing raw data.

## Auto-Continuation

Continue to card 134 after every selected text route passes.

## Outcome

Completed 2026-07-29.

- all 14 selected text-operation profiles expose exact prepared activity
  evidence
- Alibaba Model Studio, Anthropic Messages, Amazon Bedrock Runtime,
  DeepSeek, Kimi Platform, OpenAI background Responses, xAI Responses,
  Ollama, and attached llama.cpp project assistant activity on their existing
  event streams
- Kimi Platform alone exposes its qualified client-visible thought text as
  reasoning-summary activity; DeepSeek reasoning continuation and xAI
  encrypted thinking remain private
- Anthropic web search remains provider-owned; Anthropic and DeepSeek direct
  tools remain consumer-owned and correlate with their existing exchanges
- final assistant text uses `FinalAnswerText`; `OutputAvailable` remains a
  separate operation event
- deterministic selected-route fixtures passed without live credentials,
  paid inference, model downloads, or consumer changes
- the full workspace test selector and all package-facing quality gates pass
