# 049 llama.cpp Attached Driver

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../015-llama-cpp-attached-runtime-proof.md`

## Objective

Implement attached llama.cpp readiness, catalogue, and direct inference for the
selected facade.

## Scope

- external attach, health, observed route/model metadata, bounded inference,
  cancellation limits, and attached close

## Out Of Scope

- server start/stop
- artifact management
- unobserved tool/reasoning claims

## Acceptance Criteria

- [x] close never stops the server
- [x] capabilities come from deployment evidence
- [x] no model-family branch enters core/runtime

## Validation

- focused adapter fixture tests
- `git diff --check`

## Evidence

- separate model-catalogue and structured-run roles use attached loopback
  HTTP/SSE
- every catalogue and run observes `/health` and `/props` before provider work
- build, alias, ChatML capabilities, and text-only modalities must match the
  frozen deployment
- the driver acquires one host-approved endpoint and no credential, process,
  artifact, or serving-lifecycle authority
- one explicit output bound produces one streaming attempt with ordered output
  and usage; unobserved content fails closed
- cancellation and deadline close and join only owned work; tests prove the
  fake external server remains reachable
- 16 fixture and driver tests, focused clippy, and diff checks pass

## Stop Conditions

- facade behavior depends on unrecorded parser/template configuration

## Auto-Continuation

Yes, after card 050 is ready and focused validation passes.
