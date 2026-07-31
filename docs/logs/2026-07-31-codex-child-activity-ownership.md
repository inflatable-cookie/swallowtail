# Codex Child Activity Ownership

Date: 2026-07-31

## Outcome

Codex app-server now admits ordinary activity owned by the root provider
thread or by an exact child established earlier in the same turn through a
completed successful `spawnAgent` observation. The admitted set is capped at
256 exact child ids and cleared on turn termination.

Child-envelope activity is attributed through `ActivityActor::Subagent` and
does not enter root assistant output accumulation. Missing parent evidence
remains `SubagentParent::Unknown`.

## Ownership Boundary

Child admission applies only to ordinary activity. Root turn start and
completion, terminal outcome, callback, provider-request, provider-session,
and direct child-control authority did not change. Unknown, stale, and another
operation's child ids retain the existing session-mismatch failure class.

Contracts 044 and 045 and the realized architecture now record this boundary.

## Regression Evidence

- the frozen `0.146.0` case contains a completed root `spawnAgent` envelope
  followed by a separate notification whose top-level `threadId` is the child
- root spawn activity remains primary-owned
- the established child envelope is accepted and attributed to that child
- unknown, cross-operation, post-terminal, and child-terminal ownership fail
  closed
- operation cleanup empties the admitted set
- capacity rejection leaves the existing set unchanged
- `effigy validate:focused swallowtail-adapter-codex`: 139 passed
- `effigy package:verify-affected swallowtail-adapter-codex`: passed

## Boundaries

- No consumer repository changed.
- No provider payload parser or consumer workaround was added.
- No executable installation, authentication, model call, live provider test,
  publication, or other provider effect ran.
- Effigy doctor still reports the two known oversized-file errors already
  recorded outside this provider batch.

## Next

Nucleus may rebuild against the current Swallowtail worktree and rerun g05
card 026 from a fresh isolated state root. Swallowtail resumes Cursor card 012.
