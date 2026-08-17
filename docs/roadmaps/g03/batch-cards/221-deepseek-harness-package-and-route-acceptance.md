# 221 DeepSeek Harness Package And Route Acceptance

Status: planned
Owner: Tom
Created: 2026-08-17
Milestone: `../069-deepseek-harness-jsonrpc-foundation.md`
Depends on: card 220

## Goal

Complete the separately selectable DeepSeek Harness package, public route
truth, operator guidance, and deterministic plus live acceptance evidence.

## Scope

1. Review and update architecture and Contract 036 for the additive package.
2. Add package metadata, public Rustdoc, a normal-path example, and an exact
   integration guide.
3. Update route, feature, configured-instance, failure, and recovery truth.
4. Make release-baseline checks accept an additive unreleased package without
   rewriting immutable tagged inventories.
5. Add separate installed and live Effigy probes.
6. Run focused, affected-package, guide, docs, and extracted-package proof.

## Out Of Scope

- version bump, tag, GitHub Release, or registry mutation
- ACP, Web `/api`, interactive continuity, or DeepSeek-official qualification

## Acceptance Criteria

- [ ] current source and immutable tagged-release package counts stay distinct
- [ ] every public item has meaningful Rustdoc and the example compiles
- [ ] every unsupported feature remains an explicit `No`, `Not applicable`, or
      qualified note
- [ ] deterministic validation is credential-free
- [ ] one operator-authorized live smoke passes through the prepared facade
- [ ] the closeout records exact source state and does not imply publication
- [ ] Spec 008 acceptance boxes that this milestone owns are checked

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`
- `effigy package:verify-affected swallowtail-adapter-deepseek-harness`
- `effigy qa:guides`
- `effigy qa:routes`
- `effigy qa:docs`
- separately gated installed and live DeepSeek Harness probes

## Stop Conditions

- stop if additive package handling would mutate an immutable release baseline
- stop if live acceptance requires a DeepSeek account this host does not have;
      use the documented host-local model path instead and keep
      `deepseek-official` unqualified
- stop before any version bump, tag, GitHub Release, or registry mutation

## Auto-Continuation

No. Return to the operator with route evidence and the ACP / Web `/api` /
session-id checkpoint.
