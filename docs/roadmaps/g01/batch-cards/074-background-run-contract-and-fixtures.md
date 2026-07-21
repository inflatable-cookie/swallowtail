# 074 Background Run Contract And Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../023-openai-background-responses-proof.md`

## Objective

Promote the provider-owned background-run boundary selected by Research 015
and freeze the exact OpenAI Responses background HTTP/SSE corpus.

## Scope

- Contract 021 provider operation, local attachment, cursor, cancellation,
  retention, credential, and cleanup rules
- the smallest provider-neutral records or requirements needed for explicit
  background execution and temporary provider retention
- fixture-first `swallowtail-adapter-openai` crate or exact equivalent driver
  boundary
- dated create, retrieve, streaming reattach, cancel, status, usage, rate,
  failure, and redaction corpus
- loopback HTTP/SSE fixtures for disconnect, cursor recovery, duplicate or
  missing sequence, terminal race, unknown event, and unconfirmed remote stop
- exact OpenAI public-API endpoint audience, API-key access, API billing, and
  provider-support pins
- explicit exclusion of ChatGPT or Codex login, subscription OAuth, tools,
  search, files, conversations, webhooks, retry, fallback, and live access

## Acceptance Criteria

- [x] provider operation, runtime run, HTTP request, SSE attachment, provider
      cursor, and common event sequence remain separate
- [x] temporary provider retention is explicitly selected before effects
- [x] reattachment is bounded and cannot become inference retry or replay
- [x] cancellation truth includes confirmed, raced, and unconfirmed remote
      stop outcomes
- [x] deterministic fixtures fail closed on provider schema or event drift
- [x] default QA requires no credential, account, external request, or paid
      inference
- [x] card 075 has no unresolved shared-contract or access-policy decision

## Evidence

- Contract 021 fixes optional background execution, temporary retention,
  bounded reattachment, identity separation, cancellation truth, access, and
  joined cleanup.
- Core/runtime records add three opt-in capabilities, exact reattachment
  bounds, explicit operation policy, and provider cancellation outcomes.
- Existing structured-run drivers reject the new posture instead of silently
  ignoring it.
- `swallowtail-adapter-openai` freezes the dated public-API request, status,
  SSE cursor, failure, access, usage, rate, and redaction corpus.
- Loopback fixtures prove one create attempt remains separate from retrieve,
  reattach, and cancel management requests.
- Focused tests and warnings-denied clippy pass without live OpenAI access.

## Validation

- focused core, runtime, fixture, and adapter tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- current official evidence no longer supports background stream reattachment
  or native cancel
- the exact subset requires durable consumer persistence or a webhook policy
- `store=false` would be represented as no provider retention
- fixtures require live provider access

## Auto-Continuation

Continue to card 075 only when Contract 021 and the deterministic corpus leave
no production-lifecycle or access-policy decision unresolved.
