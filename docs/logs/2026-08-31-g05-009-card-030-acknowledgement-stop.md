# 2026-08-31 g05.009 Card 030 Acknowledgement Reassessment Stop

Status: complete; evidence stop
Owner: Tom
Date: 2026-08-31
Card: 030
Contracts: 037, 047, 057, 061

## Result

Card 030 audited Contract 061 Batch 9.4 acknowledgement candidates D, F, and G
against current `main` and promoted none. All three totals reconcile exactly
with no filter or exception list: D 53 rows, F 89 rows, G 48 rows, each owning
the complete census remainder of its adapter packages.

Rubric item 2 fails identically on all three. Every exact active-session
acknowledgement route validates its provider confirmation and then discards it,
so there is no retained acknowledgement to name and no active-observation
facade to bind:

- `claude-agent.acp` — `confirm_reasoning` delegates to `confirm_value`, which
  returns `Result<(), RuntimeFailure>`; `driver/access.rs` drops it and a
  mismatch becomes a static failure with no exact rejected value.
- `kimi-code.acp` — `driver.rs` writes `let _ = selection.confirm(...)?`, and
  `EffectiveReasoningSetup` can encode only requested == effective, so the
  census `rejected` state is unrepresentable through it. The plan half,
  `confirm_plan_mode`, returns `Result<(), _>`.
- `cline.acp` — `confirm_plan_mode` returns `Result<(), _>` and `driver.rs`
  drops it.

`openai.realtime` is the contrast, not a transferable precedent. Card 022 could
prove it only because that card added `RealtimeAcknowledgement` and
`RealtimeOpenRejection::rejected_effort` plus the additive
`open_session_with_projection` seam, under the operator decision recorded in
the Batch 9.1 public-baseline gate for that route alone.

Beyond the shared blocker: D is the narrowest candidate — one adapter package,
one acknowledgement row, no other non-descriptor post-open row. F is the
largest and most coupled, with a compound reasoning-and-plan acknowledgement
plus two further unproved post-open families on `kimi-code.acp`. G fits 48 rows
into all four adapter packages, leaving rubric item 5 with zero headroom; its
two no-control audits are fine under the pattern card 024 already proved.

No candidate can be narrowed around the blocker. The Batch 9.4 boundary
requires the complete package remainder and forbids exception lists, and
withholding the one acknowledgement row that defines the band would weaken a
blocker to force a selection.

## Current State

- card 030 complete as an evidence stop; no candidate promoted; no card 031
  compiled
- no Rust, manifest, release-baseline, contract, architecture, or census file
  changed
- no provider contact and no live probe
- candidates B-G and I-L still hold no card number or execution authority
- g05.009 has no ready implementation card; the acknowledgement band is blocked
- Next Task: compile the acknowledgement public-baseline gate, which needs an
  operator decision on adapter-local retention of exact effective and rejected
  values, the per-route adapter-owned additive open-with-projection result,
  whether `EffectiveReasoningSetup`'s missing rejected state stays
  adapter-local, and whether `feature.negotiated-model-options-observation` and
  post-open `control.provider-session-catalogue` need their own observation
  seams

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Authority

- [card 030](../roadmaps/g05/batch-cards/030-contract-061-acknowledgement-candidate-reassessment.md)
- [g05.009](../roadmaps/g05/009-contract-061-consumer-projection-realization.md)
- [Batch 9.4 package expansion](../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Batch 9.1 public baseline gate](../triage/2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
- [Contract 061](../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
