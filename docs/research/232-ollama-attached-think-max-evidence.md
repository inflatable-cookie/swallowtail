# 232 Ollama Attached Think Max Evidence

Status: promoted
Owner: Tom
Date: 2026-08-27
Card: g04.082 / 231

## Question

Which exact `ollama.attached` version, selected model, operation, and lifecycle
rows can dispatch native `think: "max"` with exact membership and without
silent clamp, default, template substitution, or inference from generic
thinking support?

## Method

Inspected exact official tagged source at `0.14.0`, `0.18.0`, `0.20.0`,
`0.21.2`, `0.22.0`, `0.30.0`, `0.32.1`, `0.32.14`, and `0.32.15`; froze
retrieval-dated current official thinking and chat API Markdown specimens; and
audited the prepared route's
already-bound selected-model detail parser, preparation validation, protocol
encoder, fixtures, and guide without changing production surfaces. No runtime was
started and no model request, authentication, install, pull, unload, or host
change ran.

Frozen corpus:
`crates/swallowtail-adapter-ollama/tests/fixtures/ollama-think-max-v0.14.0-v0.32.15/`.

## Tagged source identity

| Version | Tag | Commit | `api/types.go` SHA-256 | Wire parser accepts `"max"` |
| --- | --- | --- | --- | --- |
| `0.14.0` | `v0.14.0` | `02a24015968d612b418448b73cffaa1b0652d161` | `641793496655f5d55905022f89d5ad0ee0df36b5efb0c8901c91eaec9bb7cff6` | no |
| `0.18.0` | `v0.18.0` | `3980c0217d27e05a441808a446e7ee5ea7e04256` | `9189af46efbaa269252477851d879b9e8b5b789e3c831389ed4cb2fc6be49c94` | no |
| `0.20.0` | `v0.20.0` | `de9673ac3fb1c57fbf6e5e194f1f3dc5a8b48668` | `b408e7601f5434ab14d0d50c570e76ab43dff45c57bbf293e67288cdec02e4eb` | no |
| `0.21.2` | `v0.21.2` | `590109c8352e8d5a6206e8909b518a54a2b0a7b8` | `b408e7601f5434ab14d0d50c570e76ab43dff45c57bbf293e67288cdec02e4eb` | no |
| `0.22.0` | `v0.22.0` | `955112e502e34812a904e6392736d8cc40bbb9d9` | `f71977722972a9eb6742aece9c95fd6e828352e17ee0fe10f7be7eb7ccdff9a4` | yes |
| `0.30.0` | `v0.30.0` | `2c71d8d7ca6edbc9bdc1a312f71ce3b079c0fe56` | `a4f8c934d7592611238d318fb1486fc0ca6db4f779f7d9312cdf378d41ceb3aa` | yes |
| `0.32.1` | `v0.32.1` | `30c390384e20333b67cadab60da5bcb669407f01` | `a54975f59ef7240841744bead00efe4158eb0369860a65e46bf1853f2a67b592` | yes |
| `0.32.14` | `v0.32.14` | `d67ad83426633195089509347ffd4fe795120198` | `032fe8c044429afd42fd9f898c6bbd6efc5977ffeeec4dd3c5a04035e9c3d0b1` | yes |
| `0.32.15` | `v0.32.15` | `b7871fc0d1d82fe109536efa3e0e8e411c766c75` | `032fe8c044429afd42fd9f898c6bbd6efc5977ffeeec4dd3c5a04035e9c3d0b1` | yes |

`"max"` entered the tagged `ThinkValue` parser at `v0.22.0`. The immediate
stable predecessor `v0.21.2` is byte-identical to `v0.20.0` for `api/types.go`
and rejects it during JSON unmarshaling with an invalid think value error. From
`v0.22.0` through `0.32.15`, the inspected qualification points accept
`"max"` beside `"high"`, `"medium"`, and `"low"`.

Wire-parser acceptance is not exact selected-model membership.

## Selected-model detail boundary

