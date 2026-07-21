# Hosted Transport Foundations

Date: 2026-07-20
Roadmap: g01 011
Cards: 035-038

## Outcome

- repaired stale roadmap, batch-card, and front-door currentness
- refreshed harness, direct API, ACP, and self-hosted evidence
- selected attached OpenCode HTTP/SSE plus Anthropic Messages as the first
  cross-adapter tranche
- promoted Contract 014 and compiled g01 roadmaps 011-015
- bound endpoint grants and credential leases to operation scope, opaque
  reference, and endpoint audience
- added explicit awaited credential release and concrete local secret/delegated
  approvals without ambient store scanning
- made structured-run working resources optional while keeping Codex fail-closed
- added mutable model token limits and typed usage, rate, and quota observations

## Evidence

- maintained OpenCode docs and installed `1.14.48` disagree on default port;
  exact host-approved endpoint binding replaces default inference
- Anthropic official docs confirm public-API credentials, versioned requests,
  paginated models, SSE, mid-stream errors, cumulative usage, request ids, and
  rate observations
- hosted conformance proves endpoint/credential binding, awaited release,
  no-resource direct runs, provider-evidence separation, and no process service
- Codex regression tests prove its process request still receives the same
  explicit working resource and typed usage
- full `effigy qa` passes with 129 tests
- `effigy doctor` remains at the prior 19 oversized-file findings, including 7
  errors; hosted implementation was split to avoid adding new findings

## Remaining Risks

- no HTTP request or SSE parser exists outside Codex process transport yet
- local endpoint approvals preserve the exact host string but do not impose a
  universal URI/proxy/TLS client abstraction
- local secret approvals are explicit in-memory host configuration, not a
  durable credential-store integration
- OpenCode protocol stability, permission mapping, and event correlation still
  require card 039 fixtures
- Anthropic live authentication remains separately gated and must never reuse
  Claude Code subscription OAuth

## Next

Execute card 039. Freeze the attached OpenCode HTTP/SSE subset before adapter
implementation.
