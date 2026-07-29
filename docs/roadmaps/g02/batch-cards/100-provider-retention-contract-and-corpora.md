# 100 Provider Retention Contract And Corpora

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../030-provider-retention-feature-closure.md`
Depends on: card 099

## Objective

Promote the smallest shared rules and freeze exact offline corpora for the
provider-retention tranche selected by card 099.

## Scope

1. Use only exact routes selected by card 099.
2. Confirm existing contracts before adding any shared record.
3. Freeze version, binding, initial-state, action, deletion-strength,
   descendant-scope, effect-boundary, cancellation, deadline, disconnect,
   reconciliation, diagnostic, and joined-cleanup cases.
4. Keep user-directed management independent from operation-owned cleanup.
5. Use no live provider effect.

## Acceptance Criteria

- [x] exact selected versions and route authorities are fixed
- [x] every effect outcome is mechanically distinguishable
- [x] before-effect and unconfirmed-after-effect failures remain separate
- [x] unsupported actions fail before dispatch
- [x] implementation is contract-ready or has one exact gate

## Result

- Contract 038 qualifies a separate Gemini CLI stored-transcript delete role
  across the unchanged `0.51.0..=0.52.0` headless range.
- Contract 039 permits separate durable and opt-in temporary cleanup profiles.
- Contract 021 adds exact operation-owned background-response cleanup.
- Research 055 freezes exact source digests, effect strength, reconciliation,
  ordering, failure, diagnostic, and cleanup truth.
- Three dated fixture manifests cover Gemini, Claude Agent, and OpenAI without
  live effects.

Card 101 is contract-ready.

## Auto-Continuation

Continue to card 101 only when the selected corpus is contract-ready.
