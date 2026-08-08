# 167 ACP Activity Projection Contract

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../054-remaining-duplication-tranches.md`
Depends on: card 166

## Goal

Pin the portable ACP event-to-activity projection contract before the five
projections migrate onto one shared core.

## Scope

1. Record the ACP event taxonomy (user message, assistant message, tool
   submission, tool execution start, tool output, plan replacement,
   lifecycle, permission, and terminal events) and the portable activity
   fields each event family maps onto.
2. Decide the adapter-private vs shared projection boundary: which event
   decoding stays provider-owned and which mapping is provider-neutral.
3. Record the contract in Contract 039 or its companion guidance, with the
   migration rule for the five existing projections.

## Out Of Scope

- changing any realized projection or activity vocabulary
- public API changes

## Acceptance

- [x] one documented projection contract governs ACP activity across adapters
- [x] the boundary between provider decoding and shared projection is clear
- [x] no projection changes in this card

## Closeout

Operator decision (2026-08-08): the ACP event-to-activity projections stay
adapter-local, confirming the card-158 recorded disposition. No shared
projection contract is written, because there is no shared projector:

- the family was measured in card 158 (kimi=claude-agent 0.99 similarity,
  grok=cursor 1.00, gemini divergent; ~820 lines)
- no shared home exists under the recorded topology: protocol-acp is
  "without provider or runtime projection" and runtime keeps its minimal
  dependency posture; a shared home was offered (new crate vs posture
  amendment) and declined at the planning level
- the recorded boundary is therefore: ACP event decoding and projection
  stay inside each owning adapter; the portable activity vocabulary in
  `swallowtail-runtime/src/activity/` stays the shared contract surface
  those projections map onto

Card 168 (migration) is cancelled: its premise, a shared projector, was
declined. The generation returns to its evidence gate.

## Stop Conditions

- stop without agreement on the projection boundary

## Auto-Continuation

Yes, to card 168 after the contract is recorded.

## Validation

- `effigy qa:docs`, `effigy qa:routes`
