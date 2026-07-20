# Runtime Crate And Role Traits

Status: completed
Owner: Tom
Roadmap: 005 Async Runtime and Conformance
Updated: 2026-07-19

## Goal

Create `swallowtail-runtime` with the smallest object-safe executor-neutral
driver and host-role skeleton authorized by Contracts 008-010.

## Scope

- workspace crate and focused module front door
- explicit boxed `Send` future alias
- boxed `futures_core::Stream` event boundary
- separate discovery, structured-run, interactive-session, and serving roles
- dynamic registration using `Arc<dyn Trait>`
- typed capability-scoped host-service lookup
- compile tests for object safety and `Send` boundaries

## Out Of Scope

- concrete operation behavior
- global executor or Tokio public dependency
- process, network, SDK, or provider adapters
- catch-all traits with unsupported stubs

## Acceptance Criteria

- all roles store and call through dynamic trait objects
- missing role is representable before a call
- public runtime API depends only on core and `futures-core`
- no task is spawned by the skeleton

## Validation

- compile and focused runtime tests
- `effigy qa`
- `git diff --check`

## Closeout

- `swallowtail-runtime` is realized with explicit boxed `Send` futures and a
  boxed `futures-core` event-stream boundary.
- Discovery, structured-run, interactive-session, and serving-instance roles
  are separate object-safe traits stored through `Arc<dyn Trait>`.
- Registrations reject role implementations absent from their descriptor;
  missing roles remain explicit `Option` values.
- Host services are independently typed and optional. The skeleton neither
  initializes an executor nor spawns work itself.
- Runtime dependencies are only `swallowtail-core` and `futures-core`.
