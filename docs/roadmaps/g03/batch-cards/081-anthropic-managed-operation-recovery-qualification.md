# 081 Anthropic Managed Operation Recovery Qualification

Status: completed
Owner: Tom
Created: 2026-08-04
Milestone: `../032-retained-operation-reconciliation-candidate-gate.md`
Depends on: card 080

## Goal

Decide whether Anthropic Managed Agents can persist one exact route-bound
operation record and later observe authoritative retained state without
continuation or control authority.

## Scope

1. Recheck the exact managed-agent beta session, status, and persisted-event
   surfaces.
2. Identify the minimum durable provider operation/resource binding needed
   before dispatch loss.
3. Separate provider-owned retained sessions from the current driver-owned
   delete-on-close profile.
4. Qualify bounded terminal, active, waiting, unknown, and cleanup truth.
5. Record whether a non-deleting prepared profile is contract-safe or requires new operator policy.

## Validation

- official source and deterministic corpus evidence only
- `effigy qa:docs`

## Stop Conditions

- stop if observation requires a new message, interrupt, callback result, or
  resume action
- stop if current mandatory deletion prevents durable recovery authority
- stop if exact agent, environment, session, run, and route correlation cannot
  be persisted

## Auto-Continuation

Continue to card 082 after the managed-operation classification is promoted.

## Evidence

Exact session retrieval and bounded persisted-event history pass the read-only
gate. Current provisioning does not expose a durable checkpoint before work
can be lost, and recovered cleanup needs authority separate from observation.
Research 103 and Contracts 022 and 048 carry the selected mapping.
