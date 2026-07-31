# Codex Child Turn Lifecycle Ownership

Date: 2026-07-31

## Outcome

Codex app-server now treats an admitted child's top-level `turn/started` and
`turn/completed` notifications as attributed child lifecycle. One separate
operation-local map binds each admitted child to its active child turn id.
Child activity must match that id; child completion removes it.

The root provider turn id never changes from child lifecycle or activity.
Child completion, failure, and error do not complete or fail the root
operation. Root terminal, callback, provider-request, provider-session, and
direct-control authority remain unchanged.

## Classification

Installed Codex `0.146.0` generated types define both lifecycle notifications
as `{ threadId, turn }`. The tagged source translates a child's core
`TurnStarted` and `TurnComplete` events into those same top-level app-server
methods. Spawn publishes the child thread and submits its initial input before
the parent tool emits its completed spawn item.

Combined with the Nucleus rerun—which persisted completed spawn topology and
then failed before child item activity—the rejected notification is the child
`turn/started` envelope. The frozen corpus records the observed delivery order:

1. root `item/completed` for `spawnAgent`
2. child `turn/started` with a child-local `turn.id`
3. child-owned item activity with that turn id
4. child `turn/completed` with the same turn id

Source evidence: Codex tag `rust-v0.146.0`, generated
`TurnStartedNotification`, `TurnCompletedNotification`, and `Turn` schemas;
tagged `app-server/src/bespoke_event_handling.rs` and
`core/src/agent/control/spawn.rs`.

## Ownership Boundary

- only exact children admitted by earlier completed successful spawn topology
  may start child lifecycle
- the existing 256-child admission bound is unchanged
- ordinary child activity must match the active child-local turn id
- child completion and failure emit portable subagent lifecycle observations
- child error is observational and cannot terminate the root
- unknown and cross-operation lifecycle uses a lifecycle-owner diagnostic
- mismatched or stale child turns use a child-turn diagnostic
- post-terminal child lifecycle uses a separate post-terminal diagnostic
- root termination clears both admitted children and active child turns

## Regression Evidence

- root turn start and completion retain root terminal behavior
- completed spawn followed by child start, activity, and completion succeeds
- lifecycle start and completion share stable activity and provider references
- child lifecycle and item activity carry `ActivityActor::Subagent`
- child completion, failed completion, and error leave the root active
- child callbacks and provider requests remain rejected by root ownership checks
- unknown, cross-operation, mismatched, stale, and post-terminal cases fail closed
- operation cleanup clears admission and active child-turn state
- `effigy validate:focused swallowtail-adapter-codex`: 142 passed
- `effigy package:verify-affected swallowtail-adapter-codex`: passed
- `effigy qa:docs` and `effigy qa:northstar`: passed
- Codex-only `rustfmt --check` and scoped `git diff --check`: passed
- repository-wide `effigy format:check`: blocked only by unrelated concurrent
  Cursor headless files, which this batch did not rewrite

## Boundaries

- No consumer repository changed.
- No raw provider payload parser or consumer workaround was added.
- No executable installation, authentication, model call, live provider test,
  publication, or other provider effect ran.
- Effigy doctor retains the known oversized-file findings outside this batch.

## Next

Nucleus may update its Swallowtail path dependency to the resulting commit and
rerun g05 card 026 from a fresh isolated state root. Swallowtail resumes Cursor
card 013.
