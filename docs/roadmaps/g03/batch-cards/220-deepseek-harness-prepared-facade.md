# 220 DeepSeek Harness Prepared Facade

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../069-deepseek-harness-jsonrpc-foundation.md`
Depends on: card 219

## Goal

Expose consumer-safe preparation for one exact DeepSeek Harness JSON-RPC
structured run with explicit provider, model, config, and cwd, and no hidden
defaults.

## Scope

1. Bind host-approved executable, Cordis config, cwd, provider, model, and
   host services into immutable evidence.
2. Expose `prepare_deepseek_harness_jsonrpc` → `prepare_run` → `start_run`.
3. Keep a low-level driver escape hatch without inventing catalogue or
   interactive roles.
4. Record composition-granted tool posture as prepared evidence; do not
   default `danger-full-access`.

## Out Of Scope

- package topology, public guide, live selector, or Contract 036 count
  updates
- ACP, Web `/api`, or Open Platform continuation

## Acceptance Criteria

- [x] preparation fails closed on version drift, missing executable, missing
      config, or missing provider/model
- [x] activity-affecting options are immutable prepared evidence
- [x] no default model, provider, or Cordis composition is invented
- [x] deterministic prepared-facade tests pass without network credentials

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`

## Evidence

- implementation commit: `1c053dbb`
- added host-approved preparation, exact observation promotion, immutable
  prepared evidence, explicit provider/model selection, and `start_run`
- prepared plans bind ambient Cordis configuration, host-owned isolation,
  read-only working resource, Task/Process/Time services, and prohibited
  provider retention/recovery
- blank host references are rejected by the runtime constructors; target and
  release drift fail closed before process work
- focused validation: 10 tests passed
- warnings-denied Clippy passed for the package and all targets
- no default provider, model, credentials, or `danger-full-access` posture was
  added

## Stop Conditions

- stop if preparation would require Swallowtail to lease a provider key
- stop if the facade collapses `deepseek.continuation` into the harness route

## Auto-Continuation

Continue to card 221 once the prepared facade is ready for package
acceptance.
