# 079 ACP Retained History Reconciliation Qualification

Status: ready
Owner: Tom
Created: 2026-08-04
Milestone: `../031-acp-retained-history-reconciliation-qualification.md`
Depends on: card 078

## Goal

Decide whether Claude Agent ACP or Kimi ACP retained history can support an
honest read-only cross-process reconciliation mapping.

## Scope

1. Recheck the exact qualified load and history surfaces for both ACP routes.
2. Bind any candidate to its exact session, route, host, access, model, and version evidence.
3. Classify observable history without inferring a surviving turn or terminal state.
4. Prove no prompt, resume, callback response, provider request, or child-control side effect.
5. Promote the result into Research 099 and compile implementation cards only for a passing route.

## Validation

- deterministic protocol and existing corpus checks only
- `effigy qa:docs`

## Stop Conditions

- stop a route if load requires a state-changing resume or new prompt
- stop if retained history cannot be correlated to the exact durable binding

## Auto-Continuation

Continue into the selected route implementation only when its evidence passes;
otherwise return to the retained-operation candidate gate.
