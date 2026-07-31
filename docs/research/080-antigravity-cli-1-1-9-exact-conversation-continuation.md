# 080 Antigravity CLI 1.1.9 Exact Conversation Continuation

Status: promoted
Owner: Tom
Date: 2026-07-31

## Question

Can exact Antigravity CLI `1.1.9` provide portable turn-scoped continuity
without selecting ambient latest-session state or exposing provider-session
management?

## Evidence

The exact installed help surface and official CLI reference distinguish two
selectors:

- `--continue` resumes the most recent ambient conversation
- `--conversation <id>` resumes one explicit conversation

The headless stream qualified by Research 079 returns one bounded
`conversation_id` on init and repeats it on every later record. Deterministic
fixtures model a first successful turn, a later successful turn with the same
identity, a mismatched later identity, and a missing first identity.

No provider prompt or provider conversation ran. This proof qualifies the
documented protocol shape, not account availability.

## Selected Mapping

The Antigravity headless driver also implements one interactive-session role
for restarted harness continuity:

1. open an adapter runtime handle with no provider selector
2. start the first turn as a new owned print-mode child
3. accept an exact bounded conversation id only from a completed clean stream
4. start every later turn as a new owned child with
   `--conversation <exact-id>`
5. require the stream to repeat the same id
6. join the child before admitting another turn

The driver never selects `--continue`. The conversation id stays adapter
private: the handle returns no public provider-session reference or resume
binding, and stable diagnostics do not contain it.

The first qualified continuation profile is ambient-host read intent with
`--mode plan`, one exact model and working resource, durable provider-state
acceptance, 24 turns, 4,096 records per turn, and explicit per-turn deadlines.
Provider-enforced sandbox continuation is not inferred from the structured-run
profile; it can be added later with separate evidence.

## Failure And Commit Rules

Only a completed terminal result plus clean process cleanup commits the
conversation id. Missing identity, mismatched identity, provider or protocol
failure, cancellation, timeout, process failure, event-delivery failure, or
cleanup uncertainty invalidates the runtime handle.

An invalidated handle cannot start another child. There is no retry, fresh
conversation fallback, ambient lookup, enumeration, fork, archive, restore,
delete, native close, or public load/resume claim.

## Contract Result

Contract 043 already defines this exact harness-retained restarted-continuity
shape. No shared contract change is required.

## Risks

- provider state remains durable under Antigravity's own local storage policy
- the execution host sees the private conversation id in process arguments
- the exact-id route is qualified without a live account transcript
- write and provider-sandbox continuation remain unqualified

## Primary Sources

- [Antigravity CLI reference](https://antigravity.google/docs/cli/reference)
- [Antigravity headless mode](https://antigravity.google/docs/cli/headless)
- exact installed `agy` `1.1.9` help and artifact evidence from Research 078
