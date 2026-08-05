# 109 Realtime Session Replacement

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../039-provider-wide-session-usability-restoration.md`
Depends on: card 108

## Goal

Add one common realtime replacement outcome and map OpenAI Realtime and Gemini
Live without implying connection continuity.

## Scope

1. Add runtime method, outcome, constructor, and consuming operation.
2. Return exact interrupted-turn identity and one new realtime handle.
3. Map both prepared realtime facades.
4. Prove no audio, transcript, response, buffer, rollover, or terminal replay.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-openai swallowtail-adapter-gemini`
- affected-package verification for all three packages

## Stop Conditions

- stop if the outcome must flatten realtime and interactive handles
- stop if Gemini planned rollover grants restart continuity

## Auto-Continuation

Continue to card 110 when both mappings pass.

## Outcome

- runtime now exposes `FreshRealtimeSessionReplacement` and a distinct
  `RealtimeSessionReplaced` outcome carrying only the interrupted turn and new
  media-session handle
- OpenAI Realtime and Gemini Live prepared sessions map to the common operation
- deterministic two-turn replacement tests prove each new handle is usable;
  Gemini's within-session planned rollover remains separate
- focused and extracted-package validation passed for runtime and both adapters
