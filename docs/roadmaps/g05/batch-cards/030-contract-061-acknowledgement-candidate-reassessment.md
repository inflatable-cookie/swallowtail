# 030 Contract 061 Acknowledgement Candidate Reassessment

Status: complete; evidence stop; no candidate promoted; no Rust change
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 024; Batch 9.4 lifecycle-priority sequence

## Goal

Audit candidates D, F, and G against current `main`. Promote at most one exact
active-session-acknowledgement package tranche, or record an honest stop if no
candidate passes the existing Batch 9.4 rubric.

## Scope

1. Reconcile the exact census sets for candidate D (53 rows), F (89 rows), and
   G (48 rows), including candidate G's two no-control audits.
2. Name every prepared facade, active-observation facade, source identity, and
   acknowledgement state needed by each complete adapter-package remainder.
3. Trace each exact active-session acknowledgement from provider validation
   through retained adapter evidence. Prepared or documentation-only truth is
   not an acknowledgement source.
4. Identify construction-time withholding rules for catalogue-only,
   incompatible-operation, documentation-only, and unobserved rows.
5. Test each candidate against the Batch 9.4 promotion rubric: exact ledgers,
   fail-closed source/applicability assembly, lifecycle and authority
   distinctions, package extraction, public API stability, and the four-package
   focused-validation maximum.
6. Update the Batch 9.4 checkpoint with a current-main disposition for D, F,
   and G. Do not weaken a blocker to force a selection.
7. If exactly one candidate passes, compile one numbered implementation card
   with exact rows, facades, sources, counterexamples, validation, stops, and
   one reviewable PR boundary. Otherwise leave an explicit evidence or
   mechanism stop and name the next planning move.
8. Reconcile the milestone, g05 front door, indexes, closeout log, and sole
   Next Task. Stop for orchestrator exact-head review.

## Out Of Scope

- Rust, manifest, release-baseline, contract, architecture, or census edits
- provider contact, live probes, compatibility/currentness, watcher, skill
  inventory, papercut, or generation-closeout work
- implementing the selected acknowledgement candidate
- promoting per-turn candidates B/K/L, breadth candidates C/E/I/J, or more
  than one acknowledgement candidate
- compiling Batch 9.5

## Acceptance Criteria

- [x] D, F, and G totals reconcile to 53, 89, and 48 exact rows with no filter
      or exception list
- [x] every route's prepared and active-observation facade and source identity
      are named, including the exact acknowledgement transition retained
- [x] documentation, prepared success, and session existence cannot masquerade
      as active-session acknowledgement
- [x] construction-time withholding and negative no-control coverage are
      explicit for every affected route
- [x] each candidate has a concrete deterministic ledger and mixed-assembly
      proof plan that preserves lifecycle, authority, source, route,
      operation, instance, access, and acknowledgement truth
- [x] the selected candidate, if any, needs no new runtime/testkit/core public
      type, composer rule, fixed maximum, callback, registry, provider payload,
      or contract amendment
- [x] exactly one implementation card is ready, or the checkpoint records why
      none can be promoted on current `main`
- [x] shared planning surfaces and the sole Next Task agree

## Review Oracle

- a candidate promoted because the matrix documents acknowledgement, without
  retained active-observation evidence — fail
- an acknowledgement row emitted from prepared-operation success or session
  existence — fail
- a current/pending/rejected/effective state flattened into one supported
  boolean — fail
- a partial adapter package or route omitted to fit the tranche — fail
- one route's active source, acknowledgement state, or applicability borrowed
  by another route under a matching source ID — fail
- a candidate selected despite needing a new shared public or contract
  decision — stop; do not compile its implementation card

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

No Rust validation, live probe, or provider contact belongs to this planning
card.

## Closeout

Evidence stop. No candidate was promoted and no implementation card was
compiled. All three totals reconcile exactly — D 53 rows (30 `claude-agent.acp`
+ 12 `claude-code.headless` + 11 `claude-code.response-only`), F 89 rows (25 +
20 + 31 + 13), G 48 rows (11 + 8 + 11 + 9 + 9) — with each candidate owning the
complete census remainder of its adapter packages and no exception list.

Rubric item 2 fails identically on all three. On current `main` every one of
the three exact active-session acknowledgement routes validates its provider
confirmation and then discards it, so there is no retained acknowledgement to
name and no active-observation facade to bind:

