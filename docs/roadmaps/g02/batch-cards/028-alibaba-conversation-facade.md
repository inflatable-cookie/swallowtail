# 028 Alibaba Conversation Facade

Status: complete
Owner: Tom
Created: 2026-07-25
Milestone: `../010-hosted-direct-and-provider-state-facades.md`

## Objective

Add a prepared facade for provider-owned Alibaba Model Studio conversations.

## Governing Refs

- Contracts 014, 020, 025, 029-030, and 037
- Alibaba conversation fixtures
- card 027

## Scope

1. Prepare the exact regional workspace, endpoint audience, credential source,
   route, model, and retention posture.
2. Bind create, continue, inspect, and deletion operations with typed provider
   ownership.
3. Preserve item-before-conversation deletion ordering.
4. Keep consumer memory and provider conversation state separate.
5. Expose cancellation and deletion truth without fabrication.

## Acceptance Criteria

- [x] regional workspace access remains explicit
- [x] provider retention is visible before creation
- [x] private continuation stays route and access bound
- [x] deletion truth follows provider evidence
- [x] no durable consumer state is inferred

## Evidence

- exact Singapore workspace access, route, model, retention, and immutable
  open plan preparation
- bound open delegates to the unchanged provider-conversation driver
- local and remote-authoritative two-turn execution
- complete item inventory, item-first deletion, separate conversation
  deletion, joined cleanup, and credential-last release
- 18 adapter tests and example targets
- `../../../logs/2026-07-25-alibaba-model-studio-prepared-conversation.md`

## Validation

- deterministic provider-conversation corpus
- deletion, cancellation, drift, redaction, and cleanup cases
- local and remote-authoritative host fixtures

## Auto-Continuation

Yes. Continue to card 029.
