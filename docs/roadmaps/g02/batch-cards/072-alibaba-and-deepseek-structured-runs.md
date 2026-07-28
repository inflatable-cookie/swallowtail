# 072 Alibaba And DeepSeek Structured Runs

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../022-structured-run-projection-and-direct-coverage.md`

## Objective

Add two resource-free hosted structured-run branches without changing Alibaba
conversation or DeepSeek continuation semantics.

## Governing Refs

- Research 044
- Contracts 009-011, 014, 024-025, 029-030, 037, 039
- exact existing Alibaba and DeepSeek corpora

## Scope

1. Add independent structured roles, requirements, capabilities, and prepared
   operations.
2. Alibaba: send one Responses request with `store=false`, no conversation or
   previous-response reference.
3. DeepSeek: send one no-tool Chat Completions request with no private
   continuation state.
4. Map streaming output, usage, request, provider failure, cancellation,
   deadline, and joined cleanup.
5. Add local and remote-authoritative deterministic conformance.

## Acceptance Criteria

- [x] one provider inference attempt per run
- [x] no provider session, conversation, or continuation binding escapes
- [x] unsupported tools, attachments, and schemas fail before network effects
- [x] provider retention is prohibited
- [x] existing interactive drivers pass unchanged regression
- [x] diagnostics contain no content, credential, endpoint, or raw payload

## Evidence

- Both descriptors register independent structured roles and prepared
  operations beside their unchanged interactive roles.
- Alibaba sends one `store=false` Responses request with no conversation or
  previous-response field.
- DeepSeek sends one streamed, tool-free request with explicit high reasoning,
  output bound, and unmanaged-cache acceptance; private reasoning is discarded
  at terminal completion.
- Local and remote-authoritative prepared fixtures pass. Cancellation joins
  blocking work before credential release.
- The provider solution matrix changes both structured-run cells from `No` to
  `Yes`.

## Validation Evidence

- `cargo test -p swallowtail-adapter-alibaba-model-studio -p swallowtail-adapter-deepseek`
- `cargo clippy -p swallowtail-adapter-alibaba-model-studio -p swallowtail-adapter-deepseek --all-targets -- -D warnings`
- `effigy qa:docs`
- `effigy qa:routes`

## Validation

- focused Alibaba and DeepSeek adapter tests
- affected workspace checks and Clippy
- docs and route checks after claims change
- `git diff --check`

## Stop Conditions

- exact one-request behavior needs hidden continuation
- required request fields conflict with the current qualified facade
- cleanup cannot join before credential release
- fixtures require live provider access

## Auto-Continuation

Yes. Continue to card 073 after focused validation passes.
