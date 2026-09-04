# 075 Contract 061 Gemini And Grok Package Completion

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 065 accepted through PR 205; completed cards 022 and 032; current `main`

## Goal

Complete Candidate E as one exact 56-row Contract 061 tranche across
`swallowtail-adapter-gemini` and `swallowtail-adapter-grok`, using additive
adapter-owned projected-open seams for retained negotiated model options.

## Chatterbox Ruling

Use the audit's Path A. Existing Contract 061 active-session vocabulary and
card 032's accepted projected-open precedent settle the publication shape.
No new public-baseline gate or shared runtime type is required. Existing
`open_session` methods remain source-compatible and behavior-identical.

## Scope

1. Implement deterministic ledgers for exactly 14 `gemini-cli.acp`, 13
   `gemini-cli.headless`, 16 `gemini.live`, and 13 `grok-build.acp` tuples.
2. Emit 39 rows and construction-time withhold 17 rows exactly as the accepted
   audit records, with no filter or exception list.
3. Add ordinary prepared contributions to the five proved facades:
   `GeminiPreparedSession`, `GeminiHeadlessPreparedRun`,
   `GeminiPreparedLiveSession`, `GrokPreparedSession`, and `GrokPreparedRun`.
4. Add adapter-owned additive `open_session_with_projection` seams to
   `GeminiPreparedSession` and `GrokPreparedSession`, following card 032.
   Serve old and new entrypoints from one private lifecycle and preserve all
   existing success, failure, cleanup, and provider-work behavior.
5. Publish negotiated-model-options observation only from retained validated
   post-open state, through distinct prepared and active-session source IDs.
   Never infer it from configuration, documentation, or preparation.
6. Preserve the accepted applicability separation: Gemini live and ACP may
   share `InteractiveSession`, but remain separate on actual driver role,
   ownership, facade revision, instance policy, endpoint audience, and media
   requirements. Caller-supplied access profiles stay caller-supplied.
7. Add provider-free, mutation-sensitive assertions for all four route
   ledgers, source separation, applicability, active observation, and every
   withholding.
8. Stop after one reviewable two-package PR.

## Out Of Scope

Shared runtime/core/testkit changes; new contract or architecture; Gemini
version requalification; DeepSeek, Kimi, OpenCode, or another Candidate;
provider contact; live sessions; new provider operations.

## Acceptance Criteria

- all 56 tuples appear exactly once as emitted or withheld
- totals are 39 emitted / 17 withheld / 56 reconciled
- active model-option observation is impossible before successful open and
  impossible without retained validated options
- old open APIs remain behavior-identical and semantic API change is additive
  adapter-only
- the three-family applicability proof is asserted from real identities
- no provider work or execution authority is added

## Validation

- `cargo fmt -p swallowtail-adapter-gemini -p swallowtail-adapter-grok -- --check`
- `effigy validate:focused swallowtail-adapter-gemini swallowtail-adapter-grok`
- `effigy package:verify-affected swallowtail-adapter-gemini swallowtail-adapter-grok`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

## Review Oracle

Invariant: prepared truth and post-open retained observation remain separate,
and every row belongs to the exact route applicability that proves it.

Smallest counterexample: a configured option published as observed, the old
open path changes behavior, Gemini live borrows ACP identity, or any ledger
row is counted twice or omitted.

## Auto-Continuation

No. Stop for exact-head review.

## Stop Conditions

- either projected-open seam needs a shared public type or contract change
- retained options cannot be exposed without changing open behavior or cleanup
- the exact 39/17/56 ledger does not reconcile
- Gemini version requalification or provider contact becomes necessary

## Evidence

- [Candidate E audit](../../../triage/20260904-134659-contract-061-candidate-e-audit.md)
- [card 065](065-contract-061-candidate-e-breadth-audit.md)
- [card 032](032-contract-061-cline-command-code-copilot-goose-package-completion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
