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

Preparation selects the strongest supported method for the exact route.
Execution consumes the prepared facade once.

## Selection

Route-specific preparation must select reconciliation when that route has a
qualified reconciliation mapping. It may select continuation recovery only
when reconciliation is unavailable and exact bounded load/replay is qualified.

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

## Authority

Restoration grants only the authority of its selected method.

- reconciliation grants no prompt, callback, interruption, continuation,
  import, management, cleanup, or child-control authority
- continuation recovery grants only the ordinary loaded-session authority
  already present in its prepared session plan
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
- unsupported routes are not silently promoted
