# 091 Cursor And Grok ACP Recovery Drivers

Status: superseded
Owner: Tom
Created: 2026-08-05
Milestone: `../035-acp-continuation-recovery-expansion.md`
Depends on: card 090

## Goal

Implement exact load/replay and prepared continuation recovery for every route
selected by card 090.

## Scope

1. Add `load_session` only to independently qualified drivers.
2. Reuse the existing load request, replay, binding, and recovery vocabulary.
3. Preserve exact provider session, cwd, host, access, model, version, and
   configured-instance agreement.
4. Return bounded replay plus one live session with no lost-turn state claim.
5. Leave every failed candidate unchanged and unsupported.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-cursor swallowtail-adapter-grok`

## Stop Conditions

- stop if implementation needs ambient lookup, raw-id authority, or replay
  heuristics
- stop if ordinary session lifecycle or authentication behavior would change

## Auto-Continuation

Continue to card 092 when all selected mappings pass focused validation.

## Disposition

Card 090 selected no route. Cursor suppresses replay failures; Grok lacks
complete client-visible replay evidence. No driver work is authorized.
