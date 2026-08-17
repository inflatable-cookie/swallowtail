# 223 DeepSeek Harness Web `/api` Driver Core

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../070-deepseek-harness-web-api-foundation.md`
Depends on: card 222

## Goal

Implement owned-process `dsh web` discovery, loopback admission, allowlisted
HTTP+WebSocket decode, catalogue, history, prompt, native cancel, fork,
archive, deadline, and cleanup.

## Scope

1. Classify exact `@deepseek-ai/dsh@0.1.0-rc.6` on axis `deepseek-harness.web`.
2. Spawn host-approved `dsh web` on `127.0.0.1`, join on shutdown or kill.
3. Enforce the Spec 009 method allowlist and Host/Origin JSON POST fence.
4. Project list/search, history, mux events, usage, native cancel, fork, and
   archive without ingesting private bodies.

## Out Of Scope

- prepared facade, package topology, or live selector
- JSON-RPC driver changes
- credentials, settings, llm.*, directory picker, ZIP export, subagents

## Acceptance Criteria

- [x] unknown, denied, and malformed methods fail closed
- [x] history projection does not resume an Agent
- [x] cancel aborts the active turn without advertising JSON-RPC wire cancel
- [x] fork and archive use native methods and do not claim restore or delete
- [x] deadline and process-kill cleanup remain joined

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`

## Stop Conditions

- stop if discovery would spawn the JSON-RPC binary or a browser
- stop if loopback or allowlist admission cannot be tested without secrets
- stop if history cannot be shown control-free

## Auto-Continuation

Continue to card 224 after focused package validation passes.

## Evidence

- implementation commit: `01004db0`
- `cargo test -p swallowtail-adapter-deepseek-harness` — 20 tests passed
- `effigy validate:focused swallowtail-adapter-deepseek-harness` passed
- Web transport is loopback-only, bounded, host/origin-fenced, and joins its
  blocking HTTP/WebSocket workers before returning deadline or cleanup errors
- history uses `session.history` only; structured runs use native
  `session.cancel`, `session.fork`, and `workspace.archiveSession`
- no browser, credential, account, or live model was used