The prepared route binds selected-model detail from `/api/show` through
`parse_model_detail`, which reads only:

- `capabilities` as generic `"completion"` or `"thinking"`
- `details.format == "gguf"`
- non-empty `details.family`
- absence of remote/cloud fields

It does not bind `parser`, `template`, `modelfile`, or any thinking-level list.
Official `ShowResponse` at `v0.32.15` exposes generic `capabilities:
["thinking"]` only. That boolean-style capability advertises thinking support,
not `"max"` membership for a selected model.

Generic thinking capability therefore cannot close a deliver-now `max` row.

## Server-side substitution and model variance

Tagged `server/routes.go` at `v0.32.15` silently rewrites requested `"max"`
to `"high"` on harmony/gpt-oss family models before builtin parser
initialization. Current official thinking documentation states GPT-OSS expects
only `low`, `medium`, or `high`, while other thinking models may accept
`max`. Model-family names, reasoning output, and wire-parser acceptance are
insufficient to prove exact selected-level dispatch without the already-bound
detail surface exposing level membership.

## Swallowtail preparation and encoding

Production preparation admits reasoning only when selected-model detail
advertises generic thinking and the requested mode is exactly
`off|low|medium|high`. `max` fails closed at preparation with
`swallowtail.ollama.preparation.reasoning_unsupported` before chat dispatch.
The native encoder maps `off` to boolean `false` and other admitted modes to
strings. Omission and the four admitted modes remain unchanged.

Interactive transcript replay exposes no reasoning selector on
`OllamaSessionProfileInput`; reasoning control is structured-run only on the
prepared route.

## Required decision

**Honest empty deliver-now set.**

No version/model/template row on the already-bound selected-model detail and
preparation surface can prove exact `think: "max"` membership without inferring
from generic thinking, family names, template text, or live inference.

## Operation profile disposition

| Profile | Disposition | Notes |
| --- | --- | --- |
| structured inference with `max` | withheld | preparation rejects before dispatch |
| structured inference with `off\|low\|medium\|high` | unchanged | still admitted when generic thinking is present |
| structured inference omission | unchanged | no reasoning field on native request |
| interactive transcript replay | not-applicable | session profile has no reasoning selector |
| inventory observation | not-applicable | no `/api/chat` request |

## Distinction table

| Stage | `max` truth |
| --- | --- |
| requested | not admitted by Swallowtail preparation |
| encoded | not reached |
| server JSON parser (`>=0.22.0`) | accepts string `"max"` at wire layer |
| server JSON parser (`<=0.20.0`) | rejects string `"max"` |
| template-applied / server-accepted | model-dependent; harmony path may rewrite to `high` |
| effective / observed | evidence-gated; reasoning output is not selected-level confirmation |

## Primary sources

- exact tagged `api/types.go` and `server/routes.go` on the qualification points
  above
- [Ollama thinking capability Markdown](https://docs.ollama.com/capabilities/thinking.md),
  retrieved 2026-08-27, SHA-256
  `bc432ace302a74e67126f8a6eee6ff135b35ba4a66718eac3fb5ec80e6461b08`
- [Ollama chat API Markdown](https://docs.ollama.com/api/chat.md), retrieved
  2026-08-27, SHA-256
  `dc0f10c93b1ecf86dfa010f580a1585cff2ff39374fb976853c5b491fae2e35e`
- the documentation specimens are mutable retrieval-date snapshots, not
  evidence tied to tag `v0.32.15`
- existing `ollama.native-text-v1` fixture boundary and Research 184 num-ctx
  corpus for shared `/api/chat` route context
- production `validate_reasoning` and `parse_model_detail` in
  `swallowtail-adapter-ollama`

## Unresolved for future binding

Exact per-model `max` membership would require a new already-bound static fact
— for example an explicit thinking-level list on selected-model detail — plus
proof that dispatch preserves the requested value without harmony substitution
or model-specific clamping. That surface is absent today.
