# 050 Working-State Restoration Facade

Status: active
Owner: Tom
Updated: 2026-08-05

## Purpose

Provide one portable execution facade for restoring consumer working state
after a runtime handle is lost without flattening read-only reconciliation and
stateful continuation recovery.

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

`WorkingStateRestorationOutcome` preserves three variants:

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

Attachment recovery:

- `cursor-agent.acp`
- `grok-build.acp`

Fresh-session replacement:

- `antigravity.headless`
- `gemini-cli.acp`
- `pi.rpc`
- `qwen.headless`

All other production routes remain unsupported until separately qualified.

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
