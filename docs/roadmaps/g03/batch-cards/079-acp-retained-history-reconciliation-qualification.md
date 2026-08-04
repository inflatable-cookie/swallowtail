# 079 ACP Retained History Reconciliation Qualification

Status: completed
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

Passed. Stable ACP schema, qualified route corpora, both production drivers,
and installed Claude Agent ACP `0.63.0` source prove that load restores a live
resumable session before or alongside replay. No Rust mapping was added.

## Stop Conditions

- stop a route if load requires a state-changing resume or new prompt
- stop if retained history cannot be correlated to the exact durable binding

## Auto-Continuation

Completed at the stop condition. Continue with card 080; no ACP implementation
card exists.
