# 229 ZCode App-Server Package And Route Acceptance

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../071-zcode-app-server-foundation.md`
Depends on: card 228

## Goal

Complete the separately selectable ZCode package, public route truth,
operator guidance, and deterministic plus live acceptance evidence.

## Scope

1. Review and update architecture and Contract 036 for the additive
   package.
2. Add package metadata, public Rustdoc, a normal-path example, and an
   exact integration guide.
3. Update route, feature, configured-instance, failure, and recovery
   truth.
4. Make release-baseline checks accept an additive unreleased package
   without rewriting immutable tagged inventories.
5. Add separate installed and live Effigy probes.
6. Run focused, affected-package, guide, docs, and extracted-package
   proof.

## Out Of Scope

- version bump, tag, GitHub Release, or registry mutation
- `--print`, ACP, interactive continuity, or Z.AI official qualification

## Acceptance Criteria

- [x] current source and immutable tagged-release package counts stay
      distinct
- [x] every public item has meaningful Rustdoc and the example compiles
- [x] every unsupported feature remains an explicit `No`, `Not applicable`,
      or qualified note
- [x] deterministic validation is credential-free
- [x] one operator-authorized live smoke passes through the prepared
      facade
- [x] the closeout records exact source state and does not imply
      publication
- [x] Spec 010 acceptance boxes that this milestone owns are checked

## Validation

- `effigy validate:focused swallowtail-adapter-zcode`
- `effigy package:verify-affected swallowtail-adapter-zcode`
- `effigy qa:guides`
- `effigy qa:routes`
- `effigy qa:docs`
- separately gated installed and live ZCode probes

## Stop Conditions

- stop if acceptance would require a version bump or tagged-inventory
      rewrite
- stop if live proof would be claimed as Z.AI official

## Auto-Continuation

Stop. Reassess native `session/stop`, `--print`, history, ACP, and Z.AI
official as separate later gates.

## Evidence

- current source is 32 packages and 39 routes; immutable `v0.3.2` remains 30
  packages and 36 routes
- feature, lifecycle, and activity matrices pass with 31 solution rows, 39
  route identities, and 76 activity operations
- `effigy validate:focused swallowtail-adapter-zcode` passed (23 tests)
- `effigy package:verify-affected swallowtail-adapter-zcode` passed
- `effigy qa:guides`, `effigy qa:routes`, `effigy qa:docs`, and example checks
  passed
- ZCode guide, example, prepared entry, installed probe, and separate live
  selectors are present
- `effigy probe:zcode-installed` passed
- `effigy probe:zcode-live` passed through the prepared facade on host-local
  Ollama through custom provider id `zai`; that does not qualify Z.AI
  official
- live spawn is `node zcode.cjs app-server`; host settings live at
  `$HOME/.zcode/cli/config.json`; later `user-execution` preferences are
  answered; unscoped unknown events and telemetry notifications are
  content-free progress

See `docs/logs/2026-08-17-zcode-app-server-acceptance.md`.

