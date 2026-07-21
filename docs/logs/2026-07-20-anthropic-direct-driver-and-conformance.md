# Anthropic Direct Driver And Conformance

Date: 2026-07-20
Roadmap: 013
Cards: 043-044

## Changed

- added `OutputTokenLimit` capability and a nonzero maximum-output value on
  structured-run requests
- added opaque provider request correlation and explicit keepalive runtime
  observations
- implemented Anthropic public-API catalogue and structured-run roles
- implemented bounded libcurl HTTP/SSE through host blocking-work and task
  services
- normalized output, cumulative usage, rate limits, request id, safe failure,
  cancellation, deadline, and cleanup behavior
- extended the provider-neutral hosted-direct profile with explicit output-
  bound coverage

## Boundaries

- Console API key only; Workload Identity Federation and subscription OAuth
  remain out
- exact endpoint, audience, provider, model, version, and maximum output stay
  preflight or request bound
- one inference attempt; no SDK, retry, recovery, or fallback path
- cancellation stops local consumption and closes owned connection work; it
  does not claim provider-side cancellation
- absolute rate-reset timestamps remain unknown in the relative reset field
- unknown top-level SSE events are ignored; unknown output semantics fail
- no process or working-resource service

## Evidence

- 18 focused adapter, fixture, lifecycle, and hosted-profile tests pass
- full repository QA passes with 168 tests; the separately gated OpenCode
  installed probe remains ignored by default
- success, provider failure, unknown event, and cancellation each release the
  scoped secret after connection cleanup
- dependency and source scans show no consumer dependency, raw provider
  diagnostic, SDK retry layer, or ambient credential access
- doctor remains at the pre-existing 19 findings: 12 warnings and 7 errors;
  new files add none

## Continuation

Roadmap 013 is complete. Card 045 is ready for ACP v1 authority and Gemini
fixtures. Cards 046-050 remain in bounds.
