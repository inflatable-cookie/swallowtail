# 224 xAI Responses WebSocket Web Search Acceptance

Status: blocked; Research 227 empty deliver-now set
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.080 xAI Responses WebSocket Web Search](../080-xai-responses-websocket-web-search.md)
Depends on: card 223

## Goal

Prove exact web-search dispatch, preserved omission, fail-closed model/profile
gating, bounded provider-owned response/citation truth, and unchanged xAI
Responses WebSocket lifecycle, then produce one review-ready closeout.

## Scope

1. Add deterministic preparation, encoder, decoder, activity, fixture,
   structured-run, serial-session, restoration, and scenario coverage for
   every Research 227 deliver-now row and rejection boundary.
2. Assert exact model/profile membership, immutable policy/plan/evidence
   agreement, one canonical `web_search` tool, positive provider-side bound,
   and absence of every non-admitted tool or option.
3. Prove omission retains exact `tools: []` bytes on runs, first turns,
   continuation turns, and fresh restoration.
4. Prove foreign models/profiles, disabled/mismatched policy, stale facade,
   raw tools, consumer tools, host-network widening, and unqualified options
   reject before endpoint, credential, or socket work.
5. Prove exact search-call lifecycle, provider-owned activity, assistant text,
   citation delivery, usage, billed cost, provider failure, cancellation,
   deadline, disconnect, terminal ordering, and joined cleanup admitted by
   Research 227.
6. Assert a model declining search is ordinary provider choice, while malformed
   search events, disabled policy, unsupported model, quota, or tool drift fail
   through the exact qualified mapping without fallback.
7. Keep dispatch, provider acceptance, invocation, result delivery, citation
   delivery, usage, billing, and terminal truth separate in tests and docs.
8. Update the realtime guide, route/feature matrices, changelog, Research 227,
   roadmap/card state, programme, triage, logs, indexes, and sole Next Task.
9. Regenerate and review the API baseline only when the public surface changes.
10. Run the complete named validation once for the batch. Record inherited
    doctor findings and exact drift.

## Acceptance Criteria

- [ ] every admitted row dispatches the exact bounded web-search tool
- [ ] omission and all existing reasoning/output rows retain prior behavior
- [ ] unsupported or mismatched rows fail before network effects
- [ ] provider-owned search never becomes a consumer callback or host-network
      claim
- [ ] response/citation/usage/cost/terminal projection is bounded and exact
- [ ] connection, continuation, restoration, cancellation, deadline,
      invalidation, credential release, and cleanup remain correct
- [ ] one review-ready worker PR contains the complete lane or honest stop

## Validation

```sh
cargo fmt -p swallowtail-adapter-xai
effigy validate:focused swallowtail-adapter-xai
effigy package:verify-affected swallowtail-adapter-xai
effigy check:examples
effigy package:api
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy doctor
git diff --check
```

## Stop Conditions

- any admitted row cannot prove exact immutable dispatch and bounded response
  truth
- omission, decoding, activity, usage/cost, terminal, or lifecycle regresses
- acceptance requires live provider work, authority widening, or unrelated
  repair

## Out Of Scope

- another feature/route, other xAI tools, currentness, publication, merge,
  generation rollover, or g04 closure
