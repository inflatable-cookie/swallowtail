# 230 Recurring Version Currentness Process

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../072-recurring-version-currentness-and-codex-0-147.md`
Depends on: Research 127

## Goal

Make all-route version currentness a named Contract 029 checkpoint with a
matching operator runbook, without changing any qualified bound.

## Scope

1. Add the recurring checkpoint to Contract 029 and the Contract 001 working
   rule.
2. Record the realized process in architecture and product guardrails.
3. Add `docs/guides/version-currentness-checkpoint.md` and index it.
4. Point `AGENTS.md` at the checkpoint so later agents do not bulk-bump
   from `latest`.
5. Compile g03.072 with Codex `0.147.0` as the first family card.

## Out Of Scope

- changing any adapter claim, fixture, matrix version cell, or probe pin
- Grok `1.0.x`, exact-pin, Gemini, hosted facade, or Bedrock SDK repairs
- Effigy network polling or CI currentness jobs
- provider prompts, installs, or publication

## Acceptance Criteria

- [x] Contract 029 states method, cadence, classification, and one-family
      upgrade
- [x] the runbook matches the contract and names Research 091/127 as
      specimens
- [x] no qualified bound changes in this card
- [x] Next Task points at card 231, not a bulk range bump

## Validation

- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`

## Stop Conditions

- stop if the process would authorize claim changes from registry latest
- stop if cadence were encoded as required CI

## Auto-Continuation

Continue to card 231.

## Evidence

- Contract 029 Recurring Currentness Checkpoint
- `docs/guides/version-currentness-checkpoint.md`
- Research 127 promoted as the latest method specimen
