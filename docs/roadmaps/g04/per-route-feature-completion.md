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
  is complete. Research 184, cards 098-100, and adapter-local `options.num_ctx`
  dispatch are realized on `ollama.attached`.
- [g04.037 Anthropic Messages Effort](./037-anthropic-messages-effort.md) is
  complete and merged through PR 37 at `56a7b87b`. Research 185 and cards
  101-103 realize exact `claude-opus-4-7` `output_config.effort` dispatch on
  structured and fixed direct-continuation profiles.
- [g04.038 DeepSeek Continuation Reasoning Controls](./038-deepseek-continuation-reasoning-controls.md)
  is complete and merged through PR 36 at `badb400a`. Research 186 and cards
  104-106 realize exact V4 Pro `low`, `high`, and `max` effort while keeping
  thinking enabled and private continuation adapter-held.
- [g04.039 xAI Responses Reasoning And Output Bounds](./039-xai-responses-reasoning-output-bounds.md)
  is ready. Cards 107-109 start with refreshed exact model/value/profile
  evidence before binding any reasoning or output control.

## Current Worker Boundary

The xAI worker owns only its route crate, fixtures, prepared guide, numbered
milestone and cards, reserved research record, reserved closeout log, and
package-specific public API baseline. It must not edit shared architecture,
route/feature matrices, changelog, programme, roadmap front doors, indexes, or
`packages.txt` while the lane is in flight.

The orchestrator reconciles those shared surfaces after review and merge. The
worker reports the required shared delta in its closeout record and PR body; it
does not apply that delta on its route branch.

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
