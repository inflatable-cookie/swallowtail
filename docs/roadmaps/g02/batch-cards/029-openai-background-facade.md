# 029 OpenAI Background Facade

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../010-hosted-direct-and-provider-state-facades.md`

## Objective

Add a prepared facade for OpenAI public-API background Responses.

## Governing Refs

- Contracts 014, 020-021, 029-030, and 037
- OpenAI background fixtures
- card 028

## Scope

1. Prepare the public API endpoint, API-key audience, model route, output bound,
   retention agreement, and reattachment policy.
2. Bind create, bounded stream reattach, retrieve, and native cancel.
3. Keep required temporary retention explicit even with `store=false`.
4. Preserve ChatGPT, Codex, subscription, and community OAuth separation.
5. Add no automatic reattachment, retry, or credential fallback.

## Acceptance Criteria

- [x] background mode remains opt-in
- [x] maximum-one reattachment remains visible
- [x] cancellation race and remote truth remain explicit
- [x] temporary retention cannot be mistaken for durable consumer storage
- [x] public API access is the only represented audience

## Validation

- deterministic background-run corpus
- retention, reattach, cancel, deadline, usage, rate, and cleanup cases
- both host identities

## Auto-Continuation

Yes. Continue to card 030.
