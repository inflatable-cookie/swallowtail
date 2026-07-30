# 2026-07-29 Text Direct-Inference Activity Projection

## Changed

- projected all 14 selected text-operation activity profiles
- added exact facade interface claims where direct routes lacked them
- preserved provider response, message, search, and tool identities needed
  for stable operation-local activity
- correlated Anthropic and DeepSeek consumer tools with their existing
  direct-tool exchanges
- exposed Kimi Platform client-visible thought text while excluding DeepSeek
  private continuation and xAI encrypted thinking
- documented the direct-route profiles in the public guide and architecture

## Route Truth

- Alibaba Model Studio, Anthropic Messages, Amazon Bedrock Runtime, DeepSeek,
  Kimi Platform, OpenAI background Responses, xAI Responses, attached Ollama,
  and attached llama.cpp expose assistant activity
- Kimi Platform also exposes qualified reasoning-summary activity
- Anthropic structured search is provider-owned
- Anthropic and DeepSeek direct tools are consumer-owned and
  consumer-executed
- streamed final answers use `FinalAnswerText`; final operation output remains
  separate
- direct routes gain no harness plans, commands, file changes, tasks, hooks,
  subagents, or tool execution

## Evidence

- 27 focused selected-route and shared-conformance cases passed
- all nine affected adapters passed combined all-target compilation
- the full workspace unit, integration, conformance, and documentation test
  selector passed
- formatting, Clippy, docs, route matrices, and the 23-crate public API
  declaration baseline passed
- no live credential, provider request, paid inference, model download,
  attached runtime, executable, or consumer repository was used

## Current State

Card 133 is complete. Card 134 is ready. Cards 134-137 remain in bounds.

## Risks

- realtime media keeps its dedicated event model until card 134 proves the
  shared boundary
- catalogue, inventory, and serving-only negative profiles await card 134
  closeout
- permitted newer provider interfaces inherit the last qualified activity
  profile and cannot widen it
- Bedrock activity evidence is deterministic SDK-decoder and projector
  coverage; no live AWS call was made

## Next

Execute card 134: close the realtime-media and non-applicable activity
boundaries.
