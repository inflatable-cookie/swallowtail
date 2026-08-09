# 184 Command Code Package And Route Acceptance

Status: completed
Owner: Tom
Created: 2026-08-09
Milestone: `../059-command-code-headless-foundation.md`
Depends on: card 183

## Goal

Complete the separately selectable Command Code package, public route truth,
operator guidance, and deterministic plus live acceptance evidence.

## Scope

1. Review and update architecture and Contract 036 for the additive package.
2. Add package metadata, public Rustdoc, a normal-path example, and an exact
   integration guide.
3. Update route, feature, configured-instance, failure, and recovery truth.
4. Make release-baseline checks accept an additive unreleased package without
   rewriting immutable tagged inventories.
5. Add separate installed and authenticated Effigy probes.
6. Run focused, affected-package, guide, docs, and extracted-package proof.

## Acceptance

- [x] current source and immutable tagged-release package counts stay distinct
- [x] every public item has meaningful Rustdoc and the example compiles
- [x] every unsupported feature remains an explicit `No`, `Not applicable`, or
      qualified note
- [x] deterministic validation is credential-free
- [x] one operator-authorized plan-mode smoke passes through the prepared
      facade without workspace mutation or retained session state
- [x] the closeout records exact source state and does not imply publication

## Evidence

- Current source contains 29 packages and 35 production routes. Immutable
  `v0.3.1` evidence remains 28 packages and 34 routes.
- The separate unreleased Command Code semantic API baseline passes without
  changing `release-baselines/public-api-0.3.0`.
- Guide, route, feature, activity, architecture, and Contract 036 inventories
  agree on `command-code.headless`.
- Live installed probe required a Node-interpreted launch for the npm shebang
  bin; the authenticated plan-mode smoke used
  `deepseek/deepseek-v4-flash` and completed without retained session state.


## Validation

- `effigy validate:focused swallowtail-adapter-command-code`
- `effigy package:verify-affected swallowtail-adapter-command-code`
- `effigy qa:guides`
- `effigy qa:routes`
- `effigy qa:docs`
- separately gated installed and authenticated Command Code probes

## Stop Conditions

- stop if additive package handling would mutate an immutable release baseline
- stop if authenticated acceptance requires write, shell, or login authority
- stop before any version bump, tag, GitHub Release, or registry mutation

## Auto-Continuation

No. Return to the operator with route evidence and the resume / Provider API
checkpoint.
