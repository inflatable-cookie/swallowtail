# 001 Route Readiness Surface Inventory

Status: ready
Owner: Tom
Created: 2026-08-19
Milestone: `../001-route-availability-and-readiness-evidence.md`

## Goal

Map the consumer connection lifecycle onto existing Swallowtail records without
selecting a new contract or changing provider behavior.

## Scope

1. Inventory current driver descriptors, discovery outcomes, configured
   instances, access profiles, credential mechanisms, sign-in actions,
   installed-executable observation, model catalogues, prepared facades,
   Contract 047 snapshots, and Contract 029 version claims.
2. Map Poodle specimen and T3 Code connection-list surfaces onto those records:
   addable-route picker, credential or browser sign-in, admitted instance list,
   enablement versus readiness, per-instance config, auth identity, updates,
   and model lists.
3. Record what is already portable, what is host-owned, and what is currently
   consumer-assembled.
4. Write a research note. Do not promote a contract.

## Out Of Scope

- new public types, adapters, or persistence
- live provider, install, login, or billing work
- consumer repository edits
- deciding OAuth ownership, account-identity disclosure, or preference records
- treating Poodle or T3 Code UI copy as Swallowtail vocabulary

## Acceptance Criteria

- [ ] every mapped consumer surface names an existing Swallowtail record, an
      explicit gap, or a consumer overlay
- [ ] Contract 047 is classified as a selection snapshot, not an add-connection
      facade
- [ ] no credential reference, account secret, or raw provider payload is
      proposed as a public record
- [ ] remaining operator decisions are listed, not settled
- [ ] no production code changes

## Validation

- research note and named docs indexes
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no package or provider tests

## Stop Conditions

- the inventory would require Swallowtail to store secrets or run a server
- a consumer UI label is about to become a portable identity
- an existing contract already covers the whole lifecycle and the lane should
  stop

## Auto-Continuation

No. Card 002 stays planned until this inventory is accepted.