- `claude-agent.acp` — `driver/config.rs` `confirm_reasoning` delegates to
  `confirm_value`, which returns `Result<(), RuntimeFailure>`. `driver/access.rs`
  drops it. A mismatch becomes the static
  `swallowtail.claude_agent.acp.reasoning_mismatch` failure carrying no exact
  rejected value. `ClaudeAgentSessionHandle` holds no acknowledgement field and
  `ClaudeAgentPreparedSession::open_session` returns only the wrapped handle.
- `kimi-code.acp` — `driver/reasoning.rs` `KimiReasoningSelection::confirm`
  produces a runtime `EffectiveReasoningSetup`, but `driver.rs` writes
  `let _ = selection.confirm(...)?`. `EffectiveReasoningSetup` can encode only
  requested == effective; rejection becomes
  `swallowtail.negotiated_reasoning.effective_mismatch`, so `rejected` is
  unrepresentable without a new runtime public type. The plan half,
  `driver/mode.rs` `confirm_plan_mode`, also returns `Result<(), _>`.
- `cline.acp` — `driver/mode.rs` `confirm_plan_mode` returns `Result<(), _>`
  and `driver.rs` drops it. No exact rejected mode is retained.

The contrast is exact. `openai.realtime` could be proved in card 022 only
because that card added `RealtimeAcknowledgement` and
`RealtimeOpenRejection::rejected_effort` plus the additive
`open_session_with_projection` seam, under the operator decision the Batch 9.1
public-baseline gate recorded on 2026-08-31. That gate closed the route-local
acknowledgement surface for `openai.realtime` alone. Nothing closes it for
`claude-agent.acp`, `kimi-code.acp`, or `cline.acp`, so each would need
adapter-local acknowledgement retention plus a new adapter-owned open-with-
projection result family and the matching operator decision. This card's Review
Oracle stops on that.

Per-candidate current-main detail beyond the shared blocker:

- D is the narrowest. One adapter package, one acknowledgement row, and no
  other non-descriptor post-open row. `ClaudeAgentPreparedSession`,
  `ClaudeCodePreparedRun`, and `ClaudeCodeResponsePreparedRun` already exist for
  the prepared side. Only the acknowledgement blocks it.
- F is the largest and most coupled. 89 rows, two packages, four route shapes,
  a compound reasoning-and-plan acknowledgement, and two further unproved
  post-open families on `kimi-code.acp`:
  `feature.negotiated-model-options-observation` and post-open
  `control.provider-session-catalogue`. That is a second and third observation
  seam beyond the acknowledgement.
- G uses all four adapter packages, so rubric item 5 is satisfiable with zero
  headroom. Its two `audit.no-public-route-specific-selectable-control` rows on
  `copilot-cli.acp` and `goose.acp` are fine — card 024 already proved that
  negative-coverage pattern. `cline.acp` adds one further post-open family,
  `feature.negotiated-model-options-observation`.

No candidate can be narrowed around the blocker. The Batch 9.4 boundary
requires the complete package remainder and forbids exception lists, and
withholding the one acknowledgement row that defines the band would weaken a
blocker to force a selection.

Next planning move: one Batch 9.1-class public-baseline gate that closes the
route-local acknowledgement surface for `claude-agent.acp`, `kimi-code.acp`,
and `cline.acp`. It needs an operator decision on adapter-local retention of
the exact effective and rejected values, the per-route adapter-owned additive
open-with-projection result, whether `EffectiveReasoningSetup`'s missing
rejected state stays adapter-local or becomes a runtime decision, and whether
`feature.negotiated-model-options-observation` and post-open
`control.provider-session-catalogue` need their own observation seams. Card 030
names that move; it does not compile it.

No Rust, manifest, release-baseline, contract, architecture, or census file
changed. No provider was contacted and no live probe ran.

## Auto-Continuation

No. Return one reviewable planning PR. The orchestrator reviews the exact
candidate disposition and any compiled implementation card before dispatch.

## Evidence

- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [completed card 024](024-contract-061-deepagents-kiro-qoder-zcode-package-completion.md)
- [Batch 9.1 public baseline gate](../../../triage/2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
- [card 030 acknowledgement stop log](../../../logs/2026-08-31-g05-009-card-030-acknowledgement-stop.md)
