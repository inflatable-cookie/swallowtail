# 221 DeepSeek Harness Package And Route Acceptance

Status: active
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

- [x] current source and immutable tagged-release package counts stay distinct
- [x] every public item has meaningful Rustdoc and the example compiles
- [x] every unsupported feature remains an explicit `No`, `Not applicable`, or
      qualified note
- [x] deterministic validation is credential-free
- [ ] one operator-authorized live smoke passes through the prepared facade
- [x] the closeout records exact source state and does not imply publication
- [x] Spec 008 acceptance boxes that this milestone owns are checked

## Evidence

- implementation commit: `158b188c`; payload-digest admission: `96db297e`;
  merged: `52263993` (PR 1)
- current source contains 31 packages and 37 production routes; immutable
  `v0.3.2` remains 30 packages and 36 routes
- package metadata distinguishes the current 31-package source from the
  immutable 30-package release set
- semantic API proof preserves immutable `v0.3.2` files and compares reviewed
  current-source overrides for Claude response-only compatibility and the
  DeepSeek Harness package
- `effigy validate:focused
  swallowtail-adapter-deepseek-harness` passed 10 tests with warnings-denied
  Clippy; `effigy package:verify-affected
  swallowtail-adapter-deepseek-harness` passed extracted compilation
- `effigy package:docs`, `effigy check:examples`, `effigy qa:guides`,
  `effigy qa:routes`, `effigy qa:docs`, and `effigy qa:consumer-docs` passed
- the installed and live selectors are compiled and separately gated, but no
  exact host-approved `dsh-jsonrpc-agent-pkg-macos-arm64` executable or Cordis
  configuration is present on this host, so no live smoke was claimed or run
- no version bump, tag, GitHub Release, registry publication, or immutable
  release-baseline mutation ran

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`
- `effigy package:verify-affected swallowtail-adapter-deepseek-harness`
- `effigy qa:guides`
- `effigy qa:routes`
- `effigy qa:docs`
- separately gated installed and live DeepSeek Harness probes

The installed and live probes remain pending until the operator supplies the
exact packaged executable, Cordis configuration, read-only cwd, provider, and
model. The documented local-Ollama path keeps `deepseek-official` unqualified.

## Stop Conditions

- stop if additive package handling would mutate an immutable release baseline
- stop if live acceptance requires a DeepSeek account this host does not have;
      use the documented host-local model path instead and keep
      `deepseek-official` unqualified
- stop before any version bump, tag, GitHub Release, or registry mutation

## Auto-Continuation

No. Return to the operator with route evidence and the ACP / Web `/api` /
session-id checkpoint plus the exact live-probe prerequisite.
