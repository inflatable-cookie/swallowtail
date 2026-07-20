# Interactive-Session Callback Contract

Status: completed
Owner: Tom
Roadmap: 008 Nucleus Interactive-Session Readiness
Updated: 2026-07-19

## Goal

Promote concrete provider-neutral session options and callback exchange records
required by the Nucleus live-chat seam.

## Scope

- session instructions and selected reasoning option
- model reasoning metadata
- tool declarations as transport data
- callback request, id, response, error, wait, cancellation, and deadline
- active-turn ownership and event ordering
- extension handling for provider-specific callback kinds

## Out Of Scope

- Nucleus tool schemas or execution
- task, goal, memory, review, receipt, or persistence types
- Codex wire envelopes
- approval policy decisions

## Acceptance Criteria

- consumers own declarations and callback execution authority
- Swallowtail owns only bounded correlated transport
- unknown and unsupported callbacks fail explicitly
- callback wait remains cancellable and deadline-bound
- all identities remain distinct and redacted

## Validation

- compile probes and deterministic callback fixtures
- long-lived RPC profile expansion
- `effigy qa`
- `git diff --check`

## Stop Condition

Stop if the contract would introduce a generic tool executor or consumer
authority into Swallowtail.

## Closeout

- Contract 012 fixes session instructions, exact reasoning selection, bounded
  tool declarations, callback records, event order, wait states, cancellation,
  deadlines, extension rejection, and the consumer-authority boundary
- open and resume requests now carry optional session options while model
  selection remains bound to preflight
- turn handles expose an optional one-shot callback exchange with a bounded
  request stream and object-safe response port
- callback and runtime ids, declaration content, schemas, and callback bodies
  remain redacted by default
- deterministic fixtures prove correlation, exactly-once response state,
  timeout abandonment, late-response rejection, and long-lived RPC profile
  coverage
- the existing Codex proof driver rejects unimplemented session options before
  process work; provider translation remains card 027
