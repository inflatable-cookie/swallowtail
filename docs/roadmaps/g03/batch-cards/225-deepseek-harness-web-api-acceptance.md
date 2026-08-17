# 225 DeepSeek Harness Web `/api` Package And Route Acceptance

Status: planned
Owner: Tom
Created: 2026-08-17
Milestone: `../070-deepseek-harness-web-api-foundation.md`
Depends on: card 224

## Goal

Complete public route truth, operator guidance, and deterministic plus live
acceptance for `deepseek-harness.local-server` without rewriting immutable
tagged inventories.

## Scope

1. Review architecture and Contract 036 for the additive route.
2. Add example, guide, and route/feature/activity truth.
3. Keep JSON-RPC route counts honest against current source versus
   immutable `v0.3.2`.
4. Add separately gated installed and live Effigy probes.

## Out Of Scope

- version bump, tag, GitHub Release, or registry mutation
- ACP, JSON-RPC continuity, or DeepSeek-official qualification
- Contract 054 promotion unless history proof already passed

## Acceptance Criteria

- [ ] current source and immutable tagged-release route counts stay distinct
- [ ] every unsupported configuration-plane method remains an explicit `No`
- [ ] deterministic validation is credential-free
- [ ] one operator-authorized live smoke passes through the prepared facade
- [ ] the closeout does not imply publication or DeepSeek-official support

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`
- `effigy package:verify-affected swallowtail-adapter-deepseek-harness`
- `effigy qa:guides`
- `effigy qa:routes`
- `effigy qa:docs`
- separately gated installed and live DeepSeek Harness web probes

## Stop Conditions

- stop if additive route handling would mutate an immutable release baseline
- stop if live acceptance requires a DeepSeek account this host does not
      have; use the documented host-local model path and keep
      `deepseek-official` unqualified
- stop before any version bump, tag, GitHub Release, or registry mutation

## Auto-Continuation

No. Return to the operator with the Contract 054 / ACP / JSON-RPC continuity
checkpoint.
