# OpenAI Background Prepared Integration

Use this facade for one provider-owned OpenAI Responses background operation.
It binds the public API endpoint, public API-key audience, API billing,
provider support authority, exact GPT-5.6 route, temporary retention, one
maximum stream reattachment, native cancellation, and optional controlled
detachment followed by read-only restart reconciliation.

This is direct inference. ChatGPT login, ChatGPT subscription access, Codex
login, harness credentials, community OAuth, and delegated subscription access
do not satisfy this profile.

## Public API Preparation

`prepare_openai_background` requires:

- one configured-instance revision
- one execution host with `https://api.openai.com` explicitly approved
- the exact public API-key pay-as-you-go access profile
- observed or caller-asserted access evidence

`openai_background_access_profile` constructs the adapter-owned access shape
from one consumer-selected credential reference. Preparation acquires no
endpoint grant or credential. The prepared integration exposes its instance,
access evidence, available services, target-drift check, and low-level driver.

## Explicit Background Policy

`prepare_background_run` requires:

- request identity and text content
- exact route identity, revision, and `gpt-5.6` model identity
- a positive maximum-output-token bound
- one host-monotonic deadline
- `ProviderExecutionPolicy::Background`
- `ProviderRetentionPolicy::TemporaryAllowed`
- exactly one allowed stream reattachment

The full constructor keeps all three provider-operation policies visible.
`background_with_temporary_retention_and_one_reattachment` is the named
shortcut for the same fixed policy; it does not make background execution a
default for other structured runs.

`store=false` is fixed by the driver, but it is not a no-retention claim.
OpenAI temporarily retains response data so asynchronous execution, polling,
and stream reattachment can work. The prepared evidence records temporary
retention authority, not durable consumer storage or later recovery authority.

The default profile remains non-detachable. Call
`with_active_run_detachment` on `OpenAiBackgroundRunProfileInput` only when the
consumer will durably persist the emitted provider-run checkpoint and later
use the exact reconciliation path.

## Lifecycle

`start_run` delegates unchanged to the low-level structured-run driver:

1. create one response with background and streaming enabled
2. preserve the provider response reference separately from the runtime run
3. reattach at most once after the last accepted provider sequence
4. use one bounded retrieve when terminal stream truth is unavailable
5. use native provider cancellation after local cancel or deadline
6. close and join all network work before releasing the credential

Reattachment, retrieve, and cancellation manage the original inference
attempt. They never recreate input, retry inference, select another route, or
fall back to another credential. Provider cancellation may be confirmed, race
with completion, or remain unconfirmed; local cleanup does not rewrite that
remote truth.

After `response.created`, a qualified runtime event carries a
`ProviderRunCheckpoint`. Persist the whole opaque record with
`export_persisted(run.plan())`; do not parse or store the response id or cursor
separately. Persist it before requesting detachment.

An admitted detachable handle exposes `detachment()`. Requesting detachment
stops and joins only the local SSE observer, releases the credential lease,
and resolves locally as `TerminalStatus::Detached`. It sends no response
cancel or delete request. Calling ordinary close without requesting
detachment keeps the existing cancellation behavior.

## Restart Reconciliation

Restore through the same prepared integration:

1. construct `OpenAiBackgroundReconciliationInput` with a new request id, the
   exact model selection, persisted checkpoint, positive recovered-output byte
   bound, and optional deadline
2. call `prepare_run_reconciliation`
3. call `reconcile` with the host services

Preparation revalidates the exact driver, configured instance, target, host,
access profile, model route, protocol facade, and interface evidence before
network work. Reconciliation sends exactly one
`GET /v1/responses/{response_id}` and returns `Active`, `Completed`, `Failed`,
or `Cancelled`; terminal observations may carry bounded output and usage.

Reconciliation cannot create, retry, stream, cancel, delete, answer callbacks,
or operate on a provider session. It is not cross-process SSE reattachment and
does not poll. Consumers retain the opaque checkpoint until their own durable
state no longer needs reconciliation.

Webhooks, conversations, tools, files, search, batch jobs, retry, fallback,
and cross-process stream reattachment remain outside this profile.

`plan`, `request`, `evidence`, `low_level_driver`, and `into_parts` remain
available for inspection and advanced use.

See the compile-tested
[`prepared_background_response` example](../../crates/swallowtail-adapter-openai/examples/prepared_background_response.rs).
