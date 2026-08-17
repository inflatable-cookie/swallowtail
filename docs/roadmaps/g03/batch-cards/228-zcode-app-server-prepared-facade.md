# 228 ZCode App-Server Prepared Facade

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../071-zcode-app-server-foundation.md`
Depends on: card 227

## Goal

Expose consumer-safe preparation for one exact ZCode app-server structured
run with explicit config, cwd, mode, provider, and model, and no hidden
defaults.

## Scope

1. Bind host-approved Node, `zcode.cjs`, config, cwd, mode, provider,
   model, and host services into immutable evidence.
2. Expose `prepare_zcode_app_server` → `prepare_run` → `start_run`.
3. Keep a low-level driver escape hatch without inventing catalogue or
   interactive roles.
4. Record host-supplied mode as prepared evidence; do not default `yolo`.

## Out Of Scope

- package topology, public guide, live selector, or Contract 036 count
  updates
- `--print`, ACP, OpenCode, or hosted GLM HTTP

## Acceptance Criteria

- [x] preparation fails closed on version drift, missing Node, missing
      payload, missing config, or missing provider/model
- [x] activity-affecting options are immutable prepared evidence
- [x] no default model, provider, or `yolo` mode is invented
- [x] deterministic prepared-facade tests pass without network credentials

## Validation

- `effigy validate:focused swallowtail-adapter-zcode` — passed with card 227

## Stop Conditions

- stop if preparation would require Swallowtail to lease a provider key
- stop if the facade collapses OpenCode or hosted GLM HTTP into the
      harness route

## Auto-Continuation

Continue to card 229 once the prepared facade is ready for package
acceptance.
