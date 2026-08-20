# 029 First-Proof Tranche Confirmation

Status: completed
Owner: Tom
Created: 2026-08-20
Milestone: `../010-first-proof-route-inventory.md`
Depends on: card 028

## Goal

Confirm g04.011 hosted API-key Anthropic Messages as the next
implementation roadmap unless inventory contradicts it.

## Scope

1. Check the 027/028 evidence against Contract 057's first-proof list.
2. Confirm Anthropic Messages API-key as the first adapter-local descriptor
   tranche.
3. Leave OAuth, Codex app-server, Ollama attach, and Contract 052 consumer
   path planned behind that first proof.
4. Do not write g04.011 implementation cards in this card.

## Out Of Scope

- compiling or executing g04.011
- adapter code
- live provider probes
- consumer path guides

## Acceptance Criteria

- [x] g04.011 remains the named next implementation roadmap, or a
      contradictory inventory fact is written
- [x] OAuth, Codex, Ollama, and 052 stay planned
- [x] no adapter implementation card is marked ready
- [x] no production code changes

## Evidence

Research 169 does not contradict hosted API-key Anthropic Messages as the
first adapter-local descriptor tranche.

## Validation

- compiled follow-on pointer in g04.010
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Roadmap g04.010 closes. Compile g04.011 only after this card.

## Stop Conditions

- Anthropic Messages is about to be skipped without a written reason
- an implementation card is about to be marked ready from this inventory
