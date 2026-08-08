# 161 Version-Claim Semantics Decision

Status: done
Closeout: 2026-08-08
Owner: Tom
Created: 2026-08-08
Milestone: `../053-claim-and-surface-consistency.md`
Depends on: card 160

## Goal

Decide one shared meaning for support status and behavior segments, so a
consumer reading support status can rely on consistent semantics across
adapters.

## Scope

1. Document the current divergence: Codex models legacy behavior as
   `InterfaceSupportStatus::Deprecated` segments
   (`adapter-codex/src/selection.rs:94-111`) while other adapters model old
   behavior as additional `Maintained` segments (for example
   `adapter-kimi/src/selection.rs:75-85`).
2. Decide the intended semantics with the operator: either Deprecated
   segments everywhere old-but-covered behavior exists, or Maintained-only
   segments, and whether behavior revisions stay segment-local.
3. Record the decision in Contract 029 or its companion guidance, and state
   the migration rule for existing claims.

## Out Of Scope

- changing any guaranteed range or claim content (card 162 follows)
- public API changes

## Acceptance

- [x] one documented semantics governs support status across all adapters
- [x] the decision is recorded in the contract surface
- [x] no claim content changes in this card

## Closeout

Operator decision (2026-08-08):

1. A segment whose behavior revision is not the claim's newest revision is
   `Deprecated` by definition: retained for existing installed harnesses,
   not targeted for new integrations. The newest-revision segment is
   `Maintained`.
2. Status is derived automatically from the behavior revision, not a manual
   per-segment judgment; explicit deprecation before removal uses the same
   label.

Recorded in Contract 029 (`Segment Support Status`, migration rule for
existing claims: label-only, deferred to card 162). Core enum doc strings
sharpened to match; no signature or claim content change. `cargo check` and
the core suite (65 tests) pass.

## Stop Conditions

- stop without operator agreement on the semantics

## Auto-Continuation

Yes, to card 162 after the decision is recorded.

## Validation

- `effigy qa:docs`, `effigy qa:routes`
