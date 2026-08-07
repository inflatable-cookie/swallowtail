# 162 Claim Identity And Claim-Less Posture

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../053-claim-and-surface-consistency.md`
Depends on: card 161

## Goal

Standardize claim identity and window numbering, and settle the claim-less
adapter posture.

## Scope

1. Adopt one claim-id scheme across adapters (currently mixed dash and dot
   schemes, for example `alibaba-deployable-models-window-1` versus
   `antigravity.catalogue.release-window-1`) and one window numbering rule.
2. Renumber windows consistently where the audit found drift (kimi at
   `window-4`, pi and qwen at `window-2` for comparable claim ages).
3. Decide the posture for the claim-less adapters (bedrock, llama-cpp): add
   exact compatibility claims like their hosted peers, or record an explicit
   opaque-only disposition in the route matrix and version posture docs.
4. Update claim ids, windows, and any claim-based tests in one batch.

## Out Of Scope

- guaranteed range content, versions, or behavior changes
- public API changes

## Acceptance

- [ ] every adapter claim follows one id scheme and window rule
- [ ] claim-less adapters carry claims or an explicit recorded disposition
- [ ] route and feature matrices stay consistent with claims

## Stop Conditions

- stop if renumbering changes claim meaning or support truth

## Auto-Continuation

Yes, to card 163 after acceptance.

## Validation

- `effigy qa:routes`, `effigy check:examples`
- focused validation for every touched adapter
