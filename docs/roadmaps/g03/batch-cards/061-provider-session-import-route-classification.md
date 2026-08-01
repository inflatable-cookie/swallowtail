# 061 Provider Session Import Route Classification

Status: planned
Owner: Tom
Created: 2026-08-01
Milestone: `../023-provider-session-import-acceptance-and-handoff.md`
Depends on: card 060

## Goal

Classify every harness route against the complete discovery, import, replay,
and continuation profile without inferring support from a provider family.

## Scope

1. Audit every harness route for catalogue, lookup, history, load, resume,
   resource binding, activity truth, and exact version evidence.
2. Classify routes as supported, discovery-only, attachment-only, blocked, or
   not applicable.
3. Reassess Kimi local server, Claude, Cursor, Pi, and remaining harnesses.
4. Record exact unblock evidence for every partial route.
5. Implement no route whose complete evidence is not already present.

## Out Of Scope

- new provider research beyond a bounded classification delta
- direct-model routes where provider sessions do not apply
- consumer persistence, UI, synchronization, or management binding storage

## Acceptance Criteria

- [ ] every harness route has one evidence-backed classification
- [ ] alternate transports do not inherit each other's capability
- [ ] only complete list, revalidation, replay, and continuation report support
- [ ] partial routes retain exact promotion gates
- [ ] public counts derive from the route inventory
- [ ] card 062 becomes the sole ready and next task

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no provider prompt or broad suite

## Auto-Continuation

Yes. Continue to card 062 after classification acceptance.
