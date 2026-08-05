# Provider State And Resource Lifecycle

Use this runbook for continuation, discovery, restart recovery, provider
session management, remote cleanup, and owned local runtimes. These are
separate authorities. Support for one does not imply another.

## Ordinary Continuation

An `InteractiveSessionHandle` may expose an opaque `SessionResumeBinding`.
Persist it only through `export_persisted` against the exact prepared session
plan. After restart, parse `PersistedSessionResumeBinding` and call
`restore_persisted` against the newly prepared matching plan, working
resource, and access policy.

Use `load_session` when the route returns bounded ordered
`SessionReplayItem`s plus a live handle. Use `resume_session` when the route
returns only a live handle. Neither operation synchronizes the consumer
database. A raw provider session id, copied binding, or transcript cannot mint
continuation authority.

Provider-private continuation on Anthropic and DeepSeek direct routes is not
serializable. It contains route-private state valid only inside the live
session and is destroyed on close. Those routes can create a fresh usable
session after process loss, but cannot recover the private continuation chain.

## Discover And Import External Sessions

Provider session catalogue and import are separate prepared operations.
Catalogue candidates are bounded display evidence. The consumer selects one;
import revalidates exact route, host, version, access, working resource, model,
and lifecycle before returning an ordinary binding.

Follow [Provider Session Catalogue And Explicit Import](provider-session-import.md)
for Codex app-server, Kimi ACP, and OpenCode HTTP. Do not construct bindings
from candidate ids or treat discovery as attachment authority.

## Recover After Process Loss

If an operation was persisted as active and its handle was lost, do not send
another prompt or mark it cancelled.

1. Restore the exact route binding or run checkpoint.
2. Prepare that route's `PreparedWorkingStateRestoration`.
3. Inspect `WorkingStateRestorationMethod` before execution.
4. Call `restore` once and preserve the exact outcome strength.
5. Apply complete replay only as complete replay; keep the interrupted turn
   unresolved for continuation, attachment, and replacement outcomes.

The common facade may reconcile exact provider state, recover a continuation,
reattach without authoritative replay, or replace a lost session/connection.
It never falls back between those strengths. See
[Working-State Restoration](working-state-restoration.md),
[Provider Operation Reconciliation](provider-operation-reconciliation.md),
and [Provider Operation Detachment](provider-operation-detachment.md).

Controlled detachment is only for qualified active operations with durable
later reconciliation. It stops and joins local observation while preserving
provider uncertainty. It is not ordinary close or cancellation.

## Provider Session Management

`ProviderSessionManagementBinding` is separate from a resume binding. Prepare
one exact `ArchiveProviderSessionRequest`, `RestoreProviderSessionRequest`, or
`DeleteProviderSessionRequest` through the selected route's
`ProviderSessionManagementDriver`.

Management binds an exact inactive provider session, action, expected effect,
working resource where applicable, access, version, and explicit destructive
authority. Archive, restore, history removal, data deletion, and hard deletion
remain distinct. `ProviderSessionManagementOutcome` reports provider effect
truth; it does not mutate or delete the consumer thread.

Native session close is route-specific. Closing a Swallowtail handle normally
releases the attachment and joins local work; it must not be presented as
archive or deletion unless the route's prepared plan explicitly says so.

## Owned Remote Resource Cleanup

Provider-owned runs may emit a
`ProviderRecoveredResourceCleanupBinding`. Persist it separately from the run
checkpoint. After exact inactive reconciliation, a separately prepared
`ProviderRecoveredResourceCleanupDriver` may consume it under an explicit
cleanup agreement.

Cleanup cannot prompt, retry, resume, cancel active work, answer callbacks, or
infer inactivity. Preserve `RemoteResourceDeletionOutcome::Confirmed` and
`Unconfirmed`; an attempted delete is not proof of deletion.

## Attached And Owned Local Runtimes

`AttachedServingHandle::close` releases an attachment. It exposes no generic
stop authority. `OwnedServingHandle::stop` is available only when Swallowtail
started and owns the serving instance. Stop must join the owned process and
leases; it does not delete model artifacts.

Installation, model pull, mutation, unload, arbitrary server discovery, and
provider-owned service lifecycle remain outside these handles. Consult the
Ollama and llama.cpp route guides for their exact attached/owned split.

## Consumer Ownership

The consumer owns local thread identity, atomic persistence, retention UX,
selection, refresh, destructive confirmation, replay deduplication, and retry
policy. Persist route identity beside every opaque binding or checkpoint.

Swallowtail owns exact binding validation, bounded provider observation,
route-qualified mutation, outcome strength, and joined cleanup. It does not
own a synchronization database, automatic recovery, provider fallback, or
consumer deletion policy.

Unknown, stale, cross-route, cross-host, cross-resource, active, corrupted, or
unsupported state fails closed. Never repair an opaque record or fall back to
creating a replacement under the same local thread without making the context
loss explicit.

## Route Applicability And Validation

The [feature matrix](provider-solution-feature-matrix.csv) covers load, resume,
persistent posture, catalogue/import, retained work, reattachment, managed
recovery, archive/restore/delete, native close, remote cleanup, and owned
runtime lifecycle. The specialist guides above preserve stronger recovery
differences that cannot be reduced to one boolean.

Every applicable route has a compiling normal-path example in the
[integration guide map](integration-guide-map.md).

```sh
effigy check:examples
effigy qa:docs
effigy qa:routes
```

Deterministic tests use frozen provider evidence. Destructive provider
management, live recovery, authentication, and remote deletion remain
separately operator-gated.
