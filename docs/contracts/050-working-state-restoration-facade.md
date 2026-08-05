# 050 Working-State Restoration Facade

Status: active
Owner: Tom
Updated: 2026-08-05

## Purpose

Provide portable execution facades for restoring consumer working state after
a runtime handle is lost without flattening read-only reconciliation,
stateful continuation recovery, or a later settled-session attachment.

## Boundary

Working-state restoration is a prepared convenience operation. It does not
select a provider, route, model, credential, working resource, or persisted
identity. The consumer must restore the exact configured facade and supply the
route-specific preparation input already required by Contracts 017 and 048.

The facade wraps exactly one qualified method:

- `ProviderSessionReconciliation`
- `ProviderRunReconciliation`
- `ProviderSessionContinuationRecovery`
- `ProviderSessionAttachmentRecovery`
- `FreshSessionReplacement`

Preparation selects the strongest supported method for the exact route.
Execution consumes the prepared facade once.

## Selection

Route-specific preparation must select reconciliation when that route has a
qualified reconciliation mapping. It may select continuation recovery only
when reconciliation is unavailable and exact bounded load/replay is qualified.
It may select attachment recovery only when neither stronger method is
available and an exact bound session can be reattached without claiming its
replay. It may select fresh-session replacement only when no provider context
can be restored and the ordinary new-session path is qualified.

Selection happens before provider work. A reconciliation error must be
returned unchanged. It must not trigger continuation recovery, retry, import,
prompt replay, another route, or another credential profile.

Unsupported routes fail during preparation. A provider-family capability or
another transport cannot authorize restoration.

## Outcomes

`WorkingStateRestorationOutcome` preserves five variants:

- `SessionReconciled(ProviderSessionReconciliationOutcome)`
- `RunReconciled(ProviderRunReconciliationOutcome)`
- `SessionRecovered(ProviderSessionContinuationRecoveryOutcome)`
- `SessionReattached(ProviderSessionAttachmentRecoveryOutcome)`
- `SessionReplaced(FreshSessionReplacementOutcome)`

Reconciled outcomes retain their exact existing attribution, state, replay,
output, usage, bounds, and cleanup truth.

A recovered session carries:

- the original consumer `RuntimeTurnId`
- bounded retained replay from the exact restored binding
- one live `InteractiveSessionHandle`

It carries no interrupted-turn provider state. The consumer must keep the lost
turn unresolved unless later independent provider evidence settles it. A new
turn may be sent only through the returned live session under ordinary session
policy.

A reattached session carries the original consumer `RuntimeTurnId` and one
live handle for the exact durable provider-session binding. It carries no
replay and no completeness claim. Provider updates received while attaching
are bounded, identity-checked, and discarded as non-authoritative.

A replacement carries the original consumer `RuntimeTurnId` and one newly
created live session. It explicitly means that provider context for the prior
session was not restored. The consumer may present that loss or later send its
own reconstruction; Swallowtail does not replay prompts, messages, tools, or
side effects.

## Continuation Recovery

Continuation recovery reuses the existing exact `LoadSessionRequest` and
`InteractiveSessionDriver::load_session` contract. It does not introduce a
second load vocabulary or claim that load is read-only.

The prepared session plan, resume binding, model route, model, execution host,
working resource, access policy, version, and configured instance must match.
Replay remains bounded by the qualified route. Load failure returns a safe
runtime failure and no live handle.

ACP load may reconnect requested MCP servers and establish live session
authority. The outcome therefore remains distinct from reconciliation even
when replay contains a terminal-looking message.

## Attachment Recovery And Replacement

Attachment recovery reuses the exact durable binding and plan agreement but
has a separate driver operation from load and resume. A provider-native load
may implement it only through the bounded discard phase defined by Contract
017. It cannot manufacture `LoadedSession`, weaken replay requirements, or
surface discarded updates as transcript evidence.

Fresh-session replacement consumes an already prepared ordinary open-session
operation. It grants authority only for the new session. It does not inspect,
cancel, mutate, archive, delete, or settle the lost provider session.

## Authority

Restoration grants only the authority of its selected method.

- reconciliation grants no prompt, callback, interruption, continuation,
  import, management, cleanup, or child-control authority
- continuation recovery grants only the ordinary loaded-session authority
  already present in its prepared session plan
- attachment recovery grants only the exact reattached-session authority
- fresh replacement grants only the ordinary new-session authority and
  explicitly grants no continuity with the lost session
- the facade never answers a waiting callback or infers terminal state from
  provider prose or transcript shape
- recovered-resource cleanup remains a separate exact operation

## Production Mappings

Session reconciliation:

- `codex.app-server`
- `opencode.http`
- `kimi-code.local-server`

Run reconciliation:

- `openai.background`
- `anthropic.managed-agent`

