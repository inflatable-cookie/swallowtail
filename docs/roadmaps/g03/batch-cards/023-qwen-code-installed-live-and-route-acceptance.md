# 023 Qwen Code Installed, Live, And Route Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../009-qwen-code-installed-range-closure.md`
Depends on: card 022

## Goal

Accept the widened Qwen range against the installed exact executable, with
provider effects separately gated and public route truth reconciled.

## Scope

1. Add or repair an explicit installed-version probe selector for Qwen.
2. Prove the host-approved `0.21.2` executable classifies as qualified.
3. When delegated harness access is ready, run an identity-only model catalogue
   probe and one bounded read-only prompt without workspace writes.
4. Keep live failures diagnostic; they do not revoke deterministic range proof.
5. Reconcile route, feature, activity, architecture, front-door, roadmap, and
   log currentness.

## Acceptance Criteria

- [x] installed discovery binds exact `0.21.2`
- [x] live selectors are explicit, ignored by default, and bounded
- [x] no credential, raw provider payload, host path, or prompt leaks through
  stable diagnostics
- [x] no write-capable or permissive-approval run occurs
- [x] deterministic, focused, package, route, docs, and Northstar checks pass
- [x] authenticated evidence is recorded honestly as passed, unavailable, or
  failed without changing deterministic support truth
- [x] roadmap g03.009 closes with one clear next checkpoint

## Validation

- `effigy validate:focused swallowtail-adapter-qwen`
- `effigy package:verify-affected swallowtail-adapter-qwen`
- explicit Qwen installed and authenticated selectors only
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `cargo fmt --all --check`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

No. Return to the g03 provider-maintenance checkpoint after closeout.

## Evidence

- installed exact `0.21.2` selector passed
- catalogue selector failed with an authentication-configuration class; raw
  provider output was not printed or retained
- read-only prompt selector was not run without working access and one explicit
  model id
- 35 focused tests passed with warnings denied
- the extracted 61-file package compiled
- 32-route and 25-solution public matrices passed
- no workspace-write, permissive-approval, credential mutation, or broad suite
  ran
