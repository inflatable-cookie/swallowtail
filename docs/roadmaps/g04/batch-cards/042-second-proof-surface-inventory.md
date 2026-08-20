# 042 Second-Proof Surface Inventory

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../015-second-proof-addable-inventory.md`
Depends on: completed g04.014

## Goal

Map DeepSeek continuation, Claude Agent ACP, and llama.cpp attached onto
Contract 057 without writing adapter descriptors.

## Scope

1. Inventory each named route's driver identity, topology, credential
   mechanism, discovery, prepared facade, 047 path, and 029/032 claim.
2. Name what a 057 addable descriptor would require.
3. Write a research note. Do not promote architecture or compile
   implementation cards.

## Out Of Scope

- adapter-local descriptors
- hosted OAuth
- live provider, install, or login work
- compiling g04.016

## Acceptance Criteria

- [x] each named route has an existing-surface map
- [x] each named route has an explicit 057 gap list
- [x] no production code changes

## Evidence

Research 170.

## Validation

- research note and named docs indexes
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, into card 043.

## Stop Conditions

- inventory would require Swallowtail to store secrets or run a server
- Claude Agent is about to be treated as hosted OAuth
