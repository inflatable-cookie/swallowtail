# 007 Joined Local Host Service Composition

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../002-prepared-consumer-integration-boundary.md`

## Objective

Provide reusable local task and host-service composition without introducing a
global executor, implicit target search, or detached work.

## Governing Refs

- Contract 037 (active)
- Contracts 004, 009-010, and 032
- completed cards 005-006

## Scope

1. Add a per-host joined scoped task service to `swallowtail-host-local`.
2. Compose exact host-owned services through one inspectable builder/result.
3. Support explicit executable target selection and approval as a separate host
   action before installed discovery.
4. Preserve opaque references, host identity, resource scope, limits, and
   cleanup authority.
5. Prove local and remote-authoritative service sets do not substitute.
6. Add compile-tested low-level host examples.

## Acceptance Criteria

- [x] consumers no longer need their own thread-backed `ScopedTaskService`
- [x] every task remains operation-scoped and joined
- [x] no process-global executor or detached thread exists
- [x] target selection is explicit and discovery receives one approved target
- [x] raw paths remain host-private
- [x] remote-authoritative topology remains supported
- [x] roadmap g02.002 closes with card 008 ready

## Validation

- focused host-local and runtime tests
- cancellation, panic, deadline, and cleanup failure fixtures
- local/remote host identity conformance
- warnings-denied clippy and docs
- `effigy check:rust`
- `git diff --check`

## Evidence Required

- service ownership and join-order assertions
- explicit target-selection fixture
- public example and API diff
- roadmap closeout log and card 008 readiness

## Stop Conditions

- a task can outlive its host or operation
- convenience scans ambient targets without explicit selection
- a raw host path enters a stable public record
- remote topology needs a local-only assumption

## Auto-Continuation

Yes, only if roadmap g02.002 closes and card 008 is explicitly ready.

## Closeout

Completed 2026-07-24.

- `LocalScopedTaskService` owns one thread per scoped task and joins it through
  the returned handle or its drop fallback
- task spawn and panic failures normalize to stable safe diagnostics
- `LocalHostServices` composes the exact local task, time, process, network,
  credential, resource, attachment, artifact, endpoint, and schema services
  under one execution-host identity
- `approve_installed_executable` performs one explicit path approval and
  returns one opaque target for Contract 032 discovery
- local and remote-authoritative host identities still reject substitution
- the public example compiles without ambient target or credential discovery

Roadmap g02.002 is complete. Card 008 is ready.
