# 088 ACP Continuation Recovery

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../034-working-state-restoration-facade.md`
Depends on: card 087

## Goal

Map exact Claude Agent ACP and Kimi ACP load/replay into honest continuation
recovery without calling it reconciliation.

## Scope

1. Prepare recovery only from an exact matching session plan and resume binding.
2. Consume the common facade into existing qualified load behavior.
3. Preserve bounded replay, live handle, cleanup, and safe failure behavior.
4. Freeze no-state-claim and drift rejection in deterministic adapter tests.

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent swallowtail-adapter-kimi`

## Stop Conditions

- stop if ACP history is parsed for terminal inference
- stop if load can follow failed reconciliation automatically

## Auto-Continuation

Continue to card 089 when both ACP mappings pass.

## Closeout

- Claude Agent ACP and Kimi ACP now prepare exact continuation recovery from
  the existing session plan and resume binding
- both return bounded replay and one live session without lost-turn state
- 175 focused adapter tests passed
