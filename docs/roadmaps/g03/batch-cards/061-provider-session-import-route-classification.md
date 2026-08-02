# 061 Provider Session Import Route Classification

Status: completed
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

- [x] every harness route has one evidence-backed classification
- [x] alternate transports do not inherit each other's capability
- [x] only complete list, revalidation, replay, and continuation report support
- [x] partial routes retain exact promotion gates
- [x] public counts derive from the route inventory
- [x] card 062 becomes the sole ready and next task

## Evidence

- Research 096 splits combined solution rows into 19 distinct harness routes
  and audits catalogue, lookup, replay, load, resume, resource, activity, and
  exact-version truth
- exactly three routes are supported: Codex app-server, Kimi Code ACP, and
  OpenCode HTTP
- Gemini headless is discovery-only; Claude ACP and Kimi local server are
  attachment-only; Cursor ACP and Pi RPC remain blocked; 11 routes are not
  applicable
- every partial route names its exact promotion gate
- no adapter, provider, executable, credential, prompt, or consumer changed
- `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` passed

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no provider prompt or broad suite

## Auto-Continuation

Yes. Continue to card 062 after classification acceptance.
