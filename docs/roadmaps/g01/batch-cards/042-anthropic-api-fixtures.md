# 042 Anthropic API Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../013-anthropic-direct-inference-proof.md`

## Objective

Freeze the exact provider-supported Models and Messages HTTP/SSE subset.

## Scope

- auth/version headers, model pagination, message request, SSE event order,
  cumulative usage, rate headers, request id, errors, and cancellation limits
- deterministic fake endpoint

## Out Of Scope

- Claude Code OAuth
- live credential
- provider tools or beta features

## Acceptance Criteria

- [x] public API audience and auth are explicit
- [x] mid-stream errors and unknown events are fixture-covered
- [x] one-attempt rule is testable
- [x] card 043 needs no fresh protocol decision

## Validation

- fixture/parser tests
- `git diff --check`

## Evidence

- official authentication, API overview, versioning, Models, Messages,
  streaming, errors, and rate-limit docs rechecked on 2026-07-20
- the public subset binds `api.anthropic.com`, Console API-key auth,
  `anthropic-version: 2023-06-01`, two routes, no beta header, and no Claude
  subscription credential
- deterministic payloads cover bounded catalogue pagination, known and unknown
  token limits, explicit consumer-owned `max_tokens`, exact message streaming,
  cumulative usage, request ids, rate headers, HTTP errors, mid-stream errors,
  disconnect, and forward-compatible unknown events
- unknown top-level events are ignored; unknown content semantics fail closed
- cancellation stops local connection work only; no cancel or recovery request
  belongs to the attempt
- the fake endpoint enforces exact headers, routes, cursors, and one message
  attempt; nine focused tests pass
- Contract 014 now forbids adapter or catalogue defaults for a provider-
  required maximum-output bound
- full repository QA passes with 159 tests

## Stop Conditions

- official docs leave required stable behavior unresolved
- first proof requires a beta API

## Auto-Continuation

Yes, only after card 043 is marked ready.
