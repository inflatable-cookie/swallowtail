# 048 Provider Session Management Conformance

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../015-provider-session-management-foundation.md`

## Objective

Prove the provider-neutral management contract before any production adapter
uses it.

## Governing Refs

- Contracts 011 and 038
- cards 046-047
- existing local and remote-authoritative recording hosts

## Scope

1. Add a composable persistent-session management fixture pack.
2. Cover archive, restore, each deletion strength, already-absent,
   unsupported, incompatible, unverified-newer, and descendant-scope cases.
3. Cover arbitrary-id rejection and every binding or plan drift dimension.
4. Cover cancellation and deadline on both sides of dispatch.
5. Prove joined task, transport, resource, and credential ordering.
6. Assert consumer thread state and driver-owned cleanup never enter the role.

## Acceptance Criteria

- [x] fixtures use only public core/runtime/testkit APIs
- [x] local and remote-authoritative host identities both pass
- [x] no case promotes weak or unconfirmed deletion truth
- [x] no unsupported case dispatches
- [x] diagnostics remain bounded and redacted
- [x] existing thirteen conformance profiles remain unchanged

## Validation

- focused core, runtime, and testkit tests
- all provider-neutral conformance profiles
- `effigy check:rust`
- `effigy format:check`
- `git diff --check`

## Stop Conditions

- conformance needs provider-private payloads
- a shared assertion flattens archive and deletion
- the new pack widens existing operation profiles implicitly

## Auto-Continuation

No. Close roadmap 015 and make card 049 ready.

## Completion Evidence

- public fixtures cover qualified, unverified-newer, incompatible, and
  unsupported routes
- the assertion pack covers archive, restore, all three deletion strengths,
  already-absent, descendant scope, local and remote-authoritative hosts
- every constructible binding field and immutable plan field is drifted
  independently; raw copied session identity cannot replace the planned
  binding
- cancellation and deadlines are covered before and after dispatch
- task join precedes resource and credential release
- all 13 existing operation profiles remain unchanged
- 180 focused core, runtime, and testkit tests pass
- workspace Rust check, formatting, docs QA, Northstar QA, and diff checks pass
