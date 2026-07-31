# 035 Kimi Code 0.31.1 Artifact And Package Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../013-kimi-code-0-31-1-range-maintenance.md`
Depends on: card 034

## Goal

Accept the widened Kimi Code range against the exact official `0.31.1`
artifact and close the milestone without changing the installed CLI or making a
provider request.

## Scope

1. Record exact artifact digest, signature, version, and ACP initialization
   proof.
2. Run focused and extracted-package validation.
3. Reconcile architecture, route truth, front doors, roadmap state, and one
   meaningful closeout log.
4. Return to the g03 maintenance checkpoint.

## Acceptance Criteria

- [x] official `0.31.1` artifact reports its exact version and initializes ACP
- [x] artifact identity matches official digest and signer evidence
- [x] installed `0.31.0` remains untouched
- [x] focused, package, docs, Northstar, formatting, and diff checks pass
- [x] no authentication, provider prompt, model request, or durable mutation ran
- [x] roadmap g03.013 closes with one clear next checkpoint

## Validation

- `effigy validate:focused swallowtail-adapter-kimi`
- `effigy package:verify-affected swallowtail-adapter-kimi`
- `effigy qa:docs`
- `effigy qa:northstar`
- `cargo fmt --all --check`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

No. Return to the g03 compatibility-maintenance checkpoint after closeout.

## Evidence

- official archive digest and Apple signer identity matched
- exact artifact reported `0.31.1` and completed prompt-free ACP initialization
  with zero stderr bytes
- installed Kimi Code remains unchanged at `0.31.0`
- 89 focused tests and the extracted 212-file package passed
- route, docs, Northstar, formatting, and diff checks passed
- no authentication, provider prompt, model request, or durable mutation ran
