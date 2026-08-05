# 099 Pi Persistent Recovery Implementation

Status: superseded
Owner: Tom
Created: 2026-08-05
Milestone: `../037-retained-session-recovery-promotion.md`
Depends on: card 097 selecting Pi

## Goal

Implement a separate Pi persistent-session load/replay profile after exact cwd
attachment becomes publicly provable.

## Scope

1. Leave ephemeral Pi interactive and structured routes unchanged.
2. Add exact persistent binding, bounded ordered load, and replay-free resume.
3. Map qualified load into continuation recovery.
4. Prove resource mismatch, switch failure, overflow, cancellation, disconnect,
   and joined cleanup.
5. Expose no session path or raw provider payload.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-pi`
- `effigy package:verify-affected swallowtail-adapter-pi`

## Stop Conditions

- do not execute unless card 097 closes the cwd-binding gate
- stop if process cwd or stored cwd is the only attachment evidence

## Auto-Continuation

Continue to card 101 when Pi passes independently.

## Disposition

Research 107 confirms that Pi RPC `0.83.0` still cannot caller-bind or
corroborate effective cwd through its public switch path. Card 097 selected no
Pi implementation. Revalidate only after the public attachment gate changes.
