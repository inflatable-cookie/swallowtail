# 260 g05 Harness Visibility Runway Compilation

Status: completed
Owner: Tom
Created: 2026-08-28
Milestone: `../091-generation-closeout-and-g05-cutover.md`
Depends on: card 259

## Goal

Compile the first g05 evidence lane without selecting a watcher mechanism,
public API, or route implementation.

## Scope

1. Create g05.001 with three evidence-gated batches.
2. Start with exact prompt-free production-harness skill and background-process
   surface inventory.
3. Put vocabulary, privacy, authority, activity, and turn-gate decisions behind
   the inventory.
4. Put architecture, contract, and proof-route selection behind those decisions.
5. Make only g05 card 001 ready and reserve Research 255.

## Acceptance Criteria

- [x] the first card needs no unresolved product policy
- [x] later cards retain the operator decisions from the promoted triage note
- [x] no watcher mechanism is selected by implication
- [x] no prompt, credential, install, host scan, or process mutation is required
- [x] one clear g05 Next Task remains

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. g04.091 closes. Continue through g05.001 card 001.
