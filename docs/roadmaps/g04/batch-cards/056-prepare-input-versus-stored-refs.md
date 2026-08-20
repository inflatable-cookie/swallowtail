# 056 Prepare Input Versus Stored Refs

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../020-config-ref-prepare-handoff.md`
Depends on: completed g04.019

## Goal

Map the six addable `prepare_*` entries onto stored config and credential
refs without changing prepare yet.

## Scope

1. Inventory Anthropic Messages, DeepSeek continuation, Codex app-server,
   Claude Agent ACP, Ollama attach, and llama.cpp attached.
2. Name each prepare input that still duplicates an admitted
   `ConfigFieldRef` or `CredentialRef`.
3. Say whether 057 or 037 needs a seam amendment. Do not resolve refs.

## Out Of Scope

- changing prepare signatures (card 057)
- adapter proofs (card 058)
- overlay or 047 fields
- hosted OAuth

## Acceptance Criteria

- [x] each of the six routes has a prepare-input versus stored-ref map
- [x] leak and 047-target risks are named
- [x] no production prepare signature changes in this card

## Evidence

- Inventory: `docs/logs/2026-08-20-g04-020-config-ref-prepare-inventory.md`
- `effigy qa:docs:index:logs`
- `git diff --check`

## Validation

- `effigy qa:docs:index:logs`
- `git diff --check`

## Auto-Continuation

Yes, into card 057.

## Stop Conditions

- Stop if the map requires storing paths, URLs, or env bodies.
