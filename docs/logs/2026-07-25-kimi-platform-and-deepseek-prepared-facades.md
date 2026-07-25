# 2026-07-25 Kimi Platform And DeepSeek Prepared Facades

Status: complete

## Changed

`swallowtail-adapter-kimi-platform` now exposes separate prepared catalogue and
one-attempt K3 structured-run values. Preparation binds one approved endpoint
target, exact `api.moonshot.ai` audience, provider-supported pay-as-you-go
Platform API-key profile, dated facade, access provenance, and execution host.
Inference requires exact `kimi-k3`, explicit reasoning, content, output bound,
and optional deadline.

`swallowtail-adapter-deepseek` now exposes separate prepared catalogue and
direct-continuation session values. Preparation binds the exact public
endpoint, audience, Open Platform API-key profile, opaque dated facade claim,
and V4 Pro route. Session preparation requires high reasoning, declared tools,
and explicit acceptance of provider-managed caching without management
authority.

Both adapters retain `plan`, `request`, `evidence`, `low_level_driver`, and
`into_parts`. The low-level roles remain independently callable.

## Current Evidence

Official Kimi documentation still names `https://api.moonshot.ai`, bearer
Platform keys, `GET /v1/models`, `POST /v1/chat/completions`, and `kimi-k3`.
It now documents additional K3 tool workflows, but that does not widen the
frozen one-attempt Swallowtail route:

- [Kimi API overview](https://platform.kimi.ai/docs/api/overview)
- [Kimi K3](https://platform.kimi.ai/docs/guide/kimi-k3-quickstart)
- [Kimi model catalogue](https://platform.kimi.ai/docs/api/list-models)

Official DeepSeek documentation still names the public endpoint, V4 Pro and
Flash, thinking-mode tool calls, and required `reasoning_content` replay. The
documented retirement cutoff for `deepseek-chat` and `deepseek-reasoner` has
passed. No live alias request was made:

- [DeepSeek models and pricing](https://api-docs.deepseek.com/quick_start/pricing)
- [DeepSeek thinking mode](https://api-docs.deepseek.com/guides/thinking_mode/)
- [DeepSeek tool calls](https://api-docs.deepseek.com/guides/tool_calls)

This revalidation strengthens the exact-model boundary. It does not require a
new contract or facade revision.

## Native Boundaries

- Kimi Membership, Kimi Code, regional Platform keys, and subscription
  metering do not satisfy the Kimi Platform profile
- Kimi catalogue observations do not select a model or prove top-up
- Kimi prepared inference declares no tools or direct continuation
- DeepSeek catalogue observations do not select V4 Pro
- DeepSeek private reasoning remains adapter-held and non-portable
- a user turn authorizes its first attempt; correlated tool-result submission
  alone authorizes another
- DeepSeek cache acceptance grants no read, deletion, retention, or retry
  authority
- compatible JSON/SSE structure grants no cross-provider route, credential,
  model, lifecycle, or fallback

## Validation

- Kimi Platform: 17 unit, driver, conformance, prepared-facade, and example
  targets pass
- DeepSeek: 17 unit, driver, conformance, prepared-facade, and example targets
  pass
- prepared operations execute under local and remote-authoritative host
  identities
- alternate models, Kimi membership metering, and unaccepted DeepSeek cache
  posture fail before effects
- credential releases occur after joined blocking work
- full Effigy QA passes
- Doctor remains at the known 19 oversized-file findings: 7 errors and 12
  warnings

## Next

Card 028 adds the Alibaba Model Studio provider-owned conversation facade.
Cards 028-036 remain in the provider-wide facade, package-proof, and
replacement-candidate runway.
