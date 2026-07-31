# 167 g03 Compatibility Runway Compilation

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../049-generation-closeout-and-g03-cutover.md`
Depends on: card 166

## Objective

Compile the first g03 evidence lane without preselecting a provider change.

## Governing Refs

- Vision 001
- Contracts 005, 011, 029, 036, 037, and 044
- provider route and feature matrices
- g02 compatibility and consumer-soak evidence
- g03 generation programme

## Scope

1. Create roadmap g03.001 with three meaningful batches.
2. Start with a repository-local installed-harness and shared-protocol
   maintenance inventory.
3. Put external currentness revalidation behind the inventory.
4. Put implementation-roadmap selection behind authoritative evidence.
5. Make only g03 card 001 ready.
6. Leave one sole Next Task in the roadmap front door.

## Acceptance Criteria

- [x] the first card needs no unresolved provider policy
- [x] later cards require current authoritative evidence before selection
- [x] supported, unsupported, and unverified-newer versions remain distinct
- [x] no hard upper-bound denial or latest-only support policy is introduced
- [x] no runtime test or provider effect is required for compilation
- [x] one clear g03 next task remains

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor`
- `git diff --check`

## Auto-Continuation

No. Roadmap g02.049 closes. Continue through g03 card 001.

## Evidence

- g03.001 contains three evidence-gated batches
- only g03 card 001 is ready
- cards 002-003 remain planned behind inventory and authoritative currentness
- roadmap front-door Next Task points only to g03 card 001
- docs QA, Northstar QA, Doctor review, and diff checks pass