Continuation recovery:

- `claude-agent.acp`
- `kimi-code.acp`
- `alibaba.conversations` retained profile

Attachment recovery:

- `cursor-agent.acp`
- `grok-build.acp`

Fresh-session replacement:

- `antigravity.headless`
- `gemini-cli.acp`
- `pi.rpc`
- `qwen.headless`

The Alibaba mapping is resource-free and exists only on the separate retained
conversation profile. The operation-owned delete-on-close profile does not
inherit it. All other production routes remain unsupported until separately
qualified.

## Settled Observe Then Attach

Some reconciliation routes can return a usable live session only after the
read-only observation proves that provider work is no longer active. This is
a separate consuming sequence, not another
`WorkingStateRestorationMethod`.

`PreparedSettledSessionRestoration` owns two independently prepared bound
operations before provider work begins:

1. one exact provider-session reconciliation
2. one exact attachment for the same durable session binding

The second operation is either load with its own bounded replay or replay-free
resume. The reconciliation outcome cannot create, modify, or substitute its
binding, plan, request, cancellation, deadline, host, resource, access,
version, model, or provider-session authority. Adapter preparation must reject
any mismatch before the sequence exists.

The sequence consumes itself once. It always finishes reconciliation before
deciding whether attachment may start.

## Settled Eligibility

Attachment is eligible only for these validated reconciliation states:

- `Completed`
- `Failed`
- `Cancelled`
- `InactiveUnresolved`

Terminal states already require exact provider-turn attribution under
Contract 048. `InactiveUnresolved` may be exact-turn or provider-session
attributed; it authorizes attachment only because qualified route evidence
proved the session inactive. It does not settle the interrupted consumer turn.

These states never start attachment:

- `Active`
- `WaitingForProviderInput`
- `Unknown`

Waiting does not grant callback-answer authority. Unknown does not degrade to
inactive. Replay completeness, transcript shape, idle-looking prose, or a
terminal-looking replay item cannot change eligibility.

## Settled Sequence Outcomes

`SettledSessionRestorationOutcome` preserves one of two successful paths:

- `Observed(ProviderSessionReconciliationOutcome)` when the validated state is
  ineligible for attachment
- `Attached(SettledSessionAttachmentOutcome)` when attachment succeeds

`SettledSessionAttachmentOutcome` retains the complete reconciliation outcome
beside one distinct `SettledSessionAttachment`:

- `Loaded(LoadedSession)` preserves the attachment operation's bounded ordered
  replay and live handle
- `Resumed(InteractiveSessionHandle)` explicitly carries no replay

The first-phase replay and load replay remain separate snapshots. Swallowtail
does not merge, deduplicate, or promote either into consumer transcript truth.

`SettledSessionRestorationFailure` preserves failure phase:

- `Reconciliation(RuntimeFailure)` means no attachment operation started
- `Attachment { reconciliation, failure }` retains the successful complete
  reconciliation outcome beside the second-phase failure

An attachment failure does not erase or weaken observed provider truth. It
returns no live handle. The consumer may persist or present the reconciliation
outcome, then explicitly prepare a later action under current evidence.

## Settled Sequence Lifecycle

Reconciliation and attachment retain their own prebound cancellation and
deadline rules. Cancellation, deadline, cleanup failure, disconnect, stale
binding, or provider failure in phase one returns reconciliation failure and
starts no attachment. The same conditions in phase two return attachment
failure beside the completed reconciliation outcome.

The sequence adds no retry, prompt, callback answer, provider request,
cancellation of provider work, import, management, cleanup, child control,
route fallback, or credential fallback. It issues no attachment after a first-
phase failure or ineligible state.

The first production mappings are:

- Codex app-server reconciliation followed by bounded load/replay
- OpenCode HTTP reconciliation followed by bounded load/replay
- Kimi local-server exact-turn reconciliation followed by replay-free resume

## Conformance

Portable and route tests must prove:

- the facade reports its selected method before execution
- execution consumes the prepared facade
- each reconciliation variant preserves its original exact outcome
- continuation recovery returns the exact interrupted consumer turn, bounded
  replay, and one live loaded session without a provider-state claim
- exact binding, plan, attachment, and route drift fail closed
- reconciliation failure cannot invoke load
- recovery failure returns no handle
- attachment updates are bounded and discarded without a replay claim
- replacement reports provider-context loss and performs no prompt replay
- unsupported routes are not silently promoted
- both settled-sequence operations are fully prepared before provider work
- eligible state dispatches exactly one attachment after reconciliation
- active, waiting, and unknown state dispatch no attachment
- first-phase failure dispatches no attachment
- attachment failure preserves the complete reconciliation outcome and returns
  no handle
- loaded and resumed attachments retain distinct replay truth
