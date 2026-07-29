# Retained Execution Contract Fit And Corpora

Status: promoted
Owner: Tom
Date: 2026-07-28

## Question

What shared contract and offline corpus are required to make Kimi
harness-managed retry honest and add bounded local-server active-turn
reattachment?

## Method

- traced `ProviderRecoveryPolicy`, `StreamReattachmentPolicy`, and their
  existing provider-managed implementations
- inspected the Kimi headless and local-server validation and retry decoders
- traced WebSocket v2 subscription, acknowledgement, cursor, resync,
  cancellation, deadline, and cleanup code
- reused exact Kimi release and source identity from Research 046
- froze one bounded secret-free contract corpus

No executable, account, credential, provider request, paid operation,
container, or model server was used.

## Contract Decision

Contract 042 owns two harness-specific lifecycle dimensions.

Managed recovery is explicit acceptance of exact harness retry behavior. It
does not authorize Swallowtail retry, prompt replay, session replacement,
fallback, or model selection.

Active-turn stream reattachment replaces one failed stream attachment while
preserving the exact harness session, prompt, turn, runtime, cursor, access,
model, deadline, and cancellation identity. It is not Contract 021 background
execution, Contract 017 session resume, or Contract 027 planned rollover.

Contract 039 now requires an exact structured harness route to satisfy
Contract 042 before accepting either policy.

## Exact Kimi Range

The guaranteed range remains unchanged:

| Route | Qualified points |
| --- | --- |
| headless managed recovery | `0.29.0`, `0.29.1`, `0.29.2` |
| local-server managed recovery | `0.28.1`, `0.29.0`, `0.29.1`, `0.29.2` |
| local-server reattachment | `0.28.1`, `0.29.0`, `0.29.1`, `0.29.2` |

The exact source identities, behavior milestones, selected runner, event
schemas, WebSocket control, and global-event delta remain those frozen by
Research 046. Releases above `0.29.2` remain visible unverified-newer and do
not extend guaranteed lifecycle behavior.

Kimi ACP inherits no recovery or reattachment claim. The experimental v2
headless runner remains excluded.

## Managed-Recovery Corpus

Both selected Kimi routes expose `turn.step.retrying`.

The corpus fixes:

- prohibited policy rejection before process, endpoint, credential, prompt,
  or provider effects
- explicit `ManagedAllowed` acceptance on the exact selected role
- failed attempt `1`, next attempt `2`, maximum `3`, delay `100 ms`, and safe
  status `429`
- completion after the harness-owned retry without another Swallowtail run
- missing, decreasing, over-maximum, contradictory, foreign, and malformed
  retry rejection
- no provider error name, message, payload, session, prompt, credential, or
  endpoint in stable diagnostics

The local-server event is correlated with the active provider turn. The
headless event remains process-run scoped. Neither path chooses the retry or
its delay.

## Reattachment Corpus

The local-server corpus fixes:

- one maximum automatic reattachment
- the same session, prompt, provider turn, runtime turn, credential lease,
  endpoint, model, deadline, and execution host
- last accepted cursor `{seq: 11, epoch: fixture-epoch-private}`
- exact `subscribe` from that cursor
- acknowledgement accepting exactly the bound session with no resync
- first new durable event at sequence `12` and the same epoch
- duplicate suppression only at or below the accepted cursor
- gap, epoch change, foreign session, foreign turn, resync, malformed
  acknowledgement, and second disconnect failure

No reattachment path may dispatch prompt submit, session create, callback
reply, model change, route change, or credential reacquisition.

Disconnect before a valid prompt id is known remains unconfirmed remote state.
After a valid id, reattachment failure also cannot prove remote stop.
Cancellation and deadline still target the same provider prompt through the
already-qualified native abort path.

## Cleanup

The current route already owns joined WebSocket blocking work, task work,
credential release, and attached versus owned-foreground cleanup.

Implementation must join the failed attachment before or as the replacement
attachment is admitted, then join the final attachment before credential
release. An owned foreground Kimi server cannot be detached to preserve work.
An attached server remains external.

## Implementation Boundary

Card 105 is contract-ready for three matrix movements:

1. headless Kimi managed recovery, represented as route-dependent on the
   combined ACP/headless solution
2. local-server Kimi managed recovery
3. local-server Kimi maximum-one active-turn stream reattachment

No new operation role, credential mechanism, endpoint, version segment,
background-run policy, session-resume binding, or consumer API is required.
The existing provider recovery and stream reattachment policy types remain
the common surface.

## Sources

- [Kimi Code `0.29.2` WebSocket control](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/protocol/ws-control.ts)
- [Kimi Code `0.29.2` event schemas](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/packages/kap-server/src/protocol/events-zod.ts)
- [Kimi Code `0.29.2` headless renderer](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.29.2/apps/kimi-code/src/cli/prompt-render.ts)
