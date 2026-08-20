# 017 Cline Stable Clippy Result Large Err

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: none
Vision tags: consumer integration
Contract refs: 001
Planning state: cards 048-049 ready

## Problem

Stable CI Clippy 1.98.0 fails `clippy::result_large_err` on
`ClineAcpDriver::start_session` because the Err variant is
`(RuntimeFailure, ResourceLease)` at least 128 bytes. Pinned MSRV clippy
on the same SHA is green. The function is unchanged on g04.016. PR 13
cannot go green until this lands on `main` and that branch restacks.

## Generation Runway Goal

Keep the addable-coverage lane unblocked. This is a workspace Stable
clippy papercut, not a new addable descriptor.

## Goals

- [ ] box the `start_session` Err pair so Stable clippy 1.98.0 is quiet
- [ ] keep the caller releasing the `ResourceLease` on failure
- [ ] close the papercut after focused Cline clippy and tests pass

## Non-Goals

- DeepSeek, Claude Agent ACP, llama.cpp, or hosted OAuth
- changing `RuntimeFailure` or `ResourceLease` layout
- allowing the lint instead of boxing
- rewriting `public-api-0.3.3`
- restacking PR 13 inside this lane

## Execution Plan

### Batch 17.1 — Box The Err Pair

- [ ] Execute card 048.
- [ ] change `start_session` to
      `Result<ClineSessionHandle, Box<(RuntimeFailure, ResourceLease)>>`
- [ ] update every `Err((error, resource))` construction and the
      `open_session` match that releases the lease

### Batch 17.2 — Prove The Lint

- [ ] Execute card 049 after card 048.
- [ ] `cargo clippy -p swallowtail-adapter-cline --all-targets --all-features -- -D warnings`
- [ ] mark the papercut closed

## Acceptance Criteria

- [ ] Stable-style clippy on the Cline package does not report
      `result_large_err` on `start_session`
- [ ] failed session start still releases the working-resource lease
- [ ] no DeepSeek or g04.016 files change
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.016 DeepSeek continuation blocked on this lint
- this milestone: Cline Stable clippy papercut
- next: restack PR 13 after this lands
- later: Claude Agent ACP, llama.cpp attached, hosted OAuth gate

## Decision Gates

- Stop if the fix changes session behavior beyond the Err type.
- Stop if DeepSeek or another adapter is edited.
- Stop if the lint is silenced with a blanket workspace allow.
