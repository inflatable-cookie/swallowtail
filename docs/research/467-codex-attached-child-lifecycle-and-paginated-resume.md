# Codex Attached-Child Lifecycle And Paginated Resume

Status: promoted.
Date: 2026-11-02.
Scope: `codex.app-server` thread delete, ordinary thread listing, and selected
`thread/resume` on `codex.cli`. Research 467.

## Question

Research 463 froze the `0.165.0` hop and named two gates before either Codex
ceiling could move: an unexplained shipped Linux `bwrap` byte change, and a new
attached-child delete guard whose effect on the selected lifecycle and on
paginated resume was unqualified. This record answers the lifecycle and resume
half.

1. Can the adapter keep claiming `ProviderDefinedDescendants` hard delete when
   an externally forked, imported, restored, or pre-existing parent holds an
   attached child that ordinary listing hides?
2. Is the `0.165.0` paginated-resume model-context reuse equivalent to the
   `0.164.0` unconditional reload for the request shape Swallowtail sends?

## Decision

Evidence-only private adaptation. No public operation, capability, diagnostic
code, behavior revision, or range claim changes.

- Delete: the guarantee holds unchanged. `0.165.0` refuses the delete before
  any effect when any delete candidate has an attached child, both in the
  app-server precheck and again under writer ownership in the store. The
  refusal is a JSON-RPC `invalid_request`, which the adapter already maps to
  `failed_before_effect` with no `ThreadDeleted` notification. Contract 038
  already separates a refused attempt from an applied one, so
  `ProviderDefinedDescendants` describes only the successful path and stays
  accurate.
- A real consumer-visible limitation is now recorded rather than fixed: while
  any candidate (root or descendant) has an attached child, delete is
  terminally refused through this route, and the adapter's ordinary listing
  cannot surface that child for a follow-up delete. Removing the limitation
  needs either a new `attachedToThreadId` listing scope or a distinct stable
  refusal diagnostic; both are consumer-visible additions under Contracts
  038/046/061 and need a separately recorded ruling.
- Resume: `0.165.0` retains the prepared model context only across the config
  recheck loop and reuses it only when the stored thread is paginated, the
  conversation id, rollout path, and revision still match, and
  `is_model_context_current` returns true. A mismatch, missing revision, false,
  or error reloads the latest model context, and the store revalidates the
  revision again under writer ownership. The selected request never supplies
  history or a path, so the model-visible context equals the `0.164.0`
  unconditional reload. No new claim, no behavior revision, no equivalence
  beyond the selected path.
- `0.165.0` stays `UnverifiedNewer` and both `0.164.0` ceilings stay. The
  shipped Linux `bwrap` provenance gate from Research 463 is untouched and
  still open.

## Method

Re-fetched every cited file from the exact frozen tag commits by path and
byte-verified each against the per-file SHA-256 set Research 463 already froze
(`docs/research/463-...`, `contentSha256`). No downloaded vendor binary was
executed, no prompt was sent, no login or live session was used, and nothing on
the host was mutated. The per-file hashes, the frozen control-flow orders, and
the adapter expectations live in
`crates/swallowtail-adapter-codex/tests/fixtures/codex-attached-child-lifecycle/trace.json`.

The new proof drives the real prepared Codex adapter against the scripted
app-server process fixture in
`crates/swallowtail-adapter-codex/tests/prepared_profile_cases/attached_child_lifecycle.rs`.
The fixture speaks JSONL RPC, records every request and notification, and
reports process join state; it is a prepared-driver fake, not a live session.

| Version | Tag | Commit |
| --- | --- | --- |
| `0.164.0` | `rust-v0.164.0` | `6aaac694803004a63c6f393d153c96d22521a6b9` |
| `0.165.0` | `rust-v0.165.0` | `360319d8dee62d87e2d59fc0c21805d79519cfed` |

## Attached-child relations

`0.164.0` has no attached-child concept at all: no migration, no guard symbol,
and no `attachedToThreadId` list field. The whole relation is new in `0.165.0`.

The relation is persisted twice and both are read before a delete.

- `codex-rs/state/migrations/0062_threads_attached_parent.sql` adds
  `threads.attached_thread_parent_id TEXT` plus a partial index. It exists only
  at `0.165.0`.
- Rollout session metadata carries `forked_from_id`, retained as an attached
  parent only when `attached_thread` is true
  (`codex-rs/rollout/src/rollout_reference_index.rs`).

