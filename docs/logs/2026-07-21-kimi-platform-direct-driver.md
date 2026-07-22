# 2026-07-21 Kimi Platform Direct Driver

## Outcome

Card 085 is complete. Swallowtail now has a production Kimi Platform driver
separate from the Kimi Code ACP harness adapter.

## Realized Boundary

- crate: `swallowtail-adapter-kimi-platform`
- driver: `swallowtail.kimi-platform.direct-chat`
- transport: host-approved HTTP/SSE
- audience: exact `api.moonshot.ai`
- credential: leased public-platform API key
- catalogue: one bounded authenticated `GET /v1/models`
- inference: one `POST /v1/chat/completions` attempt
- route: exact Moonshot `kimi-k3`
- inputs: one text message, explicit `low`, `high`, or `max` reasoning, and a
  positive output bound no larger than the frozen K3 context bound

The driver consumes the common compatible-chat codec for structural request,
JSON, SSE, unknown-field, disconnect, and `[DONE]` handling. Provider access,
catalogue projection, K3 reasoning, returned-model agreement, errors, event
order, cancellation, deadline, cleanup, and retry posture remain adapter-owned.

## Evidence

Six protocol tests and six production-driver tests pass without a provider
credential or external request. The local fixture server proves the frozen
request, bearer header, one-attempt bound, reasoning/output/usage order,
provider failure, unknown semantics, model mismatch, partial-record disconnect,
cancellation, elapsed and in-flight deadline, redaction, and awaited credential
release after connection join.

Focused warnings-denied clippy and all-target workspace compilation pass. No
common runtime or codec type carries Kimi identity. No SDK retry, reconnect,
provider state, route alias, subscription access, regional key, or fallback was
added. The first doctor pass found two new warning-level oversized files; both
were split before closeout. Doctor returns to the inherited 19 findings: 12
warnings and seven errors.

## Continuation

Card 086 remains in bounds. It runs the unchanged hosted-direct conformance
profile under local and remote-authoritative execution hosts, regresses the
common codec and llama.cpp consumer, closes roadmap 028, and returns provider
breadth selection to DeepSeek, Z.AI, and Alibaba Model Studio.
