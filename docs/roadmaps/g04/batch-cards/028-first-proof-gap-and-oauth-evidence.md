# 028 First-Proof Gap And OAuth Evidence

Status: planned
Owner: Tom
Created: 2026-08-20
Milestone: `../010-first-proof-route-inventory.md`
Depends on: card 027

## Goal

Classify first-proof gaps and settle the hosted OAuth candidate from
evidence, or keep that gate explicit.

## Scope

1. Re-read the card 027 research note against Contracts 011, 014, 037, 047,
   and 057.
2. Classify each gap as reuse of a prepared facade, adapter-local descriptor
   work, deterministic-harness work, live-only evidence, or still gated.
3. Decide whether Anthropic or Claude subscription can be proved without
   extracting secrets. If neither can, keep the OAuth gate explicit.
4. Do not compile implementation cards.

## Out Of Scope

- adapter implementation
- live login that extracts tokens or cookies
- compiling g04.011
- Contract 052 consumer-path publication

## Acceptance Criteria

- [ ] each inventoried gap has a reuse, descriptor, harness, live, or gated
      classification
- [ ] OAuth is a named candidate with evidence, or a remaining gate
- [ ] Contract 011 deterministic harnesses stay first
- [ ] no production code changes

## Validation

- updated research note
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, into card 029.

## Stop Conditions

- OAuth would be selected without a no-secret-extraction proof
- live-only evidence is treated as a ready implementation card
