# 138 Muse Code Package And Route Acceptance

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../045-muse-code-headless-foundation.md`
Depends on: card 137

## Goal

Complete the separately selectable Muse package, public route truth, operator
guidance, and deterministic plus live acceptance evidence.

## Scope

1. Review and update architecture and Contract 036 for the additive package.
2. Add package metadata, public Rustdoc, a normal-path example, and an exact
   integration guide.
3. Update route, feature, configured-instance, failure, and recovery truth.
4. Make release-baseline checks accept an additive unreleased package without
   rewriting immutable `v0.1.0` or `v0.1.1` evidence.
5. Add separate installed and authenticated Effigy probes.
6. Run focused, affected-package, guide, docs, and extracted-package proof.

## Acceptance

- [x] current source and immutable tagged-release package counts stay distinct
- [x] every public item has meaningful Rustdoc and the example compiles
- [x] every unsupported feature remains an explicit `No`, `Not applicable`, or
      qualified note
- [x] deterministic validation is credential-free
- [x] one operator-authorized low-effort Meta smoke passes through the prepared
      facade without workspace mutation or retained session state
- [x] the closeout records exact source state and does not imply publication

## Evidence

- Current source contains 28 packages and 34 production routes. Immutable
  `v0.1.0` and `v0.1.1` evidence remains 27 packages and 33 routes.
- The separate unreleased Muse semantic API baseline passes without changing
  any tagged-package API baseline.
- Public Rustdoc and the prepared Muse example compile with denied warnings.
- Deterministic proof passes 20 package tests, the five-test corpus validator,
  route, guide, docs, metadata, semantic API, and extracted-package gates.
- `effigy probe:muse-installed` classified the exact signed
  `muse-bin-0.1.0-R708.1` payload.
- Three operator-authorized `meta` / `muse-spark-1.2` / low invocations ran:
  two reached provider terminal output while exposing the post-terminal parser
  mismatch, and the final prepared-facade probe passed. Write, shell, web,
  foreign context, and session logging stayed disabled throughout.
- Live evidence found one bounded `session.workspace_branch.observed` record
  after the provider terminal. The adapter now preserves it only as
  namespaced unknown activity; it cannot alter terminal or callback truth.
- Work remains uncommitted on `main` atop
  `7a614732412b0b7dc93b4f83a98badd23b4f24d5`. No version, tag, GitHub
  Release, or registry mutation ran.

## Validation

- `effigy validate:focused swallowtail-adapter-muse`
- `effigy package:verify-affected swallowtail-adapter-muse`
- `effigy qa:guides`
- `effigy qa:routes`
- `effigy qa:docs`
- separately gated installed and authenticated Muse probes

## Stop Conditions

- stop if additive package handling would mutate an immutable release baseline
- stop if authenticated acceptance requires write, shell, web, or login
  authority
- stop before any version bump, tag, GitHub Release, or registry mutation

## Auto-Continuation

No. Return to the operator with route evidence and the retained-session and
Meta Model API checkpoint.
