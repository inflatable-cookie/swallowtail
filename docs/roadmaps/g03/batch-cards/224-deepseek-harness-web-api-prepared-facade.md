# 224 DeepSeek Harness Web `/api` Prepared Facade

Status: planned
Owner: Tom
Created: 2026-08-17
Milestone: `../070-deepseek-harness-web-api-foundation.md`
Depends on: card 223

## Goal

Expose host-approved preparation and exact catalogue, history, structured-run,
fork, and archive facades for `deepseek-harness.local-server`.

## Scope

1. Bind exact CLI, Cordis patch, loopback endpoint, access evidence, cwd,
   provider, and model.
2. Separate catalogue, history, structured prompt, fork, and archive
   operation shapes.
3. Keep JSON-RPC preparation and `deepseek.continuation` unchanged.

## Out Of Scope

- package metadata, guides, matrices, or live selectors
- ACP or browser UI
- Contract 054 public support until history proof is accepted

## Acceptance Criteria

- [ ] prepared evidence names the web axis, CLI pin, loopback bind, and
      allowlist
- [ ] structured-run preparation does not imply JSON-RPC transport
- [ ] history preparation does not imply resume or interactive handle
- [ ] no credential lease is minted

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`

## Stop Conditions

- stop if preparation would require settings or credentials methods
- stop if the facade would flatten onto JSON-RPC types

## Auto-Continuation

Continue to card 225 after focused package validation passes.
