# 132 Direct Activity Applicability And Corpora

Status: completed
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

- [x] every production solution and operation is accounted for
- [x] supported, unavailable, and not applicable remain distinct
- [x] direct tool ownership is exact
- [x] private continuation is not display content
- [x] realtime-media lifecycle remains separate
- [x] every selected mapping has current authoritative evidence

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

## Completion Evidence

- Research 067 revalidates all 13 non-harness production routes against
  current official or maintained evidence and their exact qualified subsets
- the machine corpus records 14 positive text-operation profiles and 13
  not-applicable catalogue, inventory, realtime, and serving operations
- Anthropic provider web search remains distinct from consumer-executed
  Anthropic and DeepSeek tools
- Kimi K3 client-visible thought updates are selected; DeepSeek and xAI
  private continuation remains excluded
- current Alibaba, Anthropic, Ollama, llama.cpp, Gemini Live, and OpenAI
  Realtime richness does not widen the qualified route automatically
- existing positive, unavailable, unknown, malformed, and failure corpora are
  referenced directly; Bedrock retains typed SDK fixtures
- Contracts 026, 030, and 044 are sufficient; no contract delta is required
- no credential, provider call, paid inference, model download, attached
  runtime, executable, or consumer repository was used
