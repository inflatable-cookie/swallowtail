# 027 First-Proof Surface Inventory

Status: ready
Owner: Tom
Created: 2026-08-20
Milestone: `../010-first-proof-route-inventory.md`
Depends on: completed g04.009

## Goal

Map existing Anthropic Messages, Codex app-server, and Ollama attach
surfaces onto Contract 057 without writing adapter descriptors.

## Scope

1. Inventory each named route's driver identity, topology, credential
   mechanism, discovery, prepared facade, 047 snapshot path, and 029/032
   claim.
2. Name what a 057 addable descriptor, admission, sign-in, refresh, subject,
   update, and overlay path would require on that route.
3. Record unused `SignInAction`s, missing host ports, and secret-free
   constraints.
4. Write a research note. Do not promote architecture or compile
   implementation cards.

## Out Of Scope

- adapter-local descriptors or prepared-facade edits
- live provider, install, login, or billing work
- choosing the OAuth candidate (card 028)
- OpenHands production wiring
- consumer repository edits

## Acceptance Criteria

- [ ] each named first-proof route has an existing-surface map
- [ ] each named route has an explicit 057 gap list
- [ ] no credential secret, harness token, or raw provider payload is
      proposed as a public record
- [ ] no production code changes

## Validation

- research note and named docs indexes
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no package or provider tests

## Auto-Continuation

Yes, into card 028.

## Stop Conditions

- the inventory would require Swallowtail to store secrets or run a server
- a live OAuth or install probe is about to start
- OpenHands is about to become a production route
