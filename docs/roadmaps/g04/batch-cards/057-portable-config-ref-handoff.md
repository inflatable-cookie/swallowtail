# 057 Portable Config-Ref Handoff

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../020-config-ref-prepare-handoff.md`
Depends on: card 056

## Goal

Realize a portable handoff from admitted config refs to Contract 037
prepare inputs.

## Scope

1. Host still resolves refs. Swallowtail never stores values.
2. 037 remains after admission and still binds an exact target.
3. Amend 057 only if the handoff is a durable rule.
4. Additive API in `public-api-unreleased` if public types are added.

## Out Of Scope

- wiring all six adapters (card 058)
- overlay or 047 fields
- hosted OAuth
- rewriting `public-api-0.3.3`

## Acceptance Criteria

- [ ] a consumer can hand an admitted instance to prepare without a
      second copy of the target identity
- [ ] portable records still expose no path, URL, or env body
- [ ] 047 still has no targets

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local`
- `git diff --check`
- `effigy package:api` if public types are added

## Auto-Continuation

Yes, into card 058.

## Stop Conditions

- Stop if values leak into diagnostics, the store, or 047.
- Stop if admission prepares.
