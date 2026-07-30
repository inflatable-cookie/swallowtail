# 163 Claude Typed User-Input Exchange

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../048-claude-agent-form-elicitation.md`

## Goal

Expose losslessly representable Claude form elicitation through the common
typed callback exchange.

## Scope

1. Advertise `clientCapabilities.elicitation.form`.
2. Dispatch `elicitation/create`.
3. Validate session, active turn, request id, count, size, schema, and fields.
4. Decode both qualified option-description revisions.
5. Preserve single, multiple, Other, and skipped answers.
6. Translate `HarnessUserInputResponse` into accepted ACP content.
7. Translate consumer failure into decline.
8. Decline richer forms without flattening.

## Acceptance Criteria

- [x] callback request precedes `CallbackRequested`
- [x] exact provider request and turn correlation preserved
- [x] responses accepted once
- [x] foreign, duplicate, malformed, and richer forms stay off provider wire
- [x] cancellation and terminal completion abandon pending callbacks
- [x] diagnostics expose no question, answer, form, or raw payload

## Validation

- `effigy validate:focused swallowtail-protocol-acp
  swallowtail-adapter-claude-agent`
- `git diff --check`

## Auto-Continuation

Yes. Continue to card 164.

## Evidence

- `elicitation/create` dispatch and form capability negotiation
- typed single-, multiple-, Other-, and skipped-answer projection
- exactly-once provider request correlation and terminal abandonment
- richer and preview-bearing form decline
- 147 focused protocol and adapter tests
