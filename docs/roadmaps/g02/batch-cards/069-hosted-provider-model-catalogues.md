# 069 Hosted Provider Model Catalogues

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../021-model-catalogue-coverage.md`

## Objective

Add source-scoped hosted catalogue branches for the existing OpenAI, Gemini,
xAI, and Alibaba Model Studio direct solutions.

## Scope

1. Freeze OpenAI `GET /v1/models`, Gemini paginated `models.list`, xAI
   `/v1/language-models`, and Alibaba deployable-model corpora.
2. Bind each provider's existing public endpoint audience and credential
   profile without cross-provider codec authority.
3. Project only bounded documented metadata.
4. Add typed prepared catalogue branches to the applicable adapter facades.
5. Review whether shared provider preparation justifies composite OpenAI or
   Gemini facades; do not combine them from corporate ownership alone.
6. Keep catalogue presence separate from Realtime, Live, background, or
   WebSocket route compatibility.

## Acceptance Criteria

- [x] no catalogue entry constructs an invocation route
- [x] pagination, deadline, redaction, and credential release
      are deterministic
- [x] catalogue and inference endpoint audiences remain exact
- [x] one provider result cannot widen another provider or operation
- [x] no live authentication, paid inference, or publication is required

Portable consumer cancellation remains outside `ModelCatalogRequest`; each
operation retains deadline and joined cleanup truth without inventing it.

## Evidence

- provider-specific bounded decoders for official OpenAI, Gemini, xAI, and
  Alibaba response shapes
- separate descriptors, configured instances, access profiles, facade
  bindings, immutable plans, and typed prepared operations
- Gemini and Alibaba bounded pagination
- no inference route construction or compatibility inference

## Auto-Continuation

No. Return for provider-facade and remaining-`No` review.
