# 098 Portable Activity Key And Cross-Operation Isolation

Status: promoted
Owner: Tom
Date: 2026-08-04

## Question

Can a consumer safely persist Swallowtail observable activity when one provider
reuses a message or item id across threads, sessions, or turns?

## Evidence

T3 Code issue 871 reproduces a cross-thread overwrite. Two distinct consumer
threads persist assistant projections with the same provider-backed message id.
The second upsert replaces the first because the database conflict target is
`message_id` alone. The first thread then returns no row.

This is legal provider behavior. Provider item and message references are not
consumer-global identities. The ACP activity schema also makes `messageId`
optional. Swallowtail's Cursor projection therefore uses the provider value
when supplied and a turn-local fallback when absent. Both forms may repeat in
another operation.

Swallowtail already carries `ActivityOperationId` and defines `ActivityId` as
operation-local. Its buffers and adapter projections are operation-owned, so no
cross-operation overwrite occurs inside Swallowtail. The public API still
requires consumers to assemble the pair manually, and the main activity example
reads only `activity_id()`. No deterministic case freezes repeated provider and
fallback ids across operations.

Runtime run and turn ids are consumer-supplied. Their uniqueness scope is not
currently explicit. A composite key cannot isolate retained operations if the
consumer reuses the operation owner itself.

## Decision

Promote one provider-neutral `ActivityKey` composed of:

- the exact `ActivityOperationId`
- the operation-local `ActivityId`

Every `ActivityObservation` exposes that key directly. `ActivityId` and
`ProviderActivityRef` remain unsafe as standalone durable keys and are not
rewritten to manufacture global provider identity.

Consumer-supplied runtime run and turn ids must remain unique across the
consumer's active and retained activity-projection domain. Consumers retain
their own thread and transcript-message identities; an activity key does not
become either one.

This rule applies to every route. Cursor supplies the immediate regression
because both repeated provider ids and its no-message-id fallback demonstrate
the boundary exactly.

## Validation Needs

- common proof that equal activity and provider references under different
  runtime operations produce distinct keys
- redacted default formatting for the composite key
- Cursor proof for repeated explicit `messageId` values across turns
- Cursor proof for repeated no-message-id fallback values across turns
- public guidance which projects and persists by `ActivityKey`
- no provider prompt, authentication, consumer edit, or transcript migration

## Source

- [T3 Code issue 871](https://github.com/pingdotgg/t3code/issues/871)
