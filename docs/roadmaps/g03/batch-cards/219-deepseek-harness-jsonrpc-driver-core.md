# 219 DeepSeek Harness JSON-RPC Driver Core

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../069-deepseek-harness-jsonrpc-foundation.md`
Depends on: card 218

## Goal

Implement exact installed discovery and one bounded DeepSeek Harness JSON-RPC
structured run without consumer or provider policy inference.

## Scope

1. Add the exact runtime-bin claim and payload digest for `0.1.0rc6`.
2. Add target-bound discovery against the host-approved executable.
3. Spawn with host-approved Cordis config, cwd, and stdio JSON-RPC ownership.
4. Decode initialize, prompt enqueue, session events, usage, thinking
   progress, text, tools, idle, terminal, safe failure, and unknowns.
5. Fold the Swallowtail-owned idle interval; do not treat `messageId` as a
   result.
6. Join process-kill cancellation, deadline, event delivery, shutdown, and
   cleanup.

## Out Of Scope

- prepared facade, guide, matrices, or live Effigy selector
- native JSON-RPC cancel, session resume, catalogue, or subagent control

## Acceptance Criteria

- [x] no Python SDK wrap, login, or Web UI boot occurs
- [x] output and terminal records correlate to the admitted operation
- [x] missing-key and `PI_AI_ERROR` map to portable credential/provider
      failure without leaking secrets
- [x] unsupported semantic records fail or remain explicitly namespaced
- [x] all process exits preserve separate provider, harness, host, runtime,
      and cleanup truth

## Validation

- `effigy validate:focused swallowtail-adapter-deepseek-harness`
- deterministic driver and corpus suites only

## Evidence

- implementation commit: `9bbf4f61`
- exact target-bound discovery, rc6 compatibility claim, bounded NDJSON
  handshake, event decoder, idle fold, activity projection, process-kill
  cancellation, deadline, and cleanup landed
- focused validation: 8 tests passed
- `cargo fmt --all -- --check` passed
- warnings-denied Clippy passed for the package and all targets
- no Python SDK wrap, login, Web UI, ACP, or native JSON-RPC cancel path was
  introduced

## Stop Conditions

- stop if the process service cannot keep stdout protocol-only
- stop if idle fold requires parsing private session files
- stop if cancellation would claim a wire method that does not exist

## Auto-Continuation

Continue to card 220 once exact low-level discovery and execution are ready.
