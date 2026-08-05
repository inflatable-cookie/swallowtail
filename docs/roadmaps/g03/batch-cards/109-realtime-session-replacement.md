# 109 Realtime Session Replacement

Status: planned
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
