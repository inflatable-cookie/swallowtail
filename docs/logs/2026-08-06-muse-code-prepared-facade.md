# Muse Code Prepared Facade

Date: 2026-08-06
Roadmap: g03.045
Card: 137

## Outcome

Added one discovery-backed prepared facade for the exact Muse Code headless
route:

- `muse_local_meta_account_access_profile` represents provider-owned local Meta
  account state without credential references, leases, or auth-file reads
- `MusePreparationInput`, `MusePreparationProbe`, and
  `prepare_muse_headless` bind the execution host, exact signed payload,
  environment, configured instance, access evidence, and qualified release
- `MuseHeadlessModelSelection` and `MuseRunProfileInput` require explicit
  `meta`, `muse-spark-1.2`, reasoning effort, read-only filesystem resource,
  prompt, and deadline
- `MusePreparedIntegration` and `MusePreparedRun` retain immutable preparation,
  plan, request, activity, access, and interface evidence
- both prepared values expose the low-level exact driver as an explicit escape
  hatch

The configured instance advertises exactly `none`, `minimal`, `low`, `medium`,
`high`, `xhigh`, and `ultra`. Each prepared operation binds only its selected
effort. The facade fixes provider sandboxing, ambient local-account
configuration, prohibited retention and recovery, disabled reattachment,
read-only resource access, and the command-level write, shell, web, foreign
context, and session-log prohibitions.

Model catalogue, interactive session, callback, recovery, task-list snapshot,
and subagent authority remain unavailable. Task lifecycle observations do not
claim a portable task-list snapshot.

## Failure Evidence

Deterministic cases reject provider, model, effort, target, signed release,
execution host, access profile/status, and working-resource drift. Preparation
starts only `--version`; prepared execution is the first model invocation.
Low-level request drift fails before a process starts.

## Validation

- `effigy validate:focused swallowtail-adapter-muse`: 20 passed across two
  binaries; warnings-denied check passed
- `effigy package:verify-affected swallowtail-adapter-muse`: extracted-package
  proof passed

No authenticated provider work ran.

## Next

Execute card 138. Add package and route documentation, matrices, example,
release-baseline handling, installed discovery, and one separately gated
authenticated low-effort prepared smoke.
