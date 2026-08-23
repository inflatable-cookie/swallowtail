# Per-Route Feature Completion

Status: active programme
Owner: Tom
Created: 2026-08-21
Evidence: `../triage/2026-08-21-advanced-route-features.md`
Governing refs: Contracts 011, 020, 024, 037, 040, 041, 047, 052

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
  is complete and merged through PR 38 at `e9ae1a49`. Research 187 and cards
  107-109 realize exact model-qualified reasoning and positive output-token
  bounds on structured and serial connection-local profiles.
- [g04.040 Copilot CLI ACP Session Effort](./040-copilot-cli-acp-session-effort.md)
  stopped after card 110 and merged through PR 39 at `da0871d5`. Research 188
  proves exact `1.0.80` model-entitles server-start effort and may substitute
  the selected model's default. The route selects no model, so cards 111-112
  are blocked and `reasoning_selection` remains `No`.
- [g04.041 Qwen Headless Reasoning Effort](./041-qwen-headless-reasoning-effort.md)
  is complete and merged through PR 40 at `709d197c`. Research 189 and cards
  113-115 realize exact `0.21.15` process-private reasoning selection for
  `qwen3.8-max` and `qwen3.8-max-preview` across runs, turns, resume, and fresh
  replacement.
- [g04.042 Cline Thinking Controls](./042-cline-thinking-controls.md) stopped
  after card 116 and merged through PR 41 at `27b34c7d`. Research 190 proves
  exact `3.0.55` ACP discards the parsed selection and headless model-entitles
  it while the route selects no model. Cards 117-118 are blocked and both
  `reasoning_selection` cells remain `No`.
- [g04.043 OpenAI Background Hosted Search](./043-openai-background-hosted-search.md)
  stopped after card 119 and merged through PR 42 at `685dbf1a`. Research 191
  proves the individual search fields but not the exact composed background
  route. Cards 120-121 are blocked and no search capability shipped.
- [g04.044 OpenAI Background Reasoning Vocabulary Correction](./044-openai-background-reasoning-vocabulary-correction.md)
  is complete and merged through PR 43 at `bdb7ea88`. Research 191 and cards
  122-123 correct exact GPT-5.6 reasoning to
  `none|low|medium|high|xhigh|max`, remove unqualified `minimal`, version the
  corrected opaque facade truth, and prove early rejection.
- [g04.045 Claude Code Headless Structured Output](./045-claude-code-headless-structured-output.md)
  stopped after card 124 and merged through PR 44 at `8a2640ea`. Research 192
  establishes draft-07 at the exact local validation boundary but admits no
  deliver-now row because runtime linkage, the full keyword subset, an
  immutable retry bound, and valid terminal/lifecycle truth remain unqualified.
  Cards 125-126 are blocked and no schema capability shipped.
- [g04.046 Gemini Live Thinking Levels](./046-gemini-live-thinking-levels.md)
  is complete and merged through PR 45 at `04cc22f2`. Research 193 and cards
  127-129 realize exact `minimal|low|medium|high` dispatch on fixed model
  `gemini-3.1-flash-live-preview`, preserve current `MINIMAL` omission bytes,
  and keep one-rollover/restoration truth under a new opaque facade point.
- [g04.047 Gemini Live Output-Token Maximum](./047-gemini-live-output-token-maximum.md)
  is complete and merged through PR 46 at `c2878262`. Research 194 and cards
  130-132 realize exact positive `1..=65_536`
  `generationConfig.maxOutputTokens` dispatch on fixed model
  `gemini-3.1-flash-live-preview`, preserve omission bytes, compose with every
  admitted thinking level, and keep one-rollover/restoration truth under the
  new opaque facade point.
- [g04.048 Gemini Live Context-Window Compression](./048-gemini-live-context-window-compression.md)
  is complete and merged through PR 47 at `47848056`. Research 195 and cards
  133-135 realize exact default-only
  `contextWindowCompression.slidingWindow = {}` dispatch on fixed model
  `gemini-3.1-flash-live-preview`, preserve omission bytes, withhold explicit
  numeric forms, and retain one-rollover/restoration truth.
- [g04.049 OpenAI Background Service Tier](./049-openai-background-service-tier.md)
  is complete and merged through PR 48 at `06c00e6c`. Research 196 and cards
  136-138 realize adapter-local explicit `service_tier: "default"` dispatch on
  ordinary attached runs and one in-process reattachment. Omission preserves
  prior bytes; detachment and selected-tier restart reconciliation fail closed;
  returned-tier observation and all other values remain withheld.
- [g04.050 DeepSeek Structured-Run Thinking Mode](./050-deepseek-structured-run-thinking-mode.md)
  is complete and merged through PR 49 at `52413da0`. Research 197 and cards
  139-141 realize adapter-local explicit `thinking.type=disabled` only for
  exact V4 Pro one-request structured runs, without a portable reasoning
  selection. Direct continuation stays enabled-only because its bounded tool
  loop requires private reasoning replay.
- [g04.051 Qwen Headless Turn And Tool Budgets](./051-qwen-headless-turn-and-tool-budgets.md)
  is complete and merged through PR 50 at `9807e322`. Research 198 and cards
  142-144 realize exact Qwen Code `0.21.15` caller-decreasing turn budgets
  `1..=24` and tool-call budgets `0..=16` across runs and every turn child.
  Omission preserves `24` / `16`; wall time and tool permissions remain fixed.
- [g04.052 Mistral Vibe Headless Maximum Turns](./052-mistral-vibe-headless-max-turns.md)
  is complete and merged through PR 51 at `2fb24536`. Research 199 and cards
  145-147 realize exact Vibe `2.24.2` caller-decreasing positive maximum turns
  `1..=8`; caller omission preserves `--max-turns 8`, and native limit remains
  provider-failed.
- [g04.053 Qoder Headless Maximum Turns](./053-qoder-headless-max-turns.md) is
  ready. Research 200 and cards 148-150 form one serial evidence-first lane for
  exact Qoder `1.1.25` caller-decreasing maximum turns. Candidate values are
  `1..=8`; caller omission preserves `--max-turns 8`. Binding and acceptance
  remain conditional on a non-empty exact deliver-now table.

## Next Planning Boundary

Execute g04.053 serially. After its evidence, review, merge, and shared
closeout, reassess the remaining promoted per-route inventory before compiling
another bounded family. Keep g04 active until explicit operator direction.

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
