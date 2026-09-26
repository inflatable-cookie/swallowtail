# Harness-Managed Recovery And Active-Turn Reattachment

Status: active
Owner: Tom
Updated: 2026-07-28

## Purpose

Represent exact harness-owned retry and active-turn stream reattachment
without granting Swallowtail inference retry, prompt replay, session
replacement, or recovery policy.

This contract applies only to independently qualified harness routes. It does
not widen provider-owned background runs, provider-hosted managed agents,
session resume, or transport reconnect.

## Separate Dimensions

The following remain independent:

- provider or harness-managed retry
- Swallowtail inference attempt
- consumer retry
- active provider turn
- durable provider session
- stream attachment
- transport connection
- provider event cursor
- common runtime event sequence

A harness may retry one internal model step while the same provider turn
continues. That does not create another Swallowtail run or authorize
Swallowtail to resend input.

An active turn may outlive one stream attachment. Attaching again to that
turn is not session load, session resume, background inference, provider
rescheduling, or prompt replay.

## Managed Recovery Agreement

`ProviderRecoveryPolicy::Prohibited` is the default. A route whose selected
harness may retry internally must reject that policy before executable,
endpoint, credential, prompt, or provider effects.

The route may proceed only when:

- the plan declares exact provider-managed-recovery capability
- the request explicitly selects `ManagedAllowed`
- the exact interface range and behavior revision qualify the retry surface
- provider retention and all other operation policy dimensions still match

Managed recovery grants the harness only its documented internal behavior. It
does not grant Swallowtail authority to:

- resend a prompt or callback result
- create or replace a provider session or turn
- choose a delay, retry count, model, route, endpoint, or credential
- continue after the harness reports terminal failure
- conceal recovery evidence or flatten it into ordinary local progress

Retry records are provider evidence. Exact attempt numbers, maximum attempts,
delay, and safe status class may be normalized when the selected interface
supplies them. Provider error messages, raw payloads, prompts, tool values,
credentials, session ids, and endpoint values remain private.

Malformed, decreasing, contradictory, foreign-turn, or out-of-range retry
evidence fails closed. Swallowtail never invents an attempt count from
disconnects or repeated events.

## Active-Turn Reattachment Agreement

Stream reattachment is separately disabled by default. A positive bounded
policy requires an exact capability and maximum count.

One reattachment must preserve:

- configured instance and exact driver
- endpoint audience and credential lease
- execution host and topology
- provider session and active turn or prompt identity
- runtime run or runtime turn
- model route and selected model
- deadline and cancellation control
- last accepted provider cursor

The adapter opens a new stream attachment and submits no prompt, callback
result, session-create, or inference request. The provider must acknowledge
the exact target and accept continuation strictly after the last durable
cursor.

Duplicate events already accepted may be discarded only by exact cursor
identity. Missing, decreasing, foreign, contradictory, malformed, unknown, or
resynchronization-required evidence fails closed. Projected runtime events
retain their own monotonic sequence.

The first Kimi proof permits one reattachment. Exhaustion is a transport
failure with potentially continuing provider work. It never becomes another
reattachment loop, polling loop, prompt replay, recovery attempt, or fallback.

## Before And After Effect Truth

Stream loss before a valid provider turn or prompt reference is known cannot
be recovered. The operation reports safe unconfirmed provider state and sends
no replacement prompt.

After a valid reference exists, one authorized reattachment may occur while
the same deadline remains live. Reattachment failure cannot prove that
provider work stopped.

Cancellation or deadline while detached still targets the same known provider
turn through an independently qualified control path. If the control request
or provider result cannot be confirmed, local terminal state and remote stop
truth remain separate.

## First Kimi Subset

The first proof binds:

- Kimi Code headless `0.29.0..=0.29.2` managed retry records
- Kimi local server exact `0.28.1`, exact `0.29.0`, and
  `0.29.1..=0.29.2` managed retry records
- Kimi local-server WebSocket protocol v2 `{seq, epoch}` cursors
- one maximum local-server active-turn reattachment
- the same bearer credential lease, loopback endpoint, provider session,
  prompt id, model, working resource, deadline, and execution host
- exact subscription acknowledgement with one accepted session and no
  resynchronization requirement

Kimi ACP does not inherit either capability. Kimi headless has no stream
reattachment. Kimi local-server background execution remains non-applicable:
the retained object is a harness session and turn, not a Contract 021
background direct-inference operation.

The selected default Kimi v1 headless runner may retry internally. The
experimental v2 runner remains excluded. Stable versions above `0.29.2`
remain visible unverified-newer and require explicit caller acceptance without
extending the guaranteed recovery or reattachment range.

## Cancellation, Deadline, And Cleanup

Every terminal path:

1. stops new input and callbacks
2. closes the current stream attachment
3. sends exact native interruption when qualified and still required
4. joins every attachment and operation-scoped task
5. releases working resources and credential leases in route order
6. stops an owned foreground harness only after its network work joins

An attached harness is preserved. An owned foreground harness is not detached
to keep provider work alive. Drop performs no reattachment, prompt replay,
cancel, or cleanup claim.

No global executor, detached reader, retry task, reconnect loop, poller,
timer, callback, credential task, or cleanup task is permitted.

## Conformance

Deterministic exact-range fixtures prove:

- prohibited recovery rejected before effects
- explicit managed recovery accepted only on selected Kimi roles
- exact retry attempt, maximum, delay, ordering, and redaction
- malformed, decreasing, contradictory, and foreign retry rejection
- reattachment disabled and one-maximum policy agreement
- same-session, same-prompt, same-turn, same-cursor reattachment
- no prompt, callback, session, model, route, or access replay
- duplicate discard and gap, epoch change, foreign session, resync, malformed
  acknowledgement, and second-disconnect failure
- cancellation and deadline before attachment, while detached, and after
  reattachment
- attached and owned-foreground topology cleanup
- joined network and task work before credential release

Default QA uses no Kimi executable, account, credential, provider request,
paid inference, container, or model server.

## Acceptance

- provider-managed recovery is explicit and route exact
- prohibited recovery cannot silently permit harness retry
- reattachment never becomes retry, replay, resume, or fallback
- session, turn, prompt, attachment, cursor, and runtime identities stay
  separate
- uncertain provider work remains uncertain
- exact version and support authority remain visible
- every local task joins before access release
