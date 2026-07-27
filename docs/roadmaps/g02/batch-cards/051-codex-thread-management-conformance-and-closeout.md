# 051 Codex Thread Management Conformance And Closeout

Status: completed
Owner: Tom
Created: 2026-07-26
Milestone: `../016-codex-thread-lifecycle-proof.md`

## Objective

Prove the Codex production mapping across version, topology, lifecycle,
failure, and cleanup boundaries.

## Governing Refs

- Contracts 011, 029, and 038
- cards 049-050
- existing Codex app-server conformance packs

## Scope

1. Run the shared management assertions against the production driver.
2. Cover baseline, lifecycle introduction boundaries, latest-qualified,
   exclusions, and one permitted unverified-newer point.
3. Cover active-target rejection, archived deletion, missing-rollout
   tolerance, unknown and repeated-delete failures, descendants, notification
   disagreement, cancellation, deadline, disconnect, and cleanup degradation.
4. Run local and remote-authoritative host fixtures.
5. Update Codex prepared guidance and roadmap evidence.

## Acceptance Criteria

- [x] every qualified lifecycle segment passes its exact behavior revision
- [x] unsupported legacy and drift cases cause no provider effect
- [x] hard deletion and descendant scope never exceed provider evidence
- [x] after-dispatch loss remains unconfirmed
- [x] app-server work joins before resource and credential release
- [x] existing exec, catalogue, session, callback, and range tests remain green

## Validation

- full Codex adapter suite
- provider-neutral management conformance
- `effigy check:rust`
- relevant docs and example checks
- one meaningful repository validation round

## Stop Conditions

- topology changes lifecycle semantics
- existing Codex range support must shrink
- a failure path loses destructive uncertainty

## Auto-Continuation

No. Close roadmap 016. Roadmaps 017 and 018 may then proceed independently.

## Completion Evidence

- the prepared production matrix covers archive at `0.80.0`, restore at
  `0.92.0`, delete at `0.140.0`, latest-qualified `0.145.0`, and explicit
  unverified-newer `0.146.0`
- exact introduction and exclusion boundaries remain frozen by the card 049
  lifecycle corpus
- tests cover new and resumed bindings, local and remote-authoritative hosts,
  binding drift, provider rejection, disconnect, malformed response,
  notification disagreement, cancellation, deadline, cleanup degradation,
  and descendant delete scope
- lifecycle operations reuse exact app-server invocation, process, working
  resource, task join, deadline, and cancellation mechanisms
- Codex prepared guidance and the provider route matrix include the new
  normal path
- the full Codex adapter suite passes 107 tests; the full workspace suite
  passes 769 tests with two existing leaky-test annotations and four skips
- Rust, format, docs, Northstar, and diff checks pass; `effigy doctor` remains
  at the pre-existing 25 findings (17 warnings, 8 errors)
- roadmap 016 is complete; card 052 is the sole next task
