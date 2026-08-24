# 160 Grok Build ACP Reasoning-Selection Acceptance

Status: blocked
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.057 Grok Build ACP Reasoning Selection](../057-grok-build-acp-reasoning-selection.md)
Depends on: card 159

## Goal

Prove exact Grok ACP reasoning negotiation for every admitted route/version/
model/value row and publish route-local guidance without overstating reasoning
depth, output quality, usage, cost, or provider-session lifecycle.

## Scope

1. Add bounded fixtures and deterministic tests for every Research 204
   deliver-now row on interactive and structured-run shapes.
2. Prove exact order: initialize, cached-token activation, `session/new`, one
   bounded option snapshot, one selection request, exact confirmation, then
   readiness or first prompt.
3. Prove omission retains the current request sequence with no config-selection
   request and no selected reasoning claim.
4. Prove malformed, duplicate, ambiguous, missing, unsupported, rejected,
   unconfirmed, mismatched, foreign, late, and drifting option states fail
   closed without value/model/route fallback.
5. Prove failure after allocation joins owned work and returns no ready handle
   while preserving provider-owned durable-session truth.
6. Prove attachment recovery never mutates reasoning, and selected new-session
   support does not imply load, resume, catalogue, management, or replay.
7. Preserve working-resource reads, permission-stop behavior, denied callbacks,
   active-turn interruption, cancellation, activity, terminal, and cleanup.
8. Update the Grok prepared guide, Research 204, cards 158-160, g04.057,
   reserved route-local closeout, examples, fixtures, and package-specific
   unreleased API baseline when applicable.
9. Record the exact architecture, Contract 029, route/feature matrix,
   programme, indexes, changelog, and sole Next Task delta in the closeout and
   PR body. Do not edit those shared surfaces on the worker branch.
10. Keep fixtures and tests split below doctor thresholds. Report the baseline
    and stop if this lane raises the 378-findings / 46-errors counts.

## Acceptance Criteria

- [ ] every admitted row and rejected boundary has deterministic coverage
- [ ] selected readiness/first-prompt ordering and exact confirmation are proved
- [ ] omission and selected negotiation remain exact and distinct
- [ ] attachment recovery, provider retention, access, callback, cancellation,
      activity, terminal, and cleanup truth remain unchanged
- [ ] docs claim no effective reasoning depth beyond the confirmed selected
      enum and no output, quality, latency, usage, cost, or billing effect
- [ ] default QA performs no install, login, account inspection, provider
      prompt, credential capture, external inference request, or paid work
- [ ] closeout records PR/head truth without claiming merge
- [ ] worker changes stay inside named code and route-local docs
- [ ] named gates pass and doctor counts do not increase

## Validation

```sh
cargo fmt -p swallowtail-adapter-grok
effigy validate:focused swallowtail-adapter-grok
effigy package:verify-affected swallowtail-adapter-grok
effigy check:examples
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy package:api
effigy doctor
git diff --check
```

Auto-continuation: No.

## Stop Conditions

- exact selection, confirmation, first-prompt order, drift, provider retention,
  or cleanup cannot be proved deterministically
- docs would infer reasoning depth, output, quality, latency, usage, cost, or
  billing from the selected enum
- another route/control family, currentness lane, contract, release,
  generation rollover, or g04 closure enters scope

## Out Of Scope

- authenticated provider prompt, shared front-door edits, publication, merge,
  generation rollover, or g04 closure

## Closeout

Not executed. No negotiation, omission-vs-selected, or confirmation proofs to
add. Current empty-options wire already matches Research 204's omission path.
Guide still says reasoning selection is not qualified.
