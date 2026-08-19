# 002 Route Readiness Gap And Contract Fit

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../001-route-availability-and-readiness-evidence.md`
Depends on: card 001

## Goal

Classify each inventoried gap as reuse, contract amendment, new contract, or
consumer-owned overlay.

## Scope

1. Re-read the card 001 research note against Contracts 006, 008, 014, 020,
   029, 032, 037, and 047.
2. Separate addable-route discovery from configured-instance snapshots.
3. Separate credential-requirement descriptors and sign-in actions from secret
   storage and host browser placement.
4. Use Spec 011's settled decisions. Reopen one only if inventory contradicts
   it.

## Out Of Scope

- promoting the contract
- implementation, adapters, or consumer edits
- reversing authenticated-subject, sign-in, persistence-port, or overlay
  policy without operator review

## Acceptance Criteria

- [x] every gap has one classification against Spec 011
- [x] overlapping 047/037/008 responsibilities stay explicit
- [x] no implementation roadmap is marked ready
- [x] g04.002 and g04.003 remain the follow-ons

## Validation

- research or log promotion note
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, into card 003.

## Evidence

Research 168 contract-fit table. New lifecycle contract; 047 stays a snapshot;
006/008/010/014/015/017/029/032/037/047 named as seam amendments only.
