# 2026-07-22 Alibaba Conversation Driver And Conformance

## Changed

- Added a production Alibaba Model Studio direct-conversation driver with one
  host-approved endpoint grant and API-key lease per session.
- Implemented provider conversation creation, two serial Responses turns over
  HTTP/SSE, bounded events, local cancellation, deadlines, and no resume.
- Implemented complete item inventory, item deletion, then conversation
  deletion before credential release.
- Added deterministic local and remote-authoritative conformance fixtures.

## Lifecycle Evidence

- The complete success transcript has nine exact requests: conversation
  creation, two Responses attempts, item inventory, four item deletions, and
  conversation deletion.
- Parallel and third turns reject before provider effects.
- Provider failure, disconnect, cancellation, deadline, cleanup failure, and
  protocol failure remain distinct.
- Cancellation and deadline close and join local connection work, end the
  session, and leave provider stop plus item and conversation deletion
  unconfirmed.
- Cleanup continues to conversation deletion after an inventory or item-delete
  failure. Credential release follows all joined work.

## Boundaries

- The proof uses the Singapore workspace-dedicated endpoint, a general Model
  Studio API key, pay-as-you-go access, and exact model
  `qwen3.7-plus-2026-05-26`.
- It does not add catalogue discovery, resume, native cancellation, tools,
  cache, retry, fallback, aliases, Coding Plan, or Token Plan access.
- It makes no live authenticated request. Live authentication remains
  separately gated.

## Validation

- All 16 Alibaba tests pass.
- Warnings-denied package clippy passes.
- Full repository QA passes with 404 tests; three installed or live probes
  remain gated.
- New files remain below the repository size threshold.
- Doctor remains at the inherited 19 findings: 12 warnings and 7 errors.

## Risks

- The provider route has no native cancellation operation. Local cancellation
  cannot confirm remote stop or deletion after a race.
- The proof covers one exact regional workspace and general-key audience. It
  is not evidence for other regions, plans, endpoints, credentials, or models.

## Continuation

Roadmap 030 and card 090 own the sole ready checkpoint. Revalidate and rank the
remaining coverage before another provider implementation.
