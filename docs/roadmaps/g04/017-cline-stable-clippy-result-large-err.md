# 017 Cline Stable Clippy Result Large Err

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: none
Vision tags: consumer integration
Contract refs: 001
Planning state: cards 048-049 ready

## Problem

Stable CI Clippy 1.98.0 fails `clippy::result_large_err` on ACP
`start_session` helpers whose Err variant is
`(RuntimeFailure, ResourceLease)` at least 128 bytes. PR 14 boxed Cline.
The same signature remains on Goose, Copilot CLI, Gemini, Kiro, and
Deep Agents. Clippy 1.98.0 reports one crate at a time. PR 13 cannot go
green until every copy is boxed.

## Generation Runway Goal

Keep the addable-coverage lane unblocked. This is a workspace Stable
clippy papercut, not a new addable descriptor.

## Goals

- [ ] box every production `start_session` Err pair of
      `(RuntimeFailure, ResourceLease)` so Stable clippy 1.98.0 is quiet
- [ ] keep each caller releasing the `ResourceLease` on failure
- [ ] close the papercut after workspace clippy `-D warnings` passes

## Non-Goals

- DeepSeek, Claude Agent ACP, llama.cpp, or hosted OAuth
- changing `RuntimeFailure` or `ResourceLease` layout
- allowing the lint instead of boxing
- rewriting `public-api-0.3.3`
- restacking PR 13 inside this lane

## Execution Plan

### Batch 17.1 — Box The Err Pair

- [ ] Execute card 048.
- [ ] box Cline, Goose, Copilot CLI, Gemini, Kiro, and Deep Agents
      `start_session` Err pairs
- [ ] update every `Err((error, resource))` construction and each
      `open_session` match that releases the lease

### Batch 17.2 — Prove The Lint

- [ ] Execute card 049 after card 048.
- [ ] `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- [ ] mark the papercut closed

## Acceptance Criteria

- [ ] workspace Stable-style clippy does not report `result_large_err` on
      those `start_session` helpers
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
