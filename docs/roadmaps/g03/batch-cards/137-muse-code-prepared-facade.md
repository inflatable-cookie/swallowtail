# 137 Muse Code Prepared Facade

Status: completed
Owner: Tom
Created: 2026-08-06
Milestone: `../045-muse-code-headless-foundation.md`
Depends on: card 136

## Goal

Expose the Muse route through one exact local-account prepared facade with
model, reasoning, resource, and lifecycle agreement.

## Scope

1. Add provider-supported local Meta account access evidence without a
   Swallowtail credential lease.
2. Bind `meta`, `muse-spark-1.2`, and one explicit supported reasoning effort.
3. Bind read-only filesystem access, ambient local-account configuration,
   provider-enforced sandboxing, no retained session log, and no shell, write,
   or web-tool authority.
4. Add immutable prepared evidence and low-level escape hatch.
5. Prove plan/request drift fails before process start.

## Acceptance

- [x] all seven advertised reasoning efforts are accepted exactly
- [x] missing or different provider, model, target, release, host, access,
      effort, or resource evidence fails closed
- [x] preparation performs discovery only and never invokes a model
- [x] normal execution uses local account state without reading auth files
- [x] unsupported catalogue, interactive, callback, recovery, task-list, and
      subagent requests remain unavailable

## Validation

- `effigy validate:focused swallowtail-adapter-muse`
- prepared-facade and common structured-run conformance

## Stop Conditions

- stop if local account access cannot be represented without credential
  extraction
- stop if exact CLI acceptance plus the correlated model event cannot bind the
  selected model and effort without inference

## Auto-Continuation

Yes. Continue to card 138 after prepared evidence passes.

## Evidence

- `MusePreparationInput`, `MusePreparationProbe`, and
  `prepare_muse_headless` admit one exact signed payload and local Meta account
  profile through discovery only.
- `MuseRunProfileInput` requires an explicit provider/model route, one of seven
  exact efforts, a read-only filesystem resource, and deadline.
- `MusePreparedIntegration` and `MusePreparedRun` retain immutable plan,
  request, access, activity, host, target, and interface evidence while
  exposing an explicit low-level driver escape hatch.
- prepared and low-level drift tests fail before model-process start.
- `effigy validate:focused swallowtail-adapter-muse`: 20 passed across two
  binaries; warnings denied.
- `effigy package:verify-affected swallowtail-adapter-muse`: extracted-package
  proof passed.
- no authenticated provider work ran.
