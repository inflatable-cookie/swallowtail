# 235 Muse Code 0.2.1-R1215.1 Identity Corpus

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../074-muse-code-0-2-1-signed-payload-pin.md`
Depends on: Research 127; Research 131

## Goal

Freeze exact Muse Code host payload `0.2.1-R1215.1` against the qualified
opaque `0.1.0-R708.1` pin, and name the pin-move shape. Do not edit the
production claim in this card.

## Scope

1. Rank remaining Research 127 exact-pin host-drift families; pick Muse.
2. Record local `muse-bin-0.2.1-R1215.1` identity, codesign, help deltas, and
   deterministic echo JSONL comparison.
3. Name card 236 as an opaque pin move reusing `muse-code.events-v1`.

## Out Of Scope

- editing Muse selection claims, discovery parser, or public matrices
- Meta-provider runs or login
- Command Code, DeepSeek, Claude, Gemini, or other 127 families
- install, update, or publication

## Acceptance Criteria

- [x] exact `0.2.1-R1215.1` payload identity is recorded
- [x] echo schema and payload-type sequence are compared to `0.1.0-R708.1`
- [x] the next card has an explicit opaque pin-move decision
- [x] no claim membership changes in this card

## Validation

- fixture comparison named in the card evidence
- `effigy validate:focused swallowtail-adapter-muse`
- `effigy qa:northstar`

## Stop Conditions

- stop if identity requires a Meta provider prompt
- stop if a new public operation is required before the pin shape is named

## Auto-Continuation

No. Compile the pin-move claim card after the identity shape is named.

## Evidence

- Research 131
- `crates/swallowtail-adapter-muse/tests/fixtures/muse-code-0.2.1-R1215.1/`
- Identity decision: opaque-pin-move. Reuse `muse-code.events-v1`. Card 236
  owns the claim change.
