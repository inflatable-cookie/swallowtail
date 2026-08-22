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
`0.32.1`, `0.32.14`, and `0.32.15`; froze current official chat API
documentation; and confirmed the prepared route rejects remote/cloud model detail.
No runtime was started and no model request, authentication, install, pull,
unload, or host change ran.

Frozen corpus:
`crates/swallowtail-adapter-ollama/tests/fixtures/ollama-num-ctx-v0.14.0-v0.32.15/`.

## Tagged source identity

| Version | Tag | Commit | `api/types.go` SHA-256 |
| --- | --- | --- | --- |
| `0.14.0` | `v0.14.0` | `02a24015968d612b418448b73cffaa1b0652d161` | `641793496655f5d55905022f89d5ad0ee0df36b5efb0c8901c91eaec9bb7cff6` |
| `0.18.0` | `v0.18.0` | `3980c0217d27e05a441808a446e7ee5ea7e04256` | `9189af46efbaa269252477851d879b9e8b5b789e3c831389ed4cb2fc6be49c94` |
| `0.30.0` | `v0.30.0` | `2c71d8d7ca6edbc9bdc1a312f71ce3b079c0fe56` | `a4f8c934d7592611238d318fb1486fc0ca6db4f779f7d9312cdf378d41ceb3aa` |
| `0.32.1` | `v0.32.1` | `30c390384e20333b67cadab60da5bcb669407f01` | `a54975f59ef7240841744bead00efe4158eb0369860a65e46bf1853f2a67b592` |
| `0.32.14` | `v0.32.14` | `d67ad83426633195089509347ffd4fe795120198` | `032fe8c044429afd42fd9f898c6bbd6efc5977ffeeec4dd3c5a04035e9c3d0b1` |
| `0.32.15` | `v0.32.15` | `b7871fc0d1d82fe109536efa3e0e8e411c766c75` | `032fe8c044429afd42fd9f898c6bbd6efc5977ffeeec4dd3c5a04035e9c3d0b1` |

Across every qualification point:

- `ChatRequest` carries `options` as `map[string]any`
- `Options` embeds `Runner` with `NumCtx int \`json:"num_ctx,omitempty"\``
- `Options.FromMap` accepts JSON integers as `int64` or `float64`
- `/api/chat` remains the native request surface

## Numeric domain

| Input | Disposition |
| --- | --- |
| positive integer `1..=4294967295` | deliver-now |
| zero | fail closed before dispatch |
| negative representation | fail closed before dispatch |
| values above `u32::MAX` | fail closed before dispatch |

Swallowtail encodes the exact selected positive integer as JSON `num_ctx` beside
`num_predict`. Ollama may apply server defaults when the field is absent, clamp
to model training context, reload the runner, truncate prompts, or reject the
request. Those outcomes remain provider/runtime truth.

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
| `/api/generate` | intentionally withheld |
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

- exact tagged `api/types.go` on all six qualification points
- [Ollama chat API](https://docs.ollama.com/api/chat)
- existing `ollama.native-text-v1` fixture boundary for `/api/chat`, reasoning,
  structured output, and interactive replay