`RolloutReferenceIndex::scan` walks **both** `archived_sessions` and `sessions`,
so an archived attached child still blocks a delete. The partial scans
(`scan_unarchived`, `scan_unarchived_threads`, `scan_archived_threads`) say in
their own comments that their reference counts are partial and must not decide
whether a rollout can be deleted or compressed; the guard uses `scan`.

Ordinary listing hides the relation. `codex-rs/state/src/runtime/threads.rs`
carries `EXCLUDE_ATTACHED_THREADS` (`threads.attached_thread_parent_id IS
NULL`) and applies it to every listing whose relation filter is not
`AttachedThreadsOf`. The wire field that widens it is the experimental
`thread/list attachedToThreadId`. The adapter sends `cursor`, `limit`,
`sourceKinds`, `archived: false`, and `cwd` only.

Reachable parents therefore include a pre-existing local parent, a restored
parent, an imported parent, an externally forked parent, and a parent whose
*descendant* holds the child. The adapter never creates attached children — it
never sends `attachToSource` — and that does not exclude them: another client,
an import, a restore, or a prior interactive run can leave one behind.

## Delete control flow

`0.165.0` app-server `thread/delete`
(`codex-rs/app-server/src/request_processors/thread_delete.rs`) runs, in order:
acquire the thread-list state permit; compute the strict spawn-edge subtree ids;
validate the root thread is deletable; call
`LocalThreadStore::ensure_no_attached_threads` over **every** candidate id;
`prepare_thread_for_delete` per candidate (`prepare_thread_for_removal` then
`log_db.flush`); delete descendants in reverse order then the root; send the
empty response and one `ThreadDeleted` per deleted id.

The store then repeats the check under writer ownership
(`codex-rs/thread-store/src/local/delete_thread.rs`). `0.165.0` moved writer
lock acquisition ahead of the reference scan and added the recheck:

- `0.164.0` single delete: lock lifecycle, lock live writer, scan, build
  thread rollouts, external-reference check, acquire writer locks, cleanup,
  delete rollout and state rows.
- `0.165.0` single delete: lock lifecycle, lock live writer, **acquire writer
  locks**, scan, build thread rollouts, **check attached children**,
  external-reference check, cleanup, delete rollout and state rows.
- `0.165.0` batch delete (`delete_threads`, the path `thread/delete` uses):
  lifecycle locks for all sorted ids, live-writer locks for all sorted ids,
  writer locks, scan, build thread rollouts, **check attached children**,
  external-reference check, cleanup, delete each rollout, delete state rows.

`check_attached_children` returns `ThreadStoreError::InvalidRequest`, which
`thread_store_delete_error` maps to JSON-RPC `invalid_request` (`-32602`). The
adapter's pump maps any JSON-RPC error to
`swallowtail.codex.app_server.request_failed`, and the management path turns
that into `failed_before_effect`. Nothing is deleted, no notification is sent,
and no deletion strength is confirmed.

Strict descendants stay honest. The subtree comes from `thread_spawn_edges`,
the delete order is descendants-then-root, and `delete_threads_strict` deletes
exactly the listed rows with no implicit cascade. The guard precedes all of it,
so either the whole computed subtree is deleted, or the attempt refuses before
effect.

## Paginated resume

`0.164.0` and `0.165.0` both reach `thread/resume` for a stored thread, and the
stored thread's history mode selects the paginated path. Swallowtail sends
`threadId`, `model`, its access-policy fields, and (resume only, from `0.129.0`)
`excludeTurns: true`. It never sends `history`, `path`, `initialTurnsPage`,
`attachedToThreadId`, or `attachToSource`.

Frozen `0.165.0` flow:

- A request-supplied `history` bypasses the stored-history path entirely. Not
  selected.
- A non-paginated stored thread reads its full history through
  `stored_thread_to_initial_history`.
- A paginated stored thread enters
  `load_resume_initial_history_from_stored_thread`. When config loading
  released the list and goal locks it stored `PreparedResumeConfig`, including
  the prepared model context, and returned `ControlFlow::Continue` to recheck
  the thread and history. On the second pass the retained context is reused
  only when the stored thread is paginated, `conversation_id` equals the
  rechecked stored id, `rollout_path` equals the rechecked stored path,
  `history_revision` is present, and `is_model_context_current` returns true.
  Anything else — mismatch, missing revision, false, or error — calls
  `load_latest_model_context` and rebuilds `ResumedHistory`.
