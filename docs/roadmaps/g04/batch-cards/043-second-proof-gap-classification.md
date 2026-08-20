# 043 Second-Proof Gap Classification

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../015-second-proof-addable-inventory.md`
Depends on: card 042

## Goal

Classify second-proof gaps and keep hosted OAuth gated.

## Scope

1. Classify each gap as reuse of a prepared facade, adapter-local
   descriptor work, or still gated.
2. Map Claude Agent ACP's addable row to the local subscription profile
   only. API-key billing stays a separate explicit profile.
3. Keep `llama-cpp.owned` out of the attached row.
4. Do not compile implementation cards.

## Out Of Scope

- adapter implementation
- live login that extracts tokens
- compiling g04.016

## Acceptance Criteria

- [x] each inventoried gap has a reuse, descriptor, or gated classification
- [x] hosted OAuth stays a remaining gate
- [x] no production code changes

## Evidence

Research 170. DeepSeek is hosted API-key. Claude Agent ACP addable row is
installed local subscription, not hosted OAuth. llama.cpp attached is
local-unauthenticated. Owned serving stays a different route.

## Validation

- updated research note
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, into card 044.

## Stop Conditions

- OAuth would be selected without a no-secret-extraction proof
- llama.cpp owned would be folded into attached
