# 102 OpenAI Background Run Checkpoint And Reconciliation

Status: promoted
Owner: Tom
Updated: 2026-08-04

## Trigger

Research 099-101 left provider-session recovery realized but structured runs
uncovered. OpenAI background Responses already outlive one HTTP/SSE attachment,
carry an exact response id, and expose an ordered event cursor.

## Evidence

Official OpenAI documentation was rechecked on 2026-08-04.

- `background=true` creates provider-owned asynchronous work
- `GET /v1/responses/{response_id}` returns queued, in-progress, or terminal
  response truth
- background responses created with `stream=true` can reopen an SSE stream
  with `starting_after=<sequence_number>`
- every streamed event carries the cursor `sequence_number`
- `POST /v1/responses/{response_id}/cancel` remains a separate state-changing
  operation
- `store=false` still requires temporary provider retention

The current Swallowtail adapter already validates response ids and event
sequence strictly, performs one bounded in-process reattachment, polls after a
second disconnect, and deletes terminal response data. It does not persist the
response/cursor pair, reconcile after process loss, or detach without invoking
ordinary cancellation cleanup.

Sources:

- [OpenAI background mode](https://developers.openai.com/api/docs/guides/background)
- [Retrieve a Response](https://developers.openai.com/api/reference/resources/responses/methods/retrieve)

No OpenAI credential, account, request, or paid inference was used.

## Portable Boundary

Provider-session reconciliation must not be stretched around a run with no
session. Add a separate run-scoped boundary:

- `ProviderRunCheckpoint` binds one consumer runtime run, exact provider run,
  adapter-owned opaque cursor, and exact prepared route fingerprint
- its persisted form is versioned, bounded, integrity-checked, and redacted
- a qualified runtime event carries a recoverable checkpoint after response
  identity is established
- `ProviderRunReconciliation` performs one read-only observation of that exact
  provider run
- the outcome carries exact state plus bounded terminal output and usage when
  available

The checkpoint is not provider authority by itself. Restoration revalidates
driver, configured instance, target, host, access profile, model route,
protocol facade, and interface evidence before any request.

## OpenAI Mapping

The first mapping uses one exact response id and one `GET` request.

- queued or in-progress maps `Active`
- completed maps `Completed` with bounded output and usage
- incomplete or failed maps `Failed`
- cancelled maps `Cancelled`
- a mismatched response id fails closed
- no create, prompt, retry, stream attachment, cancel, delete, callback, or
  provider-session operation is sent

Reconciliation does not need to consume the cursor. A single retrieve request
is the only finite read-only snapshot the provider documents for an active
response. The cursor remains necessary durable attachment evidence and permits
later separately qualified stream recovery without manufacturing a position.

## Detachment

An opt-in prepared background profile may expose structured-run detachment
after the first checkpoint is available.

- detachment closes the local SSE attachment
- it sends no cancel or delete request
- it releases the credential lease and joins the local task
- local terminal truth is `Detached`
- later reconciliation uses the persisted checkpoint

Ordinary close, cancellation, deadline, terminal response deletion, and the
default prepared profile remain unchanged. A detached response remains under
the already accepted temporary-retention policy and expires under provider
retention rather than hidden consumer cleanup.

## Decision

Extend Contracts 048-049 with a distinct provider-run mapping. Realize the
portable checkpoint/reconciliation kernel and OpenAI background mapping in
g03.030. ACP retained-history reconciliation returns to the evidence gate
after this tranche.
