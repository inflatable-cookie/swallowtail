# Per-Route Feature Completion

Status: active programme
Owner: Tom
Created: 2026-08-21
Evidence: `../triage/2026-08-21-advanced-route-features.md`
Governing refs: Contracts 011, 020, 024, 037, 041, 047, 052

## Purpose

Work through missing official capabilities on every production route without
flattening provider vocabulary or leaving the assessed feature inventory as a
research-only list.

## Delivery Rule

Use one route family and one coherent control family per numbered roadmap.
Before implementation, recheck the current official surface and classify each
candidate as:

- deliver now through the exact selected transport
- evidence-gated
- intentionally withheld
- not applicable to that route
- obsolete upstream

Only the first class becomes implementation. Every delivered control needs an
exact typed input, immutable plan/evidence binding, fail-closed validation,
deterministic fixtures, guide coverage, and feature-matrix truth. A route-local
name such as Fast, context, effort, thinking, service tier, agent, or team never
becomes a provider-neutral synonym.

## Initial Sequence

After the active Pi and Gemini decisions, start with the exact-transport set
already identified by the assessed inventory:

1. Cursor headless model parameters: Fast, context, and effort
2. Ollama attached `num_ctx`
3. Anthropic Messages `output_config.effort`
4. DeepSeek continuation reasoning controls
5. xAI Responses reasoning and output bounds

Then continue route-by-route through the production matrix. Re-rank only for a
consumer need, an upstream removal, or a shared contract dependency. New route
families do not interrupt this programme by default.

## Progress

- [g04.035 Cursor Headless Model Parameters](./035-cursor-headless-model-parameters.md)
  is complete. Research 183, cards 095-097, and typed headless model-parameter
  dispatch are realized on `cursor-agent.headless`.
- [g04.036 Ollama Attached Context Window](./036-ollama-attached-context-window.md)
  is ready. Cards 098-100 start with exact `num_ctx` evidence, then bind and
  prove only the Research 184 deliver-now native `/api/chat` profiles.
- [g04.037 Anthropic Messages Effort](./037-anthropic-messages-effort.md) is
  ready. Cards 101-103 start with exact model/value/profile evidence and bind
  only Research 185 deliver-now `output_config.effort` mappings.
- [g04.038 DeepSeek Continuation Reasoning Controls](./038-deepseek-continuation-reasoning-controls.md)
  is ready. Cards 104-106 separate effort from thinking mode and preserve
  private continuation through only Research 186 deliver-now mappings.
- The three route families may execute concurrently. Each lane remains serial;
  integrate Ollama, then Anthropic, then DeepSeek. Compile xAI after this wave.

## Exclusions

- dangerous permission-bypass defaults
- experimental process-spawning teams as a generic composer toggle
- catalogue observations presented as run controls
- unconfirmed flags or UI-label translation
- sibling-route promotion
- new route families

## Closeout Rule

Each numbered roadmap closes one route-local control family, updates the
feature inventory disposition, and names the next route. The programme may span
generation boundaries; it does not justify bulk implementation or a generation
rollover by itself.
