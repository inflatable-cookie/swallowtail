# 2026-08-04 Portable Activity Key And Cursor Isolation

## Outcome

T3 Code issue 871 exposed a consumer projection keyed by one provider-backed
message id. Reuse across two threads caused the second row to overwrite the
first. The collision is legal: provider message and item ids are not global
consumer identities.

Swallowtail now exposes one provider-neutral `ActivityKey` containing the exact
`ActivityOperationId` and operation-local `ActivityId`. Every
`ActivityObservation` exposes it through `key()`. The key supports equality,
hashing, ordering, cloning, and redacted default formatting.

Contracts 009 and 044 now require consumers to keep runtime run and turn ids
unique while an earlier operation remains active or retained. Consumers
persist or upsert activity by the complete composite key. `ActivityId` and
`ProviderActivityRef` remain opaque operation-local evidence, never durable
global keys. Consumer thread and transcript-message identities remain separate.

This boundary applies to every provider route. It does not rewrite native ids,
add a global registry, or manufacture provider identity.

## Cursor Acceptance

The Cursor ACP fixture runs two turns in one session. Both turns emit the same
explicit `messageId`, then emit an assistant chunk without a `messageId` so the
adapter uses its existing fallback id.

For each form, the standalone activity id and provider reference repeat. The
runtime operation owners and composite activity keys differ. The observations
therefore remain distinct without changing the Cursor wire projection.

Public Nucleus and Soundcheck examples now carry `ActivityKey` directly.
Guidance rejects activity-id-only and provider-reference-only projection keys.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-cursor` —
  154 tests passed across seven binaries
- `effigy package:verify-affected swallowtail-runtime swallowtail-adapter-cursor`
  — both extracted packages compiled
- `effigy qa:docs`
- `cargo fmt --all`
- `git diff --check`
- no authenticated provider work or live provider operation

## Current State

Cards 068-069 and roadmap g03.026 are complete. The sole Next Task has returned
to the g03 evidence gate.
