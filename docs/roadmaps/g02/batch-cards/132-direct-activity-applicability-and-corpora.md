# 132 Direct Activity Applicability And Corpora

Status: planned
Owner: Tom
Created: 2026-07-29
Milestone: `../039-direct-inference-activity-truth.md`
Depends on: card 131

## Goal

Classify exact observable activity for every direct, attached, realtime,
catalogue, and serving production route before changing mappings.

## Scope

1. Audit Alibaba, Anthropic Messages, Bedrock Runtime, DeepSeek, Kimi
   Platform, llama.cpp, Ollama, OpenAI background, xAI Responses WebSocket,
   OpenAI Realtime, and Gemini Live.
2. Classify catalogue-only and serving-only operations as not applicable.
3. Revalidate assistant, reasoning-summary, provider-tool, consumer-tool,
   transcript, and provider-observation surfaces.
4. Freeze exact positive, unavailable, not-applicable, unknown, malformed,
   and failure fixtures.
5. Keep private reasoning continuation and raw provider payloads excluded.
6. Select only exact mapping gaps.

## Out Of Scope

- harness activity
- generation controls or model catalogue changes
- new realtime routes
- live credentials or attached runtime effects

## Acceptance Criteria

- [ ] every production solution and operation is accounted for
- [ ] supported, unavailable, and not applicable remain distinct
- [ ] direct tool ownership is exact
- [ ] private continuation is not display content
- [ ] realtime-media lifecycle remains separate
- [ ] every selected mapping has current authoritative evidence

## Validation

- focused direct protocol fixture tests
- `effigy qa:docs`
- `effigy qa:routes`
- `effigy check:rust`

## Stop Conditions

- Stop one provider when display intent for reasoning or tool detail is
  undocumented.
- Do not turn local compute into a harness capability.

## Auto-Continuation

Continue to card 133 only for exact contract-ready text direct mappings.

