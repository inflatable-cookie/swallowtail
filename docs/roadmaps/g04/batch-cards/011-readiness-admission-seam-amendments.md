# 011 Readiness And Admission Seam Amendments

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../004-readiness-admission-contract-promotion.md`
Depends on: card 010

## Goal

Amend only the named existing-contract seams so 057 can sit in front of 047
without widening those contracts.

## Scope

Amend, only at the named bound:

| Contract | Bound |
| --- | --- |
| 006 | subject observation and sign-in loop versus credential status |
| 008 | addable route versus discovered candidate versus configured instance |
| 010 | host ports for URL open, loopback callback, device-code display |
| 014 | field descriptors versus credential leases |
| 015 / 017 | delegated harness activation stays distinct from login |
| 029 / 032 | instance update observation reuses claims |
| 037 | preparation remains after admission |
| 047 | no emails, tokens, or targets; overlay does not change selection readiness |

## Out Of Scope

- rewriting 047 into a lifecycle facade
- adding overlay presentation metadata to 047 in this card
- production code
- spec archive (card 012)

## Acceptance Criteria

- [x] each named contract records the 057 seam without absorbing 057's
      ownership
- [x] 015 / 017 still distinguish authenticate and delegated login from the
      connection sign-in loop
- [x] 047 still forbids emails, tokens, and targets
- [x] overlay markers cannot change `Ready` / `NotReady`
- [x] no unnamed contract is edited

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, into card 012.

## Stop Conditions

- Stop if an amendment would let 047 carry account identifiers or change
  selection readiness.
- Stop if login and ACP authenticate collapse into one role.

## Evidence

Named seam paragraphs added to 006, 008, 010, 014, 015, 017, 029, 032, 037,
and 047. 047 still excludes emails, tokens, and targets. Overlay markers
cannot change selection readiness. No unnamed contract was edited.