- The revision identity (`codex-rs/thread-store/src/local/history_revision.rs`,
  byte-identical across the two tags) covers the canonical path, device, inode,
  ctime and ctime nanoseconds, length, mtime, and creation time. Any concurrent
  append or replacement changes it.
- `codex-rs/thread-store/src/local/live_writer.rs` (`resume_thread`,
  byte-identical modulo surrounding edits) compares the supplied
  `history_revision` with a fresh read **after** acquiring the writer lock and
  reloads from the rollout on a mismatch, which closes the window between the
  recheck and writer ownership.

`0.164.0` always calls `load_latest_model_context` for a paginated resume.
`0.165.0` adds only the reuse shortcut. Because the selected request supplies
neither history nor path, and reuse is admitted only for an unchanged rollout
identity, the model-visible context equals what `0.164.0` reloads; the
difference is one avoided read, not a different conversation.

Restoration stays visible. `load_session` needs `thread.turns` and projects
them through the adapter replay projection; a response without them fails as
`swallowtail.codex.app_server.replay_malformed` and the connection is joined
before the failure returns. `resume_session` sends `excludeTurns: true`, gets
no turns, and continues through the adapter's own history role.

## Contract reconciliation

- Contract 038: the delete plan promises hard deletion with
  provider-defined descendant scope. A guard refusal is
  `failed_before_effect`, never `Applied`, and confirms no deletion strength;
  the applied path still deletes exactly the computed strict subtree. No
  narrowing and no new authority.
- Contract 046: catalogue scope stays one configured instance and one approved
  working resource. Attached children are a provider-hidden relation inside
  that scope, not an unadvertised Swallowtail scope; the adapter never widens
  the ordinary listing, and the record's existing rule that a page is a
  snapshot and not complete history already covers hidden providers rows.
- Contract 061: no projection row changes. Delete remains descriptive route
  support; whether the exact provider thread can currently be deleted stays a
  provider-state fact.
- Contract 029: this record qualifies behaviour inside the frozen `0.165.0`
  identity. It does not move a ceiling, publish a new stable, or claim the
  newest release.

## Proof

Real prepared-driver fake cases, all run in
`validate:current-codex-app-server`:

| Case | Assertion |
| --- | --- |
| `attached_child_delete_refusal_is_failed_before_effect` | `0.164.0` delete against an attached-child refusal returns `failed_before_effect`, no confirmed deletion strength, no affected scope, the `request_failed` diagnostic, exactly one `thread/delete`, no `thread/deleted`, and a joined process |
| `unverified_newer_attached_child_refusal_keeps_before_effect_truth` | the same refusal at `0.165.0` with explicit unverified-newer acceptance never reports provider deletion |
| `delete_after_dispatch_cancellation_joins_and_stays_unconfirmed` | cancelling a delete already on the wire yields `unconfirmed_after_effect`, no deletion strength, a forced stop and a joined process |
| `ordinary_catalogue_hides_attached_child_without_widening_scope` | a page that omits the attached child projects only the visible candidate, ends the traversal, and the recorded `thread/list` carries no relation filter, `attachedToThreadId`, `parentThreadId`, or `ancestorThreadId`, with `archived: false` |
| `load_requires_provider_history_and_resume_excludes_turns` | a load whose provider returns no turns fails as `replay_malformed` with the process joined; resume sends `excludeTurns: true` and none of the unmapped fields |
| `attached_child_trace_is_frozen_and_mutation_sensitive` | the frozen hashes and both control-flow orders are asserted, including guard-before-effect and revision files byte-identical across the hop |

## Claim shape

Both ceilings remain `0.164.0`; `0.165.0` and `0.165.1` remain
`UnverifiedNewer`. Baselines, behavior revisions, exclusions, and claim ids are
unchanged. Selected delete, archive, restore, catalogue, and resume shapes are
unchanged. The attached-child limitation and the paginated-resume reuse
conditions are recorded in the Codex guide and the route feature matrices
without adding a capability, scope, or diagnostic.

Still open from Research 463: the shipped Linux `bwrap` provenance gate. It
needs its own brief and does not follow from this record.

## Sources

- [Official npm package](https://www.npmjs.com/package/@openai/codex)
- [Codex `0.165.0` source tag](https://github.com/openai/codex/tree/rust-v0.165.0)
- [Codex `0.164.0` source tag](https://github.com/openai/codex/tree/rust-v0.164.0)
- [Research 463](./463-codex-exec-app-server-0-165-0-identity.md)
- [Research 452](./452-codex-exec-app-server-0-164-0-identity.md)
