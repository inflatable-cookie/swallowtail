# 027 Codex 0.151.0 Identity Restack

Status: ready
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../012-codex-0-151-0-useful-newer.md`
Depends on: Contract 029; PR 130 head `93d893874dbaff20a4cda4084020c7db88663ebd`

## Goal

Revalidate official npm `@openai/codex` `0.151.0` and reconstruct PR 130's
identity evidence on current `main` without a production claim edit.

## Scope

1. Recheck official stable identity. Stop if `0.151.0` is no longer the exact
   stable being qualified.
2. Preserve Research 262 and the secret-free `0.151.0` identity/protocol
   fixtures only after corroborating their source and digests.
3. Compare the selected mapped exec and app-server surfaces against current
   qualified evidence. Keep unmapped additions unmapped.
4. Reconcile PR 130 with changes merged after its old base. Do not overwrite
   Contract 061 package work or unrelated main changes.
5. Produce one identity-only commit. Selection ranges, production claims,
   feature pins, matrices, changelog, and claim log remain unchanged.
6. Record compatible extension, private milestone, new facade, or stop.

## Out Of Scope

- production claim edits or merge
- provider prompt, login, install, host update, or live probe
- another route family, feature-façade, watcher, skill, papercut, or release
  work

## Acceptance Criteria

- exact official `0.151.0` identity remains corroborated
- mapped and unmapped behavior is explicit
- current production claims are byte-for-byte unchanged by this card
- the identity commit applies cleanly on current `main`
- card 028 auto-continues only for an admitted Contract 029 segment

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

Yes, to card 028 only after an admitted segment is recorded.
