# 236 Muse Code 0.2.1-R1215.1 Pin Move

Status: completed
Owner: Tom
Created: 2026-08-18
Milestone: `../074-muse-code-0-2-1-signed-payload-pin.md`
Depends on: card 235; Research 131

## Goal

Move the opaque `muse-code.signed-payload` pin from `0.1.0-R708.1` to exact
`0.2.1-R1215.1`, keeping QualifiedOnly and behavior `muse-code.events-v1`.

## Scope

1. Replace Muse release revision and payload basename constants.
2. Update discovery version parsing so `Muse Code 0.2.1 (...)` classifies.
3. Refresh frozen production corpus pointers, focused tests, matrices, and
   guides.
4. Keep the claim opaque and exact; do not invent UnverifiedNewer for Muse.

## Out Of Scope

- retaining `0.1.0-R708.1` as a second opaque segment
- Meta-provider requalification beyond the frozen identity evidence
- Claude, Command Code, DeepSeek, Gemini, or other 127 families
- provider prompts, install, update, or publication

## Acceptance Criteria

- [x] exact `0.2.1-R1215.1` is Qualified Maintained
- [x] `0.1.0-R708.1` is no longer permitted
- [x] discovery accepts the new version line and rejects the old pin
- [x] focused Muse proof and package verify pass
- [x] matrices and guides name the new opaque pin

## Validation

- `effigy validate:focused swallowtail-adapter-muse`
- `effigy package:verify-affected swallowtail-adapter-muse`
- `effigy qa:northstar`
- named research/log/roadmap indexes as needed
- no broad workspace suite

## Stop Conditions

- stop if the pin move requires a Meta provider prompt
- stop if protocol evidence forces a new public operation or behavior
  revision name without corpus proof
- stop if `0.2.1-R1215.1` is no longer the host payload under test

## Auto-Continuation

No. After closeout, reassess remaining Research 127 families one at a time.
Do not start the AllowUnverified cluster inside this card.

## Evidence

- Research 131
- `crates/swallowtail-adapter-muse/tests/fixtures/muse-code-0.2.1-R1215.1/`
- Pin: exact opaque `0.2.1-R1215.1`
- Behavior: `muse-code.events-v1`
