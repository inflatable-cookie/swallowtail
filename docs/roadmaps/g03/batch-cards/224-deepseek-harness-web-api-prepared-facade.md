# 224 DeepSeek Harness Web `/api` Prepared Facade

Status: completed
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

- [x] prepared evidence names the web axis, CLI pin, loopback bind, and
      allowlist
- [x] structured-run preparation does not imply JSON-RPC transport
- [x] history preparation does not imply resume or interactive handle
- [x] no credential lease is minted

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`

## Stop Conditions

- stop if preparation would require settings or credentials methods
- stop if the facade would flatten onto JSON-RPC types

## Auto-Continuation

Continue to card 225 after focused package validation passes.

## Evidence

- prepared Web facade implementation is included in the card-224 change
- `cargo test -p swallowtail-adapter-deepseek-harness` — 22 tests passed
- `effigy validate:focused swallowtail-adapter-deepseek-harness` passed
- preparation binds the Web axis, exact `dsh` target, loopback endpoint,
  protocol facade, 11-method allowlist, and local unauthenticated access
- structured-run startup checks `host.describe` against the prepared
  provider/model route before creating a session; the Web create payload only
  carries the bound cwd
- structured run, catalogue/search/models, control-free history, native fork,
  and target-only archive remain route-specific; history exposes no resume
  handle and fork does not invent a provider-neutral fork contract
- no credential lease, browser, account, or live model was used
