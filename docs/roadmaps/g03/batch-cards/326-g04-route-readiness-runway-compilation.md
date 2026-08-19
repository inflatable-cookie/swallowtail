# 326 g04 Route Readiness Runway Compilation

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../106-generation-closeout-and-g04-cutover.md`
Depends on: card 325

## Objective

Compile the first g04 evidence lane without preselecting a readiness contract
or flattening consumer UI policy.

## Governing Refs

- Vision 001
- Contracts 005-006, 008, 014, 020, 029, 032, 037, 047
- product guardrails
- Poodle specimen and T3 Code connection-list evidence
- g04 generation programme

## Scope

1. Create roadmap g04.001 with three meaningful batches.
2. Start with a repository-local inventory of existing instance, access,
   discovery, catalogue, version, and prepared-facade records against the
   consumer connection lifecycle.
3. Put gap classification and contract-fit behind that inventory.
4. Put implementation-roadmap selection behind evidence and operator
   decisions.
5. Make only g04 card 001 ready.
6. Leave one sole Next Task in the roadmap front door.

## Acceptance Criteria

- [x] the first card needs no unresolved product policy
- [x] later cards require current evidence and operator decisions before
      selection
- [x] Swallowtail remains a library, not a connection server
- [x] credentials, UI, routing, and consumer persistence stay out of the
      first inventory
- [x] no runtime test or provider effect is required for compilation
- [x] one clear g04 next task remains

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Roadmap g03.106 closes. Continue through g04 card 001.

## Evidence

- g04.001 contains three evidence-gated batches
- only g04 card 001 is ready
- cards 002-003 remain planned behind inventory, contract-fit, and operator
  decisions
- roadmap front-door Next Task points only to g04 card 001
