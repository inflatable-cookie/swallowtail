# 225 DeepSeek Harness Web `/api` Package And Route Acceptance

Status: completed
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

- [x] current source and immutable tagged-release route counts stay distinct
- [x] every unsupported configuration-plane method remains an explicit `No`
- [x] deterministic validation is credential-free
- [x] one operator-authorized live smoke passes through the prepared facade
- [x] the closeout does not imply publication or DeepSeek-official support

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

## Evidence

- current source route truth is 31 packages and 38 routes; immutable `v0.3.2`
  remains 30 packages and 36 routes
- feature, lifecycle, and activity matrices pass with 30 solution rows, 38
  route identities, and 75 activity operations
- `effigy validate:focused swallowtail-adapter-deepseek-harness` passed
- `effigy package:verify-affected swallowtail-adapter-deepseek-harness` passed
- `effigy qa:guides`, `effigy qa:routes`, `effigy qa:docs`, and example checks
  passed
- Web guide, example, prepared entry, installed probe, and separate live
  selectors are present
- `effigy probe:deepseek-harness-web-installed` passed
- `effigy probe:deepseek-harness-web-live` passed through the prepared facade
  on host-local Ollama; that does not qualify `deepseek-official`
- live spawn binds `dsh web --patch`, cwd create fallback, ignored mux
  snapshots, unknown session events as progress, and local runtime event
  sequence

See `docs/logs/2026-08-17-deepseek-harness-web-api-live-acceptance.md`.
