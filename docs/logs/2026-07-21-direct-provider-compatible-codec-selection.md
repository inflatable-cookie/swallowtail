# 2026-07-21 Direct Provider Compatible-Codec Selection

## Changed

- revalidated Kimi Platform, DeepSeek, Z.AI, and Alibaba Model Studio from
  current official sources
- kept public API, membership, coding-plan, regional, workspace, endpoint,
  credential, entitlement, metering, and support authority separate
- selected Kimi Platform K3 as the first direct-provider breadth proof
- promoted Contract 024's structural Chat Completions codec boundary
- compiled roadmap 028 and cards 084-086 for the common codec, offline Kimi K3
  corpus, production driver, conformance, and closeout
- closed card 083 and roadmap 027

## Decision

Kimi K3 is the strongest first proof. It is the current flagship, exposes a
stateless streaming Chat Completions route, and adds an authenticated model
catalogue. The route uses a `platform.kimi.ai` API key, pay-as-you-go metering,
and the `api.moonshot.ai` audience. Kimi Membership, Kimi Code, regional keys,
subscription metering, retries, and fallback remain excluded.

Shared reuse stops at bounded JSON and SSE structure. llama.cpp and Kimi must
both pass the common codec. Provider semantics stay in their adapters.

## Remaining Risks

- Kimi K3 launched on the evidence date. Model, reasoning, output, catalogue,
  error, pricing, or access drift needs a dated corpus update
- K3 access requires top-up; catalogue presence cannot prove entitlement or
  successful inference
- DeepSeek automatically maps some Anthropic model names and ignores or maps
  fields; its later driver needs an exact native-model and field-policy corpus
- Z.AI Coding Plan and Alibaba Coding Plan restrict their audiences. Neither is
  valid for a generic Swallowtail direct driver
- Alibaba region, workspace, key, protocol, and stored Responses state remain
  separate configured boundaries

## Validation

- every provider claim is tied to official evidence accessed 2026-07-21
- Contract 024, research, roadmap, batch-card, and front-door links pass docs QA
- doctor remains at the inherited 19 findings: 12 warnings and 7 errors
- no credential, provider account, network inference request, paid call, or
  live catalogue was used

## Continuation

Card 084 is the sole ready task. Extract the provider-neutral structural codec,
prove it against llama.cpp and Kimi, and freeze the Kimi K3 corpus offline.
