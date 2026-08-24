# 159 Grok Build ACP Reasoning-Selection Binding

Status: blocked
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.057 Grok Build ACP Reasoning Selection](../057-grok-build-acp-reasoning-selection.md)
Depends on: card 158; promoted Research 204 with a non-empty deliver-now set

## Goal

Bind only Research 204's exact version/model/value rows through portable
reasoning inputs, immutable plan/evidence, request agreement, and the adapter-
private ACP option sequence.

## Scope

1. Add the smallest optional portable reasoning input to the prepared
   interactive and structured-run shapes admitted by Research 204. Preserve
   existing constructors and omission behavior.
2. Add exact `ReasoningSelection` capability constraints only for selected
   values. Keep the route-fixed `GrokModelSelection` unchanged and reject every
   version/model/value mismatch during preparation when knowable.
3. Bind the selected mode through `SessionOptions` for interactive sessions and
   the structured run's `OperationPolicy`, immutable plan/evidence, request, and
   configured low-level driver agreement. Expose no raw provider option map.
4. After `session/new`, validate one bounded exact option snapshot, send one
   correlated `session/set_config_option`, and require the exact Research 204
   effective confirmation before returning a handle or sending the run prompt.
5. Keep omission byte- and behavior-stable: no selection request, no new
   readiness wait, and no inferred default selection.
6. Reject malformed, ambiguous, missing, unsupported, rejected, unconfirmed,
   mismatched, foreign, or drifting options without fallback.
7. On failure after provider-session allocation, join attachment, process,
   credential, resource, callback, and task work. Preserve durable provider-
   session retention; do not claim rollback or deletion.
8. Do not apply selected reasoning during attachment recovery, load, or resume.
   If exact request/plan truth cannot prohibit that mutation, stop.
9. Preserve access, ambient working-resource authority, callback rejection,
   permission-stop behavior, cancellation, activity, terminal, and cleanup
   semantics.
10. Put new protocol and prepared-facade proofs in focused modules. Do not add
    to existing god files or worsen the doctor 378-findings / 46-errors
    baseline.

## Acceptance Criteria

- [ ] only Research 204 deliver-now rows prepare
- [ ] input, route-fixed model, plan constraint, evidence, request, driver,
      snapshot, selection request, and confirmation agree exactly
- [ ] omission preserves current wire and behavior
- [ ] selected sessions are not ready and selected runs do not prompt before
      confirmation
- [ ] known mismatches reject before provider work; post-allocation drift joins
      every owned surface
- [ ] attachment recovery, provider retention, permissions, callbacks,
      cancellation, activity, terminal, and cleanup remain honest
- [ ] no generic settings map, raw option strings, model switching, fallback, or
      sibling-route claim enters the API
- [ ] doctor finding/error counts do not increase

## Validation

```sh
cargo fmt -p swallowtail-adapter-grok
effigy validate:focused swallowtail-adapter-grok
effigy package:verify-affected swallowtail-adapter-grok
effigy package:api
effigy qa:northstar
effigy doctor
git diff --check
```

Auto-continue to card 160 when exact negotiation, omission, early rejection,
post-allocation cleanup, and doctor-baseline checks pass.

## Stop Conditions

- implementation needs a generic provider config API, model switch, contract or
  currentness change, load/resume mutation, or breaking public API
- exact confirmation cannot remain correlated and bounded
- selected structured runs can prompt before confirmation
- failure after allocation cannot retain honest provider-session and cleanup
  truth

## Out Of Scope

- route guide, shared matrices/indexes, live provider work, another Grok
  control, release, publication, merge, generation rollover, or g04 closure

## Closeout

Not executed. Research 204 admits no deliver-now row. Exact 1.0.5's open-time
hint is fail-open, and exact `x.ai/sessionConfig.options` effort/selected
truth is unfrozen. Contract 040 could consider a request-body mapping only
with that confirmation and without ignore/default.
