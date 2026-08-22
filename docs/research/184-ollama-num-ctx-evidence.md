# 184 Ollama Num Ctx Evidence

Status: promoted
Owner: Tom
Date: 2026-08-22
Card: g04.036 / 098

## Question

Which exact positive `options.num_ctx` values and operation profiles can
Swallowtail bind on the qualified local `ollama.attached` `/api/chat` route
without a live runtime, generic options map, or portable context-window
capability?

## Method

Inspected exact official tagged source at `0.14.0`, `0.18.0`, `0.30.0`,
`0.32.1`, `0.32.14`, and `0.32.15`; froze current official FAQ and native API
documentation specimens; and confirmed the prepared route rejects remote/cloud
model detail. No runtime was started and no model request, authentication,
install, pull, unload, or host change ran.

Frozen corpus:
`crates/swallowtail-adapter-ollama/tests/fixtures/ollama-num-ctx-v0.14.0-v0.32.15/`.

## Tagged source identity

| Version | Tag | Commit | `api/types.go` | `server/sched.go` | `llm/server.go` |
| --- | --- | --- | --- | --- | --- |
| `0.14.0` | `v0.14.0` | `02a24015968d612b418448b73cffaa1b0652d161` | `641793496655f5d55905022f89d5ad0ee0df36b5efb0c8901c91eaec9bb7cff6` | `00de242cf8f8b21e9305fb041edea1b6d4c297418ff1a2144778b7288372eb54` | `fe4093ddb77839b41991ffc6e387a9f4b4300cfe253a7b9f4779997d208b25bc` |
| `0.18.0` | `v0.18.0` | `3980c0217d27e05a441808a446e7ee5ea7e04256` | `9189af46efbaa269252477851d879b9e8b5b789e3c831389ed4cb2fc6be49c94` | `ce2253490c6e69c678809deb89b022948f4af578747093980587dd13ee6ad31d` | `a0d819036ce9093cc4705bd3a7636171769da855d2b8428e18c3911a802b195d` |
| `0.30.0` | `v0.30.0` | `2c71d8d7ca6edbc9bdc1a312f71ce3b079c0fe56` | `a4f8c934d7592611238d318fb1486fc0ca6db4f779f7d9312cdf378d41ceb3aa` | `51b182c15b5885c52a2cfb103934e7011e8bb81e9567431729c988186305c828` | `1709459fc5b6a7c0cc598c3464ba9f6ca8a0b5bbde5bb85e750a9cfc0e797d1e` |
| `0.32.1` | `v0.32.1` | `30c390384e20333b67cadab60da5bcb669407f01` | `a54975f59ef7240841744bead00efe4158eb0369860a65e46bf1853f2a67b592` | `5034d03652f77641cb9733f44b8a34739c81ab3e65f5f7d1d7886dbf362c2429` | `1709459fc5b6a7c0cc598c3464ba9f6ca8a0b5bbde5bb85e750a9cfc0e797d1e` |
| `0.32.14` | `v0.32.14` | `d67ad83426633195089509347ffd4fe795120198` | `032fe8c044429afd42fd9f898c6bbd6efc5977ffeeec4dd3c5a04035e9c3d0b1` | `e4058db54c61b1af1a0c1799836731adbbf855d59c78757cce81d9d7dca3a07b` | `a695536408125cb1b27aaba36aade99c6ae04f7e4c07adc1dc48f0c2db036154` |
| `0.32.15` | `v0.32.15` | `b7871fc0d1d82fe109536efa3e0e8e411c766c75` | `032fe8c044429afd42fd9f898c6bbd6efc5977ffeeec4dd3c5a04035e9c3d0b1` | `e4058db54c61b1af1a0c1799836731adbbf855d59c78757cce81d9d7dca3a07b` | `a695536408125cb1b27aaba36aade99c6ae04f7e4c07adc1dc48f0c2db036154` |

Across every qualification point:

- `ChatRequest` carries `options` as `map[string]any`
- `Options` embeds `Runner` with `NumCtx int \`json:"num_ctx,omitempty"\``
- `Options.FromMap` accepts JSON integers as `int64` or `float64`
- `/api/chat` remains the native request surface

## Official documentation specimens

| Surface | Source | Frozen specimen |
| --- | --- | --- |
| FAQ context-window API example | `docs/faq.mdx` at `v0.32.15` | `faq-num-ctx-v0.32.15.mdx.excerpt` |
| Native API generate options example | `docs/api.md` at `v0.32.15` | `api-generate-options-num-ctx-v0.32.15.md.excerpt` |
| Scheduler minimum and vision floor | `server/sched.go` at `v0.32.15` | `sched-get-runner-num-ctx-v0.32.15.go.excerpt` |
| Training-context clamp | `llm/server.go` at `v0.32.15` | cited in corpus metadata with per-tag digests above |

Current official URLs checked on 2026-08-22:

- https://docs.ollama.com/api/chat
- https://docs.ollama.com/api/generate
- https://docs.ollama.com/faq

## Numeric domain

| Input | Disposition |
| --- | --- |
| positive integer `4..=2147483647` | deliver-now |
| `1..=3` | wire-only; exact `v0.32.15` scheduler raises to `4` before runner scheduling |
| zero | fail closed before dispatch |
| negative representation | fail closed before dispatch |
| values above `2147483647` | fail closed before dispatch |
| values above `4294967295` | fail closed before dispatch |

Swallowtail encodes the exact selected positive integer as JSON `num_ctx` beside
`num_predict`. The admitted ceiling is `i32::MAX` because the tagged wire field
is a signed Go `int`; dispatch does not prove the value survives scheduler
minimum floors, vision-model raises, training-context clamps, reload, or
effective allocation.

Exact `v0.32.15` scheduler truth on the qualified attached text route:

- `opts.NumCtx < 4` becomes `4` in `Scheduler.getRunner`
- vision-capable models raise to at least `2048`; the prepared text subset does
  not admit vision-only selection through this route
- `llm.NewLlamaServer` clamps above model training context with a warning

## Operation profile disposition

| Profile | Disposition | Notes |
| --- | --- | --- |
| structured inference | deliver-now | one fixed value per prepared attempt |
| interactive transcript replay | deliver-now | one fixed value bound at session preparation and dispatched on every clean replay turn and fresh restoration |
| inventory observation | not-applicable | no `/api/chat` request |
| remote/cloud model detail | obsolete on prepared route | local attached route rejects cloud tags |

## Withheld surfaces

| Surface | Disposition |
| --- | --- |
| `/api/generate` | intentionally withheld from binding; documentation retained as sibling evidence only |
| OpenAI-compatible endpoints | intentionally withheld |
| Ollama Cloud | intentionally withheld |
| `OLLAMA_CONTEXT_LENGTH` / Modelfile / CLI `ollama run` | not-applicable to request-local dispatch |
| catalogue-derived context defaults | evidence-gated |
| effective allocation or memory-fit claims | evidence-gated |

## Claim boundary

Swallowtail may claim qualified dispatch of the exact positive integer inside
the native `options` object only. Dispatch does not prove provider acceptance,
effective context allocation, truncation outcome, or resource feasibility.

## Primary sources

- exact tagged `api/types.go`, `server/sched.go`, and `llm/server.go` on all six
  qualification points
- [Ollama chat API](https://docs.ollama.com/api/chat)
- [Ollama generate API](https://docs.ollama.com/api/generate)
- [Ollama FAQ](https://docs.ollama.com/faq)
- existing `ollama.native-text-v1` fixture boundary for `/api/chat`, reasoning,
  structured output, and interactive replay
