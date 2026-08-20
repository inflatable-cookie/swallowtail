# 059 Unmarked Overlay Classification

Status: planned
Owner: Tom
Created: 2026-08-20
Milestone: `../021-unmarked-overlay-rows.md`
Depends on: completed g04.020

## Goal

Classify overlay behavior for catalogue rows that omit `provider_id`.

## Scope

1. Codex, Claude Agent ACP, Ollama, and llama.cpp attached are the
   unmarked addable catalogues.
2. Preferred direction: overlay keys instance plus model when
   `provider_id` is absent. Do not invent a provider id.
3. Write the classification. Do not change overlay yet.

## Out Of Scope

- overlay implementation (card 060)
- 047 presentation metadata
- inventing catalogue `provider_id`

## Acceptance Criteria

- [ ] unmarked versus instance-plus-model keying is classified
- [ ] gateway-flattening risk is named
- [ ] no overlay code changes in this card

## Validation

- `effigy qa:docs:index:logs`
- `git diff --check`

## Auto-Continuation

Yes, into card 060 unless the classification is still forked.

## Stop Conditions

- Stop and ask if instance-plus-model keying would flatten mixed gateway
  rows.
- Stop if the next step would invent a catalogue provider id.
